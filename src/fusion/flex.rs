use super::values::{Color, Quat, Vec2, Vec3};
use std::path::PathBuf;

macro_rules! push_to_vec {
	($vec:expr, $thing_to_pass:expr) => {{
		{
			match crate::fusion::flex::FlexBuffable::from($thing_to_pass) {
				crate::fusion::flex::FlexBuffable::Bool(v) => {$vec.push(v)},
				crate::fusion::flex::FlexBuffable::UInt(v) => {$vec.push(v)},
				crate::fusion::flex::FlexBuffable::Int(v) => {$vec.push(v)},
				crate::fusion::flex::FlexBuffable::Float(v) => {$vec.push(v)},
				crate::fusion::flex::FlexBuffable::Vec2(vec3) => {flex_from_vec2!($vec, vec3)},
				crate::fusion::flex::FlexBuffable::Vec3(vec3) => {flex_from_vec3!($vec, vec3)},
				crate::fusion::flex::FlexBuffable::Quat(quat) => {flex_from_quat!($vec, quat)},
				crate::fusion::flex::FlexBuffable::Color(color) => {flex_from_color!($vec, color)},
				crate::fusion::flex::FlexBuffable::String(v) => {$vec.push(v.as_str())},
			}
		}
	}};
	($vec:expr, $first_thing:expr, $($thing_to_pass:expr),+) => {{
		{
			push_to_vec! {$vec, $first_thing}
			push_to_vec! {$vec, $($thing_to_pass),+}
		}
	}};
}
pub enum FlexBuffable {
	Bool(bool),
	UInt(u64),
	Int(i64),
	Float(f32),
	Vec3(Vec3),
	Vec2(Vec2),
	Quat(Quat),
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
impl From<Vec3> for FlexBuffable {
	fn from(var: Vec3) -> Self {
		FlexBuffable::Vec3(var)
	}
}
impl From<Vec2> for FlexBuffable {
	fn from(var: Vec2) -> Self {
		FlexBuffable::Vec2(var)
	}
}
impl From<Quat> for FlexBuffable {
	fn from(var: Quat) -> Self {
		FlexBuffable::Quat(var)
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
