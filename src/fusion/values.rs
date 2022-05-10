use crate::fusion::values::Alignment::{XCenter, XLeft, XRight, YBottom, YCenter, YTop};

pub type Vec2 = mint::Vector2<f32>;
pub type Vec3 = mint::Vector3<f32>;
pub type Quat = mint::Quaternion<f32>;
pub type Color = color::Rgba;
pub enum Alignment {
	XLeft        = 1 << 0,
	YTop         = 1 << 1,
	XCenter      = 1 << 2,
	YCenter      = 1 << 3,
	XRight       = 1 << 4,
	YBottom      = 1 << 5,
	Center       = XCenter  as isize | YCenter as isize,
	CenterLeft   = XLeft    as isize | YCenter as isize,
	CenterRight  = XRight   as isize | YCenter as isize,
	TopCenter    = XCenter  as isize | YTop    as isize,
	TopLeft      = XLeft    as isize | YTop    as isize,
	TopRight     = XRight   as isize | YTop    as isize,
	BottomCenter = XCenter  as isize | YBottom as isize,
	BottomLeft   = XLeft    as isize | YBottom as isize,
	BottomRight  = XRight   as isize | YBottom as isize,
}
pub enum TextFit {
	Wrap     = 1 << 0,
	Clip     = 1 << 1,
	Squeeze  = 1 << 2,
	Exact    = 1 << 3,
	Overflow = 1 << 4,
}
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
macro_rules! flex_to_vec2 {
	($F:expr) => {{
		let vec = $F.get_vector().unwrap();
		mint::Vector2 {
			x: vec.idx(0).as_f32(),
			y: vec.idx(1).as_f32(),
		};
	}};
}
#[macro_export]
macro_rules! flex_to_vec3 {
	($F:expr) => {{
		let vec = $F.get_vector().unwrap();
		mint::Vector3 {
			x: vec.idx(0).as_f32(),
			y: vec.idx(1).as_f32(),
			z: vec.idx(2).as_f32(),
		}
	}};
}
#[macro_export]
macro_rules! flex_to_quat {
	($F:expr) => {{
		let vec = $F.get_vector().unwrap();
		mint::Quaternion {
			v: mint::Vector3::from([
				vec.idx(0).as_f32(),
				vec.idx(1).as_f32(),
				vec.idx(2).as_f32(),
			]),
			s: vec.idx(3).as_f32(),
		}
	}};
}
