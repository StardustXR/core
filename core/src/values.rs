use mint::{Quaternion, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
	pub position: Option<Vector3<f32>>,
	pub rotation: Option<Quaternion<f32>>,
	pub scale: Option<Vector3<f32>>,
}
