// macro_rules! impl_Coordinates {
// 	($T:ident) => {
// 		impl Coordinates for $T {
// 			fn coordinate(&self) -> (f64, f64) { (self.x, self.y) }
// 		}
// 	}
// }

use std::path::{Path, PathBuf};
#[path = "../flex.rs"]
mod flex;
#[path = "../messenger.rs"]
mod messenger;
#[path = "values.rs"]
mod values;

// macro_rules! node_trait {
// 	($T:ident, $N:ident) => {
// 		impl $T for $N {
// 			fn get_path(&self) -> &Path {
// 				self.path.as_path()
// 			}
// 			fn get_messenger<'a>(&self) -> &'a messenger::Messenger {
// 				self.messenger
// 			}
// 		}
// 	};
// }

pub struct Node<'a> {
	path: PathBuf,
	messenger: &'a mut messenger::Messenger,
}

pub trait NodeBase {
	fn get_path(&self) -> &Path;
	fn get_messenger(&mut self) -> &mut messenger::Messenger;

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

impl<'a> NodeBase for Node<'a> {
	fn get_path(&self) -> &Path {
		self.path.as_path()
	}
	fn get_messenger(&mut self) -> &mut messenger::Messenger {
		self.messenger
	}
}
// node_trait!(NodeBase, Node);

pub trait Spatial {
	fn get_transform(&self, space: &dyn Spatial) -> (values::Vec3, values::Quat, values::Vec3);
	fn set_transform(
		&self,
		space: &dyn Spatial,
		position: Option<values::Vec3>,
		rotation: Option<values::Quat>,
		scale: Option<values::Vec3>,
	);
}
