use super::node::{Node, NodeError};
use crate::scenegraph;
use std::collections::HashMap;

pub struct Scenegraph<'a> {
	nodes: HashMap<String, &'a Node<'a>>,
}

impl<'a> scenegraph::Scenegraph for Scenegraph<'a> {
	fn send_signal(&self, path: &str, method: &str, data: &[u8]) {
		self.nodes
			.get(path)
			.unwrap()
			.send_local_signal(method, data)
			.unwrap()
	}
	fn execute_method(&self, path: &str, method: &str, data: &[u8]) -> Vec<u8> {
		self.nodes
			.get(path)
			.unwrap()
			.execute_local_method(method, data)
			.unwrap()
	}
}

