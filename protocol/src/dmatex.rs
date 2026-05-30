pub use crate::protocol::dmatex::*;
use std::{error::Error, fmt::Display};

impl Error for DmatexImportError {}
impl Display for DmatexImportError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DmatexImportError::InvalidSize => f.write_str("Invalid size"),
			DmatexImportError::InvalidFormat => f.write_str("Invalid format"),
			DmatexImportError::UnsupportedArrayLayers {
				max_supported_layers,
			} => f.write_str(&format!(
				"Unsupported amount of array layers, max supported layers: {max_supported_layers}"
			)),
			DmatexImportError::InvalidPlanes => f.write_str("Invalid planes"),
			DmatexImportError::InvalidTimelineFd => f.write_str("Invalide timeline syncobj"),
		}
	}
}
