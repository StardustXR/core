use mint::{Quaternion, RowMatrix4, Vector3};
use once_cell::sync::OnceCell;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Tip {
	pub origin: Vector3<f32>,
	pub orientation: Quaternion<f32>,
	pub radius: f32,
	transform: OnceCell<RowMatrix4<f32>>,
}
impl Tip {
	pub(super) fn new(origin: Vector3<f32>, orientation: Quaternion<f32>, radius: f32) -> Self {
		Self {
			origin,
			orientation,
			radius,
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
	pub fn radius(&self) -> f32 {
		self.radius
	}
}
impl Debug for Tip {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Tip")
			.field("origin", &self.origin)
			.field("orientation", &self.orientation)
			.field("radius", &self.radius)
			.finish()
	}
}
