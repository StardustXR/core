//! A library for Stardust XR clients to use with abstractions over the client, nodes, and event loop.

#![allow(dead_code)]
#![allow(clippy::derivable_impls)]

use std::{fmt::Debug, marker::PhantomData, os::fd::OwnedFd};

pub use client::*;
use serde::Serialize;
pub use stardust_xr as core;
pub use stardust_xr::values;
use stardust_xr::{
	scenegraph::{MethodResponse, ScenegraphError},
	values::MethodResult,
};

#[macro_use]
pub mod node;

pub mod audio;
pub mod client;
pub mod data;
pub mod drawable;
pub mod fields;
pub mod input;
pub mod items;
pub mod objects;
pub mod root;
mod scenegraph;
pub mod spatial;

pub struct TypedMethodResponse<T: Serialize>(pub(crate) MethodResponse, pub(crate) PhantomData<T>);
impl<T: Serialize> TypedMethodResponse<T> {
	pub fn send(self, result: MethodResult<T>) {
		let data = match result {
			Ok(d) => d,
			Err(e) => {
				let _ = self.0.send(Err(ScenegraphError::MemberError {
					error: e.to_string(),
				}));
				return;
			}
		};
		let Ok(serialized) = stardust_xr::schemas::flex::serialize(data) else {
			let _ = self.0.send(Err(ScenegraphError::MemberError {
				error: "Internal: Failed to serialize".to_string(),
			}));
			return;
		};
		let _ = self.0.send(Ok((serialized, Vec::<OwnedFd>::new())));
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

#[macro_export]
macro_rules! impl_aspects {
    ($node:ident: $( $aspect:ident ),+) => {
		$(impl $aspect for $node {})+
    }
}
