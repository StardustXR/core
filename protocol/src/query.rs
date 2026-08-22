use std::{error::Error, fmt::Display};

pub use crate::protocol::query::*;

impl Error for QueryableError {}
impl Display for QueryableError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			QueryableError::NotOwnedSpatial => {
				f.write_str("This server doesn't own this SpatialRef!")
			}
			QueryableError::NotOwnedField => f.write_str("This server doesn't own this FieldRef!"),
			QueryableError::DuplicateInterface => {
				f.write_str("This queryable already advertises an interface with that id!")
			}
		}
	}
}
