use super::Pointer;
use anyhow::{anyhow, bail};
use ouroboros::self_referencing;
use schemas::input::{InputDataRawT, InputDataT};
use std::{convert::TryFrom, fmt::Debug};

#[derive(Debug, Clone)]
pub enum InputDataType {
	Pointer(Pointer),
	// Hand(Hand),
}

#[self_referencing]
pub struct InputData {
	pub uid: String,
	pub input: InputDataType,
	pub distance: f32,
	datamap_raw: Vec<u8>,

	#[borrows(datamap_raw)]
	#[not_covariant]
	pub datamap: flexbuffers::MapReader<&'this [u8]>,
}
impl TryFrom<InputDataT> for InputData {
	type Error = anyhow::Error;

	fn try_from(input: InputDataT) -> Result<Self, Self::Error> {
		InputData::try_new(
			input.uid,
			match input.input {
				InputDataRawT::Pointer(pointer) => InputDataType::Pointer(Pointer::new(
					pointer.origin.into(),
					pointer.orientation.into(),
					pointer.deepest_point.into(),
				)),
				InputDataRawT::Hand(_hand) => todo!("need hand struct format"),
				_ => bail!("Invalid input type"),
			},
			input.distance,
			input.datamap.ok_or_else(|| anyhow!("No datamap!"))?,
			|datamap_raw| flexbuffers::Reader::get_root(datamap_raw.as_slice())?.get_map(),
		)
		.map_err(anyhow::Error::from)
	}
}
impl Clone for InputData {
	fn clone(&self) -> Self {
		Self::new(
			self.borrow_uid().clone(),
			self.borrow_input().clone(),
			*self.borrow_distance(),
			self.borrow_datamap_raw().clone(),
			|datamap_raw| {
				flexbuffers::Reader::get_root(datamap_raw.as_slice())
					.unwrap()
					.get_map()
					.unwrap()
			},
		)
	}
}
impl Debug for InputData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("InputData")
			.field("uid", self.borrow_uid())
			.field("input", self.borrow_input())
			.field("distance", self.borrow_distance())
			.field(
				"datamap_raw",
				&String::from_utf8_lossy(self.borrow_datamap_raw()).into_owned(),
			)
			.finish_non_exhaustive()
	}
}
