use crate::protocol::suis::{Finger, Hand, Joint, Pointer, Thumb, Tip};
use crate::protocol::types::Vec3F;
use crate::suis::{Chirality, InputHandler};
use crate::types::Posef;
use glam::{FloatExt, Quat, Vec3A, vec3a};

impl InputHandler {
	pub const QUERY_INTERFACE: &str = "org.stardustxr.SUIS.Handler";
}

impl Default for Joint {
	fn default() -> Self {
		Joint {
			pose: Posef::default(),
			radius: 0.0,
			distance: 0.0,
		}
	}
}

impl Default for Finger {
	fn default() -> Self {
		Finger {
			tip: Default::default(),
			distal: Default::default(),
			intermediate: Default::default(),
			proximal: Default::default(),
			metacarpal: Default::default(),
		}
	}
}

impl Default for Thumb {
	fn default() -> Self {
		Thumb {
			tip: Default::default(),
			distal: Default::default(),
			proximal: Default::default(),
			metacarpal: Default::default(),
		}
	}
}

impl Default for Hand {
	fn default() -> Self {
		Hand {
			chirality: Chirality::Left,
			thumb: Default::default(),
			index: Default::default(),
			middle: Default::default(),
			ring: Default::default(),
			little: Default::default(),
			palm: Default::default(),
			wrist: Default::default(),
			elbow: Default::default(),
		}
	}
}

impl Default for Pointer {
	fn default() -> Self {
		Pointer {
			pose: Posef::default(),
			deepest_point: 0.0,
		}
	}
}

impl Default for Tip {
	fn default() -> Self {
		Tip {
			pose: Posef::default(),
			chirality: None,
			grip_pose: None,
			grip_surface_pose: None,
			simulated_hand: None,
		}
	}
}

// Heuristics made possible by https://github.com/ultraleap/UnityPlugin
// Thank you Leap Motion!

impl Finger {
	/// Length of finger from knuckle to tip.
	pub fn length(&self) -> f32 {
		let proximal_position: Vec3A = self.proximal.pose.translation.mint();
		let distal_position: Vec3A = self.distal.pose.translation.mint();
		let tip_position: Vec3A = self.tip.pose.translation.mint();

		proximal_position.distance(distal_position) + distal_position.distance(tip_position)
	}

	pub fn direction(&self) -> Vec3F {
		let proximal_position: Vec3A = self.proximal.pose.translation.mint();
		let tip_position: Vec3A = self.tip.pose.translation.mint();

		(tip_position - proximal_position).normalize().into()
	}
}

impl Thumb {
	/// Length of thumb from knuckle to tip.
	pub fn length(&self) -> f32 {
		let proximal_position: Vec3A = self.proximal.pose.translation.mint();
		let distal_position: Vec3A = self.distal.pose.translation.mint();
		let tip_position: Vec3A = self.tip.pose.translation.mint();

		proximal_position.distance(distal_position) + distal_position.distance(tip_position)
	}

	pub fn direction(&self) -> Vec3F {
		let proximal_position: Vec3A = self.proximal.pose.translation.mint();
		let tip_position: Vec3A = self.tip.pose.translation.mint();

		(tip_position - proximal_position).normalize().into()
	}
}

impl Hand {
	/// The direction vector pointing out of the palm.
	pub fn palm_normal(&self) -> Vec3F {
		(self.palm.pose.orientation.mint::<Quat>() * vec3a(0.0, -1.0, 0.0)).into()
	}

	/// The direction vector pointing from the palm to thumb.
	pub fn radial_axis(&self) -> Vec3F {
		(self.palm.pose.orientation.mint::<Quat>()
			* if self.chirality == Chirality::Right {
				vec3a(-1.0, 0.0, 0.0)
			} else {
				vec3a(1.0, 0.0, 0.0)
			})
		.into()
	}

	/// The direction vector pointing from the palm towards fingers.
	pub fn distal_axis(&self) -> Vec3F {
		(self.palm.pose.orientation.mint::<Quat>() * vec3a(0.0, 0.0, -1.0)).into()
	}

	pub fn finger_curl(&self, finger: &Finger) -> f32 {
		let distal_axis: Vec3A = self.distal_axis().mint();
		let direction: Vec3A = finger.direction().mint();
		direction.dot(-distal_axis).remap(-1.0, 1.0, 0.0, 1.0)
	}

	pub fn thumb_curl(&self) -> f32 {
		let radial_axis: Vec3A = self.radial_axis().mint();
		let thumb_direction: Vec3A = self.thumb.direction().mint();
		thumb_direction.dot(-radial_axis).remap(-1.0, 1.0, 0.0, 1.0)
	}

	pub fn pinch_distance(&self, finger: &Finger) -> f32 {
		let thumb_tip: Vec3A = self.thumb.tip.pose.translation.mint();
		let finger_tip: Vec3A = finger.tip.pose.translation.mint();
		thumb_tip.distance(finger_tip)
	}

	/// Unstabilized pinch position.
	pub fn pinch_position(&self) -> Vec3F {
		let thumb_tip: Vec3A = self.thumb.tip.pose.translation.mint();
		let index_tip: Vec3A = self.index.tip.pose.translation.mint();

		((2.0 * thumb_tip + index_tip) * 0.3333333).into()
	}

	/// Predicted pinch position without influence from thumb or index tip.
	/// Useful for extremely stable pinch calculations.
	pub fn stable_pinch_position(&self) -> Vec3F {
		let index_knuckle: Vec3A = self.index.proximal.pose.translation.mint();
		let index_length = self.index.length();

		let radial_axis: Vec3A = self.radial_axis().mint();
		let palm_normal: Vec3A = self.palm_normal().mint();
		let distal_axis: Vec3A = self.distal_axis().mint();

		let stable_pinch_position = index_knuckle
			+ (palm_normal * index_length * 0.85)
			+ (distal_axis * index_length * 0.20)
			+ (radial_axis * index_length * 0.20);

		stable_pinch_position.into()
	}

	/// A decent approximation of where the hand will pinch even if index and thumb are far apart.
	pub fn predicted_pinch_position(&self) -> Vec3F {
		let thumb_tip: Vec3A = self.thumb.tip.pose.translation.mint();
		let index_tip: Vec3A = self.index.tip.pose.translation.mint();
		let index_knuckle: Vec3A = self.index.proximal.pose.translation.mint();
		let index_length = self.index.length();

		let radial_axis: Vec3A = self.radial_axis().mint();
		let palm_normal: Vec3A = self.palm_normal().mint();
		let distal_axis: Vec3A = self.distal_axis().mint();

		let thumb_influence = (thumb_tip - index_knuckle)
			.normalize()
			.dot(radial_axis)
			.remap(0.0, 1.0, 0.5, 0.0);

		let mut predicted_pinch_point = index_knuckle
			+ palm_normal * index_length * 0.85
			+ distal_axis * index_length * 0.20
			+ radial_axis * index_length * 0.20;

		predicted_pinch_point = predicted_pinch_point.lerp(thumb_tip, thumb_influence);
		predicted_pinch_point = predicted_pinch_point.lerp(index_tip, 0.15);

		predicted_pinch_point.into()
	}

	fn hand_scale(&self) -> f32 {
		let index_metacarpal: Vec3A = self.index.metacarpal.pose.translation.mint();
		let index_proximal: Vec3A = self.index.proximal.pose.translation.mint();

		let middle_metacarpal: Vec3A = self.middle.metacarpal.pose.translation.mint();
		let middle_proximal: Vec3A = self.middle.proximal.pose.translation.mint();

		let ring_metacarpal: Vec3A = self.ring.metacarpal.pose.translation.mint();
		let ring_proximal: Vec3A = self.ring.proximal.pose.translation.mint();

		let little_metacarpal: Vec3A = self.little.metacarpal.pose.translation.mint();
		let little_proximal: Vec3A = self.little.proximal.pose.translation.mint();

		let mut scale = 0.0;
		scale += index_metacarpal.distance(index_proximal) / 0.06812;
		scale += middle_metacarpal.distance(middle_proximal) / 0.06460;
		scale += ring_metacarpal.distance(ring_proximal) / 0.05800;
		scale += little_metacarpal.distance(little_proximal) / 0.05369;

		scale / 4.0
	}

	/// Confidence value from 0-1 of how strong this hand is pinching.
	pub fn pinch_strength(&self) -> f32 {
		let thumb_tip: Vec3A = self.thumb.tip.pose.translation.mint();
		let index_tip: Vec3A = self.index.tip.pose.translation.mint();
		let middle_tip: Vec3A = self.middle.tip.pose.translation.mint();
		let ring_tip: Vec3A = self.ring.tip.pose.translation.mint();
		let little_tip: Vec3A = self.little.tip.pose.translation.mint();

		let min_distance = index_tip
			.distance_squared(thumb_tip)
			.min(middle_tip.distance_squared(thumb_tip))
			.min(ring_tip.distance_squared(thumb_tip))
			.min(little_tip.distance_squared(thumb_tip))
			.sqrt();

		let scale = self.hand_scale();
		let distance_zero = 0.0600 * scale;
		let distance_one = 0.0220 * scale;

		((min_distance - distance_zero) / (distance_one - distance_zero)).clamp(0.0, 1.0)
	}

	/// Confidence value from 0-1 of how strong this hand is making a fist.
	pub fn fist_strength(&self) -> f32 {
		let radial_axis: Vec3A = self.radial_axis().mint();
		let distal_axis: Vec3A = self.distal_axis().mint();

		let thumb_direction: Vec3A = self.thumb.direction().mint();
		let index_direction: Vec3A = self.index.direction().mint();
		let middle_direction: Vec3A = self.middle.direction().mint();
		let ring_direction: Vec3A = self.ring.direction().mint();
		let little_direction: Vec3A = self.little.direction().mint();

		(thumb_direction.dot(-radial_axis)
			+ index_direction.dot(-distal_axis)
			+ middle_direction.dot(-distal_axis)
			+ ring_direction.dot(-distal_axis)
			+ little_direction.dot(-distal_axis))
		.remap(-5.0, 5.0, 0.0, 1.0)
	}
}

impl Pointer {
	pub fn direction(&self) -> Vec3F {
		(self.pose.orientation.mint::<Quat>() * vec3a(0.0, 0.0, -1.0)).into()
	}
}
