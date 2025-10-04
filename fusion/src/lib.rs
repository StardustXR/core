//! A library for Stardust XR clients to use with abstractions over the client, nodes, and event loop.

#![allow(dead_code)]
#![allow(clippy::derivable_impls)]

use std::{error::Error, fmt::Debug, marker::PhantomData, os::fd::OwnedFd};

pub use client::*;
use serde::Serialize;
pub use stardust_xr as core;
pub use stardust_xr::values;
use stardust_xr::{
	messenger::MethodResponse, scenegraph::ScenegraphError, schemas::flex::serialize,
};

use crate::node::NodeError;

#[macro_use]
pub mod node;

pub mod audio;
pub mod client;
pub mod drawable;
pub mod fields;
pub mod input;
pub mod items;
pub mod objects;
mod protocol;
pub mod root;
pub mod scenegraph;
pub mod spatial;
pub mod query_impl;

pub use stardust_xr::schemas::dbus::query as query;
pub use stardust_xr::schemas::dbus::list_query as list_query;
pub use stardust_xr::schemas::impl_queryable_for_proxy;

pub struct TypedMethodResponse<T: Serialize>(pub(crate) MethodResponse, pub(crate) PhantomData<T>);
impl<T: Serialize> TypedMethodResponse<T> {
	pub fn send_ok(self, value: T) {
		self.send(Ok::<T, NodeError>(value))
	}
	pub fn send<E: Error>(self, result: Result<T, E>) {
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
	pub fn wrap<E: Error, F: FnOnce() -> Result<T, E>>(self, f: F) {
		self.send(f())
	}
	pub fn wrap_async<E: Error>(
		self,
		f: impl Future<Output = Result<(T, Vec<OwnedFd>), E>> + Send + 'static,
	) {
		tokio::task::spawn(async move {
			let (value, fds) = match f.await {
				Ok(d) => d,
				Err(e) => {
					self.0.send(Err(ScenegraphError::MemberError {
						error: e.to_string(),
					}));
					return;
				}
			};
			let Ok(serialized) = serialize(value) else {
				self.0.send(Err(ScenegraphError::MemberError {
					error: "Internal: Failed to serialize".to_string(),
				}));
				return;
			};
			self.0.send(Ok((&serialized, fds)));
		});
	}
}
impl<T: Serialize> Debug for TypedMethodResponse<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TypedMethodResponse").finish()
	}
}
