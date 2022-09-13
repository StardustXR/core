#![allow(unused_imports)]
include!(concat!(env!("OUT_DIR"), "/mod.rs"));

use common::{Quat, QuatT, Vec3, Vec3T};

impl From<mint::Vector3<f32>> for Vec3 {
	fn from(vec: mint::Vector3<f32>) -> Self {
		Vec3::new(vec.x, vec.y, vec.z)
	}
}
impl From<mint::Quaternion<f32>> for Quat {
	fn from(quat: mint::Quaternion<f32>) -> Self {
		Quat::new(quat.v.x, quat.v.y, quat.v.z, quat.s)
	}
}

impl Into<mint::Vector3<f32>> for Vec3T {
	fn into(self) -> mint::Vector3<f32> {
		mint::Vector3::from([self.x, self.y, self.z])
	}
}
impl Into<mint::Quaternion<f32>> for QuatT {
	fn into(self) -> mint::Quaternion<f32> {
		mint::Quaternion::from([self.x, self.y, self.z, self.w])
	}
}
