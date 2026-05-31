use std::{error::Error, fmt::Display};

pub use crate::protocol::spatial_query::*;

impl Error for QueryError {}
impl Display for QueryError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			QueryError::InvalidRef => f.write_str("Invalid Ref, the server doesn't own a SpatialRef or FieldRef!"),
			QueryError::NoRequiredInterfaces => f.write_str("At least one required interface is required"),
		}
	}
}
