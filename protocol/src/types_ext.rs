use crate::types::{Mat4F, Posef, Quatf, Timestamp, Vec2F, Vec3F, Vec4F};
use color::{AlphaColor, Rgba, color_space::LinearRgb};
use mint::{Quaternion, Vector3};
use rustix::time::ClockId;

pub use color::rgba_linear;
pub type Color = color::Rgba<f32, color::color_space::LinearRgb>;

impl Default for Vec2F {
	fn default() -> Self {
		Self { x: 0.0, y: 0.0 }
	}
}
impl From<mint::Vector2<f32>> for Vec2F {
	fn from(v: mint::Vector2<f32>) -> Self {
		Self { x: v.x, y: v.y }
	}
}
impl From<Vec2F> for mint::Vector2<f32> {
	fn from(v: Vec2F) -> Self {
		Self { x: v.x, y: v.y }
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
impl From<mint::Vector3<f32>> for Vec3F {
	fn from(v: mint::Vector3<f32>) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
		}
	}
}
impl From<Vec3F> for mint::Vector3<f32> {
	fn from(v: Vec3F) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
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
impl From<mint::Vector4<f32>> for Vec4F {
	fn from(v: mint::Vector4<f32>) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
			w: v.w,
		}
	}
}
impl From<Vec4F> for mint::Vector4<f32> {
	fn from(v: Vec4F) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
			w: v.w,
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
impl From<mint::Quaternion<f32>> for Quatf {
	fn from(v: mint::Quaternion<f32>) -> Self {
		Self {
			x: v.v.x,
			y: v.v.y,
			z: v.v.z,
			w: v.s,
		}
	}
}
impl From<Quatf> for mint::Quaternion<f32> {
	fn from(v: Quatf) -> Self {
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

impl Default for Posef {
	fn default() -> Self {
		Self {
			position: [0.0; 3].into(),
			orientation: Quaternion {
				v: [0.0; 3].into(),
				s: 1.0,
			},
		}
	}
}

impl From<mint::ColumnMatrix4<f32>> for Mat4F {
	fn from(value: mint::ColumnMatrix4<f32>) -> Self {
		Self {
			x: value.x.into(),
			y: value.y.into(),
			z: value.z.into(),
			w: value.w.into(),
		}
	}
}
impl From<Mat4F> for mint::ColumnMatrix4<f32> {
	fn from(value: Mat4F) -> Self {
		Self {
			x: value.x.into(),
			y: value.y.into(),
			z: value.z.into(),
			w: value.w.into(),
		}
	}
}

impl From<Color> for crate::types::Color {
	fn from(value: color::Rgba<f32, LinearRgb>) -> Self {
		Self {
			r: value.c.r,
			g: value.c.g,
			b: value.c.b,
			a: value.a,
		}
	}
}
impl From<crate::types::Color> for Color {
	fn from(value: crate::types::Color) -> Self {
		color::rgba_linear!(value.r, value.g, value.b, value.a)
	}
}

impl Timestamp {
	pub fn now() -> Self {
		let time = rustix::time::clock_gettime(ClockId::Monotonic);
		Timestamp {
			seconds: time.tv_sec,
			nanoseconds: time.tv_nsec,
		}
	}
}
