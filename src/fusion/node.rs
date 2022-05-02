// macro_rules! impl_Coordinates {
// 	($T:ident) => {
// 		impl Coordinates for $T {
// 			fn coordinate(&self) -> (f64, f64) { (self.x, self.y) }
// 		}
// 	}
// }

use std::path::{Path, PathBuf};

pub struct Node<'a> {
	path: PathBuf,
	messenger: &'a mut messenger::Messenger,
}

impl<'a> Node<'a> {
	pub fn send_signal(&mut self, method: &str, data: &[u8]) -> Result<()> {
		self.messenger.send_signal(self.path, method, data)
	}
	pub fn execute_remote_method(
		&mut self,
		object: &str,
		method: &str,
		data: &[u8],
		callback: messenger::RawCallback
	) -> Result<()> {
		self.messenger
			.execute_remote_method(self.path, method, data, callback)
	}

	fn get_path_as_string(&self) -> Option<String> {
		self.get_path().to_str().map(|s| s.to_owned())
	}

	fn destroy(&mut self) {
		let path = self.get_path_as_string().unwrap();
		self.get_messenger().send_signal(&path, "destroy", &[0; 0]);
	}
	fn set_enabled(&mut self, enabled: bool) {
		let path = self.get_path_as_string().unwrap();
		self.get_messenger().send_signal(
			&path,
			"setEnabled",
			flex::flexbuffer_from_arguments(|fbb| fbb.build_singleton(enabled)).as_slice(),
		);
	}
}

pub struct Spatial<'a> {
	node: &'a Node<'a>,
}

impl<'a> Spatial<'a> {
	fn get_transform(
		&self,
		space: &Spatial,
		callback: &impl Fn(values::Vec3, values::Quat, values::Vec3),
	) {
		// self.node.messenger
	}
	fn set_transform(
		&self,
		space: &Spatial,
		position: Option<values::Vec3>,
		rotation: Option<values::Quat>,
		scale: Option<values::Vec3>,
	) {
	}
}
