#![allow(unused, clippy::single_match, clippy::match_single_binding)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Spatial",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "Transform",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "BoundingBox",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
    ],
};
///Transform
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    pub position: super::types::Vec3F,
    pub rotation: super::types::Quatf,
    pub scale: f32,
}
impl gluon_wire::GluonConvertable for Transform {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.position.write(data)?;
        self.rotation.write(data)?;
        self.scale.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let position = gluon_wire::GluonConvertable::read(data)?;
        let rotation = gluon_wire::GluonConvertable::read(data)?;
        let scale = gluon_wire::GluonConvertable::read(data)?;
        Ok(Transform {
            position,
            rotation,
            scale,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.position.write_owned(data)?;
        self.rotation.write_owned(data)?;
        self.scale.write_owned(data)?;
        Ok(())
    }
}
///Bounding box
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoundingBox {
    pub center: super::types::Vec3F,
    pub extents: super::types::Vec3F,
}
impl gluon_wire::GluonConvertable for BoundingBox {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.center.write(data)?;
        self.extents.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let center = gluon_wire::GluonConvertable::read(data)?;
        let extents = gluon_wire::GluonConvertable::read(data)?;
        Ok(BoundingBox { center, extents })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.center.write_owned(data)?;
        self.extents.write_owned(data)?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct SpatialRef {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for SpatialRef {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(data)
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(data)?;
        Ok(SpatialRef::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl SpatialRef {
    pub fn from_handler<H: SpatialRefHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
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
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        SpatialRef {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for SpatialRef {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait SpatialRefHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
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
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
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
pub struct Spatial {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for Spatial {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(data)
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(data)?;
        Ok(Spatial::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl Spatial {
    ///Get the spatial ref for this spatial object.
    pub async fn get_ref(&self) -> Result<SpatialRef, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_ref_blocking()).await.unwrap()
    }
    pub fn get_ref_blocking(&self) -> Result<SpatialRef, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the bounding box of this spatial and its children relative to itself
    pub async fn get_local_bounding_box(
        &self,
    ) -> Result<BoundingBox, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_local_bounding_box_blocking())
            .await
            .unwrap()
    }
    pub fn get_local_bounding_box_blocking(
        &self,
    ) -> Result<BoundingBox, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the bounding box of this spatial and its children relative to another spatial.
    pub async fn get_world_bounding_box(
        &self,
        relative_to: SpatialRef,
    ) -> Result<BoundingBox, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.get_world_bounding_box_blocking(relative_to)
            })
            .await
            .unwrap()
    }
    pub fn get_world_bounding_box_blocking(
        &self,
        relative_to: SpatialRef,
    ) -> Result<BoundingBox, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        relative_to.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the transform of this spatial object.
    pub async fn get_relative_transform(
        &self,
        relative_to: SpatialRef,
    ) -> Result<Transform, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.get_relative_transform_blocking(relative_to)
            })
            .await
            .unwrap()
    }
    pub fn get_relative_transform_blocking(
        &self,
        relative_to: SpatialRef,
    ) -> Result<Transform, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        relative_to.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 11u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    /**Sets the parent of this spatial object, keeping the local transform.
It will silently error and not set the spatial parent if it is to a child of itself.*/
    pub fn set_parent(
        &self,
        parent: SpatialRef,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        parent.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 12u32, builder.to_payload())?;
        Ok(())
    }
    /**Sets the parent of this spatial object, keeping its position in space.
It will silently error and not set the spatial parent if it is to a child of itself.*/
    pub fn set_parent_in_place(
        &self,
        parent: SpatialRef,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        parent.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 13u32, builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: SpatialHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
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
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        Spatial { obj, drop_notification }
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
impl binderbinder::binder_object::ToBinderObjectOrRef for Spatial {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait SpatialHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Get the spatial ref for this spatial object.
    fn get_ref(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = SpatialRef> + Send + Sync;
    ///Get the bounding box of this spatial and its children relative to itself
    fn get_local_bounding_box(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = BoundingBox> + Send + Sync;
    ///Get the bounding box of this spatial and its children relative to another spatial.
    fn get_world_bounding_box(
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
    fn set_parent(&self, _ctx: gluon_wire::GluonCtx, parent: SpatialRef);
    /**Sets the parent of this spatial object, keeping its position in space.
It will silently error and not set the spatial parent if it is to a child of itself.*/
    fn set_parent_in_place(&self, _ctx: gluon_wire::GluonCtx, parent: SpatialRef);
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
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
                    let (spatial) = self.get_ref(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                9u32 => {
                    let (bounding_box) = self.get_local_bounding_box(ctx).await;
                    bounding_box.write_owned(&mut out)?;
                }
                10u32 => {
                    let (bounding_box) = self
                        .get_world_bounding_box(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    bounding_box.write_owned(&mut out)?;
                }
                11u32 => {
                    let (transform) = self
                        .get_relative_transform(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    transform.write_owned(&mut out)?;
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                12u32 => {
                    self.set_parent(ctx, gluon_wire::GluonConvertable::read(data)?);
                }
                13u32 => {
                    self.set_parent_in_place(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
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
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for SpatialInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(data)
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(data)?;
        Ok(SpatialInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl SpatialInterface {
    ///Create a new spatial object.
    pub async fn create_spatial(
        &self,
    ) -> Result<SpatialRef, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.create_spatial_blocking())
            .await
            .unwrap()
    }
    pub fn create_spatial_blocking(
        &self,
    ) -> Result<SpatialRef, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the relative bounding box of a spatial object relative to another spatial.
    pub async fn get_relative_bounding_box(
        &self,
        relative_to: SpatialRef,
        spatial: SpatialRef,
    ) -> Result<BoundingBox, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.get_relative_bounding_box_blocking(relative_to, spatial)
            })
            .await
            .unwrap()
    }
    pub fn get_relative_bounding_box_blocking(
        &self,
        relative_to: SpatialRef,
        spatial: SpatialRef,
    ) -> Result<BoundingBox, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        relative_to.write(&mut builder)?;
        spatial.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the relative transform of a spatial object relative to another spatial.
    pub async fn get_relative_transform(
        &self,
        relative_to: SpatialRef,
        spatial: SpatialRef,
    ) -> Result<Transform, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.get_relative_transform_blocking(relative_to, spatial)
            })
            .await
            .unwrap()
    }
    pub fn get_relative_transform_blocking(
        &self,
        relative_to: SpatialRef,
        spatial: SpatialRef,
    ) -> Result<Transform, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        relative_to.write(&mut builder)?;
        spatial.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: SpatialInterfaceHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
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
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        SpatialInterface {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for SpatialInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait SpatialInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Create a new spatial object.
    fn create_spatial(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = SpatialRef> + Send + Sync;
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
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
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
                    let (spatial) = self.create_spatial(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                9u32 => {
                    let (bounding_box) = self
                        .get_relative_bounding_box(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    bounding_box.write_owned(&mut out)?;
                }
                10u32 => {
                    let (transform) = self
                        .get_relative_transform(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    transform.write_owned(&mut out)?;
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
