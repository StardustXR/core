#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
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
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for SpatialRef {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(SpatialRef::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl SpatialRef {
    pub fn from_handler<H: SpatialRefHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> SpatialRef {
        SpatialRef::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> SpatialRef {
        SpatialRef { obj }
    }
}
impl From<SpatialRef> for gluon::ObjectOrRef {
    fn from(value: SpatialRef) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for SpatialRef {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for Spatial {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(Spatial::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Spatial {
    ///Get the spatial ref for this spatial object.
    pub async fn spatial_ref(&self) -> Result<SpatialRef, gluon::SendError> {
        tracing::trace!(interface = "Spatial", method = "spatial_ref", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
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
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
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
            interface = "Spatial", method = "get_relative_bounding_box", relative_to =
            "SpatialRef", "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
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
            interface = "Spatial", method = "get_relative_transform", relative_to =
            "SpatialRef", "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
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
        tracing::trace!(
            interface = "Spatial", method = "set_parent", parent = "SpatialRef", "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        parent.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
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
            interface = "Spatial", method = "set_parent_in_place", parent = "SpatialRef",
            "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        parent.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 13u32, gluon_builder.to_payload())?;
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
        self.obj
            .device()
            .transact_one_way(&self.obj, 14u32, gluon_builder.to_payload())?;
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
            interface = "Spatial", method = "set_relative_transform", relative_to =
            "SpatialRef", ? transform, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        relative_to.write(&mut gluon_builder)?;
        transform.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 15u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: SpatialHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> Spatial {
        Spatial::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> Spatial {
        Spatial { obj }
    }
}
impl From<Spatial> for gluon::ObjectOrRef {
    fn from(value: Spatial) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for Spatial {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
    ///Get the bounding box of this spatial and its children relative to itself
    fn get_local_bounding_box(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = BoundingBox> + Send + Sync;
    ///Get the bounding box of this spatial and its children relative to another spatial.
    fn get_relative_bounding_box(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
    ) -> impl Future<
        Output = Result<BoundingBox, super::types::CreateError>,
    > + Send + Sync;
    ///Get the transform of this spatial object.
    fn get_relative_transform(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
    ) -> impl Future<
        Output = Result<Transform, super::types::CreateError>,
    > + Send + Sync;
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
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    tracing::trace!(
                        interface = "Spatial", method = "spatial_ref", "dispatching"
                    );
                    let (spatial) = self.spatial_ref(ctx).await;
                    drop(gluon_data);
                    tracing::trace!(
                        interface = "Spatial", method = "spatial_ref", ? spatial, "←"
                    );
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    tracing::trace!(
                        interface = "Spatial", method = "get_local_bounding_box",
                        "dispatching"
                    );
                    let (bounding_box) = self.get_local_bounding_box(ctx).await;
                    drop(gluon_data);
                    tracing::trace!(
                        interface = "Spatial", method = "get_local_bounding_box", ?
                        bounding_box, "←"
                    );
                    bounding_box.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "get_relative_bounding_box",
                        param_relative_to = "SpatialRef", "dispatching"
                    );
                    let (bounding_box) = self
                        .get_relative_bounding_box(ctx, param_relative_to)
                        .await;
                    drop(gluon_data);
                    tracing::trace!(
                        interface = "Spatial", method = "get_relative_bounding_box", ?
                        bounding_box, "←"
                    );
                    bounding_box.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                11u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "get_relative_transform",
                        param_relative_to = "SpatialRef", "dispatching"
                    );
                    let (transform) = self
                        .get_relative_transform(ctx, param_relative_to)
                        .await;
                    drop(gluon_data);
                    tracing::trace!(
                        interface = "Spatial", method = "get_relative_transform", ?
                        transform, "←"
                    );
                    transform.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                12u32 => {
                    let param_parent = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "set_parent", param_parent =
                        "SpatialRef", "dispatching"
                    );
                    drop(gluon_data);
                    self.set_parent(ctx, param_parent).await;
                }
                13u32 => {
                    let param_parent = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "set_parent_in_place",
                        param_parent = "SpatialRef", "dispatching"
                    );
                    drop(gluon_data);
                    self.set_parent_in_place(ctx, param_parent).await;
                }
                14u32 => {
                    let param_transform = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "set_local_transform", ?
                        param_transform, "dispatching"
                    );
                    drop(gluon_data);
                    self.set_local_transform(ctx, param_transform).await;
                }
                15u32 => {
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    let param_transform = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Spatial", method = "set_relative_transform",
                        param_relative_to = "SpatialRef", ? param_transform,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.set_relative_transform(ctx, param_relative_to, param_transform)
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for SpatialInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(SpatialInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
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
            interface = "SpatialInterface", method = "create_spatial", parent =
            "SpatialRef", ? transform, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        parent.write(&mut gluon_builder)?;
        transform.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
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
            interface = "SpatialInterface", method = "get_relative_bounding_box",
            relative_to = "SpatialRef", spatial = "SpatialRef", "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
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
            interface = "SpatialInterface", method = "get_relative_transform",
            relative_to = "SpatialRef", spatial = "SpatialRef", "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_transform = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "SpatialInterface", method = "get_relative_transform", ?
            __ret_transform, "←"
        );
        Ok(__ret_transform)
    }
    pub fn from_handler<H: SpatialInterfaceHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> SpatialInterface {
        SpatialInterface::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> SpatialInterface {
        SpatialInterface { obj }
    }
}
impl From<SpatialInterface> for gluon::ObjectOrRef {
    fn from(value: SpatialInterface) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for SpatialInterface {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
    ///Get the relative bounding box of a spatial object relative to another spatial.
    fn get_relative_bounding_box(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
        spatial: SpatialRef,
    ) -> impl Future<Output = Result<BoundingBox, SpatialRefOpError>> + Send + Sync;
    ///Get the relative transform of a spatial object relative to another spatial.
    fn get_relative_transform(
        &self,
        _ctx: gluon::Context,
        relative_to: SpatialRef,
        spatial: SpatialRef,
    ) -> impl Future<Output = Result<Transform, SpatialRefOpError>> + Send + Sync;
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
                    let param_parent = gluon::Convertable::read(&mut gluon_data)?;
                    let param_transform = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SpatialInterface", method = "create_spatial",
                        param_parent = "SpatialRef", ? param_transform, "dispatching"
                    );
                    let (spatial) = self
                        .create_spatial(ctx, param_parent, param_transform)
                        .await;
                    drop(gluon_data);
                    tracing::trace!(
                        interface = "SpatialInterface", method = "create_spatial", ?
                        spatial, "←"
                    );
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SpatialInterface", method =
                        "get_relative_bounding_box", param_relative_to = "SpatialRef",
                        param_spatial = "SpatialRef", "dispatching"
                    );
                    let (bounding_box) = self
                        .get_relative_bounding_box(ctx, param_relative_to, param_spatial)
                        .await;
                    drop(gluon_data);
                    tracing::trace!(
                        interface = "SpatialInterface", method =
                        "get_relative_bounding_box", ? bounding_box, "←"
                    );
                    bounding_box.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SpatialInterface", method =
                        "get_relative_transform", param_relative_to = "SpatialRef",
                        param_spatial = "SpatialRef", "dispatching"
                    );
                    let (transform) = self
                        .get_relative_transform(ctx, param_relative_to, param_spatial)
                        .await;
                    drop(gluon_data);
                    tracing::trace!(
                        interface = "SpatialInterface", method =
                        "get_relative_transform", ? transform, "←"
                    );
                    transform.write_owned(&mut gluon_out)?;
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
