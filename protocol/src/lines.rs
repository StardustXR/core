use color::rgba_linear;

pub use crate::protocol::lines::*;

impl Default for LinePoint {
	fn default() -> Self {
		Self {
			point: [0.0; 3].into(),
			thickness: 0.01,
			color: rgba_linear!(1.0, 1.0, 1.0, 1.0),
		}
	}
}

impl std::hash::Hash for LinePoint {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.color.c.r.to_bits().hash(state);
		self.color.c.g.to_bits().hash(state);
		self.color.c.b.to_bits().hash(state);
		self.color.a.to_bits().hash(state);

		self.point.x.to_bits().hash(state);
		self.point.y.to_bits().hash(state);
		self.point.z.to_bits().hash(state);

		self.thickness.to_bits().hash(state);
	}
}
