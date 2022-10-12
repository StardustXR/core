mod deserialize;
mod serialize;

pub use deserialize::deserialize;
pub use serialize::{serialize, FlexSerializeError};

#[test]
fn round_trip_flex_serialize() {
	use mint::{Quaternion, Vector2, Vector3};
	use serde::{Deserialize, Serialize};
	use serde_repr::{Deserialize_repr, Serialize_repr};
	#[derive(Debug, PartialEq, Clone, Serialize_repr, Deserialize_repr)]
	#[repr(u32)]
	enum TestEnum {
		Value1,
		Value2,
	}
	#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
	struct TestStruct {
		b: bool,
		i1: i8,
		i2: i16,
		i3: i32,
		i4: i64,
		u1: u8,
		u2: u16,
		u3: u32,
		u4: u64,
		f1: f32,
		f2: f64,
		test_vec2: Vector2<f32>,
		test_vec3: Vector3<f32>,
		test_quat: Quaternion<f32>,
		string: String,
		test_enum: TestEnum,
		test_struct: Option<Box<TestStruct>>,
	}

	let mut test_struct = TestStruct {
		b: true,
		i1: 25,
		i2: 25,
		i3: 25,
		i4: 25,
		u1: 25,
		u2: 25,
		u3: 25,
		u4: 25,
		f1: 0.5,
		f2: 0.5,
		test_vec2: Vector2::from([0.7; 2]),
		test_vec3: Vector3::from([0.63; 3]),
		test_quat: Quaternion {
			v: Vector3::from([3.14; 3]),
			s: 12.0,
		},
		string: "Test Test".to_string(),
		test_enum: TestEnum::Value1,
		test_struct: None,
	};
	test_struct.test_struct = Some(Box::new(test_struct.clone()));
	let serialized = serialize(test_struct.clone()).unwrap();
	let flex = flexbuffers::Reader::get_root(serialized.as_slice()).unwrap();
	println!("{}", flex);
	let deserialized: TestStruct = deserialize(&serialized).unwrap();
	assert_eq!(test_struct, deserialized, "Round trip lost data");
}
