use crate::values::{Quat, Vec3};
use glam::Mat4;
use mint::RowMatrix4;
use once_cell::sync::OnceCell;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Pointer {
	origin: Vec3,
	orientation: Quat,
	deepest_point: Vec3,
	transform: OnceCell<RowMatrix4<f32>>,
}
impl Pointer {
	pub(super) fn new(origin: Vec3, orientation: Quat, deepest_point: Vec3) -> Self {
		Self {
			origin,
			orientation,
			deepest_point,
			transform: OnceCell::new(),
		}
	}

	pub fn transform(&self) -> RowMatrix4<f32> {
		*self.transform.get_or_init(|| {
			glam::Mat4::from_rotation_translation(self.orientation.into(), self.origin.into())
				.into()
		})
	}
	pub fn origin(&self) -> Vec3 {
		self.origin
	}
	pub fn orientation(&self) -> Quat {
		self.orientation
	}
	pub fn direction(&self) -> Vec3 {
		let transform: Mat4 = self.transform().into();

		transform
			.transform_vector3(glam::vec3(0.0, 0.0, 1.0))
			.into()
	}
	pub fn deepest_point(&self) -> Vec3 {
		self.deepest_point
	}
}
impl Debug for Pointer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Pointer")
			.field("origin", &self.origin)
			.field("orientation", &self.orientation)
			.field("deepest_point", &self.deepest_point)
			.finish()
	}
}
