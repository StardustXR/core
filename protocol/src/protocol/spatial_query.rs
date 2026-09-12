#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_ipc::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon_ipc::ExternalProtocol = gluon_ipc::ExternalProtocol {
    protocol_name: "org.stardustxr.SpatialQuery",
    types: &[
        gluon_ipc::ExternalGluonType {
            name: "BeamQuery",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(10u32),
            proxy: None,
        },
        gluon_ipc::ExternalGluonType {
            name: "ZoneQuery",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(10u32),
            proxy: None,
        },
        gluon_ipc::ExternalGluonType {
            name: "PointsQuery",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(10u32),
            proxy: None,
        },
        gluon_ipc::ExternalGluonType {
            name: "Point",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon_ipc::ExternalGluonType {
            name: "QueryError",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///shoot a beam and return everything it hit
#[derive(Debug, Clone, PartialEq)]
pub struct BeamQuery {
    pub handler: BeamQueryHandler,
    pub interfaces: Vec<super::query::InterfaceDependency>,
    pub reference_spatial: super::spatial::SpatialRef,
    pub origin: crate::types::Vec3F,
    pub direction: crate::types::Vec3F,
    ///Maximum length of the beam in meters, can be the max f32 value
    pub max_length: f32,
    ///Maximum distance the ray can be from a queryable's field, accounts for near misses
    pub margin: f32,
}
impl gluon_ipc::Convertable for BeamQuery {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
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
        self.margin.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let handler = gluon_ipc::Convertable::read(gluon_data)?;
        let interfaces = gluon_ipc::Convertable::read(gluon_data)?;
        let reference_spatial = gluon_ipc::Convertable::read(gluon_data)?;
        let origin: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon_ipc::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let direction: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon_ipc::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let max_length = gluon_ipc::Convertable::read(gluon_data)?;
        let margin = gluon_ipc::Convertable::read(gluon_data)?;
        Ok(BeamQuery {
            handler,
            interfaces,
            reference_spatial,
            origin,
            direction,
            max_length,
            margin,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
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
        self.margin.write_owned(gluon_data)?;
        Ok(())
    }
}
///Get interfaces intersecting this field
#[derive(Debug, Clone, PartialEq)]
pub struct ZoneQuery {
    pub handler: ZoneQueryHandler,
    pub interfaces: Vec<super::query::InterfaceDependency>,
    pub zone_field: super::field::FieldRef,
    pub margin: f32,
}
impl gluon_ipc::Convertable for ZoneQuery {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.handler.write(gluon_data)?;
        self.interfaces.write(gluon_data)?;
        self.zone_field.write(gluon_data)?;
        self.margin.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let handler = gluon_ipc::Convertable::read(gluon_data)?;
        let interfaces = gluon_ipc::Convertable::read(gluon_data)?;
        let zone_field = gluon_ipc::Convertable::read(gluon_data)?;
        let margin = gluon_ipc::Convertable::read(gluon_data)?;
        Ok(ZoneQuery {
            handler,
            interfaces,
            zone_field,
            margin,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.handler.write_owned(gluon_data)?;
        self.interfaces.write_owned(gluon_data)?;
        self.zone_field.write_owned(gluon_data)?;
        self.margin.write_owned(gluon_data)?;
        Ok(())
    }
}
///Get interfaces of fields containing any points
#[derive(Debug, Clone, PartialEq)]
pub struct PointsQuery {
    pub handler: PointsQueryHandler,
    pub interfaces: Vec<super::query::InterfaceDependency>,
    pub reference_spatial: super::spatial::SpatialRef,
    pub points: Vec<Point>,
}
impl gluon_ipc::Convertable for PointsQuery {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.handler.write(gluon_data)?;
        self.interfaces.write(gluon_data)?;
        self.reference_spatial.write(gluon_data)?;
        self.points.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let handler = gluon_ipc::Convertable::read(gluon_data)?;
        let interfaces = gluon_ipc::Convertable::read(gluon_data)?;
        let reference_spatial = gluon_ipc::Convertable::read(gluon_data)?;
        let points = gluon_ipc::Convertable::read(gluon_data)?;
        Ok(PointsQuery {
            handler,
            interfaces,
            reference_spatial,
            points,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
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
impl gluon_ipc::Convertable for Point {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        {
            let __w: super::types::proxied::Vec3F = self.point.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.margin.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let point: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon_ipc::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let margin = gluon_ipc::Convertable::read(gluon_data)?;
        Ok(Point { point, margin })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
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
impl gluon_ipc::Convertable for QueryError {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
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
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => QueryError::InvalidRef,
                1u16 => QueryError::NoRequiredInterfaces,
                v => return Err(gluon_ipc::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
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
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for BeamQueryHandler {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let obj = gluon_ipc::Ref::read(gluon_data)?;
        Ok(BeamQueryHandler::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl BeamQueryHandler {
    const ID: &'static str = "org.stardustxr.SpatialQuery.BeamQueryHandler";
}
impl gluon_ipc::Interface for BeamQueryHandler {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: BeamQueryHandlerHandler> gluon_ipc::HandledBy<H> for BeamQueryHandler {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type BeamQueryHandlerLocal<H> = gluon_ipc::LocalRef<BeamQueryHandler, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: BeamQueryHandlerHandler> From<BeamQueryHandlerLocal<H>> for BeamQueryHandler {
    fn from(value: BeamQueryHandlerLocal<H>) -> BeamQueryHandler {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for BeamQueryHandler {
    fn from_ref(obj: gluon_ipc::Ref) -> BeamQueryHandler {
        BeamQueryHandler { obj }
    }
}
impl BeamQueryHandler {
    pub fn intersected(
        &self,
        obj: impl Into<super::query::QueryableId>,
        field: impl Into<super::field::FieldRef>,
        spatial: impl Into<super::spatial::SpatialRef>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
        spatial_info: impl Into<super::field::RayMarchResult>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        let field: super::field::FieldRef = field.into();
        let spatial: super::spatial::SpatialRef = spatial.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        let spatial_info: super::field::RayMarchResult = spatial_info.into();
        tracing::trace!(
            interface = "BeamQueryHandler", method = "intersected", ? obj, ? field, ?
            spatial, ? interfaces, ? spatial_info, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        spatial_info.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    pub fn interfaces_changed(
        &self,
        obj: impl Into<super::query::QueryableId>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        tracing::trace!(
            interface = "BeamQueryHandler", method = "interfaces_changed", ? obj, ?
            interfaces, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 9u32, gluon_builder)?;
        Ok(())
    }
    pub fn moved(
        &self,
        obj: impl Into<super::query::QueryableId>,
        spatial_info: impl Into<super::field::RayMarchResult>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        let spatial_info: super::field::RayMarchResult = spatial_info.into();
        tracing::trace!(
            interface = "BeamQueryHandler", method = "moved", ? obj, ? spatial_info,
            "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        spatial_info.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 10u32, gluon_builder)?;
        Ok(())
    }
    ///stop caring about this object: it left the beam, or it was destroyed
    pub fn left(
        &self,
        obj: impl Into<super::query::QueryableId>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        tracing::trace!(interface = "BeamQueryHandler", method = "left", ? obj, "→");
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 11u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> BeamQueryHandler {
        BeamQueryHandler { obj }
    }
}
impl From<BeamQueryHandler> for gluon_ipc::Ref {
    fn from(value: BeamQueryHandler) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for BeamQueryHandler {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for BeamQueryHandler {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
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
pub trait BeamQueryHandlerHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn intersected(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
        field: super::field::FieldRef,
        spatial: super::spatial::SpatialRef,
        interfaces: Vec<super::query::QueriedInterface>,
        spatial_info: super::field::RayMarchResult,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn interfaces_changed(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
        interfaces: Vec<super::query::QueriedInterface>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn moved(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
        spatial_info: super::field::RayMarchResult,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///stop caring about this object: it left the beam, or it was destroyed
    fn left(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_ipc::DataReader,
        ctx: gluon_ipc::Context,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_field = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_spatial_info = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "BeamQueryHandler", method = "intersected", ?
                        param_obj, ? param_field, ? param_spatial, ? param_interfaces, ?
                        param_spatial_info, "dispatching"
                    );
                    drop(gluon_data);
                    self.intersected(
                            ctx,
                            param_obj,
                            param_field,
                            param_spatial,
                            param_interfaces,
                            param_spatial_info,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "BeamQueryHandler", method =
                                "intersected", method_id = 8u32
                            ),
                        )
                        .await;
                }
                9u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "BeamQueryHandler", method = "interfaces_changed", ?
                        param_obj, ? param_interfaces, "dispatching"
                    );
                    drop(gluon_data);
                    self.interfaces_changed(ctx, param_obj, param_interfaces)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "BeamQueryHandler", method =
                                "interfaces_changed", method_id = 9u32
                            ),
                        )
                        .await;
                }
                10u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_spatial_info = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "BeamQueryHandler", method = "moved", ? param_obj, ?
                        param_spatial_info, "dispatching"
                    );
                    drop(gluon_data);
                    self.moved(ctx, param_obj, param_spatial_info)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "BeamQueryHandler", method =
                                "moved", method_id = 10u32
                            ),
                        )
                        .await;
                }
                11u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "BeamQueryHandler", method = "left", ? param_obj,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.left(ctx, param_obj)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "BeamQueryHandler", method =
                                "left", method_id = 11u32
                            ),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<BeamQueryHandler, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        BeamQueryHandler::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<BeamQueryHandler, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        BeamQueryHandler::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct BeamQueryHandle {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for BeamQueryHandle {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let obj = gluon_ipc::Ref::read(gluon_data)?;
        Ok(BeamQueryHandle::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl BeamQueryHandle {
    const ID: &'static str = "org.stardustxr.SpatialQuery.BeamQueryHandle";
}
impl gluon_ipc::Interface for BeamQueryHandle {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: BeamQueryHandleHandler> gluon_ipc::HandledBy<H> for BeamQueryHandle {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type BeamQueryHandleLocal<H> = gluon_ipc::LocalRef<BeamQueryHandle, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: BeamQueryHandleHandler> From<BeamQueryHandleLocal<H>> for BeamQueryHandle {
    fn from(value: BeamQueryHandleLocal<H>) -> BeamQueryHandle {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for BeamQueryHandle {
    fn from_ref(obj: gluon_ipc::Ref) -> BeamQueryHandle {
        BeamQueryHandle { obj }
    }
}
impl BeamQueryHandle {
    pub fn update(
        &self,
        origin: crate::types::Vec3F,
        direction: crate::types::Vec3F,
        max_length: impl Into<f32>,
        margin: impl Into<f32>,
    ) -> Result<(), gluon_ipc::SendError> {
        let origin: super::types::proxied::Vec3F = origin.into();
        let direction: super::types::proxied::Vec3F = direction.into();
        let max_length: f32 = max_length.into();
        let margin: f32 = margin.into();
        tracing::trace!(
            interface = "BeamQueryHandle", method = "update", ? origin, ? direction, ?
            max_length, ? margin, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        origin.write(&mut gluon_builder)?;
        direction.write(&mut gluon_builder)?;
        max_length.write(&mut gluon_builder)?;
        margin.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> BeamQueryHandle {
        BeamQueryHandle { obj }
    }
}
impl From<BeamQueryHandle> for gluon_ipc::Ref {
    fn from(value: BeamQueryHandle) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for BeamQueryHandle {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for BeamQueryHandle {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for BeamQueryHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for BeamQueryHandle {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for BeamQueryHandle {}
pub trait BeamQueryHandleHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn update(
        &self,
        _ctx: gluon_ipc::Context,
        origin: crate::types::Vec3F,
        direction: crate::types::Vec3F,
        max_length: f32,
        margin: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_ipc::DataReader,
        ctx: gluon_ipc::Context,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let __wire_param_origin: super::types::proxied::Vec3F = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let __wire_param_direction: super::types::proxied::Vec3F = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_max_length = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_margin = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "BeamQueryHandle", method = "update", param_origin =
                        ? __wire_param_origin, param_direction = ?
                        __wire_param_direction, ? param_max_length, ? param_margin,
                        "dispatching"
                    );
                    let param_origin: crate::types::Vec3F = {
                        let __w = __wire_param_origin;
                        __w.into()
                    };
                    let param_direction: crate::types::Vec3F = {
                        let __w = __wire_param_direction;
                        __w.into()
                    };
                    drop(gluon_data);
                    self.update(
                            ctx,
                            param_origin,
                            param_direction,
                            param_max_length,
                            param_margin,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "BeamQueryHandle", method =
                                "update", method_id = 8u32
                            ),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<BeamQueryHandle, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        BeamQueryHandle::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<BeamQueryHandle, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        BeamQueryHandle::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct ZoneQueryHandler {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for ZoneQueryHandler {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let obj = gluon_ipc::Ref::read(gluon_data)?;
        Ok(ZoneQueryHandler::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl ZoneQueryHandler {
    const ID: &'static str = "org.stardustxr.SpatialQuery.ZoneQueryHandler";
}
impl gluon_ipc::Interface for ZoneQueryHandler {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: ZoneQueryHandlerHandler> gluon_ipc::HandledBy<H> for ZoneQueryHandler {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type ZoneQueryHandlerLocal<H> = gluon_ipc::LocalRef<ZoneQueryHandler, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: ZoneQueryHandlerHandler> From<ZoneQueryHandlerLocal<H>> for ZoneQueryHandler {
    fn from(value: ZoneQueryHandlerLocal<H>) -> ZoneQueryHandler {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for ZoneQueryHandler {
    fn from_ref(obj: gluon_ipc::Ref) -> ZoneQueryHandler {
        ZoneQueryHandler { obj }
    }
}
impl ZoneQueryHandler {
    pub fn entered(
        &self,
        obj: impl Into<super::query::QueryableId>,
        field: impl Into<super::field::FieldRef>,
        spatial: impl Into<super::spatial::SpatialRef>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
        relative_position: crate::types::Vec3F,
        spatial_info: impl Into<super::field::FieldSample>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        let field: super::field::FieldRef = field.into();
        let spatial: super::spatial::SpatialRef = spatial.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        let relative_position: super::types::proxied::Vec3F = relative_position.into();
        let spatial_info: super::field::FieldSample = spatial_info.into();
        tracing::trace!(
            interface = "ZoneQueryHandler", method = "entered", ? obj, ? field, ?
            spatial, ? interfaces, ? relative_position, ? spatial_info, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        relative_position.write(&mut gluon_builder)?;
        spatial_info.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    pub fn interfaces_changed(
        &self,
        obj: impl Into<super::query::QueryableId>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        tracing::trace!(
            interface = "ZoneQueryHandler", method = "interfaces_changed", ? obj, ?
            interfaces, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 9u32, gluon_builder)?;
        Ok(())
    }
    pub fn moved(
        &self,
        obj: impl Into<super::query::QueryableId>,
        relative_position: crate::types::Vec3F,
        spatial_info: impl Into<super::field::FieldSample>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        let relative_position: super::types::proxied::Vec3F = relative_position.into();
        let spatial_info: super::field::FieldSample = spatial_info.into();
        tracing::trace!(
            interface = "ZoneQueryHandler", method = "moved", ? obj, ? relative_position,
            ? spatial_info, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        relative_position.write(&mut gluon_builder)?;
        spatial_info.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 10u32, gluon_builder)?;
        Ok(())
    }
    ///stop caring about this object: it left the zone, or it was destroyed
    pub fn left(
        &self,
        obj: impl Into<super::query::QueryableId>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        tracing::trace!(interface = "ZoneQueryHandler", method = "left", ? obj, "→");
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 11u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> ZoneQueryHandler {
        ZoneQueryHandler { obj }
    }
}
impl From<ZoneQueryHandler> for gluon_ipc::Ref {
    fn from(value: ZoneQueryHandler) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for ZoneQueryHandler {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for ZoneQueryHandler {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
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
pub trait ZoneQueryHandlerHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn entered(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
        field: super::field::FieldRef,
        spatial: super::spatial::SpatialRef,
        interfaces: Vec<super::query::QueriedInterface>,
        relative_position: crate::types::Vec3F,
        spatial_info: super::field::FieldSample,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn interfaces_changed(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
        interfaces: Vec<super::query::QueriedInterface>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn moved(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
        relative_position: crate::types::Vec3F,
        spatial_info: super::field::FieldSample,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///stop caring about this object: it left the zone, or it was destroyed
    fn left(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_ipc::DataReader,
        ctx: gluon_ipc::Context,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_field = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let __wire_param_relative_position: super::types::proxied::Vec3F = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_spatial_info = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "ZoneQueryHandler", method = "entered", ? param_obj,
                        ? param_field, ? param_spatial, ? param_interfaces,
                        param_relative_position = ? __wire_param_relative_position, ?
                        param_spatial_info, "dispatching"
                    );
                    let param_relative_position: crate::types::Vec3F = {
                        let __w = __wire_param_relative_position;
                        __w.into()
                    };
                    drop(gluon_data);
                    self.entered(
                            ctx,
                            param_obj,
                            param_field,
                            param_spatial,
                            param_interfaces,
                            param_relative_position,
                            param_spatial_info,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ZoneQueryHandler", method =
                                "entered", method_id = 8u32
                            ),
                        )
                        .await;
                }
                9u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "ZoneQueryHandler", method = "interfaces_changed", ?
                        param_obj, ? param_interfaces, "dispatching"
                    );
                    drop(gluon_data);
                    self.interfaces_changed(ctx, param_obj, param_interfaces)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ZoneQueryHandler", method =
                                "interfaces_changed", method_id = 9u32
                            ),
                        )
                        .await;
                }
                10u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let __wire_param_relative_position: super::types::proxied::Vec3F = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_spatial_info = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "ZoneQueryHandler", method = "moved", ? param_obj,
                        param_relative_position = ? __wire_param_relative_position, ?
                        param_spatial_info, "dispatching"
                    );
                    let param_relative_position: crate::types::Vec3F = {
                        let __w = __wire_param_relative_position;
                        __w.into()
                    };
                    drop(gluon_data);
                    self.moved(
                            ctx,
                            param_obj,
                            param_relative_position,
                            param_spatial_info,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ZoneQueryHandler", method =
                                "moved", method_id = 10u32
                            ),
                        )
                        .await;
                }
                11u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "ZoneQueryHandler", method = "left", ? param_obj,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.left(ctx, param_obj)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ZoneQueryHandler", method =
                                "left", method_id = 11u32
                            ),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<ZoneQueryHandler, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        ZoneQueryHandler::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<ZoneQueryHandler, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        ZoneQueryHandler::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct ZoneQueryHandle {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for ZoneQueryHandle {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let obj = gluon_ipc::Ref::read(gluon_data)?;
        Ok(ZoneQueryHandle::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl ZoneQueryHandle {
    const ID: &'static str = "org.stardustxr.SpatialQuery.ZoneQueryHandle";
}
impl gluon_ipc::Interface for ZoneQueryHandle {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: ZoneQueryHandleHandler> gluon_ipc::HandledBy<H> for ZoneQueryHandle {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type ZoneQueryHandleLocal<H> = gluon_ipc::LocalRef<ZoneQueryHandle, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: ZoneQueryHandleHandler> From<ZoneQueryHandleLocal<H>> for ZoneQueryHandle {
    fn from(value: ZoneQueryHandleLocal<H>) -> ZoneQueryHandle {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for ZoneQueryHandle {
    fn from_ref(obj: gluon_ipc::Ref) -> ZoneQueryHandle {
        ZoneQueryHandle { obj }
    }
}
impl ZoneQueryHandle {
    pub fn update(&self, margin: impl Into<f32>) -> Result<(), gluon_ipc::SendError> {
        let margin: f32 = margin.into();
        tracing::trace!(
            interface = "ZoneQueryHandle", method = "update", ? margin, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        margin.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> ZoneQueryHandle {
        ZoneQueryHandle { obj }
    }
}
impl From<ZoneQueryHandle> for gluon_ipc::Ref {
    fn from(value: ZoneQueryHandle) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for ZoneQueryHandle {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for ZoneQueryHandle {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for ZoneQueryHandle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for ZoneQueryHandle {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for ZoneQueryHandle {}
pub trait ZoneQueryHandleHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn update(
        &self,
        _ctx: gluon_ipc::Context,
        margin: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_ipc::DataReader,
        ctx: gluon_ipc::Context,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let param_margin = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "ZoneQueryHandle", method = "update", ? param_margin,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.update(ctx, param_margin)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ZoneQueryHandle", method =
                                "update", method_id = 8u32
                            ),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<ZoneQueryHandle, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        ZoneQueryHandle::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<ZoneQueryHandle, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        ZoneQueryHandle::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct PointsQueryHandler {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for PointsQueryHandler {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let obj = gluon_ipc::Ref::read(gluon_data)?;
        Ok(PointsQueryHandler::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PointsQueryHandler {
    const ID: &'static str = "org.stardustxr.SpatialQuery.PointsQueryHandler";
}
impl gluon_ipc::Interface for PointsQueryHandler {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: PointsQueryHandlerHandler> gluon_ipc::HandledBy<H> for PointsQueryHandler {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type PointsQueryHandlerLocal<H> = gluon_ipc::LocalRef<PointsQueryHandler, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: PointsQueryHandlerHandler> From<PointsQueryHandlerLocal<H>>
for PointsQueryHandler {
    fn from(value: PointsQueryHandlerLocal<H>) -> PointsQueryHandler {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for PointsQueryHandler {
    fn from_ref(obj: gluon_ipc::Ref) -> PointsQueryHandler {
        PointsQueryHandler { obj }
    }
}
impl PointsQueryHandler {
    pub fn entered(
        &self,
        obj: impl Into<super::query::QueryableId>,
        field: impl Into<super::field::FieldRef>,
        spatial: impl Into<super::spatial::SpatialRef>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
        spatial_info: impl Into<super::field::FieldSample>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        let field: super::field::FieldRef = field.into();
        let spatial: super::spatial::SpatialRef = spatial.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        let spatial_info: super::field::FieldSample = spatial_info.into();
        tracing::trace!(
            interface = "PointsQueryHandler", method = "entered", ? obj, ? field, ?
            spatial, ? interfaces, ? spatial_info, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        spatial.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        spatial_info.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    pub fn interfaces_changed(
        &self,
        obj: impl Into<super::query::QueryableId>,
        interfaces: impl Into<Vec<super::query::QueriedInterface>>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        let interfaces: Vec<super::query::QueriedInterface> = interfaces.into();
        tracing::trace!(
            interface = "PointsQueryHandler", method = "interfaces_changed", ? obj, ?
            interfaces, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        interfaces.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 9u32, gluon_builder)?;
        Ok(())
    }
    pub fn moved(
        &self,
        obj: impl Into<super::query::QueryableId>,
        spatial_info: impl Into<super::field::FieldSample>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        let spatial_info: super::field::FieldSample = spatial_info.into();
        tracing::trace!(
            interface = "PointsQueryHandler", method = "moved", ? obj, ? spatial_info,
            "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        spatial_info.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 10u32, gluon_builder)?;
        Ok(())
    }
    ///stop caring about this object: no point is in range of it any more, or it was destroyed
    pub fn left(
        &self,
        obj: impl Into<super::query::QueryableId>,
    ) -> Result<(), gluon_ipc::SendError> {
        let obj: super::query::QueryableId = obj.into();
        tracing::trace!(interface = "PointsQueryHandler", method = "left", ? obj, "→");
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        obj.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 11u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> PointsQueryHandler {
        PointsQueryHandler { obj }
    }
}
impl From<PointsQueryHandler> for gluon_ipc::Ref {
    fn from(value: PointsQueryHandler) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for PointsQueryHandler {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for PointsQueryHandler {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
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
pub trait PointsQueryHandlerHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn entered(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
        field: super::field::FieldRef,
        spatial: super::spatial::SpatialRef,
        interfaces: Vec<super::query::QueriedInterface>,
        spatial_info: super::field::FieldSample,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn interfaces_changed(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
        interfaces: Vec<super::query::QueriedInterface>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn moved(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
        spatial_info: super::field::FieldSample,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///stop caring about this object: no point is in range of it any more, or it was destroyed
    fn left(
        &self,
        _ctx: gluon_ipc::Context,
        obj: super::query::QueryableId,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_ipc::DataReader,
        ctx: gluon_ipc::Context,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_field = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_spatial = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_spatial_info = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "PointsQueryHandler", method = "entered", ?
                        param_obj, ? param_field, ? param_spatial, ? param_interfaces, ?
                        param_spatial_info, "dispatching"
                    );
                    drop(gluon_data);
                    self.entered(
                            ctx,
                            param_obj,
                            param_field,
                            param_spatial,
                            param_interfaces,
                            param_spatial_info,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PointsQueryHandler", method =
                                "entered", method_id = 8u32
                            ),
                        )
                        .await;
                }
                9u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_interfaces = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "PointsQueryHandler", method = "interfaces_changed",
                        ? param_obj, ? param_interfaces, "dispatching"
                    );
                    drop(gluon_data);
                    self.interfaces_changed(ctx, param_obj, param_interfaces)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PointsQueryHandler", method =
                                "interfaces_changed", method_id = 9u32
                            ),
                        )
                        .await;
                }
                10u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_spatial_info = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "PointsQueryHandler", method = "moved", ? param_obj,
                        ? param_spatial_info, "dispatching"
                    );
                    drop(gluon_data);
                    self.moved(ctx, param_obj, param_spatial_info)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PointsQueryHandler", method =
                                "moved", method_id = 10u32
                            ),
                        )
                        .await;
                }
                11u32 => {
                    let param_obj = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PointsQueryHandler", method = "left", ? param_obj,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.left(ctx, param_obj)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PointsQueryHandler", method =
                                "left", method_id = 11u32
                            ),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<PointsQueryHandler, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        PointsQueryHandler::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<PointsQueryHandler, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        PointsQueryHandler::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct PointsQueryHandle {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for PointsQueryHandle {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let obj = gluon_ipc::Ref::read(gluon_data)?;
        Ok(PointsQueryHandle::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl PointsQueryHandle {
    const ID: &'static str = "org.stardustxr.SpatialQuery.PointsQueryHandle";
}
impl gluon_ipc::Interface for PointsQueryHandle {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: PointsQueryHandleHandler> gluon_ipc::HandledBy<H> for PointsQueryHandle {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type PointsQueryHandleLocal<H> = gluon_ipc::LocalRef<PointsQueryHandle, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: PointsQueryHandleHandler> From<PointsQueryHandleLocal<H>> for PointsQueryHandle {
    fn from(value: PointsQueryHandleLocal<H>) -> PointsQueryHandle {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for PointsQueryHandle {
    fn from_ref(obj: gluon_ipc::Ref) -> PointsQueryHandle {
        PointsQueryHandle { obj }
    }
}
impl PointsQueryHandle {
    pub fn update(
        &self,
        points: impl Into<Vec<Point>>,
    ) -> Result<(), gluon_ipc::SendError> {
        let points: Vec<Point> = points.into();
        tracing::trace!(
            interface = "PointsQueryHandle", method = "update", ? points, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        points.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> PointsQueryHandle {
        PointsQueryHandle { obj }
    }
}
impl From<PointsQueryHandle> for gluon_ipc::Ref {
    fn from(value: PointsQueryHandle) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for PointsQueryHandle {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for PointsQueryHandle {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
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
pub trait PointsQueryHandleHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn update(
        &self,
        _ctx: gluon_ipc::Context,
        points: Vec<Point>,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_ipc::DataReader,
        ctx: gluon_ipc::Context,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let param_points = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "PointsQueryHandle", method = "update", ?
                        param_points, "dispatching"
                    );
                    drop(gluon_data);
                    self.update(ctx, param_points)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "PointsQueryHandle", method =
                                "update", method_id = 8u32
                            ),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<PointsQueryHandle, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        PointsQueryHandle::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<PointsQueryHandle, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        PointsQueryHandle::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct SpatialQueryInterface {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for SpatialQueryInterface {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let obj = gluon_ipc::Ref::read(gluon_data)?;
        Ok(SpatialQueryInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl SpatialQueryInterface {
    const ID: &'static str = "org.stardustxr.SpatialQuery.SpatialQueryInterface";
}
impl gluon_ipc::Interface for SpatialQueryInterface {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: SpatialQueryInterfaceHandler> gluon_ipc::HandledBy<H> for SpatialQueryInterface {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type SpatialQueryInterfaceLocal<H> = gluon_ipc::LocalRef<SpatialQueryInterface, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: SpatialQueryInterfaceHandler> From<SpatialQueryInterfaceLocal<H>>
for SpatialQueryInterface {
    fn from(value: SpatialQueryInterfaceLocal<H>) -> SpatialQueryInterface {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for SpatialQueryInterface {
    fn from_ref(obj: gluon_ipc::Ref) -> SpatialQueryInterface {
        SpatialQueryInterface { obj }
    }
}
impl SpatialQueryInterface {
    pub async fn beam_query(
        &self,
        query: impl Into<BeamQuery>,
    ) -> Result<Result<BeamQueryHandle, QueryError>, gluon_ipc::SendError> {
        let query: BeamQuery = query.into();
        tracing::trace!(
            interface = "SpatialQueryInterface", method = "beam_query", ? query, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_handle = gluon_ipc::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "SpatialQueryInterface", method = "beam_query", ? __ret_handle,
            "←"
        );
        Ok(__ret_handle)
    }
    pub async fn zone_query(
        &self,
        query: impl Into<ZoneQuery>,
    ) -> Result<Result<ZoneQueryHandle, QueryError>, gluon_ipc::SendError> {
        let query: ZoneQuery = query.into();
        tracing::trace!(
            interface = "SpatialQueryInterface", method = "zone_query", ? query, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_handle = gluon_ipc::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "SpatialQueryInterface", method = "zone_query", ? __ret_handle,
            "←"
        );
        Ok(__ret_handle)
    }
    pub async fn points_query(
        &self,
        query: impl Into<PointsQuery>,
    ) -> Result<Result<PointsQueryHandle, QueryError>, gluon_ipc::SendError> {
        let query: PointsQuery = query.into();
        tracing::trace!(
            interface = "SpatialQueryInterface", method = "points_query", ? query, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        query.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 10u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_handle = gluon_ipc::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "SpatialQueryInterface", method = "points_query", ? __ret_handle,
            "←"
        );
        Ok(__ret_handle)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> SpatialQueryInterface {
        SpatialQueryInterface { obj }
    }
}
impl From<SpatialQueryInterface> for gluon_ipc::Ref {
    fn from(value: SpatialQueryInterface) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for SpatialQueryInterface {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for SpatialQueryInterface {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
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
pub trait SpatialQueryInterfaceHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn beam_query(
        &self,
        _ctx: gluon_ipc::Context,
        query: BeamQuery,
    ) -> impl Future<Output = Result<BeamQueryHandle, QueryError>> + Send + Sync;
    ///Dispatched instead of [`Self::beam_query`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `beam_query` and sends the result through `reply`. Override this method instead of `beam_query` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn beam_query_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        query: BeamQuery,
        reply: gluon_ipc::ReplySender<Result<BeamQueryHandle, QueryError>>,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            let handle = self.beam_query(_ctx, query).await;
            reply.send(handle)
        }
    }
    fn zone_query(
        &self,
        _ctx: gluon_ipc::Context,
        query: ZoneQuery,
    ) -> impl Future<Output = Result<ZoneQueryHandle, QueryError>> + Send + Sync;
    ///Dispatched instead of [`Self::zone_query`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `zone_query` and sends the result through `reply`. Override this method instead of `zone_query` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn zone_query_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        query: ZoneQuery,
        reply: gluon_ipc::ReplySender<Result<ZoneQueryHandle, QueryError>>,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            let handle = self.zone_query(_ctx, query).await;
            reply.send(handle)
        }
    }
    fn points_query(
        &self,
        _ctx: gluon_ipc::Context,
        query: PointsQuery,
    ) -> impl Future<Output = Result<PointsQueryHandle, QueryError>> + Send + Sync;
    ///Dispatched instead of [`Self::points_query`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `points_query` and sends the result through `reply`. Override this method instead of `points_query` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn points_query_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        query: PointsQuery,
        reply: gluon_ipc::ReplySender<Result<PointsQueryHandle, QueryError>>,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            let handle = self.points_query(_ctx, query).await;
            reply.send(handle)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_ipc::DataReader,
        ctx: gluon_ipc::Context,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_query = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SpatialQueryInterface", method = "beam_query", ?
                        param_query, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<
                        Result<BeamQueryHandle, QueryError>,
                    > = gluon_ipc::ReplySender::new(
                        return_callback,
                        |handle, gluon_out| {
                            tracing::trace!(
                                interface = "SpatialQueryInterface", method = "beam_query",
                                ? handle, "←"
                            );
                            handle.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.beam_query_oneway(ctx, param_query, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "SpatialQueryInterface", method =
                                "beam_query", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_query = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SpatialQueryInterface", method = "zone_query", ?
                        param_query, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<
                        Result<ZoneQueryHandle, QueryError>,
                    > = gluon_ipc::ReplySender::new(
                        return_callback,
                        |handle, gluon_out| {
                            tracing::trace!(
                                interface = "SpatialQueryInterface", method = "zone_query",
                                ? handle, "←"
                            );
                            handle.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.zone_query_oneway(ctx, param_query, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "SpatialQueryInterface", method =
                                "zone_query", method_id = 9u32
                            ),
                        )
                        .await?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_query = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SpatialQueryInterface", method = "points_query", ?
                        param_query, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<
                        Result<PointsQueryHandle, QueryError>,
                    > = gluon_ipc::ReplySender::new(
                        return_callback,
                        |handle, gluon_out| {
                            tracing::trace!(
                                interface = "SpatialQueryInterface", method =
                                "points_query", ? handle, "←"
                            );
                            handle.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.points_query_oneway(ctx, param_query, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "SpatialQueryInterface", method =
                                "points_query", method_id = 10u32
                            ),
                        )
                        .await?;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<SpatialQueryInterface, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        SpatialQueryInterface::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<SpatialQueryInterface, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        SpatialQueryInterface::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
