pub struct Vec2(pub mint::Vector2<f32>);
pub struct Vec3(pub mint::Vector3<f32>);
pub struct Quat(pub mint::Quaternion<f32>);
pub struct Color(pub color::Rgba);

#[macro_export]
macro_rules! flex_vec2 {
	($B:expr, $V:expr) => {{
		let mut vec = $B.start_vector();
		vec.push($V.0.x);
		vec.push($V.0.y);
		vec.end_vector();
	}}
}
#[macro_export]
macro_rules! flex_vec3 {
	($B:expr, $V:expr) => {{
		let mut vec = $B.start_vector();
		vec.push($V.0.x);
		vec.push($V.0.y);
		vec.push($V.0.z);
		vec.end_vector();
	}}
}
#[macro_export]
macro_rules! flex_quat {
	($B:expr, $V:expr) => {{
		let mut vec = $B.start_vector();
		vec.push($V.0.v.x);
		vec.push($V.0.v.y);
		vec.push($V.0.v.z);
		vec.push($V.0.s);
		vec.end_vector();
	}}
}


// pub trait FlexBuffConvertable {
// 	fn convert(self, parent_builder: &mut flexbuffers::Builder);
// }
// impl FlexBuffConvertable for Vec2 {
// 	fn convert(self, parent_builder: &mut flexbuffers::Builder) {
// 		let mut vec2 = parent_builder.start_vector();
// 		vec2.push(self.0.x);
// 		vec2.push(self.0.y);
// 		vec2.end_vector();
// 	}
// }
// impl FlexBuffConvertable for Vec3 {
// 	fn convert(self, parent_builder: &mut flexbuffers::Builder) {
// 		let mut vec3 = parent_builder.start_vector();
// 		vec3.push(self.0.x);
// 		vec3.push(self.0.y);
// 		vec3.push(self.0.z);
// 		vec3.end_vector();
// 	}
// }
// impl FlexBuffConvertable for Quat {
// 	fn convert(self, parent_builder: &mut flexbuffers::Builder) {
// 		let mut vec3 = parent_builder.start_vector();
// 		vec3.push(self.0.v.x);
// 		vec3.push(self.0.v.y);
// 		vec3.push(self.0.v.z);
// 		vec3.push(self.0.s);
// 		vec3.end_vector();
// 	}
// }
// impl FlexBuffConvertable for Color {
// 	fn convert(self, parent_builder: &mut flexbuffers::Builder) {
// 		let mut color = parent_builder.start_vector();
// 		color.push(self.0.c.r);
// 		color.push(self.0.c.g);
// 		color.push(self.0.c.b);
// 		color.push(self.0.a);
// 		color.end_vector();
// 	}
// }


