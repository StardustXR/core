#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.SUIS",
    types: &[
        gluon::ExternalGluonType {
            name: "Joint",
            supported_derives: gluon::Derives::from_bits_truncate(11u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Finger",
            supported_derives: gluon::Derives::from_bits_truncate(11u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Thumb",
            supported_derives: gluon::Derives::from_bits_truncate(11u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Hand",
            supported_derives: gluon::Derives::from_bits_truncate(11u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Pointer",
            supported_derives: gluon::Derives::from_bits_truncate(11u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Tip",
            supported_derives: gluon::Derives::from_bits_truncate(11u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "SemanticData",
            supported_derives: gluon::Derives::from_bits_truncate(0u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "SpatialData",
            supported_derives: gluon::Derives::from_bits_truncate(11u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Chirality",
            supported_derives: gluon::Derives::from_bits_truncate(31u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "InputDataType",
            supported_derives: gluon::Derives::from_bits_truncate(11u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DatamapData",
            supported_derives: gluon::Derives::from_bits_truncate(10u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///A hand joint. Distance from input handler's field is given because it's cheap to calculate and laggy to request from the server.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Joint {
    ///Pose of the joint relative to the input handler.
    pub pose: super::types::Posef,
    ///Radius of the joint in meters.
    pub radius: f32,
    ///Distance from the center of the joint to the input handler's field.
    pub distance: f32,
}
impl gluon::Convertable for Joint {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.pose.write(gluon_data)?;
        self.radius.write(gluon_data)?;
        self.distance.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let pose = gluon::Convertable::read(gluon_data)?;
        let radius = gluon::Convertable::read(gluon_data)?;
        let distance = gluon::Convertable::read(gluon_data)?;
        Ok(Joint { pose, radius, distance })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.pose.write_owned(gluon_data)?;
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
    pub intermediate: Joint,
    pub proximal: Joint,
    pub metacarpal: Joint,
}
impl gluon::Convertable for Finger {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.tip.write(gluon_data)?;
        self.distal.write(gluon_data)?;
        self.intermediate.write(gluon_data)?;
        self.proximal.write(gluon_data)?;
        self.metacarpal.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let tip = gluon::Convertable::read(gluon_data)?;
        let distal = gluon::Convertable::read(gluon_data)?;
        let intermediate = gluon::Convertable::read(gluon_data)?;
        let proximal = gluon::Convertable::read(gluon_data)?;
        let metacarpal = gluon::Convertable::read(gluon_data)?;
        Ok(Finger {
            tip,
            distal,
            intermediate,
            proximal,
            metacarpal,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.tip.write_owned(gluon_data)?;
        self.distal.write_owned(gluon_data)?;
        self.intermediate.write_owned(gluon_data)?;
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
impl gluon::Convertable for Thumb {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.tip.write(gluon_data)?;
        self.distal.write(gluon_data)?;
        self.proximal.write(gluon_data)?;
        self.metacarpal.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let tip = gluon::Convertable::read(gluon_data)?;
        let distal = gluon::Convertable::read(gluon_data)?;
        let proximal = gluon::Convertable::read(gluon_data)?;
        let metacarpal = gluon::Convertable::read(gluon_data)?;
        Ok(Thumb {
            tip,
            distal,
            proximal,
            metacarpal,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.tip.write_owned(gluon_data)?;
        self.distal.write_owned(gluon_data)?;
        self.proximal.write_owned(gluon_data)?;
        self.metacarpal.write_owned(gluon_data)?;
        Ok(())
    }
}
///A fully articulated and tracked hand (https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#convention-of-hand-joints).
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Hand {
    pub chirality: Chirality,
    pub thumb: Thumb,
    pub index: Finger,
    pub middle: Finger,
    pub ring: Finger,
    pub little: Finger,
    pub palm: Joint,
    pub wrist: Joint,
    pub elbow: Option<Joint>,
}
impl gluon::Convertable for Hand {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.chirality.write(gluon_data)?;
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
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let chirality = gluon::Convertable::read(gluon_data)?;
        let thumb = gluon::Convertable::read(gluon_data)?;
        let index = gluon::Convertable::read(gluon_data)?;
        let middle = gluon::Convertable::read(gluon_data)?;
        let ring = gluon::Convertable::read(gluon_data)?;
        let little = gluon::Convertable::read(gluon_data)?;
        let palm = gluon::Convertable::read(gluon_data)?;
        let wrist = gluon::Convertable::read(gluon_data)?;
        let elbow = gluon::Convertable::read(gluon_data)?;
        Ok(Hand {
            chirality,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.chirality.write_owned(gluon_data)?;
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
    ///Often corresponds to the aim pose (https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#semantic-paths-standard-pose-identifiers)
    pub pose: super::types::Posef,
    /**The point that is the most inside the input handler's field.
Useful for telling how close to the center it's pointing or for thin objects can take the place of a point of intersection.*/
    pub deepest_point: f32,
}
impl gluon::Convertable for Pointer {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.pose.write(gluon_data)?;
        self.deepest_point.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let pose = gluon::Convertable::read(gluon_data)?;
        let deepest_point = gluon::Convertable::read(gluon_data)?;
        Ok(Pointer { pose, deepest_point })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.pose.write_owned(gluon_data)?;
        self.deepest_point.write_owned(gluon_data)?;
        Ok(())
    }
}
///Represents a controller, pen tip, spatial cursor, etc. that is just a single point.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tip {
    ///Pose you can use to tap/poke elements (in front of the face of a controller, or on a pen tip).
    pub pose: super::types::Posef,
    ///Is this tip in the left or right hand? This may change at any time.
    pub chirality: Option<Chirality>,
    ///Center of the controller treated as a gripped rod (https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#semantic-paths-standard-pose-identifiers)
    pub grip_pose: Option<super::types::Posef>,
    ///Center of the palm contacting the controller (https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#semantic-paths-standard-pose-identifiers)
    pub grip_surface_pose: Option<super::types::Posef>,
    ///Non-articulated hand data (for index or similar)
    pub simulated_hand: Option<Hand>,
}
impl gluon::Convertable for Tip {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.pose.write(gluon_data)?;
        self.chirality.write(gluon_data)?;
        self.grip_pose.write(gluon_data)?;
        self.grip_surface_pose.write(gluon_data)?;
        self.simulated_hand.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let pose = gluon::Convertable::read(gluon_data)?;
        let chirality = gluon::Convertable::read(gluon_data)?;
        let grip_pose = gluon::Convertable::read(gluon_data)?;
        let grip_surface_pose = gluon::Convertable::read(gluon_data)?;
        let simulated_hand = gluon::Convertable::read(gluon_data)?;
        Ok(Tip {
            pose,
            chirality,
            grip_pose,
            grip_surface_pose,
            simulated_hand,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.pose.write_owned(gluon_data)?;
        self.chirality.write_owned(gluon_data)?;
        self.grip_pose.write_owned(gluon_data)?;
        self.grip_surface_pose.write_owned(gluon_data)?;
        self.simulated_hand.write_owned(gluon_data)?;
        Ok(())
    }
}
///Information about a given input method's state.
#[derive(Debug)]
pub struct SemanticData {
    ///Non-spatial data in a map. Keys will be a superset of the keys returned by InputHandler::suggested_bindings
    pub datamap: std::collections::HashMap<String, DatamapData>,
    ///There are [order] objects that got this input data before this one.
    pub order: u32,
    ///Is this input handler capturing this input method?
    pub captured: bool,
}
impl gluon::Convertable for SemanticData {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.datamap.write(gluon_data)?;
        self.order.write(gluon_data)?;
        self.captured.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let datamap = gluon::Convertable::read(gluon_data)?;
        let order = gluon::Convertable::read(gluon_data)?;
        let captured = gluon::Convertable::read(gluon_data)?;
        Ok(SemanticData {
            datamap,
            order,
            captured,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.datamap.write_owned(gluon_data)?;
        self.order.write_owned(gluon_data)?;
        self.captured.write_owned(gluon_data)?;
        Ok(())
    }
}
///Information about a given input method's spatial state. All coordinates are relative to the InputHandlers SpatialRef.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpatialData {
    ///All vectors and quaternions are relative to the input handler spatial ref.
    pub input: InputDataType,
    ///Closest distance from the input method to the field.
    pub distance: f32,
}
impl gluon::Convertable for SpatialData {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.input.write(gluon_data)?;
        self.distance.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let input = gluon::Convertable::read(gluon_data)?;
        let distance = gluon::Convertable::read(gluon_data)?;
        Ok(SpatialData { input, distance })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.input.write_owned(gluon_data)?;
        self.distance.write_owned(gluon_data)?;
        Ok(())
    }
}
///Chirality
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Chirality {
    Left,
    Right,
}
impl gluon::Convertable for Chirality {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            Chirality::Left => {
                gluon_data.write_u16(0u16)?;
            }
            Chirality::Right => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => Chirality::Left,
                1u16 => Chirality::Right,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            Chirality::Left => {
                gluon_data.write_u16(0u16)?;
            }
            Chirality::Right => {
                gluon_data.write_u16(1u16)?;
            }
        };
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
impl gluon::Convertable for InputDataType {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
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
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => {
                    let data = gluon::Convertable::read(gluon_data)?;
                    InputDataType::Pointer { data }
                }
                1u16 => {
                    let data = gluon::Convertable::read(gluon_data)?;
                    InputDataType::Hand { data }
                }
                2u16 => {
                    let data = gluon::Convertable::read(gluon_data)?;
                    InputDataType::Tip { data }
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
    Vec2 { value: crate::types::Vec2F },
    Vec3 { value: crate::types::Vec3F },
    String { value: String },
}
impl gluon::Convertable for DatamapData {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
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
                {
                    let __w: super::types::proxied::Vec2F = value.clone().into();
                    __w.write_owned(gluon_data)?;
                }
            }
            DatamapData::Vec3 { value } => {
                gluon_data.write_u16(3u16)?;
                {
                    let __w: super::types::proxied::Vec3F = value.clone().into();
                    __w.write_owned(gluon_data)?;
                }
            }
            DatamapData::String { value } => {
                gluon_data.write_u16(4u16)?;
                value.write(gluon_data)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => {
                    let value = gluon::Convertable::read(gluon_data)?;
                    DatamapData::Bool { value }
                }
                1u16 => {
                    let value = gluon::Convertable::read(gluon_data)?;
                    DatamapData::Float { value }
                }
                2u16 => {
                    let value: crate::types::Vec2F = {
                        let __w: super::types::proxied::Vec2F = gluon::Convertable::read(
                            gluon_data,
                        )?;
                        __w.into()
                    };
                    DatamapData::Vec2 { value }
                }
                3u16 => {
                    let value: crate::types::Vec3F = {
                        let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                            gluon_data,
                        )?;
                        __w.into()
                    };
                    DatamapData::Vec3 { value }
                }
                4u16 => {
                    let value = gluon::Convertable::read(gluon_data)?;
                    DatamapData::String { value }
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
                {
                    let __w: super::types::proxied::Vec2F = value.into();
                    __w.write_owned(gluon_data)?;
                }
            }
            DatamapData::Vec3 { value } => {
                gluon_data.write_u16(3u16)?;
                {
                    let __w: super::types::proxied::Vec3F = value.into();
                    __w.write_owned(gluon_data)?;
                }
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for InputHandler {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(InputHandler::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl InputHandler {
    /**All input coordinates will be relative to this
This is considered static and should not change after handler creation.*/
    pub async fn get_spatial(
        &self,
    ) -> Result<super::spatial::SpatialRef, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    ///This is considered static and should not change after handler creation.
    pub async fn get_field(&self) -> Result<super::field::FieldRef, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    /**Returns suggested bindings. The map key will equal a key in the datamap.
This is considered static and should not change after handler creation.*/
    pub async fn suggested_bindings(
        &self,
    ) -> Result<std::collections::HashMap<String, Vec<String>>, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    /**Returns a list of groups, for example the client app id and "grabbable".
This is considered static and should not change after handler creation.*/
    pub async fn handler_groups(&self) -> Result<Vec<String>, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    ///An input method just started sending input to this handler.
    pub fn input_gained(
        &self,
        method: impl Into<InputMethod>,
        time: impl Into<super::types::Timestamp>,
        spatial: impl Into<SpatialData>,
        semantic: impl Into<SemanticData>,
    ) -> Result<(), gluon::SendError> {
        let method: InputMethod = method.into();
        let time: super::types::Timestamp = time.into();
        let spatial: SpatialData = spatial.into();
        let semantic: SemanticData = semantic.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        method.write(&mut gluon_builder)?;
        time.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        semantic.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///An input method's data has been updated.
    pub fn input_updated(
        &self,
        method: impl Into<InputMethod>,
        time: impl Into<super::types::Timestamp>,
        spatial: impl Into<SpatialData>,
        semantic: impl Into<SemanticData>,
    ) -> Result<(), gluon::SendError> {
        let method: InputMethod = method.into();
        let time: super::types::Timestamp = time.into();
        let spatial: SpatialData = spatial.into();
        let semantic: SemanticData = semantic.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        method.write(&mut gluon_builder)?;
        time.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        semantic.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 13u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///An input method just stopped sending input to this handler.
    pub fn input_left(
        &self,
        method: impl Into<InputMethod>,
        time: impl Into<super::types::Timestamp>,
    ) -> Result<(), gluon::SendError> {
        let method: InputMethod = method.into();
        let time: super::types::Timestamp = time.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        method.write(&mut gluon_builder)?;
        time.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 14u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: InputHandlerHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> InputHandler {
        InputHandler::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> InputHandler {
        InputHandler { obj }
    }
}
impl From<InputHandler> for gluon::ObjectOrRef {
    fn from(value: InputHandler) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for InputHandler {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl std::hash::Hash for InputHandler {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for InputHandler {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for InputHandler {}
pub trait InputHandlerHandler: gluon::Handler + Send + Sync + 'static {
    /**All input coordinates will be relative to this
This is considered static and should not change after handler creation.*/
    fn get_spatial(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::spatial::SpatialRef> + Send + Sync;
    ///This is considered static and should not change after handler creation.
    fn get_field(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::field::FieldRef> + Send + Sync;
    /**Returns suggested bindings. The map key will equal a key in the datamap.
This is considered static and should not change after handler creation.*/
    fn suggested_bindings(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<
        Output = std::collections::HashMap<String, Vec<String>>,
    > + Send + Sync;
    /**Returns a list of groups, for example the client app id and "grabbable".
This is considered static and should not change after handler creation.*/
    fn handler_groups(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = Vec<String>> + Send + Sync;
    ///An input method just started sending input to this handler.
    fn input_gained(
        &self,
        _ctx: gluon::Context,
        method: InputMethod,
        time: super::types::Timestamp,
        spatial: SpatialData,
        semantic: SemanticData,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///An input method's data has been updated.
    fn input_updated(
        &self,
        _ctx: gluon::Context,
        method: InputMethod,
        time: super::types::Timestamp,
        spatial: SpatialData,
        semantic: SemanticData,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///An input method just stopped sending input to this handler.
    fn input_left(
        &self,
        _ctx: gluon::Context,
        method: InputMethod,
        time: super::types::Timestamp,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (spatial) = self.get_spatial(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (field) = self.get_field(ctx).await;
                    drop(gluon_data);
                    field.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (suggested_bindings) = self.suggested_bindings(ctx).await;
                    drop(gluon_data);
                    suggested_bindings.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                11u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (groups) = self.handler_groups(ctx).await;
                    drop(gluon_data);
                    groups.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                12u32 => {
                    let param_method = gluon::Convertable::read(&mut gluon_data)?;
                    let param_time = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_semantic = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.input_gained(
                            ctx,
                            param_method,
                            param_time,
                            param_spatial,
                            param_semantic,
                        )
                        .await;
                }
                13u32 => {
                    let param_method = gluon::Convertable::read(&mut gluon_data)?;
                    let param_time = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_semantic = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.input_updated(
                            ctx,
                            param_method,
                            param_time,
                            param_spatial,
                            param_semantic,
                        )
                        .await;
                }
                14u32 => {
                    let param_method = gluon::Convertable::read(&mut gluon_data)?;
                    let param_time = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.input_left(ctx, param_method, param_time).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct InputMethod {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for InputMethod {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(InputMethod::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl InputMethod {
    ///Request to capture the input method with the given handler.
    pub fn request_capture(
        &self,
        handler: impl Into<InputHandler>,
    ) -> Result<(), gluon::SendError> {
        let handler: InputHandler = handler.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        handler.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///If this input method captured by this handler, release the capture (e.g. the object is let go of after grabbing).
    pub fn release_capture(
        &self,
        handler: impl Into<InputHandler>,
    ) -> Result<(), gluon::SendError> {
        let handler: InputHandler = handler.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        handler.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    /**Get spatial data relative to the input handler at a specific point in time.
Should return None when the InputMethod is captured by another InputHandler.*/
    pub async fn get_spatial_data(
        &self,
        handler: impl Into<InputHandler>,
        time: impl Into<super::types::Timestamp>,
    ) -> Result<Option<SpatialData>, gluon::SendError> {
        let handler: InputHandler = handler.into();
        let time: super::types::Timestamp = time.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        handler.write(&mut gluon_builder)?;
        time.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn from_handler<H: InputMethodHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> InputMethod {
        InputMethod::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> InputMethod {
        InputMethod { obj }
    }
}
impl From<InputMethod> for gluon::ObjectOrRef {
    fn from(value: InputMethod) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for InputMethod {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl std::hash::Hash for InputMethod {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for InputMethod {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for InputMethod {}
pub trait InputMethodHandler: gluon::Handler + Send + Sync + 'static {
    ///Request to capture the input method with the given handler.
    fn request_capture(
        &self,
        _ctx: gluon::Context,
        handler: InputHandler,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///If this input method captured by this handler, release the capture (e.g. the object is let go of after grabbing).
    fn release_capture(
        &self,
        _ctx: gluon::Context,
        handler: InputHandler,
    ) -> impl Future<Output = ()> + Send + Sync;
    /**Get spatial data relative to the input handler at a specific point in time.
Should return None when the InputMethod is captured by another InputHandler.*/
    fn get_spatial_data(
        &self,
        _ctx: gluon::Context,
        handler: InputHandler,
        time: super::types::Timestamp,
    ) -> impl Future<Output = Option<SpatialData>> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let param_handler = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.request_capture(ctx, param_handler).await;
                }
                9u32 => {
                    let param_handler = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.release_capture(ctx, param_handler).await;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_handler = gluon::Convertable::read(&mut gluon_data)?;
                    let param_time = gluon::Convertable::read(&mut gluon_data)?;
                    let (data) = self
                        .get_spatial_data(ctx, param_handler, param_time)
                        .await;
                    drop(gluon_data);
                    data.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
pub mod proxied {
    use super::*;
}
