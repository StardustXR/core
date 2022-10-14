use mint::{Quaternion, Vector3};

use crate::input_pointer::PointerT;

#[derive(Debug, Clone, Copy)]
pub struct Pointer {
	pub origin: Vector3<f32>,
	pub orientation: Quaternion<f32>,
	pub deepest_point: Vector3<f32>,
}
impl Pointer {
	pub fn direction(&self) -> Vector3<f32> {
		let transform =
			glam::Mat4::from_rotation_translation(self.orientation.into(), self.origin.into());

		transform
			.transform_vector3(glam::vec3(0.0, 0.0, 1.0))
			.into()
	}
}

impl From<PointerT> for Pointer {
	fn from(pointer: PointerT) -> Self {
		Pointer {
			origin: pointer.origin.into(),
			orientation: pointer.orientation.into(),
			deepest_point: pointer.deepest_point.into(),
		}
	}
}
impl From<Pointer> for PointerT {
	fn from(pointer: Pointer) -> Self {
		PointerT {
			origin: pointer.origin.into(),
			orientation: pointer.orientation.into(),
			deepest_point: pointer.deepest_point.into(),
		}
	}
}
