use mint::{ColumnMatrix4, Quaternion, Vector2, Vector3, Vector4};

use crate::protocol::types::{Color, Mat4F, Quatf, Vec2F, Vec3F, Vec4F};

macro_rules! impl_mint {
    ($proto:ty, $mint:ty, $($field:ident),*) => {

	impl $proto {
	    /// Convert to any mint compatible type
	    pub fn mint<T: From<$mint>>(&self) -> T {
		type Mint = $mint;
		let v = Mint {$($field: self.$field),*};
		v.into()
	    }
	}
	impl<T: Into<$mint>> From<T> for $proto {
	    fn from(v: T) -> Self {
		let v: $mint = v.into();
		Self {$($field: v.$field),*}
	    }
	}
    };
}

impl_mint!(Vec2F, Vector2<f32>, x, y);
impl_mint!(Vec3F, Vector3<f32>, x, y, z);
impl_mint!(Vec4F, Vector4<f32>, x, y, z, w);

impl Quatf {
	/// Convert to any mint compatible type
	pub fn mint<T: From<Quaternion<f32>>>(&self) -> T {
		let v = Quaternion {
			v: Vector3 {
				x: self.x,
				y: self.y,
				z: self.z,
			},
			s: self.w,
		};
		v.into()
	}
}
impl<T: Into<Quaternion<f32>>> From<T> for Quatf {
	fn from(v: T) -> Self {
		let v: Quaternion<f32> = v.into();
		Self {
			x: v.v.x,
			y: v.v.y,
			z: v.v.z,
			w: v.s,
		}
	}
}

impl Mat4F {
	/// Convert to any mint compatible type
	pub fn mint<T: From<ColumnMatrix4<f32>>>(&self) -> T {
		let v = ColumnMatrix4 {
			x: self.x.mint(),
			y: self.y.mint(),
			z: self.z.mint(),
			w: self.w.mint(),
		};
		v.into()
	}
}
impl<T: Into<ColumnMatrix4<f32>>> From<T> for Mat4F {
	fn from(v: T) -> Self {
		let v: ColumnMatrix4<f32> = v.into();
		Self {
			x: v.x.into(),
			y: v.y.into(),
			z: v.z.into(),
			w: v.w.into(),
		}
	}
}

// Color convenience
impl Color {
	pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
		Color { r, g, b, a }
	}
	pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
		Color { r, g, b, a: 1.0 }
	}
	pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
	pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
	pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
}
