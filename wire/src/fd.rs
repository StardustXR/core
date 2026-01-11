use std::{
	cell::RefCell,
	error::Error,
	os::fd::{AsFd, AsRawFd, IntoRawFd, OwnedFd, RawFd},
};

use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug)]
pub struct ProtocolFd(pub OwnedFd);

pub fn with_fd_serialization_ctx<T, E: Error>(
	func: impl FnOnce() -> Result<T, E>,
) -> Result<(T, Vec<OwnedFd>), E> {
	FD_SERIALIZATION_CTX.set(Some(Vec::new()));
	let v = func();
	let Some(fds) = FD_SERIALIZATION_CTX.with_borrow_mut(|v| v.take()) else {
		unreachable!()
	};
	Ok((v?, fds))
}

pub fn with_fd_deserialization_ctx<T>(
	fds: impl IntoIterator<Item = OwnedFd>,
	func: impl FnOnce() -> T,
) -> T {
	let fds = fds.into_iter().map(Some).collect::<Vec<_>>();
	FD_DESERIALIZATION_CTX.set(Some(fds));
	let v = func();
	let Some(fds) = FD_DESERIALIZATION_CTX.with_borrow_mut(|v| v.take()) else {
		unreachable!()
	};
	let count = fds.into_iter().filter_map(|v| v).count();
	if count != 0 {
		warn!("{count} unused fds during deserialization");
	}
	v
}

impl Serialize for ProtocolFd {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let index = FD_SERIALIZATION_CTX.with_borrow_mut(|ctx| {
			let Some(ctx) = ctx else {
				return Err(<S::Error as serde::ser::Error>::custom(
					"tried to serialize ProtocolFd without fd serialization context",
				));
			};
			ctx.push(self.0.try_clone().unwrap());
			Ok(ctx.len() - 1)
		})?;
		serializer.serialize_u32(index as u32)
	}
}
impl<'de> Deserialize<'de> for ProtocolFd {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let fd = FD_DESERIALIZATION_CTX.with_borrow_mut(|ctx| {
			let Some(ctx) = ctx else {
				return Err(<D::Error as serde::de::Error>::custom(
					"tried to deserialize ProtocolFd without fd deserialization context",
				));
			};
			let index = <u32 as Deserialize>::deserialize(deserializer)?;
			let Some(fd_option) = ctx.get_mut(index as usize) else {
				return Err(<D::Error as serde::de::Error>::custom(
					"tried to deserialize fd not present in the deserialization context",
				));
			};
			let Some(fd) = fd_option.take() else {
				return Err(<D::Error as serde::de::Error>::custom(
					"tried to deserialize the same fd twice, should be impossible",
				));
			};

			Ok(fd)
		})?;
		Ok(Self(fd))
	}
}

thread_local! {
static FD_SERIALIZATION_CTX: RefCell<Option<Vec<OwnedFd>>> = RefCell::new(None);
static FD_DESERIALIZATION_CTX: RefCell<Option<Vec<Option<OwnedFd>>>> = RefCell::new(None);
}

impl AsFd for ProtocolFd {
	fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
		self.0.as_fd()
	}
}
impl AsRawFd for ProtocolFd {
	fn as_raw_fd(&self) -> RawFd {
		self.0.as_raw_fd()
	}
}
impl IntoRawFd for ProtocolFd {
	fn into_raw_fd(self) -> RawFd {
		self.0.into_raw_fd()
	}
}
impl From<OwnedFd> for ProtocolFd {
	fn from(value: OwnedFd) -> Self {
		Self(value)
	}
}
impl From<ProtocolFd> for OwnedFd {
	fn from(value: ProtocolFd) -> OwnedFd {
		value.0
	}
}
