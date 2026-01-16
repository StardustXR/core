//! A library for Stardust XR clients to use with abstractions over the client, nodes, and event loop.

#![allow(dead_code)]
#![allow(clippy::derivable_impls)]

use serde::Serialize;
use stardust_xr_wire::{flex::serialize, messenger::MethodResponse, scenegraph::ScenegraphError};
use std::{error::Error, fmt::Debug, marker::PhantomData};

pub use client::*;
pub use stardust_xr_gluon::*;
pub use stardust_xr_wire::values;

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
pub mod query_impl;
pub mod root;
pub mod scenegraph;
pub mod spatial;
pub mod camera;

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
		let Ok((serialized, fds)) = stardust_xr_wire::flex::serialize(data) else {
			self.0.send(Err(ScenegraphError::MemberError {
				error: "Internal: Failed to serialize".to_string(),
			}));
			return;
		};
		self.0.send(Ok((&serialized, fds)));
	}
	pub fn wrap<E: Error, F: FnOnce() -> Result<T, E>>(self, f: F) {
		self.send(f())
	}
	pub fn wrap_async<E: Error>(self, f: impl Future<Output = Result<T, E>> + Send + 'static) {
		tokio::task::spawn(async move {
			let value = match f.await {
				Ok(d) => d,
				Err(e) => {
					self.0.send(Err(ScenegraphError::MemberError {
						error: e.to_string(),
					}));
					return;
				}
			};
			let Ok((serialized, fds)) = serialize(value) else {
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
