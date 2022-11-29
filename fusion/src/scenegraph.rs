use crate::node::NodeInternals;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use stardust_xr::{scenegraph, scenegraph::ScenegraphError};
use std::sync::{Arc, Weak};

#[derive(Default)]
pub struct Scenegraph {
	nodes: Mutex<FxHashMap<String, Weak<NodeInternals>>>,
}

impl Scenegraph {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn add_node(&self, node_internals: &Arc<NodeInternals>) {
		self.nodes
			.lock()
			.insert(node_internals.path(), Arc::downgrade(&node_internals));
	}

	pub fn remove_node(&self, node_path: &str) {
		self.nodes.lock().remove(node_path);
	}

	// pub fn get_node(&self, path: &str) -> Option<Node> {
	// 	self.nodes.lock().get(path).cloned().unwrap_or_default()
	// }
}

impl scenegraph::Scenegraph for Scenegraph {
	fn send_signal(&self, path: &str, method: &str, data: &[u8]) -> Result<(), ScenegraphError> {
		let node = self
			.nodes
			.lock()
			.get(path)
			.and_then(Weak::upgrade)
			.ok_or(ScenegraphError::NodeNotFound)?;
		let local_signals = node.local_signals.lock();
		let signal = local_signals
			.get(method)
			.ok_or(ScenegraphError::SignalNotFound)?
			.clone();
		signal(data).map_err(|e| ScenegraphError::SignalError { error: e })
	}
	fn execute_method(
		&self,
		path: &str,
		method: &str,
		data: &[u8],
	) -> Result<Vec<u8>, ScenegraphError> {
		let node = self
			.nodes
			.lock()
			.get(path)
			.and_then(Weak::upgrade)
			.ok_or(ScenegraphError::NodeNotFound)?;
		let local_methods = node.local_methods.lock();
		let method = local_methods
			.get(method)
			.ok_or(ScenegraphError::MethodNotFound)?
			.clone();
		method(data).map_err(|e| ScenegraphError::MethodError { error: e })
	}
}
