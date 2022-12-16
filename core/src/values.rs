use mint::{Quaternion, Vector3};
use serde::{Deserialize, Serialize};

/// Simple transform
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Quaternion<f32>,
	pub scale: Vector3<f32>,
}
impl Default for Transform {
	fn default() -> Self {
		Self {
			position: Vector3 {
				x: 0.0,
				y: 0.0,
				z: 0.0,
			},
			rotation: Quaternion {
				v: Vector3 {
					x: 0.0,
					y: 0.0,
					z: 0.0,
				},
				s: 1.0,
			},
			scale: Vector3 {
				x: 1.0,
				y: 1.0,
				z: 1.0,
			},
		}
	}
}
