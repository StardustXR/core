use super::generated::{
	self,
	input::{InputDataRawT, InputDataT},
};
use crate::flex::Datamap;
use ouroboros::self_referencing;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Debug, hash::Hash};

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
	/// There are [order] objects that got this input data before this one.
	pub order: u32,
	/// Is this input handler capturing this input method?
	pub captured: bool,
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
			datamap: Datamap::from_raw(datamap).map_err(|e| e.to_string())?,
			order: input.order,
			captured: input.captured,
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
			datamap: Some(self.datamap.raw().clone()),
			order: self.order,
			captured: self.captured,
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
