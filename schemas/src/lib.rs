#![allow(unused_imports)]
include!(concat!(env!("OUT_DIR"), "/mod.rs"));

use common::{Quat, QuatT, Vec3, Vec3T};

impl From<mint::Vector3<f32>> for Vec3 {
	fn from(vec: mint::Vector3<f32>) -> Self {
		Vec3::new(vec.x, vec.y, vec.z)
	}
}
impl From<mint::Vector3<f32>> for Vec3T {
	fn from(vec: mint::Vector3<f32>) -> Self {
		let vec: Vec3 = vec.into();
		vec.unpack()
	}
}
impl Into<mint::Vector3<f32>> for Vec3T {
	fn into(self) -> mint::Vector3<f32> {
		mint::Vector3::from([self.x, self.y, self.z])
	}
}

impl From<mint::Quaternion<f32>> for Quat {
	fn from(quat: mint::Quaternion<f32>) -> Self {
		Quat::new(quat.v.x, quat.v.y, quat.v.z, quat.s)
	}
}
impl From<mint::Quaternion<f32>> for QuatT {
	fn from(quat: mint::Quaternion<f32>) -> Self {
		let quat: Quat = quat.into();
		quat.unpack()
	}
}
impl Into<mint::Quaternion<f32>> for QuatT {
	fn into(self) -> mint::Quaternion<f32> {
		mint::Quaternion::from([self.x, self.y, self.z, self.w])
	}
}

// use common::{Joint as FlatJoint, JointT};
// use input_hand::{
// 	Finger as FlatFinger, FingerT, Hand as FlatHand, HandT, Thumb as FlatThumb, ThumbT,
// };

// #[derive(Debug, Clone, Copy)]
// pub struct Joint {
// 	position: mint::Vector3<f32>,
// 	rotation: mint::Quaternion<f32>,
// 	radius: f32,
// }
// impl From<JointT> for Joint {
// 	fn from(joint: JointT) -> Self {
// 		Joint {
// 			position: joint.position.into(),
// 			rotation: joint.rotation.into(),
// 			radius: joint.radius,
// 		}
// 	}
// }
// impl Into<FlatJoint> for Joint {
// 	fn into(self) -> FlatJoint {
// 		FlatJoint::new(&self.position.into(), &self.rotation.into(), self.radius)
// 	}
// }

// #[derive(Debug, Clone, Copy)]
// pub struct Finger {
// 	tip: Joint,
// 	distal: Joint,
// 	intermediate: Joint,
// 	proximal: Joint,
// 	metacarpal: Joint,
// }
// impl From<FingerT> for Finger {
// 	fn from(finger: FingerT) -> Self {
// 		Finger {
// 			tip: finger.tip.into(),
// 			distal: finger.distal.into(),
// 			intermediate: finger.intermediate.into(),
// 			proximal: finger.proximal.into(),
// 			metacarpal: finger.metacarpal.into(),
// 		}
// 	}
// }
// impl Into<FlatFinger> for Finger {
// 	fn into(self) -> FlatFinger {
// 		FlatFinger::new(
// 			&self.tip.into(),
// 			&self.distal.into(),
// 			&self.intermediate.into(),
// 			&self.proximal.into(),
// 			&self.metacarpal.into(),
// 		)
// 	}
// }

// #[derive(Debug, Clone, Copy)]
// pub struct Thumb {
// 	tip: Joint,
// 	distal: Joint,
// 	proximal: Joint,
// 	metacarpal: Joint,
// }
// impl From<ThumbT> for Thumb {
// 	fn from(thumb: ThumbT) -> Self {
// 		Thumb {
// 			tip: thumb.tip.into(),
// 			distal: thumb.distal.into(),
// 			proximal: thumb.proximal.into(),
// 			metacarpal: thumb.metacarpal.into(),
// 		}
// 	}
// }
// impl Into<FlatThumb> for Thumb {
// 	fn into(self) -> FlatThumb {
// 		FlatThumb::new(
// 			&self.tip.into(),
// 			&self.distal.into(),
// 			&self.proximal.into(),
// 			&self.metacarpal.into(),
// 		)
// 	}
// }

// #[derive(Debug, Clone, Copy)]
// pub struct Hand {
// 	thumb: Thumb,
// 	index: Finger,
// 	middle: Finger,
// 	ring: Finger,
// 	little: Finger,
// 	palm: Joint,
// 	wrist: Joint,
// 	elbow: Option<Joint>,
// }

// impl From<HandT> for Hand {
// 	fn from(hand: HandT) -> Self {
// 		Hand {
// 			thumb: hand.thumb.into(),
// 			index: hand.index.into(),
// 			middle: hand.middle.into(),
// 			ring: hand.ring.into(),
// 			little: hand.little.into(),
// 			palm: hand.palm.into(),
// 			wrist: hand.wrist.into(),
// 			elbow: hand.elbow.into(),
// 		}
// 	}
// }
