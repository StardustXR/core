// macro_rules! impl_Coordinates {
// 	($T:ident) => {
// 		impl Coordinates for $T {
// 			fn coordinate(&self) -> (f64, f64) { (self.x, self.y) }
// 		}
// 	}
// }

use crate::flex;
use crate::messenger;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
	#[error("invalid path")]
	InvalidPath,
}

pub struct Node<'a> {
	path: String,
	trailing_slash_pos: usize,
	messenger: &'a mut messenger::Messenger,
}

impl<'a> Node<'a> {
	pub fn get_name(&self) -> &str {
		&self.path[self.trailing_slash_pos + 1..]
	}
	pub fn get_path(&self) -> &str {
		self.path.as_str()
	}

	pub fn from_path<E>(
		messenger: &'a mut messenger::Messenger,
		path: String,
	) -> Result<Self, NodeError> {
		Ok(Node {
			path: path.clone(),
			trailing_slash_pos: path.rfind('/').ok_or(NodeError::InvalidPath)?,
			messenger,
		})
	}

	pub fn send_signal(&mut self, method: &str, data: &[u8]) -> Result<(), std::io::Error> {
		self.messenger.send_signal(self.path.as_str(), method, data)
	}
	pub fn execute_remote_method(
		&mut self,
		method: &str,
		data: &[u8],
		callback: messenger::RawCallback,
	) -> Result<(), std::io::Error> {
		self.messenger
			.execute_remote_method(self.path.as_str(), method, data, callback)
	}

	fn destroy(&mut self) -> Result<(), std::io::Error> {
		self.send_signal("destroy", &[0; 0])
	}
	fn set_enabled(&mut self, enabled: bool) -> Result<(), std::io::Error> {
		self.send_signal(
			"setEnabled",
			flex::flexbuffer_from_arguments(|fbb| fbb.build_singleton(enabled)).as_slice(),
		)
	}
}
