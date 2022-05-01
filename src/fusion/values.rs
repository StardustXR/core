pub type Vec2 = mint::Vector2<f32>;
pub type Vec3 = mint::Vector3<f32>;
pub type Quat = mint::Quaternion<f32>;
pub type Color = color::Rgba;

pub trait FlexBuffConvertable {
	fn convert(self, parent_builder: &mut flexbuffers::Builder);
}
impl FlexBuffConvertable for Vec2 {
	fn convert(self, parent_builder: &mut flexbuffers::Builder) {
		let mut vec2 = parent_builder.start_vector();
		vec2.push(self.x);
		vec2.push(self.y);
		vec2.end_vector();
	}
}
impl FlexBuffConvertable for Vec3 {
	fn convert(self, parent_builder: &mut flexbuffers::Builder) {
		let mut vec3 = parent_builder.start_vector();
		vec3.push(self.x);
		vec3.push(self.y);
		vec3.push(self.z);
		vec3.end_vector();
	}
}
impl FlexBuffConvertable for Quat {
	fn convert(self, parent_builder: &mut flexbuffers::Builder) {
		let mut vec3 = parent_builder.start_vector();
		vec3.push(self.v.x);
		vec3.push(self.v.y);
		vec3.push(self.v.z);
		vec3.push(self.s);
		vec3.end_vector();
	}
}
impl FlexBuffConvertable for Color {
	fn convert(self, parent_builder: &mut flexbuffers::Builder) {
		let mut color = parent_builder.start_vector();
		color.push(self.c.r);
		color.push(self.c.g);
		color.push(self.c.b);
		color.push(self.a);
		color.end_vector();
	}
}
