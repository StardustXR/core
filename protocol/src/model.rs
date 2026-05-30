use crate::types::{Color, QuatF, Vec3F};
use color::rgba_linear;
use std::error::Error;
use std::fmt::Display;
use std::hash::Hash;

pub use crate::protocol::model::*;

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
