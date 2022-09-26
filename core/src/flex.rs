use crate::{
	flex_from_color, flex_from_quat, flex_from_vec2, flex_from_vec3,
	values::{Color, Quat, Transform, Vec2, Vec3},
};
use flexbuffers::{Builder, VectorBuilder};
use std::path::PathBuf;

pub enum FlexBuffable {
	Bool(bool),
	UInt(u64),
	Int(i64),
	Float(f32),
	Vec2(Vec2),
	Vec3(Vec3),
	Quat(Quat),
	Transform(Transform),
	Color(Color),
	String(String),
}
impl From<bool> for FlexBuffable {
	fn from(var: bool) -> Self {
		FlexBuffable::Bool(var)
	}
}
impl From<u8> for FlexBuffable {
	fn from(var: u8) -> Self {
		FlexBuffable::UInt(var as u64)
	}
}
impl From<u16> for FlexBuffable {
	fn from(var: u16) -> Self {
		FlexBuffable::UInt(var as u64)
	}
}
impl From<u32> for FlexBuffable {
	fn from(var: u32) -> Self {
		FlexBuffable::UInt(var as u64)
	}
}
impl From<u64> for FlexBuffable {
	fn from(var: u64) -> Self {
		FlexBuffable::UInt(var)
	}
}
impl From<i8> for FlexBuffable {
	fn from(var: i8) -> Self {
		FlexBuffable::Int(var as i64)
	}
}
impl From<i16> for FlexBuffable {
	fn from(var: i16) -> Self {
		FlexBuffable::Int(var as i64)
	}
}
impl From<i32> for FlexBuffable {
	fn from(var: i32) -> Self {
		FlexBuffable::Int(var as i64)
	}
}
impl From<i64> for FlexBuffable {
	fn from(var: i64) -> Self {
		FlexBuffable::Int(var)
	}
}
impl From<f32> for FlexBuffable {
	fn from(var: f32) -> Self {
		FlexBuffable::Float(var)
	}
}
impl From<Vec2> for FlexBuffable {
	fn from(var: Vec2) -> Self {
		FlexBuffable::Vec2(var)
	}
}
impl From<Vec3> for FlexBuffable {
	fn from(var: Vec3) -> Self {
		FlexBuffable::Vec3(var)
	}
}
impl From<Quat> for FlexBuffable {
	fn from(var: Quat) -> Self {
		FlexBuffable::Quat(var)
	}
}
impl From<Transform> for FlexBuffable {
	fn from(var: Transform) -> Self {
		FlexBuffable::Transform(var)
	}
}
impl From<Color> for FlexBuffable {
	fn from(var: Color) -> Self {
		FlexBuffable::Color(var)
	}
}
impl From<String> for FlexBuffable {
	fn from(var: String) -> Self {
		FlexBuffable::String(var)
	}
}
impl From<&str> for FlexBuffable {
	fn from(var: &str) -> Self {
		FlexBuffable::String(String::from(var))
	}
}
impl From<PathBuf> for FlexBuffable {
	fn from(var: PathBuf) -> Self {
		FlexBuffable::String(String::from(var.to_str().unwrap()))
	}
}
impl FlexBuffable {
	pub fn push_to_vector(&self, vec: &mut VectorBuilder) {
		match self {
			FlexBuffable::Bool(v) => vec.push(*v),
			FlexBuffable::UInt(v) => vec.push(*v),
			FlexBuffable::Int(v) => vec.push(*v),
			FlexBuffable::Float(v) => vec.push(*v),
			FlexBuffable::Vec2(vec3) => {
				flex_from_vec2!(vec, vec3)
			}
			FlexBuffable::Vec3(vec3) => {
				flex_from_vec3!(vec, vec3)
			}
			FlexBuffable::Quat(quat) => {
				flex_from_quat!(vec, quat)
			}
			FlexBuffable::Transform(transform) => {
				let mut transform_vec = vec.start_vector();
				if let Some(translation) = transform.position {
					flex_from_vec3!(transform_vec, translation);
				} else {
					transform_vec.push(());
				}
				if let Some(rotation) = transform.rotation {
					flex_from_quat!(transform_vec, rotation);
				} else {
					transform_vec.push(());
				}
				if let Some(scale) = transform.scale {
					flex_from_vec3!(transform_vec, scale);
				} else {
					transform_vec.push(());
				}
			}
			FlexBuffable::Color(color) => {
				flex_from_color!(vec, color)
			}
			FlexBuffable::String(v) => vec.push(v.as_str()),
		}
	}

	pub fn build_singleton(&self) -> Vec<u8> {
		match self {
			FlexBuffable::Bool(v) => flexbuffers::singleton(*v),
			FlexBuffable::UInt(v) => flexbuffers::singleton(*v),
			FlexBuffable::Int(v) => flexbuffers::singleton(*v),
			FlexBuffable::Float(v) => flexbuffers::singleton(*v),
			FlexBuffable::Vec2(vec2) => flexbuffer_from_arguments(|fbb| flex_from_vec2!(fbb, vec2)),
			FlexBuffable::Vec3(vec3) => flexbuffer_from_arguments(|fbb| flex_from_vec3!(fbb, vec3)),
			FlexBuffable::Quat(quat) => flexbuffer_from_arguments(|fbb| flex_from_quat!(fbb, quat)),
			FlexBuffable::Transform(transform) => flexbuffer_from_vector_arguments(|vec| {
				if let Some(translation) = transform.position {
					flex_from_vec3!(vec, translation);
				} else {
					vec.push(());
				}
				if let Some(rotation) = transform.rotation {
					flex_from_quat!(vec, rotation);
				} else {
					vec.push(());
				}
				if let Some(scale) = transform.scale {
					flex_from_vec3!(vec, scale);
				} else {
					vec.push(());
				}
			}),
			FlexBuffable::Color(color) => {
				flexbuffer_from_arguments(|fbb| flex_from_color!(fbb, color))
			}
			FlexBuffable::String(v) => flexbuffers::singleton(v.as_str()),
		}
	}
}

#[macro_export]
macro_rules! push_to_vec {
	($vec:expr, $thing_to_pass:expr) => {{
		{
			$crate::flex::FlexBuffable::from($thing_to_pass).push_to_vector($vec);
		}
	}};
	($vec:expr, $first_thing:expr, $($thing_to_pass:expr),+) => {{
		{
			$crate::push_to_vec! {$vec, $first_thing}
			$crate::push_to_vec! {$vec, $($thing_to_pass),+}
		}
	}};
}

pub fn flexbuffer_from_arguments<S>(args_constructor: S) -> Vec<u8>
where
	S: FnOnce(&mut Builder),
{
	let mut fbb = Builder::default();
	args_constructor(&mut fbb);
	fbb.view().to_vec()
}

pub fn flexbuffer_from_vector_arguments<S>(args_constructor: S) -> Vec<u8>
where
	S: FnOnce(&mut VectorBuilder),
{
	let mut fbb = Builder::default();
	let mut vec = fbb.start_vector();
	args_constructor(&mut vec);
	vec.end_vector();
	fbb.take_buffer()
}
