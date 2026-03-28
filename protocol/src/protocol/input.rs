#![allow(unused, clippy::single_match, clippy::match_single_binding)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Input",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "Joint",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Finger",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Thumb",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Hand",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Pointer",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Tip",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "InputData",
            supported_derives: gluon_wire::Derives::from_bits_truncate(0u32),
        },
        gluon_wire::ExternalGluonType {
            name: "InputDataType",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "DatamapData",
            supported_derives: gluon_wire::Derives::from_bits_truncate(10u32),
        },
    ],
};
///A hand joint. Distance from input handler's field is given because it's cheap to calculate and laggy to request from the server.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Joint {
    ///Position of the joint relative to the input handler.
    pub position: super::types::Vec3F,
    ///Orientation of the joint relative to the input handler.
    pub rotation: super::types::Quatf,
    ///Radius of the joint in meters.
    pub radius: f32,
    ///Distance from the center of the joint to the input handler's field.
    pub distance: f32,
}
impl gluon_wire::GluonConvertable for Joint {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.position.write(gluon_data)?;
        self.rotation.write(gluon_data)?;
        self.radius.write(gluon_data)?;
        self.distance.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let position = gluon_wire::GluonConvertable::read(gluon_data)?;
        let rotation = gluon_wire::GluonConvertable::read(gluon_data)?;
        let radius = gluon_wire::GluonConvertable::read(gluon_data)?;
        let distance = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Joint {
            position,
            rotation,
            radius,
            distance,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.position.write_owned(gluon_data)?;
        self.rotation.write_owned(gluon_data)?;
        self.radius.write_owned(gluon_data)?;
        self.distance.write_owned(gluon_data)?;
        Ok(())
    }
}
///Finger
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Finger {
    pub tip: Joint,
    pub distal: Joint,
    pub proximal: Joint,
    pub metacarpal: Joint,
}
impl gluon_wire::GluonConvertable for Finger {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.tip.write(gluon_data)?;
        self.distal.write(gluon_data)?;
        self.proximal.write(gluon_data)?;
        self.metacarpal.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let tip = gluon_wire::GluonConvertable::read(gluon_data)?;
        let distal = gluon_wire::GluonConvertable::read(gluon_data)?;
        let proximal = gluon_wire::GluonConvertable::read(gluon_data)?;
        let metacarpal = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Finger {
            tip,
            distal,
            proximal,
            metacarpal,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.tip.write_owned(gluon_data)?;
        self.distal.write_owned(gluon_data)?;
        self.proximal.write_owned(gluon_data)?;
        self.metacarpal.write_owned(gluon_data)?;
        Ok(())
    }
}
///Different than finger to be explicit about number of joints.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Thumb {
    pub tip: Joint,
    pub distal: Joint,
    pub proximal: Joint,
    pub metacarpal: Joint,
}
impl gluon_wire::GluonConvertable for Thumb {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.tip.write(gluon_data)?;
        self.distal.write(gluon_data)?;
        self.proximal.write(gluon_data)?;
        self.metacarpal.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let tip = gluon_wire::GluonConvertable::read(gluon_data)?;
        let distal = gluon_wire::GluonConvertable::read(gluon_data)?;
        let proximal = gluon_wire::GluonConvertable::read(gluon_data)?;
        let metacarpal = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Thumb {
            tip,
            distal,
            proximal,
            metacarpal,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.tip.write_owned(gluon_data)?;
        self.distal.write_owned(gluon_data)?;
        self.proximal.write_owned(gluon_data)?;
        self.metacarpal.write_owned(gluon_data)?;
        Ok(())
    }
}
///A fully articulated and tracked hand according to OpenXR spec for its coordinate system and joints.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hand {
    pub right: bool,
    pub thumb: Thumb,
    pub index: Finger,
    pub middle: Finger,
    pub ring: Finger,
    pub little: Finger,
    pub palm: Joint,
    pub wrist: Joint,
    pub elbow: Option<Joint>,
}
impl gluon_wire::GluonConvertable for Hand {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.right.write(gluon_data)?;
        self.thumb.write(gluon_data)?;
        self.index.write(gluon_data)?;
        self.middle.write(gluon_data)?;
        self.ring.write(gluon_data)?;
        self.little.write(gluon_data)?;
        self.palm.write(gluon_data)?;
        self.wrist.write(gluon_data)?;
        self.elbow.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let right = gluon_wire::GluonConvertable::read(gluon_data)?;
        let thumb = gluon_wire::GluonConvertable::read(gluon_data)?;
        let index = gluon_wire::GluonConvertable::read(gluon_data)?;
        let middle = gluon_wire::GluonConvertable::read(gluon_data)?;
        let ring = gluon_wire::GluonConvertable::read(gluon_data)?;
        let little = gluon_wire::GluonConvertable::read(gluon_data)?;
        let palm = gluon_wire::GluonConvertable::read(gluon_data)?;
        let wrist = gluon_wire::GluonConvertable::read(gluon_data)?;
        let elbow = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Hand {
            right,
            thumb,
            index,
            middle,
            ring,
            little,
            palm,
            wrist,
            elbow,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.right.write_owned(gluon_data)?;
        self.thumb.write_owned(gluon_data)?;
        self.index.write_owned(gluon_data)?;
        self.middle.write_owned(gluon_data)?;
        self.ring.write_owned(gluon_data)?;
        self.little.write_owned(gluon_data)?;
        self.palm.write_owned(gluon_data)?;
        self.wrist.write_owned(gluon_data)?;
        self.elbow.write_owned(gluon_data)?;
        Ok(())
    }
}
///A 3D pointer, such as a gaze pointer for eye tracking or a mouse or a ray from a controller.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pointer {
    pub origin: super::types::Vec3F,
    pub orientation: super::types::Quatf,
    /**The point that is the most inside the input handler's field.
Useful for telling how close to the center it's pointing or for thin objects can take the place of a point of intersection.*/
    pub deepest_point: f32,
}
impl gluon_wire::GluonConvertable for Pointer {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.origin.write(gluon_data)?;
        self.orientation.write(gluon_data)?;
        self.deepest_point.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let origin = gluon_wire::GluonConvertable::read(gluon_data)?;
        let orientation = gluon_wire::GluonConvertable::read(gluon_data)?;
        let deepest_point = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Pointer {
            origin,
            orientation,
            deepest_point,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.origin.write_owned(gluon_data)?;
        self.orientation.write_owned(gluon_data)?;
        self.deepest_point.write_owned(gluon_data)?;
        Ok(())
    }
}
///Represents a controller, pen tip, spatial cursor, etc. that is just a single point.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tip {
    pub origin: super::types::Vec3F,
    pub orientation: super::types::Vec3F,
}
impl gluon_wire::GluonConvertable for Tip {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.origin.write(gluon_data)?;
        self.orientation.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let origin = gluon_wire::GluonConvertable::read(gluon_data)?;
        let orientation = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Tip { origin, orientation })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.origin.write_owned(gluon_data)?;
        self.orientation.write_owned(gluon_data)?;
        Ok(())
    }
}
///Information about a given input method's state. All coordinates are relative to the InputHandlers SpatialRef.
#[derive(Debug)]
pub struct InputData {
    ///All vectors and quaternions are relative to the input handler if deserialized.
    pub input: InputDataType,
    ///Closest distance from the input method to the field.
    pub distance: f32,
    ///Non-spatial data in a map.
    pub datamap: std::collections::HashMap<String, DatamapData>,
    ///There are [order] objects that got this input data before this one.
    pub order: u32,
    ///Is this input handler capturing this input method?
    pub captured: bool,
}
impl gluon_wire::GluonConvertable for InputData {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.input.write(gluon_data)?;
        self.distance.write(gluon_data)?;
        self.datamap.write(gluon_data)?;
        self.order.write(gluon_data)?;
        self.captured.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let input = gluon_wire::GluonConvertable::read(gluon_data)?;
        let distance = gluon_wire::GluonConvertable::read(gluon_data)?;
        let datamap = gluon_wire::GluonConvertable::read(gluon_data)?;
        let order = gluon_wire::GluonConvertable::read(gluon_data)?;
        let captured = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(InputData {
            input,
            distance,
            datamap,
            order,
            captured,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.input.write_owned(gluon_data)?;
        self.distance.write_owned(gluon_data)?;
        self.datamap.write_owned(gluon_data)?;
        self.order.write_owned(gluon_data)?;
        self.captured.write_owned(gluon_data)?;
        Ok(())
    }
}
///The special type of an InputMethod.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputDataType {
    Pointer { data: Pointer },
    Hand { data: Hand },
    Tip { data: Tip },
}
impl gluon_wire::GluonConvertable for InputDataType {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            InputDataType::Pointer { data } => {
                gluon_data.write_u16(0u16)?;
                data.write(gluon_data)?;
            }
            InputDataType::Hand { data } => {
                gluon_data.write_u16(1u16)?;
                data.write(gluon_data)?;
            }
            InputDataType::Tip { data } => {
                gluon_data.write_u16(2u16)?;
                data.write(gluon_data)?;
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
                    let data = gluon_wire::GluonConvertable::read(gluon_data)?;
                    InputDataType::Pointer { data }
                }
                1u16 => {
                    let data = gluon_wire::GluonConvertable::read(gluon_data)?;
                    InputDataType::Hand { data }
                }
                2u16 => {
                    let data = gluon_wire::GluonConvertable::read(gluon_data)?;
                    InputDataType::Tip { data }
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
            InputDataType::Pointer { data } => {
                gluon_data.write_u16(0u16)?;
                data.write_owned(gluon_data)?;
            }
            InputDataType::Hand { data } => {
                gluon_data.write_u16(1u16)?;
                data.write_owned(gluon_data)?;
            }
            InputDataType::Tip { data } => {
                gluon_data.write_u16(2u16)?;
                data.write_owned(gluon_data)?;
            }
        };
        Ok(())
    }
}
///Data types for datamap
#[derive(Debug, Clone, PartialEq)]
pub enum DatamapData {
    Bool { value: bool },
    Float { value: f32 },
    Vec2 { value: super::types::Vec2F },
    Vec3 { value: super::types::Vec3F },
    String { value: String },
}
impl gluon_wire::GluonConvertable for DatamapData {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            DatamapData::Bool { value } => {
                gluon_data.write_u16(0u16)?;
                value.write(gluon_data)?;
            }
            DatamapData::Float { value } => {
                gluon_data.write_u16(1u16)?;
                value.write(gluon_data)?;
            }
            DatamapData::Vec2 { value } => {
                gluon_data.write_u16(2u16)?;
                value.write(gluon_data)?;
            }
            DatamapData::Vec3 { value } => {
                gluon_data.write_u16(3u16)?;
                value.write(gluon_data)?;
            }
            DatamapData::String { value } => {
                gluon_data.write_u16(4u16)?;
                value.write(gluon_data)?;
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
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    DatamapData::Bool { value }
                }
                1u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    DatamapData::Float { value }
                }
                2u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    DatamapData::Vec2 { value }
                }
                3u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    DatamapData::Vec3 { value }
                }
                4u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    DatamapData::String { value }
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
            DatamapData::Bool { value } => {
                gluon_data.write_u16(0u16)?;
                value.write_owned(gluon_data)?;
            }
            DatamapData::Float { value } => {
                gluon_data.write_u16(1u16)?;
                value.write_owned(gluon_data)?;
            }
            DatamapData::Vec2 { value } => {
                gluon_data.write_u16(2u16)?;
                value.write_owned(gluon_data)?;
            }
            DatamapData::Vec3 { value } => {
                gluon_data.write_u16(3u16)?;
                value.write_owned(gluon_data)?;
            }
            DatamapData::String { value } => {
                gluon_data.write_u16(4u16)?;
                value.write_owned(gluon_data)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct InputHandler {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for InputHandler {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
        Ok(InputHandler::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl InputHandler {
    ///All input coordinates will be relative to this
    pub async fn get_spatial(
        &self,
    ) -> Result<super::spatial::SpatialRef, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_spatial_blocking()).await.unwrap()
    }
    pub fn get_spatial_blocking(
        &self,
    ) -> Result<super::spatial::SpatialRef, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn get_field(
        &self,
    ) -> Result<super::field::FieldRef, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_field_blocking()).await.unwrap()
    }
    pub fn get_field_blocking(
        &self,
    ) -> Result<super::field::FieldRef, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///An input method just started sending input to this handler.
    pub fn input_gained(
        &self,
        method: InputMethod,
        data: InputData,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        method.write(&mut gluon_builder)?;
        data.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///An input method's data has been updated.
    pub fn input_updated(
        &self,
        method: InputMethod,
        data: InputData,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        method.write(&mut gluon_builder)?;
        data.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///An input method just stopped sending input to this handler.
    pub fn input_left(
        &self,
        method: InputMethod,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        method.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: InputHandlerHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> InputHandler {
        InputHandler::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> InputHandler {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        InputHandler {
            obj,
            drop_notification,
        }
    }
    pub fn death_or_drop(&self) -> impl Future<Output = ()> + Send + Sync + 'static {
        let death_notification_future = match &self.obj {
            binderbinder::binder_object::BinderObjectOrRef::Ref(r) => {
                Some(r.death_notification())
            }
            binderbinder::binder_object::BinderObjectOrRef::WeakRef(r) => {
                Some(r.death_notification())
            }
            _ => None,
        };
        let drop_notification = self.drop_notification.clone();
        async move {
            if let Some(death) = death_notification_future {
                tokio::select! {
                    _ = death => {} _ = drop_notification.wait() => {}
                }
            } else {
                drop_notification.wait().await;
            }
        }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for InputHandler {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait InputHandlerHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///All input coordinates will be relative to this
    fn get_spatial(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::spatial::SpatialRef> + Send + Sync;
    fn get_field(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::field::FieldRef> + Send + Sync;
    ///An input method just started sending input to this handler.
    fn input_gained(
        &self,
        _ctx: gluon_wire::GluonCtx,
        method: InputMethod,
        data: InputData,
    );
    ///An input method's data has been updated.
    fn input_updated(
        &self,
        _ctx: gluon_wire::GluonCtx,
        method: InputMethod,
        data: InputData,
    );
    ///An input method just stopped sending input to this handler.
    fn input_left(&self, _ctx: gluon_wire::GluonCtx, method: InputMethod);
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                8u32 => {
                    let (spatial) = self.get_spatial(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                9u32 => {
                    let (field) = self.get_field(ctx).await;
                    field.write_owned(&mut out)?;
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = gluon_data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                10u32 => {
                    self.input_gained(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                11u32 => {
                    self.input_updated(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                12u32 => {
                    self.input_left(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct InputMethod {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for InputMethod {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
        Ok(InputMethod::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl InputMethod {
    ///Request to capture the input method with the given handler.
    pub fn request_capture(
        &self,
        handler: InputHandler,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        handler.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///If this input method captured by this handler, release the capture (e.g. the object is let go of after grabbing).
    pub fn release_capture(
        &self,
        handler: InputHandler,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        handler.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: InputMethodHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> InputMethod {
        InputMethod::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> InputMethod {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        InputMethod {
            obj,
            drop_notification,
        }
    }
    pub fn death_or_drop(&self) -> impl Future<Output = ()> + Send + Sync + 'static {
        let death_notification_future = match &self.obj {
            binderbinder::binder_object::BinderObjectOrRef::Ref(r) => {
                Some(r.death_notification())
            }
            binderbinder::binder_object::BinderObjectOrRef::WeakRef(r) => {
                Some(r.death_notification())
            }
            _ => None,
        };
        let drop_notification = self.drop_notification.clone();
        async move {
            if let Some(death) = death_notification_future {
                tokio::select! {
                    _ = death => {} _ = drop_notification.wait() => {}
                }
            } else {
                drop_notification.wait().await;
            }
        }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for InputMethod {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait InputMethodHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Request to capture the input method with the given handler.
    fn request_capture(&self, _ctx: gluon_wire::GluonCtx, handler: InputHandler);
    ///If this input method captured by this handler, release the capture (e.g. the object is let go of after grabbing).
    fn release_capture(&self, _ctx: gluon_wire::GluonCtx, handler: InputHandler);
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = gluon_data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                8u32 => {
                    self.request_capture(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                9u32 => {
                    self.release_capture(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                _ => {}
            }
            Ok(())
        }
    }
}
