#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Types",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "Size2",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Size3",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Vec2f",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Vec3f",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Vec4f",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Quatf",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Mat4f",
            supported_derives: gluon_wire::Derives::from_bits_truncate(3u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Posef",
            supported_derives: gluon_wire::Derives::from_bits_truncate(3u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Color",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Timestamp",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Resource",
            supported_derives: gluon_wire::Derives::from_bits_truncate(30u32),
        },
    ],
};
///2D vector
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Size2 {
    pub(crate) x: u32,
    pub(crate) y: u32,
}
impl gluon_wire::GluonConvertable for Size2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let y = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Size2 { x, y })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(gluon_data)?;
        self.y.write_owned(gluon_data)?;
        Ok(())
    }
}
///3D vector
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub(crate) struct Size3 {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) z: u32,
}
impl gluon_wire::GluonConvertable for Size3 {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        self.z.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let y = gluon_wire::GluonConvertable::read(gluon_data)?;
        let z = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Size3 { x, y, z })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(gluon_data)?;
        self.y.write_owned(gluon_data)?;
        self.z.write_owned(gluon_data)?;
        Ok(())
    }
}
///2D vector
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Vec2F {
    pub(crate) x: f32,
    pub(crate) y: f32,
}
impl gluon_wire::GluonConvertable for Vec2F {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let y = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Vec2F { x, y })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(gluon_data)?;
        self.y.write_owned(gluon_data)?;
        Ok(())
    }
}
///3D vector
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Vec3F {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}
impl gluon_wire::GluonConvertable for Vec3F {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        self.z.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let y = gluon_wire::GluonConvertable::read(gluon_data)?;
        let z = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Vec3F { x, y, z })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(gluon_data)?;
        self.y.write_owned(gluon_data)?;
        self.z.write_owned(gluon_data)?;
        Ok(())
    }
}
///4D vector
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Vec4F {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}
impl gluon_wire::GluonConvertable for Vec4F {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        self.z.write(gluon_data)?;
        self.w.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let y = gluon_wire::GluonConvertable::read(gluon_data)?;
        let z = gluon_wire::GluonConvertable::read(gluon_data)?;
        let w = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Vec4F { x, y, z, w })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(gluon_data)?;
        self.y.write_owned(gluon_data)?;
        self.z.write_owned(gluon_data)?;
        self.w.write_owned(gluon_data)?;
        Ok(())
    }
}
///Quaternion
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Quatf {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}
impl gluon_wire::GluonConvertable for Quatf {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(gluon_data)?;
        self.y.write(gluon_data)?;
        self.z.write(gluon_data)?;
        self.w.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let y = gluon_wire::GluonConvertable::read(gluon_data)?;
        let z = gluon_wire::GluonConvertable::read(gluon_data)?;
        let w = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Quatf { x, y, z, w })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(gluon_data)?;
        self.y.write_owned(gluon_data)?;
        self.z.write_owned(gluon_data)?;
        self.w.write_owned(gluon_data)?;
        Ok(())
    }
}
///Colum major matrix
#[derive(Debug, Copy, Clone)]
pub(crate) struct Mat4F {
    pub(crate) x: crate::types::Vec4F,
    pub(crate) y: crate::types::Vec4F,
    pub(crate) z: crate::types::Vec4F,
    pub(crate) w: crate::types::Vec4F,
}
impl gluon_wire::GluonConvertable for Mat4F {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: Vec4F = self.x.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Vec4F = self.y.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Vec4F = self.z.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Vec4F = self.w.clone().into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x: crate::types::Vec4F = {
            let __w: Vec4F = gluon_wire::GluonConvertable::read(gluon_data)?;
            __w.into()
        };
        let y: crate::types::Vec4F = {
            let __w: Vec4F = gluon_wire::GluonConvertable::read(gluon_data)?;
            __w.into()
        };
        let z: crate::types::Vec4F = {
            let __w: Vec4F = gluon_wire::GluonConvertable::read(gluon_data)?;
            __w.into()
        };
        let w: crate::types::Vec4F = {
            let __w: Vec4F = gluon_wire::GluonConvertable::read(gluon_data)?;
            __w.into()
        };
        Ok(Mat4F { x, y, z, w })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: Vec4F = self.x.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Vec4F = self.y.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Vec4F = self.z.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Vec4F = self.w.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///Pose
#[derive(Debug, Copy, Clone)]
pub struct Posef {
    pub position: crate::types::Vec3F,
    pub orientation: crate::types::QuatF,
}
impl gluon_wire::GluonConvertable for Posef {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: Vec3F = self.position.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Quatf = self.orientation.clone().into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let position: crate::types::Vec3F = {
            let __w: Vec3F = gluon_wire::GluonConvertable::read(gluon_data)?;
            __w.into()
        };
        let orientation: crate::types::QuatF = {
            let __w: Quatf = gluon_wire::GluonConvertable::read(gluon_data)?;
            __w.into()
        };
        Ok(Posef { position, orientation })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: Vec3F = self.position.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Quatf = self.orientation.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///A color in linear rgb, premultiplied alpha
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Color {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
    pub(crate) a: f32,
}
impl gluon_wire::GluonConvertable for Color {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.r.write(gluon_data)?;
        self.g.write(gluon_data)?;
        self.b.write(gluon_data)?;
        self.a.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let r = gluon_wire::GluonConvertable::read(gluon_data)?;
        let g = gluon_wire::GluonConvertable::read(gluon_data)?;
        let b = gluon_wire::GluonConvertable::read(gluon_data)?;
        let a = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Color { r, g, b, a })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.r.write_owned(gluon_data)?;
        self.g.write_owned(gluon_data)?;
        self.b.write_owned(gluon_data)?;
        self.a.write_owned(gluon_data)?;
        Ok(())
    }
}
///Timestamp on the monotonic clock
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanoseconds: i64,
}
impl gluon_wire::GluonConvertable for Timestamp {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.seconds.write(gluon_data)?;
        self.nanoseconds.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let seconds = gluon_wire::GluonConvertable::read(gluon_data)?;
        let nanoseconds = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Timestamp { seconds, nanoseconds })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.seconds.write_owned(gluon_data)?;
        self.nanoseconds.write_owned(gluon_data)?;
        Ok(())
    }
}
///An identifier to a resource, such as a sound, model or texture
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Resource {
    /**An absolute path to a resource, not themed at all.
You should only use this for content not included with your client.*/
    Direct { path: String },
    /**A resource that is relative to a prefix, meant for resources that are included with the client.
Allows switching of prefix by the server as well to theme clients.*/
    Namespaced {
        ///Group that this resource is in, generally the client or library's name.
        namespace: String,
        ///Path inside namespace without the file extension, must be relative
        path: String,
    },
}
impl gluon_wire::GluonConvertable for Resource {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            Resource::Direct { path } => {
                gluon_data.write_u16(0u16)?;
                path.write(gluon_data)?;
            }
            Resource::Namespaced { namespace, path } => {
                gluon_data.write_u16(1u16)?;
                namespace.write(gluon_data)?;
                path.write(gluon_data)?;
            }
        };
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => {
                    let path = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Resource::Direct { path }
                }
                1u16 => {
                    let namespace = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let path = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Resource::Namespaced {
                        namespace,
                        path,
                    }
                }
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            Resource::Direct { path } => {
                gluon_data.write_u16(0u16)?;
                path.write_owned(gluon_data)?;
            }
            Resource::Namespaced { namespace, path } => {
                gluon_data.write_u16(1u16)?;
                namespace.write_owned(gluon_data)?;
                path.write_owned(gluon_data)?;
            }
        };
        Ok(())
    }
}
