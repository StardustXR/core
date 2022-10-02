use mint::RowMatrix4;
use once_cell::sync::OnceCell;
use stardust_xr::values::{Quat, Vec3};
use std::fmt::Debug;

#[derive(Clone)]
pub struct Tip {
	pub origin: Vec3,
	pub orientation: Quat,
	pub radius: f32,
	transform: OnceCell<RowMatrix4<f32>>,
}
impl Tip {
	pub(super) fn new(origin: Vec3, orientation: Quat, radius: f32) -> Self {
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
	pub fn origin(&self) -> Vec3 {
		self.origin
	}
	pub fn orientation(&self) -> Quat {
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
