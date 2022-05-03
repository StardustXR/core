// macro_rules! impl_Coordinates {
// 	($T:ident) => {
// 		impl Coordinates for $T {
// 			fn coordinate(&self) -> (f64, f64) { (self.x, self.y) }
// 		}
// 	}
// }

use crate::flex;
use crate::messenger;

use std::collections::HashMap;
use std::vec::Vec;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
	#[error("invalid path")]
	InvalidPath,
	#[error("node doesn't exist")]
	NodeNotFound,
	#[error("method doesn't exist")]
	MethodNotFound,
}

pub struct Node<'a> {
	path: String,
	trailing_slash_pos: usize,
	messenger: &'a mut messenger::Messenger<'a>,
	local_signals: HashMap<String, Box<dyn Fn(&[u8]) + 'a>>,
	local_methods: HashMap<String, Box<dyn Fn(&[u8]) -> Vec<u8> + 'a>>,
}

impl<'a> Node<'a> {
	pub fn get_name(&self) -> &str {
		&self.path[self.trailing_slash_pos + 1..]
	}
	pub fn get_path(&self) -> &str {
		self.path.as_str()
	}

	pub fn from_path<E>(
		messenger: &'a mut messenger::Messenger<'a>,
		path: String,
	) -> Result<Self, NodeError> {
		Ok(Node {
			path: path.clone(),
			trailing_slash_pos: path.rfind('/').ok_or(NodeError::InvalidPath)?,
			messenger,
			local_signals: HashMap::new(),
			local_methods: HashMap::new(),
		})
	}

	pub fn send_local_signal(&self, method: &str, data: &[u8]) -> Result<(), NodeError> {
		self.local_signals
			.get(method)
			.ok_or(NodeError::MethodNotFound)?(data);
		Ok(())
	}
	pub fn execute_local_method(&self, method: &str, data: &[u8]) -> Result<Vec<u8>, NodeError> {
		Ok(self
			.local_methods
			.get(method)
			.ok_or(NodeError::MethodNotFound)?(data))
	}
	pub fn send_remote_signal(&mut self, method: &str, data: &[u8]) -> Result<(), std::io::Error> {
		self.messenger
			.send_remote_signal(self.path.as_str(), method, data)
	}
	pub fn execute_remote_method(
		&mut self,
		method: &str,
		data: &[u8],
		callback: Box<dyn Fn(&[u8]) + 'a>,
	) -> Result<(), std::io::Error> {
		self.messenger
			.execute_remote_method(self.path.as_str(), method, data, callback)
	}

	fn destroy(&mut self) -> Result<(), std::io::Error> {
		self.send_remote_signal("destroy", &[0; 0])
	}
	fn set_enabled(&mut self, enabled: bool) -> Result<(), std::io::Error> {
		self.send_remote_signal(
			"setEnabled",
			flex::flexbuffer_from_arguments(|fbb| fbb.build_singleton(enabled)).as_slice(),
		)
	}
}
