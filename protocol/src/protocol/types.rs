#![allow(unused, clippy::single_match, clippy::match_single_binding)]
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
            name: "Quatf",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
    ],
};
///2D vector
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Size2 {
    pub x: u32,
    pub y: u32,
}
impl gluon_wire::GluonConvertable for Size2 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(data)?;
        self.y.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(data)?;
        let y = gluon_wire::GluonConvertable::read(data)?;
        Ok(Size2 { x, y })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(data)?;
        self.y.write_owned(data)?;
        Ok(())
    }
}
///3D vector
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Size3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
impl gluon_wire::GluonConvertable for Size3 {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(data)?;
        self.y.write(data)?;
        self.z.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(data)?;
        let y = gluon_wire::GluonConvertable::read(data)?;
        let z = gluon_wire::GluonConvertable::read(data)?;
        Ok(Size3 { x, y, z })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(data)?;
        self.y.write_owned(data)?;
        self.z.write_owned(data)?;
        Ok(())
    }
}
///2D vector
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2F {
    pub x: f32,
    pub y: f32,
}
impl gluon_wire::GluonConvertable for Vec2F {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(data)?;
        self.y.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(data)?;
        let y = gluon_wire::GluonConvertable::read(data)?;
        Ok(Vec2F { x, y })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(data)?;
        self.y.write_owned(data)?;
        Ok(())
    }
}
///3D vector
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl gluon_wire::GluonConvertable for Vec3F {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(data)?;
        self.y.write(data)?;
        self.z.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(data)?;
        let y = gluon_wire::GluonConvertable::read(data)?;
        let z = gluon_wire::GluonConvertable::read(data)?;
        Ok(Vec3F { x, y, z })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(data)?;
        self.y.write_owned(data)?;
        self.z.write_owned(data)?;
        Ok(())
    }
}
///Quaternion
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quatf {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl gluon_wire::GluonConvertable for Quatf {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write(data)?;
        self.y.write(data)?;
        self.z.write(data)?;
        self.w.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let x = gluon_wire::GluonConvertable::read(data)?;
        let y = gluon_wire::GluonConvertable::read(data)?;
        let z = gluon_wire::GluonConvertable::read(data)?;
        let w = gluon_wire::GluonConvertable::read(data)?;
        Ok(Quatf { x, y, z, w })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.x.write_owned(data)?;
        self.y.write_owned(data)?;
        self.z.write_owned(data)?;
        self.w.write_owned(data)?;
        Ok(())
    }
}
