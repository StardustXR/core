use crate::generated::{
	common::JointT,
	input_hand::{FingerT, HandT, ThumbT},
};
use glam::Quat;
use mint::{Quaternion, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Joint {
	pub position: Vector3<f32>,
	pub rotation: Quaternion<f32>,
	pub radius: f32,
}
impl Default for Joint {
	fn default() -> Self {
		Self {
			position: Vector3::from([0.0; 3]),
			rotation: Quaternion::from([0.0, 0.0, 0.0, 1.0]),
			radius: Default::default(),
		}
	}
}
impl From<JointT> for Joint {
	fn from(joint: JointT) -> Self {
		Joint {
			position: joint.position.into(),
			rotation: joint.rotation.into(),
			radius: joint.radius,
		}
	}
}
impl From<Joint> for JointT {
	fn from(joint: Joint) -> Self {
		JointT {
			position: joint.position.into(),
			rotation: joint.rotation.into(),
			radius: joint.radius,
		}
	}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Finger {
	pub tip: Joint,
	pub distal: Joint,
	pub intermediate: Joint,
	pub proximal: Joint,
	pub metacarpal: Joint,
}
impl From<FingerT> for Finger {
	fn from(finger: FingerT) -> Self {
		Finger {
			tip: finger.tip.into(),
			distal: finger.distal.into(),
			intermediate: finger.intermediate.into(),
			proximal: finger.proximal.into(),
			metacarpal: finger.metacarpal.into(),
		}
	}
}
impl From<Finger> for FingerT {
	fn from(finger: Finger) -> Self {
		FingerT {
			tip: finger.tip.into(),
			distal: finger.distal.into(),
			intermediate: finger.intermediate.into(),
			proximal: finger.proximal.into(),
			metacarpal: finger.metacarpal.into(),
		}
	}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Thumb {
	pub tip: Joint,
	pub distal: Joint,
	pub proximal: Joint,
	pub metacarpal: Joint,
}
impl From<ThumbT> for Thumb {
	fn from(thumb: ThumbT) -> Self {
		Thumb {
			tip: thumb.tip.into(),
			distal: thumb.distal.into(),
			proximal: thumb.proximal.into(),
			metacarpal: thumb.metacarpal.into(),
		}
	}
}
impl From<Thumb> for ThumbT {
	fn from(thumb: Thumb) -> Self {
		ThumbT {
			tip: thumb.tip.into(),
			distal: thumb.distal.into(),
			proximal: thumb.proximal.into(),
			metacarpal: thumb.metacarpal.into(),
		}
	}
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Hand {
	pub right: bool,
	pub thumb: Thumb,
	pub index: Finger,
	pub middle: Finger,
	pub ring: Finger,
	pub little: Finger,
	pub palm: Joint,
	pub wrist: Joint,
	pub elbow: Option<Joint>,
}
impl From<HandT> for Hand {
	fn from(hand: HandT) -> Self {
		Hand {
			right: hand.right,
			thumb: hand.thumb.into(),
			index: hand.index.into(),
			middle: hand.middle.into(),
			ring: hand.ring.into(),
			little: hand.little.into(),
			palm: hand.palm.into(),
			wrist: hand.wrist.into(),
			elbow: hand.elbow.map(|elbow| elbow.into()),
		}
	}
}
impl From<Hand> for HandT {
	fn from(hand: Hand) -> Self {
		HandT {
			right: hand.right,
			thumb: hand.thumb.into(),
			index: hand.index.into(),
			middle: hand.middle.into(),
			ring: hand.ring.into(),
			little: hand.little.into(),
			palm: hand.palm.into(),
			wrist: hand.wrist.into(),
			elbow: hand.elbow.map(|elbow| elbow.into()),
		}
	}
}
