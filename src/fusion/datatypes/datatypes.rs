use flexbuffers::{Builder, VectorBuilder};
use mint::{Vector2, Vector3, Quaternion};
use color::Hsva;
trait FlexBuffConvertable {
    fn convert(self, parent_builder: Builder) -> Builder;
}
impl FlexBuffConvertable for Vector3<f32> {
    fn convert(self, mut parent_builder: Builder) -> Builder {
        let mut vec3: VectorBuilder = parent_builder.start_vector();
        vec3.push(self.x);
        vec3.push(self.y);
        vec3.push(self.z);
        vec3.end_vector();
        return parent_builder;
    }
}
impl FlexBuffConvertable for Vector2<f32> {
    fn convert(self, mut parent_builder: Builder) -> Builder {
        let mut vec2: VectorBuilder = parent_builder.start_vector();
        vec2.push(self.x);
        vec2.push(self.y);
        vec2.end_vector();
        return parent_builder;
    }
}
impl FlexBuffConvertable for Hsva {
    fn convert(self, mut parent_builder: Builder) -> Builder {
        let mut color: VectorBuilder = parent_builder.start_vector();
        color.push(self.c.h);
        color.push(self.c.s);
        color.push(self.c.v);
        color.push(self.a);
        color.end_vector();
        return parent_builder;
    }
}
