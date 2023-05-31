use mint::{Quaternion, Vector3};

use crate::input_pointer::PointerT;

/// 3D pointer with extra information due to ray marching.
#[derive(Debug, Clone, Copy)]
pub struct Pointer {
	pub origin: Vector3<f32>,
	pub orientation: Quaternion<f32>,
	/// The point that is the most inside the input handler's field.
	/// Useful for telling how close to the center it's pointing or for thin objects can take the place of a point of intersection.
	pub deepest_point: Vector3<f32>,
}
impl Pointer {
	pub fn direction(&self) -> Vector3<f32> {
		let transform =
			glam::Mat4::from_rotation_translation(self.orientation.into(), self.origin.into());

		transform
			.transform_vector3(glam::vec3(0.0, 0.0, -1.0))
			.into()
	}
}
impl Default for Pointer {
	fn default() -> Self {
		Self {
			origin: Vector3::from([0.0; 3]),
			orientation: Quaternion::from([0.0, 0.0, 0.0, 1.0]),
			deepest_point: Vector3::from([0.0; 3]),
		}
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
