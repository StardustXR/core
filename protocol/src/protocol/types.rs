#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Types",
    types: &[
        gluon::ExternalGluonType {
            name: "Size2",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: Some("proxies::Size2"),
        },
        gluon::ExternalGluonType {
            name: "Size3",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: Some("proxies::Size3"),
        },
        gluon::ExternalGluonType {
            name: "Vec2f",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: Some("proxies::Vec2F"),
        },
        gluon::ExternalGluonType {
            name: "Vec3f",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: Some("proxies::Vec3F"),
        },
        gluon::ExternalGluonType {
            name: "Vec4f",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: Some("proxies::Vec4F"),
        },
        gluon::ExternalGluonType {
            name: "Quatf",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: Some("proxies::QuatF"),
        },
        gluon::ExternalGluonType {
            name: "Mat4f",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: Some("proxies::Mat4F"),
        },
        gluon::ExternalGluonType {
            name: "Posef",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Color",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: Some("proxies::Color"),
        },
        gluon::ExternalGluonType {
            name: "Timestamp",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Resource",
            supported_derives: gluon::Derives::from_bits_truncate(798u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "ResourceLoadError",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "CreateError",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
    pub use crate::types::Size2;
    pub use crate::types::Size3;
    pub use crate::types::Vec2F;
    pub use crate::types::Vec3F;
    pub use crate::types::Vec4F;
    pub use crate::types::QuatF;
    pub use crate::types::Mat4F;
    pub use crate::types::Color;
}
///Pose
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Posef {
    pub position: crate::types::Vec3F,
    pub orientation: crate::types::QuatF,
}
impl gluon::Convertable for Posef {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: proxied::Vec3F = self.position.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: proxied::Quatf = self.orientation.clone().into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let position: crate::types::Vec3F = {
            let __w: proxied::Vec3F = gluon::Convertable::read(gluon_data)?;
            __w.into()
        };
        let orientation: crate::types::QuatF = {
            let __w: proxied::Quatf = gluon::Convertable::read(gluon_data)?;
            __w.into()
        };
        Ok(Posef { position, orientation })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: proxied::Vec3F = self.position.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: proxied::Quatf = self.orientation.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///Timestamp on the monotonic clock
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Timestamp {
    pub seconds: i64,
    pub nanoseconds: i64,
}
impl gluon::Convertable for Timestamp {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.seconds.write(gluon_data)?;
        self.nanoseconds.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let seconds = gluon::Convertable::read(gluon_data)?;
        let nanoseconds = gluon::Convertable::read(gluon_data)?;
        Ok(Timestamp { seconds, nanoseconds })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.seconds.write_owned(gluon_data)?;
        self.nanoseconds.write_owned(gluon_data)?;
        Ok(())
    }
}
///An identifier to a resource, such as a sound, model or texture
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl gluon::Convertable for Resource {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
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
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => {
                    let path = gluon::Convertable::read(gluon_data)?;
                    Resource::Direct { path }
                }
                1u16 => {
                    let namespace = gluon::Convertable::read(gluon_data)?;
                    let path = gluon::Convertable::read(gluon_data)?;
                    Resource::Namespaced {
                        namespace,
                        path,
                    }
                }
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
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
///Error potentially produced when loading a resource
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ResourceLoadError {
    ///invalid Ref used
    InvalidRef,
    NotFound,
}
impl gluon::Convertable for ResourceLoadError {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            ResourceLoadError::InvalidRef => {
                gluon_data.write_u16(0u16)?;
            }
            ResourceLoadError::NotFound => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => ResourceLoadError::InvalidRef,
                1u16 => ResourceLoadError::NotFound,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            ResourceLoadError::InvalidRef => {
                gluon_data.write_u16(0u16)?;
            }
            ResourceLoadError::NotFound => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
}
///Error potentially produced when creating an interface
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CreateError {
    ///invalid Ref used
    InvalidRef,
}
impl gluon::Convertable for CreateError {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            CreateError::InvalidRef => {
                gluon_data.write_u16(0u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => CreateError::InvalidRef,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            CreateError::InvalidRef => {
                gluon_data.write_u16(0u16)?;
            }
        };
        Ok(())
    }
}
pub mod proxied {
    use super::*;
    ///2D vector
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Size2 {
        pub x: u32,
        pub y: u32,
    }
    impl gluon::Convertable for Size2 {
        fn write<'a, 'b: 'a>(
            &'b self,
            gluon_data: &mut gluon::DataBuilder<'a>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write(gluon_data)?;
            self.y.write(gluon_data)?;
            Ok(())
        }
        fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
            let x = gluon::Convertable::read(gluon_data)?;
            let y = gluon::Convertable::read(gluon_data)?;
            Ok(Size2 { x, y })
        }
        fn write_owned(
            self,
            gluon_data: &mut gluon::DataBuilder<'_>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write_owned(gluon_data)?;
            self.y.write_owned(gluon_data)?;
            Ok(())
        }
    }
    ///3D vector
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Size3 {
        pub x: u32,
        pub y: u32,
        pub z: u32,
    }
    impl gluon::Convertable for Size3 {
        fn write<'a, 'b: 'a>(
            &'b self,
            gluon_data: &mut gluon::DataBuilder<'a>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write(gluon_data)?;
            self.y.write(gluon_data)?;
            self.z.write(gluon_data)?;
            Ok(())
        }
        fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
            let x = gluon::Convertable::read(gluon_data)?;
            let y = gluon::Convertable::read(gluon_data)?;
            let z = gluon::Convertable::read(gluon_data)?;
            Ok(Size3 { x, y, z })
        }
        fn write_owned(
            self,
            gluon_data: &mut gluon::DataBuilder<'_>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write_owned(gluon_data)?;
            self.y.write_owned(gluon_data)?;
            self.z.write_owned(gluon_data)?;
            Ok(())
        }
    }
    ///2D vector
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Vec2F {
        pub x: f32,
        pub y: f32,
    }
    impl gluon::Convertable for Vec2F {
        fn write<'a, 'b: 'a>(
            &'b self,
            gluon_data: &mut gluon::DataBuilder<'a>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write(gluon_data)?;
            self.y.write(gluon_data)?;
            Ok(())
        }
        fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
            let x = gluon::Convertable::read(gluon_data)?;
            let y = gluon::Convertable::read(gluon_data)?;
            Ok(Vec2F { x, y })
        }
        fn write_owned(
            self,
            gluon_data: &mut gluon::DataBuilder<'_>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write_owned(gluon_data)?;
            self.y.write_owned(gluon_data)?;
            Ok(())
        }
    }
    ///3D vector
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Vec3F {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
    impl gluon::Convertable for Vec3F {
        fn write<'a, 'b: 'a>(
            &'b self,
            gluon_data: &mut gluon::DataBuilder<'a>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write(gluon_data)?;
            self.y.write(gluon_data)?;
            self.z.write(gluon_data)?;
            Ok(())
        }
        fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
            let x = gluon::Convertable::read(gluon_data)?;
            let y = gluon::Convertable::read(gluon_data)?;
            let z = gluon::Convertable::read(gluon_data)?;
            Ok(Vec3F { x, y, z })
        }
        fn write_owned(
            self,
            gluon_data: &mut gluon::DataBuilder<'_>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write_owned(gluon_data)?;
            self.y.write_owned(gluon_data)?;
            self.z.write_owned(gluon_data)?;
            Ok(())
        }
    }
    ///4D vector
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Vec4F {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    impl gluon::Convertable for Vec4F {
        fn write<'a, 'b: 'a>(
            &'b self,
            gluon_data: &mut gluon::DataBuilder<'a>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write(gluon_data)?;
            self.y.write(gluon_data)?;
            self.z.write(gluon_data)?;
            self.w.write(gluon_data)?;
            Ok(())
        }
        fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
            let x = gluon::Convertable::read(gluon_data)?;
            let y = gluon::Convertable::read(gluon_data)?;
            let z = gluon::Convertable::read(gluon_data)?;
            let w = gluon::Convertable::read(gluon_data)?;
            Ok(Vec4F { x, y, z, w })
        }
        fn write_owned(
            self,
            gluon_data: &mut gluon::DataBuilder<'_>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write_owned(gluon_data)?;
            self.y.write_owned(gluon_data)?;
            self.z.write_owned(gluon_data)?;
            self.w.write_owned(gluon_data)?;
            Ok(())
        }
    }
    ///Quaternion
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Quatf {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
    impl gluon::Convertable for Quatf {
        fn write<'a, 'b: 'a>(
            &'b self,
            gluon_data: &mut gluon::DataBuilder<'a>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write(gluon_data)?;
            self.y.write(gluon_data)?;
            self.z.write(gluon_data)?;
            self.w.write(gluon_data)?;
            Ok(())
        }
        fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
            let x = gluon::Convertable::read(gluon_data)?;
            let y = gluon::Convertable::read(gluon_data)?;
            let z = gluon::Convertable::read(gluon_data)?;
            let w = gluon::Convertable::read(gluon_data)?;
            Ok(Quatf { x, y, z, w })
        }
        fn write_owned(
            self,
            gluon_data: &mut gluon::DataBuilder<'_>,
        ) -> Result<(), gluon::WriteError> {
            self.x.write_owned(gluon_data)?;
            self.y.write_owned(gluon_data)?;
            self.z.write_owned(gluon_data)?;
            self.w.write_owned(gluon_data)?;
            Ok(())
        }
    }
    ///Colum major matrix
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Mat4F {
        pub x: crate::types::Vec4F,
        pub y: crate::types::Vec4F,
        pub z: crate::types::Vec4F,
        pub w: crate::types::Vec4F,
    }
    impl gluon::Convertable for Mat4F {
        fn write<'a, 'b: 'a>(
            &'b self,
            gluon_data: &mut gluon::DataBuilder<'a>,
        ) -> Result<(), gluon::WriteError> {
            {
                let __w: proxied::Vec4F = self.x.clone().into();
                __w.write_owned(gluon_data)?;
            }
            {
                let __w: proxied::Vec4F = self.y.clone().into();
                __w.write_owned(gluon_data)?;
            }
            {
                let __w: proxied::Vec4F = self.z.clone().into();
                __w.write_owned(gluon_data)?;
            }
            {
                let __w: proxied::Vec4F = self.w.clone().into();
                __w.write_owned(gluon_data)?;
            }
            Ok(())
        }
        fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
            let x: crate::types::Vec4F = {
                let __w: proxied::Vec4F = gluon::Convertable::read(gluon_data)?;
                __w.into()
            };
            let y: crate::types::Vec4F = {
                let __w: proxied::Vec4F = gluon::Convertable::read(gluon_data)?;
                __w.into()
            };
            let z: crate::types::Vec4F = {
                let __w: proxied::Vec4F = gluon::Convertable::read(gluon_data)?;
                __w.into()
            };
            let w: crate::types::Vec4F = {
                let __w: proxied::Vec4F = gluon::Convertable::read(gluon_data)?;
                __w.into()
            };
            Ok(Mat4F { x, y, z, w })
        }
        fn write_owned(
            self,
            gluon_data: &mut gluon::DataBuilder<'_>,
        ) -> Result<(), gluon::WriteError> {
            {
                let __w: proxied::Vec4F = self.x.into();
                __w.write_owned(gluon_data)?;
            }
            {
                let __w: proxied::Vec4F = self.y.into();
                __w.write_owned(gluon_data)?;
            }
            {
                let __w: proxied::Vec4F = self.z.into();
                __w.write_owned(gluon_data)?;
            }
            {
                let __w: proxied::Vec4F = self.w.into();
                __w.write_owned(gluon_data)?;
            }
            Ok(())
        }
    }
    ///A color in linear rgb, premultiplied alpha
    #[derive(Debug, Copy, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Color {
        pub r: f32,
        pub g: f32,
        pub b: f32,
        pub a: f32,
    }
    impl gluon::Convertable for Color {
        fn write<'a, 'b: 'a>(
            &'b self,
            gluon_data: &mut gluon::DataBuilder<'a>,
        ) -> Result<(), gluon::WriteError> {
            self.r.write(gluon_data)?;
            self.g.write(gluon_data)?;
            self.b.write(gluon_data)?;
            self.a.write(gluon_data)?;
            Ok(())
        }
        fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
            let r = gluon::Convertable::read(gluon_data)?;
            let g = gluon::Convertable::read(gluon_data)?;
            let b = gluon::Convertable::read(gluon_data)?;
            let a = gluon::Convertable::read(gluon_data)?;
            Ok(Color { r, g, b, a })
        }
        fn write_owned(
            self,
            gluon_data: &mut gluon::DataBuilder<'_>,
        ) -> Result<(), gluon::WriteError> {
            self.r.write_owned(gluon_data)?;
            self.g.write_owned(gluon_data)?;
            self.b.write_owned(gluon_data)?;
            self.a.write_owned(gluon_data)?;
            Ok(())
        }
    }
}
