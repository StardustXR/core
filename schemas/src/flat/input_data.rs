use crate::generated::{
	self,
	input::{InputDataRawT, InputDataT},
};
use ouroboros::self_referencing;
use std::{convert::TryFrom, fmt::Debug, hash::Hash};

/// A map that contains non-spatial data associated with the input in flexbuffers format.
pub struct Datamap(DatamapInner);
impl Datamap {
	/// Create a new datamap from a serialized flexbuffer map.
	pub fn new(raw: Vec<u8>) -> Result<Self, flexbuffers::ReaderError> {
		Ok(Datamap(DatamapInner::try_new(raw, |raw| {
			flexbuffers::Reader::get_root(raw.as_slice())?.get_map()
		})?))
	}

	/// Get a temporary reference to the map data inside.
	pub fn with_data<F, O>(&self, f: F) -> O
	where
		F: FnOnce(&flexbuffers::MapReader<&[u8]>) -> O,
	{
		self.0.with_reader(f)
	}
}
impl Debug for Datamap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut f = f.debug_struct("Datamap");

		f.field(
			"raw",
			&self.with_data(|datamap| {
				datamap
					.iter_keys()
					.zip(datamap.iter_values())
					.map(|(key, value)| (key.to_string(), value.to_string()))
					.collect::<Vec<_>>()
			}),
		)
		.finish_non_exhaustive()
	}
}

#[self_referencing]
struct DatamapInner {
	raw: Vec<u8>,

	#[borrows(raw)]
	#[not_covariant]
	pub reader: flexbuffers::MapReader<&'this [u8]>,
}
impl Clone for Datamap {
	fn clone(&self) -> Self {
		Self::new(self.0.borrow_raw().clone()).unwrap()
	}
}

/// Input data object struct.
#[derive(Debug, Clone)]
pub struct InputData {
	/// Used to uniquely identify the input method so state can be tracked across input events.
	pub uid: String,
	/// All vectors and quaternions are relative to the input handler if deserialized.
	pub input: InputDataType,
	/// Closest distance from the input handler to the field.
	pub distance: f32,
	/// Non-spatial data in a map.
	pub datamap: Datamap,
}
impl InputData {
	pub fn deserialize(data: &[u8]) -> Result<InputData, String> {
		let input = generated::input::root_as_input_data(data)
			.map_err(|_| "Input data is invalid".to_string())?
			.unpack();
		let datamap = input.datamap.ok_or_else(|| "No datamap!".to_string())?;
		Ok(InputData {
			uid: input.uid,
			input: match input.input {
				InputDataRawT::Pointer(pointer) => InputDataType::Pointer((*pointer).into()),
				InputDataRawT::Hand(hand) => InputDataType::Hand(Box::new((*hand).into())),
				InputDataRawT::Tip(tip) => InputDataType::Tip((*tip).into()),
				_ => return Err("Invalid input type".to_string()),
			},
			distance: input.distance,
			datamap: Datamap::new(datamap).map_err(|e| e.to_string())?,
		})
	}
	pub fn serialize(&self) -> Vec<u8> {
		let input_data = InputDataT {
			uid: self.uid.clone(),
			input: match self.input.clone() {
				InputDataType::Pointer(p) => InputDataRawT::Pointer(Box::new(p.into())),
				InputDataType::Hand(h) => InputDataRawT::Hand(Box::new((*h).into())),
				InputDataType::Tip(t) => InputDataRawT::Tip(Box::new(t.into())),
			},
			distance: self.distance,
			datamap: Some(self.datamap.0.borrow_raw().clone()),
		};

		let mut fbb = flatbuffers::FlatBufferBuilder::new();
		let offset = input_data.pack(&mut fbb);
		fbb.finish_minimal(offset);

		fbb.finished_data().to_vec()
	}
}
impl Hash for InputData {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.uid.hash(state);
	}
}
impl PartialEq for InputData {
	fn eq(&self, other: &Self) -> bool {
		self.uid == other.uid
	}
}
impl Eq for InputData {}

#[derive(Debug, Clone)]
pub enum InputDataType {
	Pointer(super::pointer::Pointer),
	Hand(Box<super::hand::Hand>),
	Tip(super::tip::Tip),
}
