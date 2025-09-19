//! A library for Stardust XR clients to use with abstractions over the client, nodes, and event loop.

#![allow(dead_code)]
#![allow(clippy::derivable_impls)]

use std::{fmt::Debug, marker::PhantomData, os::fd::OwnedFd};

pub use client::*;
use serde::Serialize;
pub use stardust_xr as core;
pub use stardust_xr::values;
use stardust_xr::{messenger::MethodResponse, scenegraph::ScenegraphError, values::MethodResult};

#[macro_use]
pub mod node;

pub mod audio;
pub mod client;
pub mod drawable;
pub mod fields;
pub mod input;
pub mod objects;
mod protocol;
pub mod root;
pub mod scenegraph;
pub mod spatial;

pub struct TypedMethodResponse<T: Serialize>(pub(crate) MethodResponse, pub(crate) PhantomData<T>);
impl<T: Serialize> TypedMethodResponse<T> {
	pub fn send(self, result: MethodResult<T>) {
		let data = match result {
			Ok(d) => d,
			Err(e) => {
				self.0.send(Err(ScenegraphError::MemberError {
					error: e.to_string(),
				}));
				return;
			}
		};
		let Ok(serialized) = stardust_xr::schemas::flex::serialize(data) else {
			self.0.send(Err(ScenegraphError::MemberError {
				error: "Internal: Failed to serialize".to_string(),
			}));
			return;
		};
		self.0.send(Ok((&serialized, Vec::<OwnedFd>::new())));
	}
	pub fn wrap<F: FnOnce() -> MethodResult<T>>(self, f: F) {
		self.send((f)())
	}
}
impl<T: Serialize> Debug for TypedMethodResponse<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TypedMethodResponse").finish()
	}
}
