use crate::input_tip::TipT;
use mint::{Quaternion, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Tip {
	pub origin: Vector3<f32>,
	pub orientation: Quaternion<f32>,
	pub radius: f32,
}
impl Default for Tip {
	fn default() -> Self {
		Self {
			origin: Vector3::from([0.0; 3]),
			orientation: Quaternion::from([0.0, 0.0, 0.0, 1.0]),
			radius: Default::default(),
		}
	}
}

impl From<TipT> for Tip {
	fn from(tip: TipT) -> Self {
		Tip {
			origin: tip.origin.into(),
			orientation: tip.orientation.into(),
			radius: tip.radius,
		}
	}
}
impl From<Tip> for TipT {
	fn from(tip: Tip) -> Self {
		TipT {
			origin: tip.origin.into(),
			orientation: tip.orientation.into(),
			radius: tip.radius,
		}
	}
}
