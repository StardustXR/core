#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Spatial",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "Transform",
            supported_derives: gluon_wire::Derives::from_bits_truncate(3u32),
        },
        gluon_wire::ExternalGluonType {
            name: "PartialTransform",
            supported_derives: gluon_wire::Derives::from_bits_truncate(3u32),
        },
        gluon_wire::ExternalGluonType {
            name: "BoundingBox",
            supported_derives: gluon_wire::Derives::from_bits_truncate(3u32),
        },
    ],
};
///Transform
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: mint::Vector3<f32>,
    pub rotation: mint::Quaternion<f32>,
    pub scale: mint::Vector3<f32>,
}
impl gluon_wire::GluonConvertable for Transform {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: super::types::Vec3F = self.translation.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Quatf = self.rotation.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Vec3F = self.scale.clone().into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let translation: mint::Vector3<f32> = {
            let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let rotation: mint::Quaternion<f32> = {
            let __w: super::types::Quatf = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let scale: mint::Vector3<f32> = {
            let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: super::types::Vec3F = self.translation.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Quatf = self.rotation.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Vec3F = self.scale.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///Transform
#[derive(Debug, Copy, Clone)]
pub struct PartialTransform {
    pub translation: Option<mint::Vector3<f32>>,
    pub rotation: Option<mint::Quaternion<f32>>,
    pub scale: Option<mint::Vector3<f32>>,
}
impl gluon_wire::GluonConvertable for PartialTransform {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: Option<super::types::Vec3F> = self
                .translation
                .clone()
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<super::types::Quatf> = self
                .rotation
                .clone()
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<super::types::Vec3F> = self
                .scale
                .clone()
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let translation: Option<mint::Vector3<f32>> = {
            let __w: Option<super::types::Vec3F> = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.map(|__v| __v.into())
        };
        let rotation: Option<mint::Quaternion<f32>> = {
            let __w: Option<super::types::Quatf> = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.map(|__v| __v.into())
        };
        let scale: Option<mint::Vector3<f32>> = {
            let __w: Option<super::types::Vec3F> = gluon_wire::GluonConvertable::read(
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: Option<super::types::Vec3F> = self
                .translation
                .map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<super::types::Quatf> = self.rotation.map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: Option<super::types::Vec3F> = self.scale.map(|__v| __v.into());
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
///Bounding box
#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    pub center: mint::Vector3<f32>,
    pub extents: mint::Vector3<f32>,
}
impl gluon_wire::GluonConvertable for BoundingBox {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: super::types::Vec3F = self.center.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Vec3F = self.extents.clone().into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let center: mint::Vector3<f32> = {
            let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let extents: mint::Vector3<f32> = {
            let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        Ok(BoundingBox { center, extents })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: super::types::Vec3F = self.center.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Vec3F = self.extents.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct SpatialRef {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for SpatialRef {
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
        Ok(SpatialRef::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl SpatialRef {
    pub fn from_handler<H: SpatialRefHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> SpatialRef {
        SpatialRef::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> SpatialRef {
        SpatialRef { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for SpatialRef {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait SpatialRefHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
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
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for Spatial {
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
        Ok(Spatial::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Spatial {
    ///Get the spatial ref for this spatial object.
    pub async fn spatial_ref(&self) -> Result<SpatialRef, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the bounding box of this spatial and its children relative to itself
    pub async fn get_local_bounding_box(
        &self,
    ) -> Result<BoundingBox, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the bounding box of this spatial and its children relative to another spatial.
    pub async fn get_relative_bounding_box(
        &self,
        relative_to: impl Into<SpatialRef>,
    ) -> Result<BoundingBox, gluon_wire::GluonSendError> {
        let relative_to: SpatialRef = relative_to.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the transform of this spatial object.
    pub async fn get_relative_transform(
        &self,
        relative_to: impl Into<SpatialRef>,
    ) -> Result<Transform, gluon_wire::GluonSendError> {
        let relative_to: SpatialRef = relative_to.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    /**Sets the parent of this spatial object, keeping the local transform.
It will silently error and not set the spatial parent if it is to a child of itself.*/
    pub fn set_parent(
        &self,
        parent: impl Into<SpatialRef>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let parent: SpatialRef = parent.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
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
    ) -> Result<(), gluon_wire::GluonSendError> {
        let parent: SpatialRef = parent.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
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
    ) -> Result<(), gluon_wire::GluonSendError> {
        let transform: PartialTransform = transform.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
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
    ) -> Result<(), gluon_wire::GluonSendError> {
        let relative_to: SpatialRef = relative_to.into();
        let transform: PartialTransform = transform.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        relative_to.write(&mut gluon_builder)?;
        transform.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 15u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: SpatialHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> Spatial {
        Spatial::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Spatial {
        Spatial { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for Spatial {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait SpatialHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Get the spatial ref for this spatial object.
    fn spatial_ref(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = SpatialRef> + Send + Sync;
    ///Get the bounding box of this spatial and its children relative to itself
    fn get_local_bounding_box(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = BoundingBox> + Send + Sync;
    ///Get the bounding box of this spatial and its children relative to another spatial.
    fn get_relative_bounding_box(
        &self,
        _ctx: gluon_wire::GluonCtx,
        relative_to: SpatialRef,
    ) -> impl Future<Output = BoundingBox> + Send + Sync;
    ///Get the transform of this spatial object.
    fn get_relative_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
        relative_to: SpatialRef,
    ) -> impl Future<Output = Transform> + Send + Sync;
    /**Sets the parent of this spatial object, keeping the local transform.
It will silently error and not set the spatial parent if it is to a child of itself.*/
    fn set_parent(
        &self,
        _ctx: gluon_wire::GluonCtx,
        parent: SpatialRef,
    ) -> impl Future<Output = ()> + Send + Sync;
    /**Sets the parent of this spatial object, keeping its position in space.
It will silently error and not set the spatial parent if it is to a child of itself.*/
    fn set_parent_in_place(
        &self,
        _ctx: gluon_wire::GluonCtx,
        parent: SpatialRef,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///Set the transform of this spatial relative to its spatial parent.
    fn set_local_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
        transform: PartialTransform,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///Set the transform of this spatial relative to another spatial.
    fn set_relative_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
        relative_to: SpatialRef,
        transform: PartialTransform,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (spatial) = self.spatial_ref(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (bounding_box) = self.get_local_bounding_box(ctx).await;
                    drop(gluon_data);
                    bounding_box.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_relative_to = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (bounding_box) = self
                        .get_relative_bounding_box(ctx, param_relative_to)
                        .await;
                    drop(gluon_data);
                    bounding_box.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                11u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_relative_to = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (transform) = self
                        .get_relative_transform(ctx, param_relative_to)
                        .await;
                    drop(gluon_data);
                    transform.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                12u32 => {
                    let param_parent = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_parent(ctx, param_parent).await;
                }
                13u32 => {
                    let param_parent = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_parent_in_place(ctx, param_parent).await;
                }
                14u32 => {
                    let param_transform = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_local_transform(ctx, param_transform).await;
                }
                15u32 => {
                    let param_relative_to = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_transform = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
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
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for SpatialInterface {
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
        Ok(SpatialInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl SpatialInterface {
    ///Create a new spatial object.
    pub async fn create_spatial(
        &self,
        parent: impl Into<SpatialRef>,
        transform: impl Into<Transform>,
    ) -> Result<Spatial, gluon_wire::GluonSendError> {
        let parent: SpatialRef = parent.into();
        let transform: Transform = transform.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        parent.write(&mut gluon_builder)?;
        transform.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the relative bounding box of a spatial object relative to another spatial.
    pub async fn get_relative_bounding_box(
        &self,
        relative_to: impl Into<SpatialRef>,
        spatial: impl Into<SpatialRef>,
    ) -> Result<BoundingBox, gluon_wire::GluonSendError> {
        let relative_to: SpatialRef = relative_to.into();
        let spatial: SpatialRef = spatial.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the relative transform of a spatial object relative to another spatial.
    pub async fn get_relative_transform(
        &self,
        relative_to: impl Into<SpatialRef>,
        spatial: impl Into<SpatialRef>,
    ) -> Result<Transform, gluon_wire::GluonSendError> {
        let relative_to: SpatialRef = relative_to.into();
        let spatial: SpatialRef = spatial.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: SpatialInterfaceHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> SpatialInterface {
        SpatialInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> SpatialInterface {
        SpatialInterface { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for SpatialInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait SpatialInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Create a new spatial object.
    fn create_spatial(
        &self,
        _ctx: gluon_wire::GluonCtx,
        parent: SpatialRef,
        transform: Transform,
    ) -> impl Future<Output = Spatial> + Send + Sync;
    ///Get the relative bounding box of a spatial object relative to another spatial.
    fn get_relative_bounding_box(
        &self,
        _ctx: gluon_wire::GluonCtx,
        relative_to: SpatialRef,
        spatial: SpatialRef,
    ) -> impl Future<Output = BoundingBox> + Send + Sync;
    ///Get the relative transform of a spatial object relative to another spatial.
    fn get_relative_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
        relative_to: SpatialRef,
        spatial: SpatialRef,
    ) -> impl Future<Output = Transform> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_parent = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_transform = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (spatial) = self
                        .create_spatial(ctx, param_parent, param_transform)
                        .await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_relative_to = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_spatial = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (bounding_box) = self
                        .get_relative_bounding_box(ctx, param_relative_to, param_spatial)
                        .await;
                    drop(gluon_data);
                    bounding_box.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_relative_to = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_spatial = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (transform) = self
                        .get_relative_transform(ctx, param_relative_to, param_spatial)
                        .await;
                    drop(gluon_data);
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
