use mint::{Quaternion, Vector3};
use serde::{Deserialize, Serialize};

/// Simple transform
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
	pub position: Vector3<f32>,
	pub rotation: Quaternion<f32>,
	pub scale: Vector3<f32>,
}
impl Transform {
	pub fn from_position(position: impl Into<Vector3<f32>>) -> Self {
		Transform {
			position: position.into(),
			..Default::default()
		}
	}
	pub fn from_rotation(rotation: impl Into<Quaternion<f32>>) -> Self {
		Transform {
			rotation: rotation.into(),
			..Default::default()
		}
	}
	pub fn from_scale(scale: impl Into<Vector3<f32>>) -> Self {
		Transform {
			scale: scale.into(),
			..Default::default()
		}
	}

	pub fn from_position_rotation(
		position: impl Into<Vector3<f32>>,
		rotation: impl Into<Quaternion<f32>>,
	) -> Self {
		Transform {
			position: position.into(),
			rotation: rotation.into(),
			..Default::default()
		}
	}
	pub fn from_rotation_scale(
		rotation: impl Into<Quaternion<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			rotation: rotation.into(),
			scale: scale.into(),
			..Default::default()
		}
	}

	pub fn from_position_scale(
		position: impl Into<Vector3<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			position: position.into(),
			scale: scale.into(),
			..Default::default()
		}
	}

	pub fn from_position_rotation_scale(
		position: impl Into<Vector3<f32>>,
		rotation: impl Into<Quaternion<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			position: position.into(),
			rotation: rotation.into(),
			scale: scale.into(),
		}
	}
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
