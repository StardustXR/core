use color::{AlphaColor, Rgba, color_space::LinearRgb};
use rustix::time::ClockId;

pub type Size2 = mint::Vector2<u32>;
pub type Size3 = mint::Vector3<u32>;
pub type Vec2F = mint::Vector2<f32>;
pub type Vec3F = mint::Vector3<f32>;
pub type Vec4F = mint::Vector4<f32>;
pub type QuatF = mint::Quaternion<f32>;
pub type Mat4F = mint::ColumnMatrix4<f32>;

use crate::protocol::types::proxied::{
	Mat4F as ProtocolMat4F, Quatf as ProtocolQuatF, Size2 as ProtocolSize2, Size3 as ProtocolSize3,
	Vec2F as ProtocolVec2F, Vec3F as ProtocolVec3F, Vec4F as ProtocolVec4F,
};
pub use crate::protocol::types::{EXTERNAL_PROTOCOL, Posef, Resource, Timestamp};
use crate::protocol::types::{Posef as ProtocolPosef, Timestamp as ProtocolTimestamp};

pub use color::rgba_linear;
pub type Color = color::Rgba<f32, color::color_space::LinearRgb>;

// Size2
impl From<Size2> for ProtocolSize2 {
	fn from(v: Size2) -> Self {
		Self { x: v.x, y: v.y }
	}
}
impl From<ProtocolSize2> for Size2 {
	fn from(v: ProtocolSize2) -> Self {
		Self { x: v.x, y: v.y }
	}
}

// Size3
impl From<Size3> for ProtocolSize3 {
	fn from(v: Size3) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
		}
	}
}
impl From<ProtocolSize3> for Size3 {
	fn from(v: ProtocolSize3) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
		}
	}
}

// Vector2
impl From<Vec2F> for ProtocolVec2F {
	fn from(v: Vec2F) -> Self {
		Self { x: v.x, y: v.y }
	}
}
impl From<ProtocolVec2F> for Vec2F {
	fn from(v: ProtocolVec2F) -> Self {
		Self { x: v.x, y: v.y }
	}
}

// Vector3
impl From<Vec3F> for ProtocolVec3F {
	fn from(v: Vec3F) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
		}
	}
}
impl From<ProtocolVec3F> for Vec3F {
	fn from(v: ProtocolVec3F) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
		}
	}
}

// Vector4
impl From<Vec4F> for ProtocolVec4F {
	fn from(v: Vec4F) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
			w: v.w,
		}
	}
}
impl From<ProtocolVec4F> for Vec4F {
	fn from(v: ProtocolVec4F) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
			w: v.w,
		}
	}
}

// Quaternion
impl From<QuatF> for ProtocolQuatF {
	fn from(v: QuatF) -> Self {
		Self {
			x: v.v.x,
			y: v.v.y,
			z: v.v.z,
			w: v.s,
		}
	}
}
impl From<ProtocolQuatF> for QuatF {
	fn from(v: ProtocolQuatF) -> Self {
		Self {
			v: mint::Vector3 {
				x: v.x,
				y: v.y,
				z: v.z,
			},
			s: v.w,
		}
	}
}

// Pose
impl Default for Posef {
	fn default() -> Self {
		Self {
			position: [0.0; 3].into(),
			orientation: QuatF {
				v: [0.0; 3].into(),
				s: 1.0,
			},
		}
	}
}

// Matrix4
impl From<Mat4F> for ProtocolMat4F {
	fn from(value: Mat4F) -> Self {
		Self {
			x: value.x,
			y: value.y,
			z: value.z,
			w: value.w,
		}
	}
}
impl From<ProtocolMat4F> for Mat4F {
	fn from(value: ProtocolMat4F) -> Self {
		Self {
			x: value.x,
			y: value.y,
			z: value.z,
			w: value.w,
		}
	}
}

// Color
impl From<Color> for crate::protocol::types::proxied::Color {
	fn from(value: color::Rgba<f32, LinearRgb>) -> Self {
		Self {
			r: value.c.r,
			g: value.c.g,
			b: value.c.b,
			a: value.a,
		}
	}
}
impl From<crate::protocol::types::proxied::Color> for Color {
	fn from(value: crate::protocol::types::proxied::Color) -> Self {
		color::rgba_linear!(value.r, value.g, value.b, value.a)
	}
}

// Timestamp
impl Timestamp {
	pub fn now() -> Self {
		let time = rustix::time::clock_gettime(ClockId::Monotonic);
		Timestamp {
			seconds: time.tv_sec,
			nanoseconds: time.tv_nsec,
		}
	}
}
