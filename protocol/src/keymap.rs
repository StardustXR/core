use std::{error::Error, fmt::Display};

pub use crate::protocol::keymap::*;

impl Display for KeymapExchangeError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			KeymapExchangeError::InvalidKeymap => f.write_str("Invalid Keymap"),
		}
	}
}
impl Error for KeymapExchangeError {}
