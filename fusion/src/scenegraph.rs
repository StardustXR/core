use crate::node::NodeInternals;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use stardust_xr::scenegraph::{self, ScenegraphError};
use std::{
	os::fd::OwnedFd,
	sync::{Arc, Weak},
};
use tokio::sync::oneshot;

/// Scenegraph full of aliases to nodes, needed so the `Messenger` can send messages to nodes.
#[derive(Default)]
pub struct Scenegraph {
	nodes: Mutex<FxHashMap<u64, Weak<NodeInternals>>>,
}

impl Scenegraph {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn add_node(&self, node_internals: &Arc<NodeInternals>) {
		self.nodes
			.lock()
			.entry(node_internals.id)
			.or_insert(Arc::downgrade(node_internals));
	}

	pub fn remove_node(&self, id: u64) {
		self.nodes.lock().remove(&id);
	}

	// pub fn get_node(&self, path: &str) -> Option<Node> {
	// 	self.nodes.lock().get(path).cloned().unwrap_or_default()
	// }
}

impl scenegraph::Scenegraph for Scenegraph {
	fn send_signal(
		&self,
		id: u64,
		method: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), ScenegraphError> {
		let node = self
			.nodes
			.lock()
			.get(&id)
			.and_then(Weak::upgrade)
			.ok_or(ScenegraphError::NodeNotFound)?;
		let local_signals = node.local_signals.lock();
		let signal = local_signals
			.get(&method)
			.ok_or(ScenegraphError::SignalNotFound)?
			.clone();
		signal(data, fds).map_err(|e| ScenegraphError::SignalError {
			error: e.to_string(),
		})
	}
	fn execute_method(
		&self,
		id: u64,
		method: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
		response: oneshot::Sender<Result<(Vec<u8>, Vec<OwnedFd>), ScenegraphError>>,
	) {
		let method_method = || {
			let node = self
				.nodes
				.lock()
				.get(&id)
				.and_then(Weak::upgrade)
				.ok_or(ScenegraphError::NodeNotFound)?;
			let local_methods = node.local_methods.lock();
			let method = local_methods
				.get(&method)
				.ok_or(ScenegraphError::MethodNotFound)?
				.clone();
			method(data, fds).map_err(|e| ScenegraphError::MethodError {
				error: e.to_string(),
			})
		};

		let _ = response.send(method_method());
	}
}
