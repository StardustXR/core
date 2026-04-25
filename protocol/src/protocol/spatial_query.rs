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
    ],
};
///shoot a beam and return everything it hit
#[derive(Debug, Clone)]
pub struct BeamQuery {
    pub handler: BeamQueryHandler,
    pub interfaces: Vec<super::query::InterfaceDependency>,
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
///Get interfaces intersecting this field
#[derive(Debug, Clone)]
pub struct ZoneQuery {
    pub handler: ZoneQueryHandler,
    pub interfaces: Vec<super::query::InterfaceDependency>,
    pub zone_field: super::field::FieldRef,
    pub margin: f32,
}
impl gluon_wire::GluonConvertable for ZoneQuery {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handler.write(gluon_data)?;
        self.interfaces.write(gluon_data)?;
        self.zone_field.write(gluon_data)?;
        self.margin.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let handler = gluon_wire::GluonConvertable::read(gluon_data)?;
        let interfaces = gluon_wire::GluonConvertable::read(gluon_data)?;
        let zone_field = gluon_wire::GluonConvertable::read(gluon_data)?;
        let margin = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(ZoneQuery {
            handler,
            interfaces,
            zone_field,
            margin,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handler.write_owned(gluon_data)?;
        self.interfaces.write_owned(gluon_data)?;
        self.zone_field.write_owned(gluon_data)?;
        self.margin.write_owned(gluon_data)?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct BeamQueryHandler {
    obj: binderbinder::binder_object::BinderObjectOrRef,
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
        obj: super::query::QueryableObjectRef,
        field: super::field::FieldRef,
        interfaces: Vec<super::query::QueriedInterface>,
        deepest_point_distance: f32,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        deepest_point_distance.write(&mut gluon_builder)?;
        distance.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn interfaces_changed(
        &self,
        obj: super::query::QueryableObjectRef,
        interfaces: Vec<super::query::QueriedInterface>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn moved(
        &self,
        obj: super::query::QueryableObjectRef,
        deepest_point_distance: f32,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        deepest_point_distance.write(&mut gluon_builder)?;
        distance.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn left(
        &self,
        obj: super::query::QueryableObjectRef,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: BeamQueryHandlerHandler>(
        obj: &binderbinder::binder_object::BinderObjectRef<H>,
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
        BeamQueryHandler { obj }
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
pub trait BeamQueryHandlerHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn intersected(
        &self,
        _ctx: gluon_wire::GluonCtx,
        obj: super::query::QueryableObjectRef,
        field: super::field::FieldRef,
        interfaces: Vec<super::query::QueriedInterface>,
        deepest_point_distance: f32,
        distance: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn interfaces_changed(
        &self,
        _ctx: gluon_wire::GluonCtx,
        obj: super::query::QueryableObjectRef,
        interfaces: Vec<super::query::QueriedInterface>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn moved(
        &self,
        _ctx: gluon_wire::GluonCtx,
        obj: super::query::QueryableObjectRef,
        deepest_point_distance: f32,
        distance: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn left(
        &self,
        _ctx: gluon_wire::GluonCtx,
        obj: super::query::QueryableObjectRef,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    self.intersected(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                9u32 => {
                    self.interfaces_changed(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                10u32 => {
                    self.moved(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                11u32 => {
                    self.left(ctx, gluon_wire::GluonConvertable::read(gluon_data)?)
                        .await;
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
        obj: super::query::QueryableObjectRef,
        field: super::field::FieldRef,
        interfaces: Vec<super::query::QueriedInterface>,
        relative_position: super::types::Vec3F,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        relative_position.write(&mut gluon_builder)?;
        distance.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn interfaces_changed(
        &self,
        obj: super::query::QueryableObjectRef,
        interfaces: Vec<super::query::QueriedInterface>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn moved(
        &self,
        obj: super::query::QueryableObjectRef,
        relative_position: super::types::Vec3F,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        relative_position.write(&mut gluon_builder)?;
        distance.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn left(
        &self,
        obj: super::query::QueryableObjectRef,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: ZoneQueryHandlerHandler>(
        obj: &binderbinder::binder_object::BinderObjectRef<H>,
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
        ZoneQueryHandler { obj }
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
pub trait ZoneQueryHandlerHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn entered(
        &self,
        _ctx: gluon_wire::GluonCtx,
        obj: super::query::QueryableObjectRef,
        field: super::field::FieldRef,
        interfaces: Vec<super::query::QueriedInterface>,
        relative_position: super::types::Vec3F,
        distance: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn interfaces_changed(
        &self,
        _ctx: gluon_wire::GluonCtx,
        obj: super::query::QueryableObjectRef,
        interfaces: Vec<super::query::QueriedInterface>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn moved(
        &self,
        _ctx: gluon_wire::GluonCtx,
        obj: super::query::QueryableObjectRef,
        relative_position: super::types::Vec3F,
        distance: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn left(
        &self,
        _ctx: gluon_wire::GluonCtx,
        obj: super::query::QueryableObjectRef,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    self.entered(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                9u32 => {
                    self.interfaces_changed(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                10u32 => {
                    self.moved(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                }
                11u32 => {
                    self.left(ctx, gluon_wire::GluonConvertable::read(gluon_data)?)
                        .await;
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
    pub async fn beam_query(
        &self,
        query: BeamQuery,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(())
    }
    pub async fn zone_query(
        &self,
        query: ZoneQuery,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(())
    }
    pub fn from_handler<H: SpatialQueryInterfaceHandler>(
        obj: &binderbinder::binder_object::BinderObjectRef<H>,
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
        SpatialQueryInterface { obj }
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
pub trait SpatialQueryInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
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
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let () = self
                        .beam_query(ctx, gluon_wire::GluonConvertable::read(gluon_data)?)
                        .await;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let () = self
                        .zone_query(ctx, gluon_wire::GluonConvertable::read(gluon_data)?)
                        .await;
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
