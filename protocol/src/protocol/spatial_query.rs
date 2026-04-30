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
            name: "PointsQuery",
            supported_derives: gluon_wire::Derives::from_bits_truncate(2u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Point",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
    ],
};
///shoot a beam and return everything it hit
#[derive(Debug, Clone)]
pub struct BeamQuery {
    pub handler: BeamQueryHandler,
    pub interfaces: Vec<super::query::InterfaceDependency>,
    pub reference_spatial: super::spatial::SpatialRef,
    pub origin: super::types::Vec3F,
    pub direction: super::types::Vec3F,
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
        self.reference_spatial.write(gluon_data)?;
        self.origin.write(gluon_data)?;
        self.direction.write(gluon_data)?;
        self.max_length.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let handler = gluon_wire::GluonConvertable::read(gluon_data)?;
        let interfaces = gluon_wire::GluonConvertable::read(gluon_data)?;
        let reference_spatial = gluon_wire::GluonConvertable::read(gluon_data)?;
        let origin = gluon_wire::GluonConvertable::read(gluon_data)?;
        let direction = gluon_wire::GluonConvertable::read(gluon_data)?;
        let max_length = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(BeamQuery {
            handler,
            interfaces,
            reference_spatial,
            origin,
            direction,
            max_length,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handler.write_owned(gluon_data)?;
        self.interfaces.write_owned(gluon_data)?;
        self.reference_spatial.write_owned(gluon_data)?;
        self.origin.write_owned(gluon_data)?;
        self.direction.write_owned(gluon_data)?;
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
///Get interfaces of fields containing any points
#[derive(Debug, Clone)]
pub struct PointsQuery {
    pub handler: PointsQueryHandler,
    pub interfaces: Vec<super::query::InterfaceDependency>,
    pub reference_spatial: super::spatial::SpatialRef,
    pub points: Vec<Point>,
}
impl gluon_wire::GluonConvertable for PointsQuery {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handler.write(gluon_data)?;
        self.interfaces.write(gluon_data)?;
        self.reference_spatial.write(gluon_data)?;
        self.points.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let handler = gluon_wire::GluonConvertable::read(gluon_data)?;
        let interfaces = gluon_wire::GluonConvertable::read(gluon_data)?;
        let reference_spatial = gluon_wire::GluonConvertable::read(gluon_data)?;
        let points = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(PointsQuery {
            handler,
            interfaces,
            reference_spatial,
            points,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handler.write_owned(gluon_data)?;
        self.interfaces.write_owned(gluon_data)?;
        self.reference_spatial.write_owned(gluon_data)?;
        self.points.write_owned(gluon_data)?;
        Ok(())
    }
}
///Point for a PointsQuery
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub point: super::types::Vec3F,
    pub margin: f32,
}
impl gluon_wire::GluonConvertable for Point {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.point.write(gluon_data)?;
        self.margin.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let point = gluon_wire::GluonConvertable::read(gluon_data)?;
        let margin = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Point { point, margin })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.point.write_owned(gluon_data)?;
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
        spatial: super::spatial::SpatialRef,
        interfaces: Vec<super::query::QueriedInterface>,
        deepest_point_distance: f32,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
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
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        spatial: super::spatial::SpatialRef,
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
        spatial: super::spatial::SpatialRef,
        interfaces: Vec<super::query::QueriedInterface>,
        relative_position: super::types::Vec3F,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
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
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        spatial: super::spatial::SpatialRef,
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
pub struct PointsQueryHandler {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for PointsQueryHandler {
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
        Ok(PointsQueryHandler::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PointsQueryHandler {
    pub fn entered(
        &self,
        obj: super::query::QueryableObjectRef,
        field: super::field::FieldRef,
        spatial: super::spatial::SpatialRef,
        interfaces: Vec<super::query::QueriedInterface>,
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
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
        distance: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        obj.write(&mut gluon_builder)?;
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
    pub fn from_handler<H: PointsQueryHandlerHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> PointsQueryHandler {
        PointsQueryHandler::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> PointsQueryHandler {
        PointsQueryHandler { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for PointsQueryHandler {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for PointsQueryHandler {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for PointsQueryHandler {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for PointsQueryHandler {}
pub trait PointsQueryHandlerHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn entered(
        &self,
        _ctx: gluon_wire::GluonCtx,
        obj: super::query::QueryableObjectRef,
        field: super::field::FieldRef,
        spatial: super::spatial::SpatialRef,
        interfaces: Vec<super::query::QueriedInterface>,
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
    ) -> Result<SpatialQueryGuard, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn zone_query(
        &self,
        query: ZoneQuery,
    ) -> Result<SpatialQueryGuard, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn points_query(
        &self,
        query: PointsQuery,
    ) -> Result<PointsQueryHandle, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: SpatialQueryInterfaceHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
    ) -> impl Future<Output = SpatialQueryGuard> + Send + Sync;
    fn zone_query(
        &self,
        _ctx: gluon_wire::GluonCtx,
        query: ZoneQuery,
    ) -> impl Future<Output = SpatialQueryGuard> + Send + Sync;
    fn points_query(
        &self,
        _ctx: gluon_wire::GluonCtx,
        query: PointsQuery,
    ) -> impl Future<Output = PointsQueryHandle> + Send + Sync;
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
                    let (guard) = self
                        .beam_query(ctx, gluon_wire::GluonConvertable::read(gluon_data)?)
                        .await;
                    guard.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (guard) = self
                        .zone_query(ctx, gluon_wire::GluonConvertable::read(gluon_data)?)
                        .await;
                    guard.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (handle) = self
                        .points_query(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    handle.write_owned(&mut gluon_out)?;
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
#[derive(Debug, Clone)]
pub struct PointsQueryHandle {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for PointsQueryHandle {
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
        Ok(PointsQueryHandle::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PointsQueryHandle {
    pub fn update_points(
        &self,
        points: Vec<Point>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        points.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: PointsQueryHandleHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> PointsQueryHandle {
        PointsQueryHandle::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> PointsQueryHandle {
        PointsQueryHandle { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for PointsQueryHandle {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for PointsQueryHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for PointsQueryHandle {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for PointsQueryHandle {}
pub trait PointsQueryHandleHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn update_points(
        &self,
        _ctx: gluon_wire::GluonCtx,
        points: Vec<Point>,
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
                    self.update_points(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
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
pub struct SpatialQueryGuard {
    obj: binderbinder::binder_object::BinderObjectOrRef,
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
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        SpatialQueryGuard { obj }
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
pub trait SpatialQueryGuardHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
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
