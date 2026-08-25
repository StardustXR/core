#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.SUIS",
    types: &[
        gluon::ExternalGluonType {
            name: "Joint",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Finger",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Thumb",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Hand",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Pointer",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Tip",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "SemanticData",
            supported_derives: gluon::Derives::from_bits_truncate(0u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "SpatialData",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Chirality",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "InputDataType",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DatamapData",
            supported_derives: gluon::Derives::from_bits_truncate(778u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///A hand joint. Distance from input handler's field is given because it's cheap to calculate and laggy to request from the server.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Joint {
    ///Pose of the joint relative to the input handler.
    pub pose: super::types::Posef,
    ///Radius of the joint in meters.
    pub radius: f32,
    ///Distance from the center of the joint to the input handler's field.
    pub distance: f32,
}
impl gluon::Convertable for Joint {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.pose.write_owned(gluon_data)?;
        self.radius.write_owned(gluon_data)?;
        self.distance.write_owned(gluon_data)?;
        Ok(())
    }
}
///Finger
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Finger {
    pub tip: Joint,
    pub distal: Joint,
    pub intermediate: Joint,
    pub proximal: Joint,
    pub metacarpal: Joint,
}
impl gluon::Convertable for Finger {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Thumb {
    pub tip: Joint,
    pub distal: Joint,
    pub proximal: Joint,
    pub metacarpal: Joint,
}
impl gluon::Convertable for Thumb {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pointer {
    ///Often corresponds to the aim pose (https://registry.khronos.org/OpenXR/specs/1.1/html/xrspec.html#semantic-paths-standard-pose-identifiers)
    pub pose: super::types::Posef,
    /**The point that is the most inside the input handler's field.
Useful for telling how close to the center it's pointing or for thin objects can take the place of a point of intersection.*/
    pub deepest_point: f32,
}
impl gluon::Convertable for Pointer {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.pose.write_owned(gluon_data)?;
        self.deepest_point.write_owned(gluon_data)?;
        Ok(())
    }
}
///Represents a controller, pen tip, spatial cursor, etc. that is just a single point.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.datamap.write_owned(gluon_data)?;
        self.order.write_owned(gluon_data)?;
        self.captured.write_owned(gluon_data)?;
        Ok(())
    }
}
///Information about a given input method's spatial state. All coordinates are relative to the InputHandlers SpatialRef.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpatialData {
    ///All vectors and quaternions are relative to the input handler spatial ref.
    pub input: InputDataType,
    ///Closest distance from the input method to the field.
    pub distance: f32,
}
impl gluon::Convertable for SpatialData {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.input.write_owned(gluon_data)?;
        self.distance.write_owned(gluon_data)?;
        Ok(())
    }
}
///Chirality
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Chirality {
    Left,
    Right,
}
impl gluon::Convertable for Chirality {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum InputDataType {
    Pointer { data: Pointer },
    Hand { data: Hand },
    Tip { data: Tip },
}
impl gluon::Convertable for InputDataType {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DatamapData {
    Bool { value: bool },
    Float { value: f32 },
    Vec2 { value: crate::types::Vec2F },
    Vec3 { value: crate::types::Vec3F },
    String { value: String },
}
impl gluon::Convertable for DatamapData {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
    obj: gluon::Ref,
}
impl gluon::Convertable for InputHandler {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(InputHandler::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for InputHandler {
    const ID: &'static str = "org.stardustxr.SUIS.InputHandler";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: InputHandlerHandler> gluon::HandledBy<H> for InputHandler {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type InputHandlerLocal<H> = gluon::LocalRef<InputHandler, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: InputHandlerHandler> From<InputHandlerLocal<H>> for InputHandler {
    fn from(value: InputHandlerLocal<H>) -> InputHandler {
        value.into_proxy()
    }
}
impl gluon::RefExt for InputHandler {
    fn from_ref(obj: gluon::Ref) -> InputHandler {
        InputHandler { obj }
    }
}
impl InputHandler {
    /**All input coordinates will be relative to this
This is considered static and should not change after handler creation.*/
    pub async fn get_spatial(
        &self,
    ) -> Result<super::spatial::SpatialRef, gluon::SendError> {
        tracing::trace!(interface = "InputHandler", method = "get_spatial", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "InputHandler", method = "get_spatial", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    ///This is considered static and should not change after handler creation.
    pub async fn get_field(&self) -> Result<super::field::FieldRef, gluon::SendError> {
        tracing::trace!(interface = "InputHandler", method = "get_field", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_field = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "InputHandler", method = "get_field", ? __ret_field, "←"
        );
        Ok(__ret_field)
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
        tracing::trace!(
            interface = "InputHandler", method = "input_gained", ? method, ? time, ?
            spatial, ? semantic, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        method.write(&mut gluon_builder)?;
        time.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        semantic.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 10u32, gluon_builder)?;
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
        tracing::trace!(
            interface = "InputHandler", method = "input_updated", ? method, ? time, ?
            spatial, ? semantic, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        method.write(&mut gluon_builder)?;
        time.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        semantic.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 11u32, gluon_builder)?;
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
        tracing::trace!(
            interface = "InputHandler", method = "input_left", ? method, ? time, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        method.write(&mut gluon_builder)?;
        time.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 12u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> InputHandler {
        InputHandler { obj }
    }
}
impl From<InputHandler> for gluon::Ref {
    fn from(value: InputHandler) -> Self {
        value.obj
    }
}
impl gluon::ToRef for InputHandler {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for InputHandler {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
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
    ///Dispatched instead of [`Self::get_spatial`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_spatial` and sends the result through `reply`. Override this method instead of `get_spatial` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_spatial_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::spatial::SpatialRef>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.get_spatial(_ctx).await;
            reply.send(spatial)
        }
    }
    ///This is considered static and should not change after handler creation.
    fn get_field(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::field::FieldRef> + Send + Sync;
    ///Dispatched instead of [`Self::get_field`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_field` and sends the result through `reply`. Override this method instead of `get_field` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_field_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::field::FieldRef>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let field = self.get_field(_ctx).await;
            reply.send(field)
        }
    }
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
                    let return_callback = gluon_data.read_ref()?;
                    tracing::trace!(
                        interface = "InputHandler", method = "get_spatial", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::spatial::SpatialRef> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "InputHandler", method = "get_spatial", ?
                                spatial, "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_spatial_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "InputHandler", method =
                                "get_spatial", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    tracing::trace!(
                        interface = "InputHandler", method = "get_field", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::field::FieldRef> = gluon::ReplySender::new(
                        return_callback,
                        |field, gluon_out| {
                            tracing::trace!(
                                interface = "InputHandler", method = "get_field", ? field,
                                "←"
                            );
                            field.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_field_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "InputHandler", method =
                                "get_field", method_id = 9u32
                            ),
                        )
                        .await?;
                }
                10u32 => {
                    let param_method = gluon::Convertable::read(&mut gluon_data)?;
                    let param_time = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_semantic = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "InputHandler", method = "input_gained", ?
                        param_method, ? param_time, ? param_spatial, ? param_semantic,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.input_gained(
                            ctx,
                            param_method,
                            param_time,
                            param_spatial,
                            param_semantic,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "InputHandler", method =
                                "input_gained", method_id = 10u32
                            ),
                        )
                        .await;
                }
                11u32 => {
                    let param_method = gluon::Convertable::read(&mut gluon_data)?;
                    let param_time = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_semantic = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "InputHandler", method = "input_updated", ?
                        param_method, ? param_time, ? param_spatial, ? param_semantic,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.input_updated(
                            ctx,
                            param_method,
                            param_time,
                            param_spatial,
                            param_semantic,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "InputHandler", method =
                                "input_updated", method_id = 11u32
                            ),
                        )
                        .await;
                }
                12u32 => {
                    let param_method = gluon::Convertable::read(&mut gluon_data)?;
                    let param_time = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "InputHandler", method = "input_left", ?
                        param_method, ? param_time, "dispatching"
                    );
                    drop(gluon_data);
                    self.input_left(ctx, param_method, param_time)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "InputHandler", method =
                                "input_left", method_id = 12u32
                            ),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct InputMethod {
    obj: gluon::Ref,
}
impl gluon::Convertable for InputMethod {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(InputMethod::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for InputMethod {
    const ID: &'static str = "org.stardustxr.SUIS.InputMethod";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: InputMethodHandler> gluon::HandledBy<H> for InputMethod {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type InputMethodLocal<H> = gluon::LocalRef<InputMethod, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: InputMethodHandler> From<InputMethodLocal<H>> for InputMethod {
    fn from(value: InputMethodLocal<H>) -> InputMethod {
        value.into_proxy()
    }
}
impl gluon::RefExt for InputMethod {
    fn from_ref(obj: gluon::Ref) -> InputMethod {
        InputMethod { obj }
    }
}
impl InputMethod {
    ///Request to capture the input method with the given handler.
    pub async fn request_capture(
        &self,
        handler: impl Into<InputHandler>,
    ) -> Result<Option<InputMethodCapture>, gluon::SendError> {
        let handler: InputHandler = handler.into();
        tracing::trace!(
            interface = "InputMethod", method = "request_capture", ? handler, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        handler.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_capture = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "InputMethod", method = "request_capture", ? __ret_capture, "←"
        );
        Ok(__ret_capture)
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
        tracing::trace!(
            interface = "InputMethod", method = "get_spatial_data", ? handler, ? time,
            "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        handler.write(&mut gluon_builder)?;
        time.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_data = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "InputMethod", method = "get_spatial_data", ? __ret_data, "←"
        );
        Ok(__ret_data)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> InputMethod {
        InputMethod { obj }
    }
}
impl From<InputMethod> for gluon::Ref {
    fn from(value: InputMethod) -> Self {
        value.obj
    }
}
impl gluon::ToRef for InputMethod {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for InputMethod {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
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
    ) -> impl Future<Output = Option<InputMethodCapture>> + Send + Sync;
    ///Dispatched instead of [`Self::request_capture`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `request_capture` and sends the result through `reply`. Override this method instead of `request_capture` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn request_capture_oneway(
        &self,
        _ctx: gluon::Context,
        handler: InputHandler,
        reply: gluon::ReplySender<Option<InputMethodCapture>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let capture = self.request_capture(_ctx, handler).await;
            reply.send(capture)
        }
    }
    /**Get spatial data relative to the input handler at a specific point in time.
Should return None when the InputMethod is captured by another InputHandler.*/
    fn get_spatial_data(
        &self,
        _ctx: gluon::Context,
        handler: InputHandler,
        time: super::types::Timestamp,
    ) -> impl Future<Output = Option<SpatialData>> + Send + Sync;
    ///Dispatched instead of [`Self::get_spatial_data`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_spatial_data` and sends the result through `reply`. Override this method instead of `get_spatial_data` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_spatial_data_oneway(
        &self,
        _ctx: gluon::Context,
        handler: InputHandler,
        time: super::types::Timestamp,
        reply: gluon::ReplySender<Option<SpatialData>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let data = self.get_spatial_data(_ctx, handler, time).await;
            reply.send(data)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_handler = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "InputMethod", method = "request_capture", ?
                        param_handler, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Option<InputMethodCapture>> = gluon::ReplySender::new(
                        return_callback,
                        |capture, gluon_out| {
                            tracing::trace!(
                                interface = "InputMethod", method = "request_capture", ?
                                capture, "←"
                            );
                            capture.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.request_capture_oneway(ctx, param_handler, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "InputMethod", method =
                                "request_capture", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_handler = gluon::Convertable::read(&mut gluon_data)?;
                    let param_time = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "InputMethod", method = "get_spatial_data", ?
                        param_handler, ? param_time, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Option<SpatialData>> = gluon::ReplySender::new(
                        return_callback,
                        |data, gluon_out| {
                            tracing::trace!(
                                interface = "InputMethod", method = "get_spatial_data", ?
                                data, "←"
                            );
                            data.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_spatial_data_oneway(ctx, param_handler, param_time, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "InputMethod", method =
                                "get_spatial_data", method_id = 9u32
                            ),
                        )
                        .await?;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct InputMethodCapture {
    obj: gluon::Ref,
}
impl gluon::Convertable for InputMethodCapture {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(InputMethodCapture::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for InputMethodCapture {
    const ID: &'static str = "org.stardustxr.SUIS.InputMethodCapture";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: InputMethodCaptureHandler> gluon::HandledBy<H> for InputMethodCapture {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type InputMethodCaptureLocal<H> = gluon::LocalRef<InputMethodCapture, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: InputMethodCaptureHandler> From<InputMethodCaptureLocal<H>>
for InputMethodCapture {
    fn from(value: InputMethodCaptureLocal<H>) -> InputMethodCapture {
        value.into_proxy()
    }
}
impl gluon::RefExt for InputMethodCapture {
    fn from_ref(obj: gluon::Ref) -> InputMethodCapture {
        InputMethodCapture { obj }
    }
}
impl InputMethodCapture {
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> InputMethodCapture {
        InputMethodCapture { obj }
    }
}
impl From<InputMethodCapture> for gluon::Ref {
    fn from(value: InputMethodCapture) -> Self {
        value.obj
    }
}
impl gluon::ToRef for InputMethodCapture {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for InputMethodCapture {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for InputMethodCapture {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for InputMethodCapture {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for InputMethodCapture {}
pub trait InputMethodCaptureHandler: gluon::Handler + Send + Sync + 'static {
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                _ => {}
            }
            Ok(())
        }
    }
}
pub mod proxied {
    use super::*;
}
