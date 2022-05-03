use super::node::Node;
use crate::scenegraph;
use crate::scenegraph::ScenegraphError;
use std::{collections::HashMap, rc::Weak};

pub struct Scenegraph<'a> {
	nodes: HashMap<String, Weak<Node<'a>>>,
}

impl<'a> Scenegraph<'a> {
	pub fn new() -> Self {
		Scenegraph {
			nodes: HashMap::new(),
		}
	}
}

impl<'a> scenegraph::Scenegraph for Scenegraph<'a> {
	fn send_signal(&self, path: &str, method: &str, data: &[u8]) -> Result<(), ScenegraphError> {
		self.nodes
			.get(path)
			.ok_or(ScenegraphError::NodeNotFound)?
			.upgrade()
			.ok_or(ScenegraphError::NodeNotFound)?
			.send_local_signal(method, data)
			.map_err(|_| ScenegraphError::MethodNotFound)
	}
	fn execute_method(
		&self,
		path: &str,
		method: &str,
		data: &[u8],
	) -> Result<Vec<u8>, ScenegraphError> {
		self.nodes
			.get(path)
			.ok_or(ScenegraphError::NodeNotFound)?
			.upgrade()
			.ok_or(ScenegraphError::NodeNotFound)?
			.execute_local_method(method, data)
			.map_err(|_| ScenegraphError::MethodNotFound)
	}
}
