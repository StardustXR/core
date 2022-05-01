use flexbuffers::{Builder, VectorBuilder};
struct Color {
    h: f32,
    s: f32,
    v: f32,
    a: f32
}
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}
struct Vec2 {
    x: f32,
    y: f32
}
struct Quaternion {
    i: f32,
    j: f32,
    k: f32,
    l: f32
}
trait FlexBuffConvertable {
    fn convert(self, parent_builder: Builder) -> Builder;
}
impl FlexBuffConvertable for Vec3 {
    fn convert(self, mut parent_builder: Builder) -> Builder {
        let mut vec3: VectorBuilder = parent_builder.start_vector();
        vec3.push(self.x);
        vec3.push(self.y);
        vec3.push(self.z);
        vec3.end_vector();
        return parent_builder;
    }
}
impl FlexBuffConvertable for Vec2 {
    fn convert(self, mut parent_builder: Builder) -> Builder {
        let mut vec2: VectorBuilder = parent_builder.start_vector();
        vec2.push(self.x);
        vec2.push(self.y);
        vec2.end_vector();
        return parent_builder;
    }
}
impl FlexBuffConvertable for Color {
    fn convert(self, mut parent_builder: Builder) -> Builder {
        let mut color: VectorBuilder = parent_builder.start_vector();
        color.push(self.h);
        color.push(self.s);
        color.push(self.v);
        color.push(self.a);
        color.end_vector();
        return parent_builder;
    }
}
