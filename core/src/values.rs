use mint::{Quaternion, Vector3};
use serde::{Deserialize, Serialize};

/// Simple transform
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
	pub position: Option<Vector3<f32>>,
	pub rotation: Option<Quaternion<f32>>,
	pub scale: Option<Vector3<f32>>,
}
impl Transform {
	pub fn from_position(position: impl Into<Vector3<f32>>) -> Self {
		Transform {
			position: Some(position.into()),
			..Default::default()
		}
	}
	pub fn from_rotation(rotation: impl Into<Quaternion<f32>>) -> Self {
		Transform {
			rotation: Some(rotation.into()),
			..Default::default()
		}
	}
	pub fn from_scale(scale: impl Into<Vector3<f32>>) -> Self {
		Transform {
			scale: Some(scale.into()),
			..Default::default()
		}
	}

	pub fn from_position_rotation(
		position: impl Into<Vector3<f32>>,
		rotation: impl Into<Quaternion<f32>>,
	) -> Self {
		Transform {
			position: Some(position.into()),
			rotation: Some(rotation.into()),
			..Default::default()
		}
	}
	pub fn from_rotation_scale(
		rotation: impl Into<Quaternion<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			rotation: Some(rotation.into()),
			scale: Some(scale.into()),
			..Default::default()
		}
	}

	pub fn from_position_scale(
		position: impl Into<Vector3<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			position: Some(position.into()),
			scale: Some(scale.into()),
			..Default::default()
		}
	}

	pub fn from_position_rotation_scale(
		position: impl Into<Vector3<f32>>,
		rotation: impl Into<Quaternion<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			position: Some(position.into()),
			rotation: Some(rotation.into()),
			scale: Some(scale.into()),
		}
	}
}
