use crate::protocol::lines::LinePoint;
use crate::protocol::model::{MaterialParamError, ModelLoadError};
use crate::protocol::types::{Color, Vec3F};
use crate::types::Quatf;
use color::rgba_linear;
use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;

impl Default for LinePoint {
	fn default() -> Self {
		Self {
			point: [0.0; 3].into(),
			thickness: 0.01,
			color: rgba_linear!(1.0, 1.0, 1.0, 1.0),
		}
	}
}

impl Hash for LinePoint {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.color.c.r.to_bits().hash(state);
		self.color.c.g.to_bits().hash(state);
		self.color.c.b.to_bits().hash(state);
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
			ModelLoadError::InvalidSpatial => f.write_str("Invalid Spatial used for Model"),
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
