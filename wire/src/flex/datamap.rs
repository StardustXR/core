use flexbuffers::{DeserializationError, Reader, ReaderError};
use serde::{Deserialize, Serialize};

/// A map that contains non-spatial data in a map in flexbuffers format.
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Datamap(Vec<u8>);
impl Datamap {
	/// Create a new datamap from a serialized flexbuffer map
	pub fn from_raw(raw: Vec<u8>) -> Result<Self, ReaderError> {
		flexbuffers::Reader::get_root(raw.as_slice())?.get_map()?;
		Ok(Datamap(raw))
	}
	/// Get a temporary reference to the map data inside
	pub fn with_data<F, O>(&self, f: F) -> O
	where
		F: FnOnce(flexbuffers::MapReader<&[u8]>) -> O,
	{
		f(flexbuffers::Reader::get_root(self.0.as_slice())
			.unwrap()
			.get_map()
			.unwrap())
	}
	/// Get a reference to the raw binary data
	pub fn raw(&self) -> &Vec<u8> {
		&self.0
	}

	/// Create a new datamap from a serializable rust struct
	pub fn from_typed<T: Serialize>(typed: T) -> Result<Self, flexbuffers::SerializationError> {
		flexbuffers::to_vec(typed).map(Datamap)
	}
	/// Try to deserialize a rust struct from the flexbuffer inside
	pub fn deserialize<'de, T: Deserialize<'de>>(&'de self) -> Result<T, DeserializationError> {
		flexbuffers::from_slice(&self.0)
	}
}
impl core::fmt::Debug for Datamap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut f = f.debug_struct("Datamap");

		f.field(
			"data",
			&Reader::get_root(self.0.as_slice()).unwrap().to_string(),
		)
		.finish_non_exhaustive()
	}
}
