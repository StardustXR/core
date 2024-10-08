//! A library for Stardust XR clients to use with abstractions over the client, nodes, and event loop.

#![allow(dead_code)]
#![allow(clippy::derivable_impls)]

pub use stardust_xr as core;
pub use stardust_xr::values;

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

use color_eyre::eyre::{anyhow, Result};
use node::{NodeError, NodeType};
pub use parking_lot::{Mutex, MutexGuard};
use std::{os::fd::OwnedFd, sync::Arc};

/// A wrapper around a node and a handler struct implementing the node's handler trait.
/// Necessary because the methods on the handler may be called at any time and bundling the 2 together makes it harder to screw up.
/// Can't be created directly, nodes that could use handlers have a `wrap()` and `wrap_raw()` method on them that consumes them and a handler and returns a `HandlerWrapper`.

#[derive(Debug)]
pub struct HandlerWrapper<N: NodeType, H: Send + Sync + 'static> {
	node: Arc<N>,
	wrapped: Arc<Mutex<H>>,
}
impl<N: NodeType, H: Send + Sync + 'static> HandlerWrapper<N, H> {
	pub(crate) fn new_raw(node: N, handler: Arc<Mutex<H>>) -> Self {
		Self {
			wrapped: handler,
			node: Arc::new(node),
		}
	}

	/// Get a reference to the node inside
	pub fn node(&self) -> &Arc<N> {
		&self.node
	}
	/// Convenience function to get the handler inside.
	///
	/// # Safety
	/// Since this is a mutex, it can deadlock.
	pub fn lock_wrapped(&self) -> MutexGuard<H> {
		self.wrapped.lock()
	}
	/// Get an `Arc<Mutex<_>>` of the handleNamespacedResourced type for portability.
	///
	/// # Safety
	/// Since this is a mutex, it can deadlock.
	pub fn wrapped(&self) -> &Arc<Mutex<H>> {
		&self.wrapped
	}

	#[allow(clippy::type_complexity)]
	pub(crate) fn add_handled_signal(
		&self,
		id: u64,
		parse: fn(Arc<N>, Arc<Mutex<H>>, &[u8], Vec<OwnedFd>) -> Result<()>,
	) -> Result<(), NodeError> {
		let node = Arc::downgrade(&self.node);
		let handler = Arc::downgrade(&self.wrapped);
		self.node.node().add_local_signal(id, move |data, fds| {
			let Some(node) = node.upgrade() else {
				return Err(anyhow!("Node broken"));
			};
			let Some(handler) = handler.upgrade() else {
				return Err(anyhow!("Handler broken"));
			};
			parse(node, handler, data, fds)
		})
	}
	#[allow(clippy::type_complexity)]
	pub(crate) fn add_handled_method(
		&self,
		id: u64,
		parse: fn(Arc<N>, Arc<Mutex<H>>, &[u8], Vec<OwnedFd>) -> Result<(Vec<u8>, Vec<OwnedFd>)>,
	) -> Result<(), NodeError> {
		let node = Arc::downgrade(&self.node);
		let handler = Arc::downgrade(&self.wrapped);
		self.node.node().add_local_method(id, move |data, fds| {
			let Some(node) = node.upgrade() else {
				return Err(anyhow!("Node broken"));
			};
			let Some(handler) = handler.upgrade() else {
				return Err(anyhow!("Handler broken"));
			};
			parse(node, handler, data, fds)
		})
	}
}

#[macro_export]
macro_rules! impl_aspects {
    ($node:ident: $( $aspect:ident ),+) => {
		$(impl $aspect for $node {})+
    }
}

#[macro_export]
macro_rules! handle_action {
    ($handler:ident, $action:ident) => {
        $handler
            .add_handled_signal(stringify!($action), |_, handler, _, _| {
                handler.lock().$action();  // No data deserialization
                Ok(())
            })
            .unwrap();
    };

    ($handler:ident, $action:ident, $name:ident) => {
        $handler
            .add_handled_signal(stringify!($action), |_, handler, data, _| {
                handler.lock().$action(stardust_xr::schemas::flex::deserialize(data)?);
                Ok(())
            })
            .unwrap();
    };

    ($handler:ident, $action:ident, ($( $name:ident ),*)) => {
        $handler
            .add_handled_signal(stringify!($action), |_, handler, data, _| {
                let ($($name),*,) = stardust_xr::schemas::flex::deserialize(data)?;
                handler.lock().$action($(
                    $name
                ),*);
                Ok(())
            })
            .unwrap();
    };
}
