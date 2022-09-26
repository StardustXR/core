use flexbuffers::{Buffer, Reader};
use mint::{Quaternion, Vector2, Vector3};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Quat = Quaternion<f32>;
pub type Color = color::Rgba;

pub struct Transform {
	pub position: Option<Vec3>,
	pub rotation: Option<Quat>,
	pub scale: Option<Vec3>,
}

// pub const VEC3_ZERO: Vec3 = Vec3 {
// 	x: 0.0,
// 	y: 0.0,
// 	z: 0.0,
// };
// pub const VEC3_ONE: Vec3 = Vec3 {
// 	x: 1.0,
// 	y: 1.0,
// 	z: 1.0,
// };
// pub const QUAT_IDENTITY: Quat = Quat {
// 	v: Vec3 {
// 		x: 0.0,
// 		y: 0.0,
// 		z: 0.0,
// 	},
// 	s: 1.0,
// };

#[macro_export]
macro_rules! flex_from_vec2 {
	($B:expr, $V:expr) => {{
		let mut vec = $B.start_vector();
		vec.push($V.x);
		vec.push($V.y);
		vec.end_vector();
	}};
}
#[macro_export]
macro_rules! flex_from_vec3 {
	($B:expr, $V:expr) => {{
		let mut vec = $B.start_vector();
		vec.push($V.x);
		vec.push($V.y);
		vec.push($V.z);
		vec.end_vector();
	}};
}
#[macro_export]
macro_rules! flex_from_quat {
	($B:expr, $V:expr) => {{
		let mut vec = $B.start_vector();
		vec.push($V.v.x);
		vec.push($V.v.y);
		vec.push($V.v.z);
		vec.push($V.s);
		vec.end_vector();
	}};
}
#[macro_export]
macro_rules! flex_from_color {
	($B:expr, $V:expr) => {{
		let mut vec = $B.start_vector();
		vec.push($V.c.r);
		vec.push($V.c.g);
		vec.push($V.c.b);
		vec.push($V.a);
		vec.end_vector();
	}};
}

pub fn parse_f32<B: Buffer>(reader: Reader<B>) -> Option<f32> {
	Some(reader.get_f64().ok()? as f32)
}
pub fn parse_vec2<B: Buffer>(reader: Reader<B>) -> Option<Vector2<f32>> {
	let vec = reader.get_vector().ok()?;

	Some(Vector2::from([
		parse_f32(vec.index(0).ok()?)?,
		parse_f32(vec.index(1).ok()?)?,
	]))
}
pub fn parse_vec3<B: Buffer>(reader: Reader<B>) -> Option<Vector3<f32>> {
	let vec = reader.get_vector().ok()?;

	Some(Vector3::from([
		parse_f32(vec.index(0).ok()?)?,
		parse_f32(vec.index(1).ok()?)?,
		parse_f32(vec.index(2).ok()?)?,
	]))
}
pub fn parse_quat<B: Buffer>(reader: Reader<B>) -> Option<Quaternion<f32>> {
	let vec = reader.get_vector().ok()?;

	Some(Quaternion::from([
		parse_f32(vec.index(0).ok()?)?,
		parse_f32(vec.index(1).ok()?)?,
		parse_f32(vec.index(2).ok()?)?,
		parse_f32(vec.index(3).ok()?)?,
	]))
}
