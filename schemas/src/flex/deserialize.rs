use flexbuffers::{BitWidth, DeserializationError, FlexBufferType, ReaderError, ReaderIterator};
use serde::{
	de::{
		DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess, Visitor,
	},
	Deserialize, Deserializer,
};
use std::fmt::Display;

/// Deserialize given flexbuffers data into whatever format.
/// This is different than the regular flexbuffers deserialization
/// because it strips the names off of struct fields,
/// instead putting the values into vectors with the same
/// order to save space and speed up deserialization.
pub fn deserialize<'a, T: Deserialize<'a>>(data: &'a [u8]) -> Result<T, DeserializationError> {
	let root = flexbuffers::Reader::get_root(data)?;
	let deserializer = FlexbuffersDeserializer(root);
	T::deserialize(deserializer)
}

struct ReaderIteratorWrapper<'d>(ReaderIterator<&'d [u8]>);

impl<'de> SeqAccess<'de> for ReaderIteratorWrapper<'de> {
	type Error = DeserializationError;

	fn next_element_seed<T>(
		&mut self,
		seed: T,
	) -> Result<Option<<T as DeserializeSeed<'de>>::Value>, Self::Error>
	where
		T: DeserializeSeed<'de>,
	{
		if let Some(elem) = self.0.next() {
			seed.deserialize(elem).map(Some)
		} else {
			Ok(None)
		}
	}

	fn size_hint(&self) -> Option<usize> {
		Some(self.0.len())
	}
}

struct EnumReader<'de> {
	variant: &'de str,
	value: Option<flexbuffers::Reader<&'de [u8]>>,
}

impl<'de> EnumAccess<'de> for EnumReader<'de> {
	type Error = DeserializationError;
	type Variant = FlexbuffersDeserializer<'de>;

	fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: DeserializeSeed<'de>,
	{
		seed.deserialize(self.variant.into_deserializer())
			.map(|v| (v, FlexbuffersDeserializer(self.value.unwrap_or_default())))
	}
}

struct MapAccessor<'de> {
	keys: ReaderIteratorWrapper<'de>,
	vals: ReaderIteratorWrapper<'de>,
}

impl<'de> MapAccess<'de> for MapAccessor<'de> {
	type Error = DeserializationError;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: DeserializeSeed<'de>,
	{
		if let Some(k) = self.keys.0.next() {
			seed.deserialize(k).map(Some)
		} else {
			Ok(None)
		}
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: DeserializeSeed<'de>,
	{
		let val = self.vals.0.next().ok_or(ReaderError::IndexOutOfBounds)?;
		seed.deserialize(val)
	}
}

struct FlexbuffersDeserializer<'de>(flexbuffers::Reader<&'de [u8]>);

impl<'de> VariantAccess<'de> for FlexbuffersDeserializer<'de> {
	type Error = DeserializationError;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Ok(())
	}

	fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: DeserializeSeed<'de>,
	{
		seed.deserialize(self)
	}

	fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>,
	{
		visitor.visit_seq(self.0.get_vector()?.iter())
	}

	fn struct_variant<V>(
		self,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>,
	{
		visitor.visit_seq(self.0.get_vector()?.iter())
	}
}

impl<'de> Deserializer<'de> for FlexbuffersDeserializer<'de> {
	type Error = DeserializationError;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>,
	{
		use BitWidth::*;
		use FlexBufferType::*;
		match (self.0.flexbuffer_type(), self.0.bitwidth()) {
			(Bool, _) => visitor.visit_bool(self.0.as_bool()),
			(UInt, bw) => match bw {
				W8 => visitor.visit_u8(self.0.as_u8()),
				W16 => visitor.visit_u16(self.0.as_u16()),
				W32 => visitor.visit_u32(self.0.as_u32()),
				W64 => visitor.visit_u64(self.0.as_u64()),
			},
			(Int, bw) => match bw {
				W8 => visitor.visit_i8(self.0.as_i8()),
				W16 => visitor.visit_i16(self.0.as_i16()),
				W32 => visitor.visit_i32(self.0.as_i32()),
				W64 => visitor.visit_i64(self.0.as_i64()),
			},
			(Float, bw) => {
				match bw {
					W32 => visitor.visit_f32(self.0.as_f32()),
					W64 => visitor.visit_f64(self.0.as_f64()),
					_ => Err(ReaderError::InvalidPackedType.into()), // f8 and f16 are not supported.
				}
			}
			(Null, _) => visitor.visit_unit(),
			(String, _) | (Key, _) => visitor.visit_borrowed_str(self.0.get_str()?),
			(Blob, _) => visitor.visit_borrowed_bytes(self.0.get_blob()?.0),
			(Map, _) => {
				let m = self.0.get_map()?;
				visitor.visit_map(MapAccessor {
					keys: ReaderIteratorWrapper(m.keys_vector().iter()),
					vals: ReaderIteratorWrapper(m.iter_values()),
				})
			}
			(ty, _) if ty.is_vector() => visitor.visit_seq(self.0.get_vector()?.iter()),
			(ty, bw) => unreachable!("TODO deserialize_any {:?} {:?}.", ty, bw),
		}
	}
	serde::forward_to_deserialize_any! {
		bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 str unit unit_struct bytes
		ignored_any map identifier struct tuple tuple_struct seq string
	}
	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>,
	{
		visitor.visit_char(self.0.get_u64()? as u8 as char)
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>,
	{
		visitor.visit_byte_buf(self.0.get_blob()?.0.to_vec())
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>,
	{
		if self.0.flexbuffer_type() == FlexBufferType::Null {
			visitor.visit_none()
		} else {
			visitor.visit_some(self)
		}
	}

	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: Visitor<'de>,
	{
		let (variant, value) = match self.0.flexbuffer_type() {
			FlexBufferType::String => (self.0.as_str(), None),
			FlexBufferType::Map => {
				let m = self.0.get_map()?;
				let variant = m.keys_vector().idx(0).get_key()?;
				let value = Some(m.idx(0));
				(variant, value)
			}
			_ => {
				return Err(flexbuffers::ReaderError::UnexpectedFlexbufferType {
					expected: FlexBufferType::Map,
					actual: self.0.flexbuffer_type(),
				}
				.into());
			}
		};
		visitor.visit_enum(EnumReader { variant, value })
	}
}
