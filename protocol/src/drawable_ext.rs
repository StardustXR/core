use crate::protocol::lines::LinePoint;
use crate::protocol::types::{Color, Vec3F};
use std::hash::Hash;

impl Default for LinePoint {
    fn default() -> Self {
        Self {
            point: Vec3F {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            thickness: 0.01,
            color: Color::WHITE,
        }
    }
}

impl Hash for LinePoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.color.r.to_bits().hash(state);
        self.color.g.to_bits().hash(state);
        self.color.b.to_bits().hash(state);
        self.color.a.to_bits().hash(state);

        self.point.x.to_bits().hash(state);
        self.point.y.to_bits().hash(state);
        self.point.z.to_bits().hash(state);

        self.thickness.to_bits().hash(state);
    }
}
