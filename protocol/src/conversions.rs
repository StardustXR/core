use crate::protocol::types::{Color, Mat4F, Quatf, Vec2F, Vec3F, Vec4F};

// Vec2F <-> glam::Vec2
impl From<Vec2F> for glam::Vec2 {
    fn from(v: Vec2F) -> Self {
        glam::Vec2::new(v.x, v.y)
    }
}
impl From<glam::Vec2> for Vec2F {
    fn from(v: glam::Vec2) -> Self {
        Vec2F { x: v.x, y: v.y }
    }
}
impl From<[f32; 2]> for Vec2F {
    fn from(v: [f32; 2]) -> Self {
        Vec2F { x: v[0], y: v[1] }
    }
}
impl From<Vec2F> for [f32; 2] {
    fn from(v: Vec2F) -> Self {
        [v.x, v.y]
    }
}

// Vec3F <-> glam::Vec3 / Vec3A
impl From<Vec3F> for glam::Vec3 {
    fn from(v: Vec3F) -> Self {
        glam::Vec3::new(v.x, v.y, v.z)
    }
}
impl From<glam::Vec3> for Vec3F {
    fn from(v: glam::Vec3) -> Self {
        Vec3F {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
impl From<Vec3F> for glam::Vec3A {
    fn from(v: Vec3F) -> Self {
        glam::Vec3A::new(v.x, v.y, v.z)
    }
}
impl From<glam::Vec3A> for Vec3F {
    fn from(v: glam::Vec3A) -> Self {
        Vec3F {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
impl From<[f32; 3]> for Vec3F {
    fn from(v: [f32; 3]) -> Self {
        Vec3F {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}
impl From<Vec3F> for [f32; 3] {
    fn from(v: Vec3F) -> Self {
        [v.x, v.y, v.z]
    }
}

// Vec4F <-> glam::Vec4
impl From<Vec4F> for glam::Vec4 {
    fn from(v: Vec4F) -> Self {
        glam::Vec4::new(v.x, v.y, v.z, v.w)
    }
}
impl From<glam::Vec4> for Vec4F {
    fn from(v: glam::Vec4) -> Self {
        Vec4F {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}

// Quatf <-> glam::Quat
impl From<Quatf> for glam::Quat {
    fn from(q: Quatf) -> Self {
        glam::Quat::from_xyzw(q.x, q.y, q.z, q.w)
    }
}
impl From<glam::Quat> for Quatf {
    fn from(q: glam::Quat) -> Self {
        Quatf {
            x: q.x,
            y: q.y,
            z: q.z,
            w: q.w,
        }
    }
}

// Mat4F <-> glam::Mat4
impl From<Mat4F> for glam::Mat4 {
    fn from(m: Mat4F) -> Self {
        glam::Mat4::from_cols(m.x.into(), m.y.into(), m.z.into(), m.w.into())
    }
}
impl From<glam::Mat4> for Mat4F {
    fn from(m: glam::Mat4) -> Self {
        Mat4F {
            x: m.x_axis.into(),
            y: m.y_axis.into(),
            z: m.z_axis.into(),
            w: m.w_axis.into(),
        }
    }
}

// Color convenience
impl Color {
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color {
            r,
            g,
            b,
            a: 1.0,
        }
    }
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    pub const TRANSPARENT: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
}
