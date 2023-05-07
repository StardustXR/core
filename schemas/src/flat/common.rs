use super::generated::common::{QuatT, Vec3T};
use mint::{Quaternion, Vector3};

impl From<Vector3<f32>> for Vec3T {
	fn from(vec: Vector3<f32>) -> Self {
		Vec3T {
			x: vec.x,
			y: vec.y,
			z: vec.z,
		}
	}
}
#[allow(clippy::from_over_into)]
impl Into<Vector3<f32>> for Vec3T {
	fn into(self) -> Vector3<f32> {
		Vector3::from([self.x, self.y, self.z])
	}
}

impl From<Quaternion<f32>> for QuatT {
	fn from(quat: Quaternion<f32>) -> Self {
		QuatT {
			x: quat.v.x,
			y: quat.v.y,
			z: quat.v.z,
			w: quat.s,
		}
	}
}
#[allow(clippy::from_over_into)]
impl Into<Quaternion<f32>> for QuatT {
	fn into(self) -> Quaternion<f32> {
		Quaternion::from([self.x, self.y, self.z, self.w])
	}
}
