use super::node::Node;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use stardust_xr::{scenegraph, scenegraph::ScenegraphError};
use std::sync::Weak;

#[derive(Default)]
pub struct Scenegraph {
	nodes: Mutex<FxHashMap<String, Weak<Node>>>,
}

impl Scenegraph {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn add_node(&self, node: Weak<Node>) {
		let node_ref = node.upgrade();
		if node_ref.is_none() {
			return;
		}
		self.nodes
			.lock()
			.insert(String::from(node_ref.unwrap().get_path()), node);
	}

	pub fn remove_node(&self, node: Weak<Node>) {
		let node_ref = node.upgrade();
		if node_ref.is_none() {
			return;
		}
		self.nodes.lock().remove(node_ref.unwrap().get_path());
	}

	pub fn get_node(&self, path: &str) -> Weak<Node> {
		self.nodes.lock().get(path).cloned().unwrap_or_default()
	}
}

impl scenegraph::Scenegraph for Scenegraph {
	fn send_signal(&self, path: &str, method: &str, data: &[u8]) -> Result<(), ScenegraphError> {
		let node = self
			.nodes
			.lock()
			.get(path)
			.and_then(Weak::upgrade)
			.ok_or(ScenegraphError::NodeNotFound)?;
		node.send_local_signal(method, data)
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
		node.execute_local_method(method, data)
	}
}
