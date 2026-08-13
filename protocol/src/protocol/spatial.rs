#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Spatial",
    types: &[
        gluon::ExternalGluonType {
            name: "Transform",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "PartialTransform",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "BoundingBox",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "CreatedSpatial",
            supported_derives: gluon::Derives::from_bits_truncate(30u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "SpatialRefOpError",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///Transform
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transform {
    pub translation: crate::types::Vec3F,
    pub rotation: crate::types::QuatF,
    pub scale: crate::types::Vec3F,
}
impl gluon::Convertable for Transform {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Vec3F = self.translation.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::proxied::Quatf = self.rotation.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::proxied::Vec3F = self.scale.clone().into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let translation: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let rotation: crate::types::QuatF = {
            let __w: super::types::proxied::Quatf = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let scale: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        Ok(Transform {
            translation,
            rotation,
            scale,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Vec3F = self.translation.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::proxied::Quatf = self.rotation.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::proxied::Vec3F = self.scale.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///Transform
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PartialTransform {
    pub translation: Option<crate::types::Vec3F>,
    pub rotation: Option<crate::types::QuatF>,
    pub scale: Option<crate::types::Vec3F>,
}
impl gluon::Convertable for PartialTransform {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: Option<super::types::proxied::Vec3F> = self
                .translation
                .clone()
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<super::types::proxied::Quatf> = self
                .rotation
                .clone()
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<super::types::proxied::Vec3F> = self
                .scale
                .clone()
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let translation: Option<crate::types::Vec3F> = {
            let __w: Option<super::types::proxied::Vec3F> = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.map(|__v| __v.into())
        };
        let rotation: Option<crate::types::QuatF> = {
            let __w: Option<super::types::proxied::Quatf> = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.map(|__v| __v.into())
        };
        let scale: Option<crate::types::Vec3F> = {
            let __w: Option<super::types::proxied::Vec3F> = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.map(|__v| __v.into())
        };
        Ok(PartialTransform {
            translation,
            rotation,
            scale,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: Option<super::types::proxied::Vec3F> = self
                .translation
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<super::types::proxied::Quatf> = self
                .rotation
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<super::types::proxied::Vec3F> = self
                .scale
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///Bounding box
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoundingBox {
    pub center: crate::types::Vec3F,
    pub extents: crate::types::Vec3F,
}
impl gluon::Convertable for BoundingBox {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Vec3F = self.center.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::proxied::Vec3F = self.extents.clone().into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let center: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let extents: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        Ok(BoundingBox { center, extents })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Vec3F = self.center.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::proxied::Vec3F = self.extents.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///Struct returned by SpatialInterface::create_spatial so it can have proper errors
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CreatedSpatial {
    pub spatial: Spatial,
    pub spatial_ref: SpatialRef,
}
impl gluon::Convertable for CreatedSpatial {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.spatial.write(gluon_data)?;
        self.spatial_ref.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let spatial = gluon::Convertable::read(gluon_data)?;
        let spatial_ref = gluon::Convertable::read(gluon_data)?;
        Ok(CreatedSpatial {
            spatial,
            spatial_ref,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.spatial.write_owned(gluon_data)?;
        self.spatial_ref.write_owned(gluon_data)?;
        Ok(())
    }
}
///Error returned when getting information from a SpatialRef
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SpatialRefOpError {
    ///The SpatialRef passed to relative_to is invalid
    RelativeToInvalid,
    ///The SpatialRef passed to spatial is invalid
    SpatialRefInvalid,
}
impl gluon::Convertable for SpatialRefOpError {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            SpatialRefOpError::RelativeToInvalid => {
                gluon_data.write_u16(0u16)?;
            }
            SpatialRefOpError::SpatialRefInvalid => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => SpatialRefOpError::RelativeToInvalid,
                1u16 => SpatialRefOpError::SpatialRefInvalid,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            SpatialRefOpError::RelativeToInvalid => {
                gluon_data.write_u16(0u16)?;
            }
            SpatialRefOpError::SpatialRefInvalid => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct SpatialRef {
    obj: gluon::Ref,
}
impl gluon::Convertable for SpatialRef {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(SpatialRef::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for SpatialRef {
    const ID: &'static str = "org.stardustxr.Spatial.SpatialRef";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: SpatialRefHandler> gluon::HandledBy<H> for SpatialRef {}
impl gluon::RefExt for SpatialRef {
    fn from_ref(obj: gluon::Ref) -> SpatialRef {
        SpatialRef { obj }
    }
}
impl SpatialRef {
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> SpatialRef {
        SpatialRef { obj }
    }
}
impl From<SpatialRef> for gluon::Ref {
    fn from(value: SpatialRef) -> Self {
        value.obj
    }
}
impl gluon::ToRef for SpatialRef {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for SpatialRef {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for SpatialRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for SpatialRef {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for SpatialRef {}
pub trait SpatialRefHandler: gluon::Handler + Send + Sync + 'static {
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
#[derive(Debug, Clone)]
pub struct Spatial {
    obj: gluon::Ref,
}
impl gluon::Convertable for Spatial {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(Spatial::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for Spatial {
    const ID: &'static str = "org.stardustxr.Spatial.Spatial";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: SpatialHandler> gluon::HandledBy<H> for Spatial {}
impl gluon::RefExt for Spatial {
    fn from_ref(obj: gluon::Ref) -> Spatial {
        Spatial { obj }
    }
}
impl Spatial {
    ///Get the spatial ref for this spatial object.
    pub async fn spatial_ref(&self) -> Result<SpatialRef, gluon::SendError> {
        tracing::trace!(interface = "Spatial", method = "spatial_ref", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Spatial", method = "spatial_ref", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    ///Get the bounding box of this spatial and its children relative to itself
    pub async fn get_local_bounding_box(&self) -> Result<BoundingBox, gluon::SendError> {
        tracing::trace!(interface = "Spatial", method = "get_local_bounding_box", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_bounding_box = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Spatial", method = "get_local_bounding_box", ?
            __ret_bounding_box, "←"
        );
        Ok(__ret_bounding_box)
    }
    ///Get the bounding box of this spatial and its children relative to another spatial.
    pub async fn get_relative_bounding_box(
        &self,
        relative_to: impl Into<SpatialRef>,
    ) -> Result<Result<BoundingBox, super::types::CreateError>, gluon::SendError> {
        let relative_to: SpatialRef = relative_to.into();
        tracing::trace!(
            interface = "Spatial", method = "get_relative_bounding_box", ? relative_to,
            "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 10u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_bounding_box = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Spatial", method = "get_relative_bounding_box", ?
            __ret_bounding_box, "←"
        );
        Ok(__ret_bounding_box)
    }
    ///Get the transform of this spatial object.
    pub async fn get_relative_transform(
        &self,
        relative_to: impl Into<SpatialRef>,
    ) -> Result<Result<Transform, super::types::CreateError>, gluon::SendError> {
        let relative_to: SpatialRef = relative_to.into();
        tracing::trace!(
            interface = "Spatial", method = "get_relative_transform", ? relative_to,
            "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 11u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_transform = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Spatial", method = "get_relative_transform", ? __ret_transform,
            "←"
        );
        Ok(__ret_transform)
    }
    /**Sets the parent of this spatial object, keeping the local transform.
It will silently error and not set the spatial parent if it is to a child of itself.*/
    pub fn set_parent(
        &self,
        parent: impl Into<SpatialRef>,
    ) -> Result<(), gluon::SendError> {
        let parent: SpatialRef = parent.into();
        tracing::trace!(interface = "Spatial", method = "set_parent", ? parent, "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        parent.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 12u32, gluon_builder)?;
        Ok(())
    }
    /**Sets the parent of this spatial object, keeping its position in space.
It will silently error and not set the spatial parent if it is to a child of itself.*/
    pub fn set_parent_in_place(
        &self,
        parent: impl Into<SpatialRef>,
    ) -> Result<(), gluon::SendError> {
        let parent: SpatialRef = parent.into();
        tracing::trace!(
            interface = "Spatial", method = "set_parent_in_place", ? parent, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        parent.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 13u32, gluon_builder)?;
        Ok(())
    }
    ///Set the transform of this spatial relative to its spatial parent.
    pub fn set_local_transform(
        &self,
        transform: impl Into<PartialTransform>,
    ) -> Result<(), gluon::SendError> {
        let transform: PartialTransform = transform.into();
        tracing::trace!(
            interface = "Spatial", method = "set_local_transform", ? transform, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        transform.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 14u32, gluon_builder)?;
        Ok(())
    }
    ///Set the transform of this spatial relative to another spatial.
    pub fn set_relative_transform(
        &self,
        relative_to: impl Into<SpatialRef>,
        transform: impl Into<PartialTransform>,
    ) -> Result<(), gluon::SendError> {
        let relative_to: SpatialRef = relative_to.into();
        let transform: PartialTransform = transform.into();
        tracing::trace!(
            interface = "Spatial", method = "set_relative_transform", ? relative_to, ?
            transform, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        relative_to.write(&mut gluon_builder)?;
        transform.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 15u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> Spatial {
        Spatial { obj }
    }
}
impl From<Spatial> for gluon::Ref {
    fn from(value: Spatial) -> Self {
        value.obj
    }
}
impl gluon::ToRef for Spatial {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for Spatial {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for Spatial {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Spatial {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Spatial {}
pub trait SpatialHandler: gluon::Handler + Send + Sync + 'static {
    ///Get the spatial ref for this spatial object.
    fn spatial_ref(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = SpatialRef> + Send + Sync;
    ///Dispatched instead of [`Self::spatial_ref`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `spatial_ref` and sends the result through `reply`. Override this method instead of `spatial_ref` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn spatial_ref_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<SpatialRef>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.spatial_ref(_ctx).await;
            reply.send(spatial)
        }
    }
    ///Get the bounding box of this spatial and its children relative to itself
    fn get_local_bounding_box(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = BoundingBox> + Send + Sync;
    ///Dispatched instead of [`Self::get_local_bounding_box`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_local_bounding_box` and sends the result through `reply`. Override this method instead of `get_local_bounding_box` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_local_bounding_box_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<BoundingBox>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let bounding_box = self.get_local_bounding_box(_ctx).await;
            reply.send(bounding_box)
        }
    }
    ///Get the bounding box of this spatial and its children relative to another spatial.
    fn get_relative_bounding_box(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
    ) -> impl Future<
        Output = Result<BoundingBox, super::types::CreateError>,
    > + Send + Sync;
    ///Dispatched instead of [`Self::get_relative_bounding_box`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_relative_bounding_box` and sends the result through `reply`. Override this method instead of `get_relative_bounding_box` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_relative_bounding_box_oneway(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
        reply: gluon::ReplySender<Result<BoundingBox, super::types::CreateError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let bounding_box = self.get_relative_bounding_box(_ctx, relative_to).await;
            reply.send(bounding_box)
        }
    }
    ///Get the transform of this spatial object.
    fn get_relative_transform(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
    ) -> impl Future<
        Output = Result<Transform, super::types::CreateError>,
    > + Send + Sync;
    ///Dispatched instead of [`Self::get_relative_transform`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_relative_transform` and sends the result through `reply`. Override this method instead of `get_relative_transform` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_relative_transform_oneway(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
        reply: gluon::ReplySender<Result<Transform, super::types::CreateError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let transform = self.get_relative_transform(_ctx, relative_to).await;
            reply.send(transform)
        }
    }
    /**Sets the parent of this spatial object, keeping the local transform.
It will silently error and not set the spatial parent if it is to a child of itself.*/
    fn set_parent(
        &self,
        _ctx: gluon::Context,
        parent: SpatialRef,
    ) -> impl Future<Output = ()> + Send + Sync;
    /**Sets the parent of this spatial object, keeping its position in space.
It will silently error and not set the spatial parent if it is to a child of itself.*/
    fn set_parent_in_place(
        &self,
        _ctx: gluon::Context,
        parent: SpatialRef,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///Set the transform of this spatial relative to its spatial parent.
    fn set_local_transform(
        &self,
        _ctx: gluon::Context,
        transform: PartialTransform,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///Set the transform of this spatial relative to another spatial.
    fn set_relative_transform(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
        transform: PartialTransform,
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
                        interface = "Spatial", method = "spatial_ref", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<SpatialRef> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "Spatial", method = "spatial_ref", ? spatial,
                                "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.spatial_ref_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Spatial", method =
                                "spatial_ref", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    tracing::trace!(
                        interface = "Spatial", method = "get_local_bounding_box",
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<BoundingBox> = gluon::ReplySender::new(
                        return_callback,
                        |bounding_box, gluon_out| {
                            tracing::trace!(
                                interface = "Spatial", method = "get_local_bounding_box", ?
                                bounding_box, "←"
                            );
                            bounding_box.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_local_bounding_box_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Spatial", method =
                                "get_local_bounding_box", method_id = 9u32
                            ),
                        )
                        .await?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "get_relative_bounding_box", ?
                        param_relative_to, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<BoundingBox, super::types::CreateError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |bounding_box, gluon_out| {
                            tracing::trace!(
                                interface = "Spatial", method = "get_relative_bounding_box",
                                ? bounding_box, "←"
                            );
                            bounding_box.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_relative_bounding_box_oneway(ctx, param_relative_to, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Spatial", method =
                                "get_relative_bounding_box", method_id = 10u32
                            ),
                        )
                        .await?;
                }
                11u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "get_relative_transform", ?
                        param_relative_to, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<Transform, super::types::CreateError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |transform, gluon_out| {
                            tracing::trace!(
                                interface = "Spatial", method = "get_relative_transform", ?
                                transform, "←"
                            );
                            transform.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_relative_transform_oneway(ctx, param_relative_to, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Spatial", method =
                                "get_relative_transform", method_id = 11u32
                            ),
                        )
                        .await?;
                }
                12u32 => {
                    let param_parent = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "set_parent", ? param_parent,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.set_parent(ctx, param_parent)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Spatial", method = "set_parent",
                                method_id = 12u32
                            ),
                        )
                        .await;
                }
                13u32 => {
                    let param_parent = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "set_parent_in_place", ?
                        param_parent, "dispatching"
                    );
                    drop(gluon_data);
                    self.set_parent_in_place(ctx, param_parent)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Spatial", method =
                                "set_parent_in_place", method_id = 13u32
                            ),
                        )
                        .await;
                }
                14u32 => {
                    let param_transform = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "set_local_transform", ?
                        param_transform, "dispatching"
                    );
                    drop(gluon_data);
                    self.set_local_transform(ctx, param_transform)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Spatial", method =
                                "set_local_transform", method_id = 14u32
                            ),
                        )
                        .await;
                }
                15u32 => {
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    let param_transform = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "set_relative_transform", ?
                        param_relative_to, ? param_transform, "dispatching"
                    );
                    drop(gluon_data);
                    self.set_relative_transform(ctx, param_relative_to, param_transform)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Spatial", method =
                                "set_relative_transform", method_id = 15u32
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
pub struct SpatialInterface {
    obj: gluon::Ref,
}
impl gluon::Convertable for SpatialInterface {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(SpatialInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for SpatialInterface {
    const ID: &'static str = "org.stardustxr.Spatial.SpatialInterface";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: SpatialInterfaceHandler> gluon::HandledBy<H> for SpatialInterface {}
impl gluon::RefExt for SpatialInterface {
    fn from_ref(obj: gluon::Ref) -> SpatialInterface {
        SpatialInterface { obj }
    }
}
impl SpatialInterface {
    ///Create a new spatial object.
    pub async fn create_spatial(
        &self,
        parent: impl Into<SpatialRef>,
        transform: impl Into<Transform>,
    ) -> Result<Result<CreatedSpatial, super::types::CreateError>, gluon::SendError> {
        let parent: SpatialRef = parent.into();
        let transform: Transform = transform.into();
        tracing::trace!(
            interface = "SpatialInterface", method = "create_spatial", ? parent, ?
            transform, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        parent.write(&mut gluon_builder)?;
        transform.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "SpatialInterface", method = "create_spatial", ? __ret_spatial,
            "←"
        );
        Ok(__ret_spatial)
    }
    ///Get the relative bounding box of a spatial object relative to another spatial.
    pub async fn get_relative_bounding_box(
        &self,
        relative_to: impl Into<SpatialRef>,
        spatial: impl Into<SpatialRef>,
    ) -> Result<Result<BoundingBox, SpatialRefOpError>, gluon::SendError> {
        let relative_to: SpatialRef = relative_to.into();
        let spatial: SpatialRef = spatial.into();
        tracing::trace!(
            interface = "SpatialInterface", method = "get_relative_bounding_box", ?
            relative_to, ? spatial, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_bounding_box = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "SpatialInterface", method = "get_relative_bounding_box", ?
            __ret_bounding_box, "←"
        );
        Ok(__ret_bounding_box)
    }
    ///Get the relative transform of a spatial object relative to another spatial.
    pub async fn get_relative_transform(
        &self,
        relative_to: impl Into<SpatialRef>,
        spatial: impl Into<SpatialRef>,
    ) -> Result<Result<Transform, SpatialRefOpError>, gluon::SendError> {
        let relative_to: SpatialRef = relative_to.into();
        let spatial: SpatialRef = spatial.into();
        tracing::trace!(
            interface = "SpatialInterface", method = "get_relative_transform", ?
            relative_to, ? spatial, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 10u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_transform = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "SpatialInterface", method = "get_relative_transform", ?
            __ret_transform, "←"
        );
        Ok(__ret_transform)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> SpatialInterface {
        SpatialInterface { obj }
    }
}
impl From<SpatialInterface> for gluon::Ref {
    fn from(value: SpatialInterface) -> Self {
        value.obj
    }
}
impl gluon::ToRef for SpatialInterface {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for SpatialInterface {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for SpatialInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for SpatialInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for SpatialInterface {}
pub trait SpatialInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    ///Create a new spatial object.
    fn create_spatial(
        &self,
        _ctx: gluon::Context,
        parent: SpatialRef,
        transform: Transform,
    ) -> impl Future<
        Output = Result<CreatedSpatial, super::types::CreateError>,
    > + Send + Sync;
    ///Dispatched instead of [`Self::create_spatial`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `create_spatial` and sends the result through `reply`. Override this method instead of `create_spatial` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn create_spatial_oneway(
        &self,
        _ctx: gluon::Context,
        parent: SpatialRef,
        transform: Transform,
        reply: gluon::ReplySender<Result<CreatedSpatial, super::types::CreateError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.create_spatial(_ctx, parent, transform).await;
            reply.send(spatial)
        }
    }
    ///Get the relative bounding box of a spatial object relative to another spatial.
    fn get_relative_bounding_box(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
        spatial: SpatialRef,
    ) -> impl Future<Output = Result<BoundingBox, SpatialRefOpError>> + Send + Sync;
    ///Dispatched instead of [`Self::get_relative_bounding_box`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_relative_bounding_box` and sends the result through `reply`. Override this method instead of `get_relative_bounding_box` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_relative_bounding_box_oneway(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
        spatial: SpatialRef,
        reply: gluon::ReplySender<Result<BoundingBox, SpatialRefOpError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let bounding_box = self
                .get_relative_bounding_box(_ctx, relative_to, spatial)
                .await;
            reply.send(bounding_box)
        }
    }
    ///Get the relative transform of a spatial object relative to another spatial.
    fn get_relative_transform(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
        spatial: SpatialRef,
    ) -> impl Future<Output = Result<Transform, SpatialRefOpError>> + Send + Sync;
    ///Dispatched instead of [`Self::get_relative_transform`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_relative_transform` and sends the result through `reply`. Override this method instead of `get_relative_transform` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_relative_transform_oneway(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
        spatial: SpatialRef,
        reply: gluon::ReplySender<Result<Transform, SpatialRefOpError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let transform = self
                .get_relative_transform(_ctx, relative_to, spatial)
                .await;
            reply.send(transform)
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
                    let param_parent = gluon::Convertable::read(&mut gluon_data)?;
                    let param_transform = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SpatialInterface", method = "create_spatial", ?
                        param_parent, ? param_transform, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<CreatedSpatial, super::types::CreateError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "SpatialInterface", method = "create_spatial", ?
                                spatial, "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.create_spatial_oneway(ctx, param_parent, param_transform, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "SpatialInterface", method =
                                "create_spatial", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SpatialInterface", method =
                        "get_relative_bounding_box", ? param_relative_to, ?
                        param_spatial, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<BoundingBox, SpatialRefOpError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |bounding_box, gluon_out| {
                            tracing::trace!(
                                interface = "SpatialInterface", method =
                                "get_relative_bounding_box", ? bounding_box, "←"
                            );
                            bounding_box.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_relative_bounding_box_oneway(
                            ctx,
                            param_relative_to,
                            param_spatial,
                            reply,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "SpatialInterface", method =
                                "get_relative_bounding_box", method_id = 9u32
                            ),
                        )
                        .await?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SpatialInterface", method =
                        "get_relative_transform", ? param_relative_to, ? param_spatial,
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<Transform, SpatialRefOpError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |transform, gluon_out| {
                            tracing::trace!(
                                interface = "SpatialInterface", method =
                                "get_relative_transform", ? transform, "←"
                            );
                            transform.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_relative_transform_oneway(
                            ctx,
                            param_relative_to,
                            param_spatial,
                            reply,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "SpatialInterface", method =
                                "get_relative_transform", method_id = 10u32
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
pub mod proxied {
    use super::*;
}
