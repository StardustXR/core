use super::Pointer;
use anyhow::{anyhow, bail};
use ouroboros::self_referencing;
use schemas::{
	input::{InputDataRawT, InputDataT},
	input_hand::HandT,
};
use std::{convert::TryFrom, fmt::Debug, hash::Hash};

#[derive(Debug, Clone)]
pub enum InputDataType {
	Pointer(Pointer),
	Hand(HandT),
}

pub struct Datamap(DatamapInner);
impl Datamap {
	pub fn with_data<F, O>(&self, f: F) -> O
	where
		F: FnOnce(&flexbuffers::MapReader<&[u8]>) -> O,
	{
		self.0.with_reader(f)
	}
}
impl Datamap {
	fn new(raw: Vec<u8>) -> anyhow::Result<Self> {
		Ok(Datamap(DatamapInner::try_new(raw, |raw| {
			flexbuffers::Reader::get_root(raw.as_slice())?.get_map()
		})?))
	}
}
impl Debug for Datamap {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Datamap")
			.field(
				"raw",
				&String::from_utf8_lossy(self.0.borrow_raw()).into_owned(),
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

#[derive(Debug)]
pub struct InputData {
	pub uid: String,
	pub input: InputDataType,
	pub distance: f32,
	pub datamap: Datamap,
}
impl TryFrom<InputDataT> for InputData {
	type Error = anyhow::Error;

	fn try_from(input: InputDataT) -> Result<Self, Self::Error> {
		Ok(InputData {
			uid: input.uid,
			input: match input.input {
				InputDataRawT::Pointer(pointer) => InputDataType::Pointer(Pointer::new(
					pointer.origin.into(),
					pointer.orientation.into(),
					pointer.deepest_point.into(),
				)),
				InputDataRawT::Hand(hand) => InputDataType::Hand(*hand),
				_ => bail!("Invalid input type"),
			},
			distance: input.distance,
			datamap: Datamap::new(input.datamap.ok_or_else(|| anyhow!("No datamap!"))?)?,
		})
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
