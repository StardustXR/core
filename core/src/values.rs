use mint::{Quaternion, Vector3};
use serde::{Deserialize, Serialize};
use drm_fourcc::{DrmFourcc, DrmModifier};

/// A box
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Box {
	pub center: Vector3<f32>,
	pub size: Vector3<f32>,
}

/// Simple transform
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
	pub position: Option<Vector3<f32>>,
	pub rotation: Option<Quaternion<f32>>,
	pub scale: Option<Vector3<f32>>,
}
impl Transform {
	pub const fn none() -> Self {
		Transform {
			position: None,
			rotation: None,
			scale: None,
		}
	}
	pub const fn identity() -> Self {
		Transform {
			position: Some(Vector3 {
				x: 0.0,
				y: 0.0,
				z: 0.0,
			}),
			rotation: Some(Quaternion {
				v: Vector3 {
					x: 0.0,
					y: 0.0,
					z: 0.0,
				},
				s: 1.0,
			}),
			scale: Some(Vector3 {
				x: 1.0,
				y: 1.0,
				z: 1.0,
			}),
		}
	}

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

#[derive(Serialize, Deserialize)]
pub struct BufferPlaneInfo{
	pub idx: u32,
	pub offset: u32,
	pub stride: u32,
	pub modifier: DrmModifier,
}

#[derive(Serialize, Deserialize)]
pub struct BufferInfo {
	pub size: (u32, u32),
	pub fourcc: DrmFourcc,
	pub flags: u32,
	pub planes: Vec<BufferPlaneInfo>,
}