pub type Vec2 = mint::Vector2<f32>;
pub type Vec3 = mint::Vector3<f32>;
pub type Quat = mint::Quaternion<f32>;
pub type Color = color::Rgba;
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

#[macro_export]
macro_rules! flex_to_vec2 {
	($F:expr) => {{
		$F.get_vector().ok().map(|v| mint::Vector2 {
			x: v.idx(0).as_f32(),
			y: v.idx(1).as_f32(),
		})
	}};
}
#[macro_export]
macro_rules! flex_to_vec3 {
	($F:expr) => {{
		$F.get_vector().ok().map(|v| mint::Vector3 {
			x: v.idx(0).as_f32(),
			y: v.idx(1).as_f32(),
			z: v.idx(2).as_f32(),
		})
	}};
}
#[macro_export]
macro_rules! flex_to_quat {
	($F:expr) => {{
		$F.get_vector().ok().map(|v| mint::Quaternion {
			v: mint::Vector3::from([v.idx(0).as_f32(), v.idx(1).as_f32(), v.idx(2).as_f32()]),
			s: v.idx(3).as_f32(),
		})
	}};
}
