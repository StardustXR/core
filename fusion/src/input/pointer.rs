use glam::Mat4;
use mint::{Quaternion, RowMatrix4, Vector3};
use once_cell::sync::OnceCell;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Pointer {
	origin: Vector3<f32>,
	orientation: Quaternion<f32>,
	deepest_point: Vector3<f32>,
	transform: OnceCell<RowMatrix4<f32>>,
}
impl Pointer {
	pub(super) fn new(
		origin: Vector3<f32>,
		orientation: Quaternion<f32>,
		deepest_point: Vector3<f32>,
	) -> Self {
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
	pub fn origin(&self) -> Vector3<f32> {
		self.origin
	}
	pub fn orientation(&self) -> Quaternion<f32> {
		self.orientation
	}
	pub fn direction(&self) -> Vector3<f32> {
		let transform: Mat4 = self.transform().into();

		transform
			.transform_vector3(glam::vec3(0.0, 0.0, 1.0))
			.into()
	}
	pub fn deepest_point(&self) -> Vector3<f32> {
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
