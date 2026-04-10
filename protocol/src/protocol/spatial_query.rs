#![allow(
    unused,
    clippy::single_match,
    clippy::match_single_binding,
    clippy::large_enum_variant
)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.SpatialQuery",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "BeamQuery",
            supported_derives: gluon_wire::Derives::from_bits_truncate(2u32),
        },
        gluon_wire::ExternalGluonType {
            name: "ZoneQuery",
            supported_derives: gluon_wire::Derives::from_bits_truncate(2u32),
        },
        gluon_wire::ExternalGluonType {
            name: "QueryInterface",
            supported_derives: gluon_wire::Derives::from_bits_truncate(30u32),
        },
    ],
};
///shoot a beam and return everything it hit
#[derive(Debug, Clone)]
pub struct BeamQuery {
    pub handler: BeamQueryHandler,
    pub interfaces: Vec<QueryInterface>,
    pub origin_spatial: super::spatial::SpatialRef,
    pub direction: super::types::Vec3F,
    ///the closest <N> things to the beam
    pub limit: u8,
    ///Maximum length of the beam in meters, can be the max f32 value
    pub max_length: f32,
}
impl gluon_wire::GluonConvertable for BeamQuery {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handler.write(gluon_data)?;
        self.interfaces.write(gluon_data)?;
        self.origin_spatial.write(gluon_data)?;
        self.direction.write(gluon_data)?;
        self.limit.write(gluon_data)?;
        self.max_length.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let handler = gluon_wire::GluonConvertable::read(gluon_data)?;
        let interfaces = gluon_wire::GluonConvertable::read(gluon_data)?;
        let origin_spatial = gluon_wire::GluonConvertable::read(gluon_data)?;
        let direction = gluon_wire::GluonConvertable::read(gluon_data)?;
        let limit = gluon_wire::GluonConvertable::read(gluon_data)?;
        let max_length = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(BeamQuery {
            handler,
            interfaces,
            origin_spatial,
            direction,
            limit,
            max_length,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handler.write_owned(gluon_data)?;
        self.interfaces.write_owned(gluon_data)?;
        self.origin_spatial.write_owned(gluon_data)?;
        self.direction.write_owned(gluon_data)?;
        self.limit.write_owned(gluon_data)?;
        self.max_length.write_owned(gluon_data)?;
        Ok(())
    }
}
///Get interfaces inside this field
#[derive(Debug, Clone)]
pub struct ZoneQuery {
    pub handler: ZoneQueryHandler,
    pub interfaces: Vec<QueryInterface>,
    pub zone_field: super::field::FieldRef,
    ///how far from the surface of the field to include spatials
    pub offset_distance: f32,
}
impl gluon_wire::GluonConvertable for ZoneQuery {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handler.write(gluon_data)?;
        self.interfaces.write(gluon_data)?;
        self.zone_field.write(gluon_data)?;
        self.offset_distance.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let handler = gluon_wire::GluonConvertable::read(gluon_data)?;
        let interfaces = gluon_wire::GluonConvertable::read(gluon_data)?;
        let zone_field = gluon_wire::GluonConvertable::read(gluon_data)?;
        let offset_distance = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(ZoneQuery {
            handler,
            interfaces,
            zone_field,
            offset_distance,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handler.write_owned(gluon_data)?;
        self.interfaces.write_owned(gluon_data)?;
        self.zone_field.write_owned(gluon_data)?;
        self.offset_distance.write_owned(gluon_data)?;
        Ok(())
    }
}
/**query interface
could maybe make this a struct idk*/
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum QueryInterface {
    Required { id: String },
    Optional { id: String },
}
impl gluon_wire::GluonConvertable for QueryInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            QueryInterface::Required { id } => {
                gluon_data.write_u16(0u16)?;
                id.write(gluon_data)?;
            }
            QueryInterface::Optional { id } => {
                gluon_data.write_u16(1u16)?;
                id.write(gluon_data)?;
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
                    let id = gluon_wire::GluonConvertable::read(gluon_data)?;
                    QueryInterface::Required { id }
                }
                1u16 => {
                    let id = gluon_wire::GluonConvertable::read(gluon_data)?;
                    QueryInterface::Optional { id }
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
            QueryInterface::Required { id } => {
                gluon_data.write_u16(0u16)?;
                id.write_owned(gluon_data)?;
            }
            QueryInterface::Optional { id } => {
                gluon_data.write_u16(1u16)?;
                id.write_owned(gluon_data)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct BeamQueryHandler {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for BeamQueryHandler {
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
        Ok(BeamQueryHandler::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl BeamQueryHandler {
    pub fn intersected(
        &self,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        interface_id: String,
        field: super::field::FieldRef,
        deepest_point_distance: f32,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        interface.write(&mut gluon_builder)?;
        interface_id.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        deepest_point_distance.write(&mut gluon_builder)?;
        distance.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn moved(
        &self,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        deepest_point_distance: f32,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        interface.write(&mut gluon_builder)?;
        deepest_point_distance.write(&mut gluon_builder)?;
        distance.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn left(
        &self,
        interface: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        interface.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: BeamQueryHandlerHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> BeamQueryHandler {
        BeamQueryHandler::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> BeamQueryHandler {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new(&obj));
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        BeamQueryHandler {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for BeamQueryHandler {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for BeamQueryHandler {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for BeamQueryHandler {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for BeamQueryHandler {}
pub trait BeamQueryHandlerHandler: binderbinder::device::TransactionHandler<
        ObjectResource = tokio::sync::RwLock<
            std::collections::HashMap<u64, gluon_wire::drop_tracking::DropNotifier>,
        >,
    > + Send + Sync + 'static {
    fn intersected(
        &self,
        _ctx: gluon_wire::GluonCtx,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        interface_id: String,
        field: super::field::FieldRef,
        deepest_point_distance: f32,
        distance: f32,
    );
    fn moved(
        &self,
        _ctx: gluon_wire::GluonCtx,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        deepest_point_distance: f32,
        distance: f32,
    );
    fn left(
        &self,
        _ctx: gluon_wire::GluonCtx,
        interface: binderbinder::binder_object::BinderObjectOrRef,
    );
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
        obj_res: &Self::ObjectResource,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                4 => {
                    use std::hash::BuildHasher as _;
                    let Ok(obj) = gluon_data.read_binder() else {
                        return Ok(out);
                    };
                    let hash = std::hash::RandomState::new().hash_one(obj.clone());
                    if out.write_u64(hash).is_err() {
                        return Ok(out);
                    }
                    obj_res
                        .write()
                        .await
                        .insert(
                            hash,
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        );
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
        obj_res: &Self::ObjectResource,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(id) = gluon_data.read_u64() else {
                        return Ok(());
                    };
                    if let Some(mut obj) = obj_res.write().await.remove(&id) {
                        obj.abort();
                    }
                }
                8u32 => {
                    self.intersected(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                9u32 => {
                    self.moved(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                10u32 => {
                    self.left(ctx, gluon_wire::GluonConvertable::read(gluon_data)?);
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct ZoneQueryHandler {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for ZoneQueryHandler {
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
        Ok(ZoneQueryHandler::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl ZoneQueryHandler {
    pub fn entered(
        &self,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        interface_id: String,
        field: super::field::FieldRef,
        relative_position: super::types::Vec3F,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        interface.write(&mut gluon_builder)?;
        interface_id.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        relative_position.write(&mut gluon_builder)?;
        distance.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn moved(
        &self,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        relative_position: super::types::Vec3F,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        interface.write(&mut gluon_builder)?;
        relative_position.write(&mut gluon_builder)?;
        distance.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn left(
        &self,
        interface: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        interface.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: ZoneQueryHandlerHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> ZoneQueryHandler {
        ZoneQueryHandler::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> ZoneQueryHandler {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new(&obj));
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        ZoneQueryHandler {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for ZoneQueryHandler {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for ZoneQueryHandler {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for ZoneQueryHandler {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for ZoneQueryHandler {}
pub trait ZoneQueryHandlerHandler: binderbinder::device::TransactionHandler<
        ObjectResource = tokio::sync::RwLock<
            std::collections::HashMap<u64, gluon_wire::drop_tracking::DropNotifier>,
        >,
    > + Send + Sync + 'static {
    fn entered(
        &self,
        _ctx: gluon_wire::GluonCtx,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        interface_id: String,
        field: super::field::FieldRef,
        relative_position: super::types::Vec3F,
        distance: f32,
    );
    fn moved(
        &self,
        _ctx: gluon_wire::GluonCtx,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        relative_position: super::types::Vec3F,
        distance: f32,
    );
    fn left(
        &self,
        _ctx: gluon_wire::GluonCtx,
        interface: binderbinder::binder_object::BinderObjectOrRef,
    );
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
        obj_res: &Self::ObjectResource,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                4 => {
                    use std::hash::BuildHasher as _;
                    let Ok(obj) = gluon_data.read_binder() else {
                        return Ok(out);
                    };
                    let hash = std::hash::RandomState::new().hash_one(obj.clone());
                    if out.write_u64(hash).is_err() {
                        return Ok(out);
                    }
                    obj_res
                        .write()
                        .await
                        .insert(
                            hash,
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        );
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
        obj_res: &Self::ObjectResource,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(id) = gluon_data.read_u64() else {
                        return Ok(());
                    };
                    if let Some(mut obj) = obj_res.write().await.remove(&id) {
                        obj.abort();
                    }
                }
                8u32 => {
                    self.entered(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                9u32 => {
                    self.moved(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                10u32 => {
                    self.left(ctx, gluon_wire::GluonConvertable::read(gluon_data)?);
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpatialQueryGuard {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for SpatialQueryGuard {
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
        Ok(SpatialQueryGuard::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl SpatialQueryGuard {
    pub fn from_handler<H: SpatialQueryGuardHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> SpatialQueryGuard {
        SpatialQueryGuard::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> SpatialQueryGuard {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new(&obj));
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        SpatialQueryGuard {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for SpatialQueryGuard {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for SpatialQueryGuard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for SpatialQueryGuard {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for SpatialQueryGuard {}
pub trait SpatialQueryGuardHandler: binderbinder::device::TransactionHandler<
        ObjectResource = tokio::sync::RwLock<
            std::collections::HashMap<u64, gluon_wire::drop_tracking::DropNotifier>,
        >,
    > + Send + Sync + 'static {
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
        obj_res: &Self::ObjectResource,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                4 => {
                    use std::hash::BuildHasher as _;
                    let Ok(obj) = gluon_data.read_binder() else {
                        return Ok(out);
                    };
                    let hash = std::hash::RandomState::new().hash_one(obj.clone());
                    if out.write_u64(hash).is_err() {
                        return Ok(out);
                    }
                    obj_res
                        .write()
                        .await
                        .insert(
                            hash,
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        );
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
        obj_res: &Self::ObjectResource,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(id) = gluon_data.read_u64() else {
                        return Ok(());
                    };
                    if let Some(mut obj) = obj_res.write().await.remove(&id) {
                        obj.abort();
                    }
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpatialQueryInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for SpatialQueryInterface {
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
        Ok(SpatialQueryInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl SpatialQueryInterface {
    pub async fn register_interface(
        &self,
        field: super::field::FieldRef,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        interface_id: String,
    ) -> Result<SpatialQueryGuard, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.register_interface_blocking(field, interface, interface_id)
            })
            .await
            .unwrap()
    }
    pub fn register_interface_blocking(
        &self,
        field: super::field::FieldRef,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        interface_id: String,
    ) -> Result<SpatialQueryGuard, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        field.write(&mut gluon_builder)?;
        interface.write(&mut gluon_builder)?;
        interface_id.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn beam_query(
        &self,
        query: BeamQuery,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.beam_query_blocking(query))
            .await
            .unwrap()
    }
    pub fn beam_query_blocking(
        &self,
        query: BeamQuery,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        query.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(())
    }
    pub async fn zone_query(
        &self,
        query: ZoneQuery,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.zone_query_blocking(query))
            .await
            .unwrap()
    }
    pub fn zone_query_blocking(
        &self,
        query: ZoneQuery,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        query.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(())
    }
    pub fn from_handler<H: SpatialQueryInterfaceHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> SpatialQueryInterface {
        SpatialQueryInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> SpatialQueryInterface {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new(&obj));
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        SpatialQueryInterface {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for SpatialQueryInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for SpatialQueryInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for SpatialQueryInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for SpatialQueryInterface {}
pub trait SpatialQueryInterfaceHandler: binderbinder::device::TransactionHandler<
        ObjectResource = tokio::sync::RwLock<
            std::collections::HashMap<u64, gluon_wire::drop_tracking::DropNotifier>,
        >,
    > + Send + Sync + 'static {
    fn register_interface(
        &self,
        _ctx: gluon_wire::GluonCtx,
        field: super::field::FieldRef,
        interface: binderbinder::binder_object::BinderObjectOrRef,
        interface_id: String,
    ) -> impl Future<Output = SpatialQueryGuard> + Send + Sync;
    fn beam_query(
        &self,
        _ctx: gluon_wire::GluonCtx,
        query: BeamQuery,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn zone_query(
        &self,
        _ctx: gluon_wire::GluonCtx,
        query: ZoneQuery,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
        obj_res: &Self::ObjectResource,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                4 => {
                    use std::hash::BuildHasher as _;
                    let Ok(obj) = gluon_data.read_binder() else {
                        return Ok(out);
                    };
                    let hash = std::hash::RandomState::new().hash_one(obj.clone());
                    if out.write_u64(hash).is_err() {
                        return Ok(out);
                    }
                    obj_res
                        .write()
                        .await
                        .insert(
                            hash,
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        );
                }
                8u32 => {
                    let (guard) = self
                        .register_interface(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    guard.write_owned(&mut out)?;
                }
                9u32 => {
                    let () = self
                        .beam_query(ctx, gluon_wire::GluonConvertable::read(gluon_data)?)
                        .await;
                }
                10u32 => {
                    let () = self
                        .zone_query(ctx, gluon_wire::GluonConvertable::read(gluon_data)?)
                        .await;
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
        obj_res: &Self::ObjectResource,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(id) = gluon_data.read_u64() else {
                        return Ok(());
                    };
                    if let Some(mut obj) = obj_res.write().await.remove(&id) {
                        obj.abort();
                    }
                }
                _ => {}
            }
            Ok(())
        }
    }
}
