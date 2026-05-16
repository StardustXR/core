use crate::protocol::spatial::{PartialTransform, Transform};
use mint::{Quaternion, Vector3};

impl Transform {
	pub const IDENTITY: Transform = Transform {
		translation: Vector3 {
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
	};

	pub fn from_translation(translation: impl Into<Vector3<f32>>) -> Self {
		Transform {
			translation: translation.into(),
			..Self::IDENTITY
		}
	}
	pub fn from_rotation(rotation: impl Into<Quaternion<f32>>) -> Self {
		Transform {
			rotation: rotation.into(),
			..Self::IDENTITY
		}
	}
	pub fn from_scale(scale: impl Into<Vector3<f32>>) -> Self {
		Transform {
			scale: scale.into(),
			..Self::IDENTITY
		}
	}
	pub fn from_translation_rotation(
		translation: impl Into<Vector3<f32>>,
		rotation: impl Into<Quaternion<f32>>,
	) -> Self {
		Transform {
			translation: translation.into(),
			rotation: rotation.into(),
			..Self::IDENTITY
		}
	}
	pub fn from_translation_scale(
		translation: impl Into<Vector3<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			translation: translation.into(),
			scale: scale.into(),
			..Self::IDENTITY
		}
	}
	pub fn from_rotation_scale(
		rotation: impl Into<Quaternion<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			rotation: rotation.into(),
			scale: scale.into(),
			..Self::IDENTITY
		}
	}
	pub fn from_translation_rotation_scale(
		translation: impl Into<Vector3<f32>>,
		rotation: impl Into<Quaternion<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			translation: translation.into(),
			rotation: rotation.into(),
			scale: scale.into(),
		}
	}
}

impl PartialTransform {
	pub const NONE: PartialTransform = PartialTransform {
		translation: None,
		rotation: None,
		scale: None,
	};

	pub fn from_translation(translation: impl Into<Vector3<f32>>) -> Self {
		PartialTransform {
			translation: Some(translation.into()),
			..Self::NONE
		}
	}
	pub fn from_rotation(rotation: impl Into<Quaternion<f32>>) -> Self {
		PartialTransform {
			rotation: Some(rotation.into()),
			..Self::NONE
		}
	}
	pub fn from_scale(scale: impl Into<Vector3<f32>>) -> Self {
		PartialTransform {
			scale: Some(scale.into()),
			..Self::NONE
		}
	}
	pub fn from_translation_rotation(
		translation: impl Into<Vector3<f32>>,
		rotation: impl Into<Quaternion<f32>>,
	) -> Self {
		PartialTransform {
			translation: Some(translation.into()),
			rotation: Some(rotation.into()),
			..Self::NONE
		}
	}
	pub fn from_translation_scale(
		translation: impl Into<Vector3<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		PartialTransform {
			translation: Some(translation.into()),
			scale: Some(scale.into()),
			..Self::NONE
		}
	}
	pub fn from_rotation_scale(
		rotation: impl Into<Quaternion<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		PartialTransform {
			rotation: Some(rotation.into()),
			scale: Some(scale.into()),
			..Self::NONE
		}
	}
	pub fn from_translation_rotation_scale(
		translation: impl Into<Vector3<f32>>,
		rotation: impl Into<Quaternion<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		PartialTransform {
			translation: Some(translation.into()),
			rotation: Some(rotation.into()),
			scale: Some(scale.into()),
		}
	}
}

impl From<Transform> for PartialTransform {
	fn from(t: Transform) -> Self {
		PartialTransform {
			translation: Some(t.translation),
			rotation: Some(t.rotation),
			scale: Some(t.scale),
		}
	}
}
