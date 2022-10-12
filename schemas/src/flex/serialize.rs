use serde::{
	ser::{
		SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
		SerializeTupleStruct, SerializeTupleVariant,
	},
	Serialize, Serializer,
};
use std::{borrow::BorrowMut, cell::RefCell, fmt::Display, marker::PhantomData};

#[derive(Debug, thiserror::Error)]
pub enum FlexSerializeError {
	#[error("Map key is not a string!")]
	MapKeyNotString,
	#[error("custom")]
	Custom(String),
}
impl serde::ser::Error for FlexSerializeError {
	fn custom<T>(msg: T) -> Self
	where
		T: Display,
	{
		FlexSerializeError::Custom(msg.to_string())
	}
}

pub fn serialize<S: Serialize>(to_serialize: S) -> Result<Vec<u8>, FlexSerializeError> {
	let mut fbb = flexbuffers::Builder::default();
	let fs = FlexSerializer { fbb: &mut fbb };
	to_serialize.serialize(fs)?;
	Ok(fbb.take_buffer())
}

struct FlexSerializer<'b> {
	fbb: &'b mut flexbuffers::Builder,
}
impl<'b> Serializer for FlexSerializer<'b> {
	type Ok = ();
	type Error = FlexSerializeError;

	type SerializeSeq = FlexVecSerializer<'b>;
	type SerializeTuple = FlexVecSerializer<'b>;
	type SerializeTupleStruct = FlexVecSerializer<'b>;
	type SerializeTupleVariant = FlexVecSerializer<'b>;
	type SerializeMap = FlexMapSerializer<'b>;
	type SerializeStruct = FlexVecSerializer<'b>;
	type SerializeStructVariant = FlexVecSerializer<'b>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v.to_string().as_str());
		Ok(())
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(v);
		Ok(())
	}

	fn serialize_none(self) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(());
		Ok(())
	}

	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, FlexSerializeError> {
		self.fbb.build_singleton(());
		Ok(())
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, FlexSerializeError> {
		self.serialize_unit()
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		variant_index: u32,
		_variant: &'static str,
	) -> Result<Self::Ok, FlexSerializeError> {
		self.serialize_u32(variant_index)
	}

	fn serialize_newtype_struct<T: ?Sized>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: ?Sized>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		// TODO: actually store variant type too
		value.serialize(self)
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.fbb.start_vector(),
		})
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.fbb.start_vector(),
		})
	}

	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.fbb.start_vector(),
		})
	}

	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, FlexSerializeError> {
		// TODO: actually store variant type too
		Ok(FlexVecSerializer {
			fvb: self.fbb.start_vector(),
		})
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, FlexSerializeError> {
		Ok(FlexMapSerializer {
			fmb: self.fbb.start_map(),
			key: FlexMapKeySerializer {
				key: String::new(),
				phantom: PhantomData::default(),
			},
		})
	}

	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.fbb.start_vector(),
		})
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, FlexSerializeError> {
		// TODO: actually store variant type too
		Ok(FlexVecSerializer {
			fvb: self.fbb.start_vector(),
		})
	}
}

struct FlexVecSerializer<'b> {
	fvb: flexbuffers::VectorBuilder<'b>,
}

struct FlexVecSerializerWrapper<'parent, 'fvb>(&'fvb mut flexbuffers::VectorBuilder<'parent>);

impl<'parent, 'fvb> Serializer for FlexVecSerializerWrapper<'parent, 'fvb> {
	type Ok = ();
	type Error = FlexSerializeError;

	type SerializeSeq = FlexVecSerializer<'fvb>;
	type SerializeTuple = FlexVecSerializer<'fvb>;
	type SerializeTupleStruct = FlexVecSerializer<'fvb>;
	type SerializeTupleVariant = FlexVecSerializer<'fvb>;
	type SerializeMap = FlexMapSerializer<'fvb>;
	type SerializeStruct = FlexVecSerializer<'fvb>;
	type SerializeStructVariant = FlexVecSerializer<'fvb>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v.to_string().as_str());
		Ok(())
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(v);
		Ok(())
	}

	fn serialize_none(self) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(());
		Ok(())
	}

	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, FlexSerializeError> {
		self.0.push(());
		Ok(())
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, FlexSerializeError> {
		self.serialize_unit()
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		variant_index: u32,
		_variant: &'static str,
	) -> Result<Self::Ok, FlexSerializeError> {
		self.serialize_u32(variant_index)
	}

	fn serialize_newtype_struct<T: ?Sized>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: ?Sized>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		// TODO: actually store variant type too
		value.serialize(self)
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.0.start_vector(),
		})
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.0.start_vector(),
		})
	}

	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.0.start_vector(),
		})
	}

	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, FlexSerializeError> {
		// TODO: actually store variant type too
		Ok(FlexVecSerializer {
			fvb: self.0.start_vector(),
		})
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, FlexSerializeError> {
		Ok(FlexMapSerializer {
			fmb: self.0.start_map(),
			key: FlexMapKeySerializer {
				key: String::new(),
				phantom: PhantomData::default(),
			},
		})
	}

	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.0.start_vector(),
		})
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, FlexSerializeError> {
		// TODO: actually store variant type too
		Ok(FlexVecSerializer {
			fvb: self.0.start_vector(),
		})
	}
}
impl<'b> SerializeSeq for FlexVecSerializer<'b> {
	type Ok = ();
	type Error = FlexSerializeError;

	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(FlexVecSerializerWrapper(&mut self.fvb))
	}

	fn end(self) -> Result<Self::Ok, FlexSerializeError> {
		Ok(())
	}
}
impl<'b> SerializeTuple for FlexVecSerializer<'b> {
	type Ok = ();
	type Error = FlexSerializeError;

	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(FlexVecSerializerWrapper(&mut self.fvb))
	}

	fn end(self) -> Result<Self::Ok, FlexSerializeError> {
		Ok(())
	}
}
impl<'b> SerializeTupleStruct for FlexVecSerializer<'b> {
	type Ok = ();
	type Error = FlexSerializeError;

	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(FlexVecSerializerWrapper(&mut self.fvb))
	}

	fn end(self) -> Result<Self::Ok, FlexSerializeError> {
		Ok(())
	}
}
impl<'b> SerializeTupleVariant for FlexVecSerializer<'b> {
	type Ok = ();
	type Error = FlexSerializeError;

	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(FlexVecSerializerWrapper(&mut self.fvb))
	}

	fn end(self) -> Result<Self::Ok, FlexSerializeError> {
		Ok(())
	}
}
impl<'b> SerializeStruct for FlexVecSerializer<'b> {
	type Ok = ();
	type Error = FlexSerializeError;

	fn serialize_field<T: ?Sized>(
		&mut self,
		_key: &'static str,
		value: &T,
	) -> Result<(), FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(FlexVecSerializerWrapper(&mut self.fvb))
	}

	fn end(self) -> Result<Self::Ok, FlexSerializeError> {
		Ok(())
	}
}

impl<'b> SerializeStructVariant for FlexVecSerializer<'b> {
	type Ok = ();
	type Error = FlexSerializeError;

	fn serialize_field<T: ?Sized>(
		&mut self,
		_key: &'static str,
		value: &T,
	) -> Result<(), FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(FlexVecSerializerWrapper(&mut self.fvb))
	}

	fn end(self) -> Result<Self::Ok, FlexSerializeError> {
		Ok(())
	}
}

struct FlexMapKeySerializer<'b> {
	key: String,
	phantom: PhantomData<&'b ()>,
}
impl<'b> Serializer for &mut FlexMapKeySerializer<'b> {
	type Ok = ();
	type Error = FlexSerializeError;

	type SerializeSeq = FlexVecSerializer<'b>;
	type SerializeTuple = FlexVecSerializer<'b>;
	type SerializeTupleStruct = FlexVecSerializer<'b>;
	type SerializeTupleVariant = FlexVecSerializer<'b>;
	type SerializeMap = FlexMapSerializer<'b>;
	type SerializeStruct = FlexVecSerializer<'b>;
	type SerializeStructVariant = FlexVecSerializer<'b>;

	fn serialize_bool(self, _v: bool) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_i8(self, _v: i8) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_i16(self, _v: i16) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_i32(self, _v: i32) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_i64(self, _v: i64) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_u8(self, _v: u8) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_u16(self, _v: u16) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_u32(self, _v: u32) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_u64(self, _v: u64) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_f32(self, _v: f32) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_f64(self, _v: f64) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, FlexSerializeError> {
		self.key = v.to_string();
		Ok(())
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, FlexSerializeError> {
		self.key = v.to_string();
		Ok(())
	}

	fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_none(self) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_unit(self) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
	) -> Result<Self::Ok, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_newtype_struct<T: ?Sized>(
		self,
		_name: &'static str,
		_value: &T,
	) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_newtype_variant<T: ?Sized>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_value: &T,
	) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, FlexSerializeError> {
		Err(FlexSerializeError::MapKeyNotString)
	}
}

struct FlexMapSerializer<'b> {
	fmb: flexbuffers::MapBuilder<'b>,
	key: FlexMapKeySerializer<'b>,
}

struct FlexMapSerializerWrapper<'parent, 'fvb>(
	&'fvb mut FlexMapKeySerializer<'parent>,
	&'fvb mut flexbuffers::MapBuilder<'parent>,
);

impl<'parent, 'fvb> Serializer for FlexMapSerializerWrapper<'parent, 'fvb> {
	type Ok = ();
	type Error = FlexSerializeError;

	type SerializeSeq = FlexVecSerializer<'fvb>;
	type SerializeTuple = FlexVecSerializer<'fvb>;
	type SerializeTupleStruct = FlexVecSerializer<'fvb>;
	type SerializeTupleVariant = FlexVecSerializer<'fvb>;
	type SerializeMap = FlexMapSerializer<'fvb>;
	type SerializeStruct = FlexVecSerializer<'fvb>;
	type SerializeStructVariant = FlexVecSerializer<'fvb>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}
	fn serialize_i8(self, v: i8) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v.to_string().as_str());
		Ok(())
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, v);
		Ok(())
	}

	fn serialize_none(self) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, ());
		Ok(())
	}

	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, FlexSerializeError> {
		self.1.push(&self.0.key, ());
		Ok(())
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, FlexSerializeError> {
		self.serialize_unit()
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		variant_index: u32,
		_variant: &'static str,
	) -> Result<Self::Ok, FlexSerializeError> {
		self.serialize_u32(variant_index)
	}

	fn serialize_newtype_struct<T: ?Sized>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: ?Sized>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, FlexSerializeError>
	where
		T: Serialize,
	{
		// TODO: actually store variant type too
		value.serialize(self)
	}

	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.1.start_vector(&self.0.key),
		})
	}

	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.1.start_vector(&self.0.key),
		})
	}

	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.1.start_vector(&self.0.key),
		})
	}

	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant, FlexSerializeError> {
		// TODO: actually store variant type too
		Ok(FlexVecSerializer {
			fvb: self.1.start_vector(&self.0.key),
		})
	}

	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, FlexSerializeError> {
		Ok(FlexMapSerializer {
			fmb: self.1.start_map(&self.0.key),
			key: FlexMapKeySerializer {
				key: String::new(),
				phantom: PhantomData::default(),
			},
		})
	}

	fn serialize_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStruct, FlexSerializeError> {
		Ok(FlexVecSerializer {
			fvb: self.1.start_vector(&self.0.key),
		})
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant, FlexSerializeError> {
		// TODO: actually store variant type too
		Ok(FlexVecSerializer {
			fvb: self.1.start_vector(&self.0.key),
		})
	}
}

impl<'b> SerializeMap for FlexMapSerializer<'b> {
	type Ok = ();
	type Error = FlexSerializeError;

	fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), FlexSerializeError>
	where
		T: Serialize,
	{
		key.serialize(&mut self.key)
	}

	fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), FlexSerializeError>
	where
		T: Serialize,
	{
		value.serialize(FlexMapSerializerWrapper(&mut self.key, &mut self.fmb))
	}

	fn end(self) -> Result<Self::Ok, FlexSerializeError> {
		Ok(())
	}
}
