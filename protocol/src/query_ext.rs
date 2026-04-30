use std::{error::Error, fmt::Display};

use crate::query::QueryableError;

impl Error for QueryableError {}
impl Display for QueryableError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			QueryableError::InvalidField => f.write_str("Invalid FieldRef, not owned by server"),
		}
	}
}
