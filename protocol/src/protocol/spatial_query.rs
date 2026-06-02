#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.SpatialQuery",
    types: &[
        gluon::ExternalGluonType {
            name: "BeamQuery",
            supported_derives: gluon::Derives::from_bits_truncate(2u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "ZoneQuery",
            supported_derives: gluon::Derives::from_bits_truncate(2u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "PointsQuery",
            supported_derives: gluon::Derives::from_bits_truncate(2u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "Point",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "QueryError",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///shoot a beam and return everything it hit
#[derive(Debug, Clone)]
pub struct BeamQuery {
    pub handler: BeamQueryHandler,
    pub interfaces: Vec<super::query::InterfaceDependency>,
    pub reference_spatial: super::spatial::SpatialRef,
    pub origin: crate::types::Vec3F,
    pub direction: crate::types::Vec3F,
    ///Maximum length of the beam in meters, can be the max f32 value
    pub max_length: f32,
}
impl gluon::Convertable for BeamQuery {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.handler.write(gluon_data)?;
        self.interfaces.write(gluon_data)?;
        self.reference_spatial.write(gluon_data)?;
        {
            let __w: super::types::proxied::Vec3F = self.origin.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::proxied::Vec3F = self.direction.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.max_length.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let handler = gluon::Convertable::read(gluon_data)?;
        let interfaces = gluon::Convertable::read(gluon_data)?;
        let reference_spatial = gluon::Convertable::read(gluon_data)?;
        let origin: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let direction: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let max_length = gluon::Convertable::read(gluon_data)?;
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
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.handler.write_owned(gluon_data)?;
        self.interfaces.write_owned(gluon_data)?;
        self.reference_spatial.write_owned(gluon_data)?;
        {
            let __w: super::types::proxied::Vec3F = self.origin.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::proxied::Vec3F = self.direction.into();
            __w.write_owned(gluon_data)?;
        }
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
impl gluon::Convertable for ZoneQuery {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.handler.write(gluon_data)?;
        self.interfaces.write(gluon_data)?;
        self.zone_field.write(gluon_data)?;
        self.margin.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let handler = gluon::Convertable::read(gluon_data)?;
        let interfaces = gluon::Convertable::read(gluon_data)?;
        let zone_field = gluon::Convertable::read(gluon_data)?;
        let margin = gluon::Convertable::read(gluon_data)?;
        Ok(ZoneQuery {
            handler,
            interfaces,
            zone_field,
            margin,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
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
impl gluon::Convertable for PointsQuery {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.handler.write(gluon_data)?;
        self.interfaces.write(gluon_data)?;
        self.reference_spatial.write(gluon_data)?;
        self.points.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let handler = gluon::Convertable::read(gluon_data)?;
        let interfaces = gluon::Convertable::read(gluon_data)?;
        let reference_spatial = gluon::Convertable::read(gluon_data)?;
        let points = gluon::Convertable::read(gluon_data)?;
        Ok(PointsQuery {
            handler,
            interfaces,
            reference_spatial,
            points,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.handler.write_owned(gluon_data)?;
        self.interfaces.write_owned(gluon_data)?;
        self.reference_spatial.write_owned(gluon_data)?;
        self.points.write_owned(gluon_data)?;
        Ok(())
    }
}
///Point for a PointsQuery
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    pub point: crate::types::Vec3F,
    pub margin: f32,
}
impl gluon::Convertable for Point {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Vec3F = self.point.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.margin.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let point: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let margin = gluon::Convertable::read(gluon_data)?;
        Ok(Point { point, margin })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Vec3F = self.point.into();
            __w.write_owned(gluon_data)?;
        }
        self.margin.write_owned(gluon_data)?;
        Ok(())
    }
}
///Error potentially returned when registering a query
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QueryError {
    ///Invalid Refs for objects owned by the server
    InvalidRef,
    ///Querying requires at least one required interface
    NoRequiredInterfaces,
}
impl gluon::Convertable for QueryError {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            QueryError::InvalidRef => {
                gluon_data.write_u16(0u16)?;
            }
            QueryError::NoRequiredInterfaces => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => QueryError::InvalidRef,
                1u16 => QueryError::NoRequiredInterfaces,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            QueryError::InvalidRef => {
                gluon_data.write_u16(0u16)?;
            }
            QueryError::NoRequiredInterfaces => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct BeamQueryHandler {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for BeamQueryHandler {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(BeamQueryHandler::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl BeamQueryHandler {
    pub fn intersected(
        &self,
        obj: impl Into<super::query::QueryableObjectRef>,
        field: impl Into<super::field::FieldRef>,
        spatial: impl Into<super::spatial::SpatialRef>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
        deepest_point_distance: impl Into<f32>,
        distance: impl Into<f32>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let field: super::field::FieldRef = field.into();
        let spatial: super::spatial::SpatialRef = spatial.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        let deepest_point_distance: f32 = deepest_point_distance.into();
        let distance: f32 = distance.into();
        let mut gluon_builder = gluon::DataBuilder::new();
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
        obj: impl Into<super::query::QueryableObjectRef>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn moved(
        &self,
        obj: impl Into<super::query::QueryableObjectRef>,
        deepest_point_distance: impl Into<f32>,
        distance: impl Into<f32>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let deepest_point_distance: f32 = deepest_point_distance.into();
        let distance: f32 = distance.into();
        let mut gluon_builder = gluon::DataBuilder::new();
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
        obj: impl Into<super::query::QueryableObjectRef>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: BeamQueryHandlerHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> BeamQueryHandler {
        BeamQueryHandler::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> BeamQueryHandler {
        BeamQueryHandler { obj }
    }
}
impl From<BeamQueryHandler> for gluon::ObjectOrRef {
    fn from(value: BeamQueryHandler) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for BeamQueryHandler {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait BeamQueryHandlerHandler: gluon::Handler + Send + Sync + 'static {
    fn intersected(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
        field: super::field::FieldRef,
        spatial: super::spatial::SpatialRef,
        interfaces: Vec<super::query::QueriedInterface>,
        deepest_point_distance: f32,
        distance: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn interfaces_changed(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
        interfaces: Vec<super::query::QueriedInterface>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn moved(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
        deepest_point_distance: f32,
        distance: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn left(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
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
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    let param_field = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon::Convertable::read(&mut gluon_data)?;
                    let param_deepest_point_distance = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_distance = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.intersected(
                            ctx,
                            param_obj,
                            param_field,
                            param_spatial,
                            param_interfaces,
                            param_deepest_point_distance,
                            param_distance,
                        )
                        .await;
                }
                9u32 => {
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.interfaces_changed(ctx, param_obj, param_interfaces).await;
                }
                10u32 => {
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    let param_deepest_point_distance = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_distance = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.moved(
                            ctx,
                            param_obj,
                            param_deepest_point_distance,
                            param_distance,
                        )
                        .await;
                }
                11u32 => {
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.left(ctx, param_obj).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct ZoneQueryHandler {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for ZoneQueryHandler {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(ZoneQueryHandler::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl ZoneQueryHandler {
    pub fn entered(
        &self,
        obj: impl Into<super::query::QueryableObjectRef>,
        field: impl Into<super::field::FieldRef>,
        spatial: impl Into<super::spatial::SpatialRef>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
        relative_position: crate::types::Vec3F,
        distance: impl Into<f32>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let field: super::field::FieldRef = field.into();
        let spatial: super::spatial::SpatialRef = spatial.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        let relative_position: super::types::proxied::Vec3F = relative_position.into();
        let distance: f32 = distance.into();
        let mut gluon_builder = gluon::DataBuilder::new();
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
        obj: impl Into<super::query::QueryableObjectRef>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn moved(
        &self,
        obj: impl Into<super::query::QueryableObjectRef>,
        relative_position: crate::types::Vec3F,
        distance: impl Into<f32>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let relative_position: super::types::proxied::Vec3F = relative_position.into();
        let distance: f32 = distance.into();
        let mut gluon_builder = gluon::DataBuilder::new();
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
        obj: impl Into<super::query::QueryableObjectRef>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: ZoneQueryHandlerHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> ZoneQueryHandler {
        ZoneQueryHandler::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> ZoneQueryHandler {
        ZoneQueryHandler { obj }
    }
}
impl From<ZoneQueryHandler> for gluon::ObjectOrRef {
    fn from(value: ZoneQueryHandler) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for ZoneQueryHandler {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait ZoneQueryHandlerHandler: gluon::Handler + Send + Sync + 'static {
    fn entered(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
        field: super::field::FieldRef,
        spatial: super::spatial::SpatialRef,
        interfaces: Vec<super::query::QueriedInterface>,
        relative_position: crate::types::Vec3F,
        distance: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn interfaces_changed(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
        interfaces: Vec<super::query::QueriedInterface>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn moved(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
        relative_position: crate::types::Vec3F,
        distance: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn left(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
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
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    let param_field = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon::Convertable::read(&mut gluon_data)?;
                    let param_relative_position: crate::types::Vec3F = {
                        let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                            &mut gluon_data,
                        )?;
                        __w.into()
                    };
                    let param_distance = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.entered(
                            ctx,
                            param_obj,
                            param_field,
                            param_spatial,
                            param_interfaces,
                            param_relative_position,
                            param_distance,
                        )
                        .await;
                }
                9u32 => {
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.interfaces_changed(ctx, param_obj, param_interfaces).await;
                }
                10u32 => {
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    let param_relative_position: crate::types::Vec3F = {
                        let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                            &mut gluon_data,
                        )?;
                        __w.into()
                    };
                    let param_distance = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.moved(ctx, param_obj, param_relative_position, param_distance)
                        .await;
                }
                11u32 => {
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.left(ctx, param_obj).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct PointsQueryHandler {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for PointsQueryHandler {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(PointsQueryHandler::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PointsQueryHandler {
    pub fn entered(
        &self,
        obj: impl Into<super::query::QueryableObjectRef>,
        field: impl Into<super::field::FieldRef>,
        spatial: impl Into<super::spatial::SpatialRef>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
        distance: impl Into<f32>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let field: super::field::FieldRef = field.into();
        let spatial: super::spatial::SpatialRef = spatial.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        let distance: f32 = distance.into();
        let mut gluon_builder = gluon::DataBuilder::new();
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
        obj: impl Into<super::query::QueryableObjectRef>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn moved(
        &self,
        obj: impl Into<super::query::QueryableObjectRef>,
        distance: impl Into<f32>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let distance: f32 = distance.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        distance.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn left(
        &self,
        obj: impl Into<super::query::QueryableObjectRef>,
    ) -> Result<(), gluon::SendError> {
        let obj: super::query::QueryableObjectRef = obj.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: PointsQueryHandlerHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> PointsQueryHandler {
        PointsQueryHandler::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> PointsQueryHandler {
        PointsQueryHandler { obj }
    }
}
impl From<PointsQueryHandler> for gluon::ObjectOrRef {
    fn from(value: PointsQueryHandler) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for PointsQueryHandler {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait PointsQueryHandlerHandler: gluon::Handler + Send + Sync + 'static {
    fn entered(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
        field: super::field::FieldRef,
        spatial: super::spatial::SpatialRef,
        interfaces: Vec<super::query::QueriedInterface>,
        distance: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn interfaces_changed(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
        interfaces: Vec<super::query::QueriedInterface>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn moved(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
        distance: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn left(
        &self,
        _ctx: gluon::Context,
        obj: super::query::QueryableObjectRef,
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
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    let param_field = gluon::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon::Convertable::read(&mut gluon_data)?;
                    let param_distance = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.entered(
                            ctx,
                            param_obj,
                            param_field,
                            param_spatial,
                            param_interfaces,
                            param_distance,
                        )
                        .await;
                }
                9u32 => {
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.interfaces_changed(ctx, param_obj, param_interfaces).await;
                }
                10u32 => {
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    let param_distance = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.moved(ctx, param_obj, param_distance).await;
                }
                11u32 => {
                    let param_obj = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.left(ctx, param_obj).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpatialQueryInterface {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for SpatialQueryInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(SpatialQueryInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl SpatialQueryInterface {
    pub async fn beam_query(
        &self,
        query: impl Into<BeamQuery>,
    ) -> Result<Result<SpatialQueryGuard, QueryError>, gluon::SendError> {
        let query: BeamQuery = query.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn zone_query(
        &self,
        query: impl Into<ZoneQuery>,
    ) -> Result<Result<SpatialQueryGuard, QueryError>, gluon::SendError> {
        let query: ZoneQuery = query.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn points_query(
        &self,
        query: impl Into<PointsQuery>,
    ) -> Result<Result<PointsQueryHandle, QueryError>, gluon::SendError> {
        let query: PointsQuery = query.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn from_handler<H: SpatialQueryInterfaceHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> SpatialQueryInterface {
        SpatialQueryInterface::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> SpatialQueryInterface {
        SpatialQueryInterface { obj }
    }
}
impl From<SpatialQueryInterface> for gluon::ObjectOrRef {
    fn from(value: SpatialQueryInterface) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for SpatialQueryInterface {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait SpatialQueryInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    fn beam_query(
        &self,
        _ctx: gluon::Context,
        query: BeamQuery,
    ) -> impl Future<Output = Result<SpatialQueryGuard, QueryError>> + Send + Sync;
    fn zone_query(
        &self,
        _ctx: gluon::Context,
        query: ZoneQuery,
    ) -> impl Future<Output = Result<SpatialQueryGuard, QueryError>> + Send + Sync;
    fn points_query(
        &self,
        _ctx: gluon::Context,
        query: PointsQuery,
    ) -> impl Future<Output = Result<PointsQueryHandle, QueryError>> + Send + Sync;
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
                    let param_query = gluon::Convertable::read(&mut gluon_data)?;
                    let (guard) = self.beam_query(ctx, param_query).await;
                    drop(gluon_data);
                    guard.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_query = gluon::Convertable::read(&mut gluon_data)?;
                    let (guard) = self.zone_query(ctx, param_query).await;
                    drop(gluon_data);
                    guard.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_query = gluon::Convertable::read(&mut gluon_data)?;
                    let (handle) = self.points_query(ctx, param_query).await;
                    drop(gluon_data);
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for PointsQueryHandle {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(PointsQueryHandle::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PointsQueryHandle {
    pub fn update_points(
        &self,
        points: impl Into<Vec<Point>>,
    ) -> Result<(), gluon::SendError> {
        let points: Vec<Point> = points.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        points.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: PointsQueryHandleHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> PointsQueryHandle {
        PointsQueryHandle::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> PointsQueryHandle {
        PointsQueryHandle { obj }
    }
}
impl From<PointsQueryHandle> for gluon::ObjectOrRef {
    fn from(value: PointsQueryHandle) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for PointsQueryHandle {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait PointsQueryHandleHandler: gluon::Handler + Send + Sync + 'static {
    fn update_points(
        &self,
        _ctx: gluon::Context,
        points: Vec<Point>,
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
                    let param_points = gluon::Convertable::read(&mut gluon_data)?;
                    drop(gluon_data);
                    self.update_points(ctx, param_points).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct SpatialQueryGuard {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for SpatialQueryGuard {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(SpatialQueryGuard::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl SpatialQueryGuard {
    pub fn from_handler<H: SpatialQueryGuardHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> SpatialQueryGuard {
        SpatialQueryGuard::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> SpatialQueryGuard {
        SpatialQueryGuard { obj }
    }
}
impl From<SpatialQueryGuard> for gluon::ObjectOrRef {
    fn from(value: SpatialQueryGuard) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for SpatialQueryGuard {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait SpatialQueryGuardHandler: gluon::Handler + Send + Sync + 'static {
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
