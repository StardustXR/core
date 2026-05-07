use rustix::time::ClockId;

use crate::types::{Posef, Quatf, Timestamp, Vec2F, Vec3F, Vec4F};

impl Default for Vec2F {
	fn default() -> Self {
		Self { x: 0.0, y: 0.0 }
	}
}
impl Default for Vec3F {
	fn default() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}
}
impl Default for Vec4F {
	fn default() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0,
			w: 0.0,
		}
	}
}
impl Default for Quatf {
	fn default() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0,
			w: 1.0,
		}
	}
}
impl Default for Posef {
	fn default() -> Self {
		Self {
			position: Vec3F::default(),
			orientation: Quatf::default(),
		}
	}
}

pub trait TimestampExt {
	fn now() -> Self;
}
impl TimestampExt for Timestamp {
	fn now() -> Self {
		let time = rustix::time::clock_gettime(ClockId::Monotonic);
		Timestamp {
			seconds: time.tv_sec,
			nanoseconds: time.tv_nsec,
		}
	}
}
