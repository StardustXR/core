use crate::model::PartialNonUniformTransform;
use crate::protocol::lines::LinePoint;
use crate::protocol::model::{MaterialParamError, ModelLoadError};
use crate::protocol::types::{Color, Vec3F};
use crate::types::Quatf;
use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;

impl Default for LinePoint {
	fn default() -> Self {
		Self {
			point: Vec3F {
				x: 0.0,
				y: 0.0,
				z: 0.0,
			},
			thickness: 0.01,
			color: Color::WHITE,
		}
	}
}

impl Hash for LinePoint {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.color.r.to_bits().hash(state);
		self.color.g.to_bits().hash(state);
		self.color.b.to_bits().hash(state);
		self.color.a.to_bits().hash(state);

		self.point.x.to_bits().hash(state);
		self.point.y.to_bits().hash(state);
		self.point.z.to_bits().hash(state);

		self.thickness.to_bits().hash(state);
	}
}

impl Error for ModelLoadError {}
impl Display for ModelLoadError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ModelLoadError::NotFound => f.write_str("Model resource not found"),
		}
	}
}
impl Error for MaterialParamError {}
impl Display for MaterialParamError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			MaterialParamError::ParamNotFound { known_params } => {
				if let Some(params) = known_params
					.iter()
					.cloned()
					.reduce(|a, b| format!("{a}, {b}"))
				{
					write!(f, "Unknown parameter, known parameters are: {}", params)
				} else {
					f.write_str("Unknown parameter.")
				}
			}
			MaterialParamError::IncorrectType { valid_type } => {
				write!(
					f,
					"Incorrect parameter type, correct type is: {}",
					valid_type
				)
			}
			MaterialParamError::Holdout => f.write_str("Holdout material was applied previously"),
		}
	}
}
impl PartialNonUniformTransform {
    pub const NONE: PartialNonUniformTransform = PartialNonUniformTransform {
        translation: None,
        rotation: None,
        scale: None,
    };

    pub fn from_position(position: impl Into<Vec3F>) -> Self {
        PartialNonUniformTransform {
            translation: Some(position.into()),
            ..Self::NONE
        }
    }
    pub fn from_rotation(rotation: impl Into<Quatf>) -> Self {
        PartialNonUniformTransform {
            rotation: Some(rotation.into()),
            ..Self::NONE
        }
    }
    pub fn from_scale(scale: impl Into<Vec3F>) -> Self {
        PartialNonUniformTransform {
            scale: Some(scale.into()),
            ..Self::NONE
        }
    }
    pub fn from_position_rotation(
        position: impl Into<Vec3F>,
        rotation: impl Into<Quatf>,
    ) -> Self {
        PartialNonUniformTransform {
            translation: Some(position.into()),
            rotation: Some(rotation.into()),
            ..Self::NONE
        }
    }
    pub fn from_position_scale(position: impl Into<Vec3F>, scale: impl Into<Vec3F>) -> Self {
        PartialNonUniformTransform {
            translation: Some(position.into()),
            scale: Some(scale.into()),
            ..Self::NONE
        }
    }
    pub fn from_rotation_scale(rotation: impl Into<Quatf>, scale: impl Into<Vec3F>) -> Self {
        PartialNonUniformTransform {
            rotation: Some(rotation.into()),
            scale: Some(scale.into()),
            ..Self::NONE
        }
    }
    pub fn from_position_rotation_scale(
        position: impl Into<Vec3F>,
        rotation: impl Into<Quatf>,
        scale: impl Into<Vec3F>,
    ) -> Self {
        PartialNonUniformTransform {
            translation: Some(position.into()),
            rotation: Some(rotation.into()),
            scale: Some(scale.into()),
        }
    }
}
