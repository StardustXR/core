#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Query",
    types: &[
        gluon::ExternalGluonType {
            name: "InterfaceDependency",
            supported_derives: gluon::Derives::from_bits_truncate(798u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "QueriedInterface",
            supported_derives: gluon::Derives::from_bits_truncate(30u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "QueryableError",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///Dependency on an interface in query
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InterfaceDependency {
    pub id: String,
    pub optional: bool,
}
impl gluon::Convertable for InterfaceDependency {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.id.write(gluon_data)?;
        self.optional.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let id = gluon::Convertable::read(gluon_data)?;
        let optional = gluon::Convertable::read(gluon_data)?;
        Ok(InterfaceDependency {
            id,
            optional,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.id.write_owned(gluon_data)?;
        self.optional.write_owned(gluon_data)?;
        Ok(())
    }
}
///A successfully queried interface
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct QueriedInterface {
    pub interface_id: String,
    pub interface: gluon::Ref,
}
impl gluon::Convertable for QueriedInterface {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.interface_id.write(gluon_data)?;
        self.interface.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let interface_id = gluon::Convertable::read(gluon_data)?;
        let interface = gluon::Convertable::read(gluon_data)?;
        Ok(QueriedInterface {
            interface_id,
            interface,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.interface_id.write_owned(gluon_data)?;
        self.interface.write_owned(gluon_data)?;
        Ok(())
    }
}
///error returned from QueryInterface::register_queryable
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QueryableError {
    ///You don't own this spatial or it didn't come from the right stardust server!
    NotOwnedSpatial,
    ///You don't own this field or it didn't come from the right stardust server!
    NotOwnedField,
}
impl gluon::Convertable for QueryableError {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            QueryableError::NotOwnedSpatial => {
                gluon_data.write_u16(0u16)?;
            }
            QueryableError::NotOwnedField => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => QueryableError::NotOwnedSpatial,
                1u16 => QueryableError::NotOwnedField,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            QueryableError::NotOwnedSpatial => {
                gluon_data.write_u16(0u16)?;
            }
            QueryableError::NotOwnedField => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct QueryableObjectRef {
    obj: gluon::Ref,
}
impl gluon::Convertable for QueryableObjectRef {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(QueryableObjectRef::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for QueryableObjectRef {
    const ID: &'static str = "org.stardustxr.Query.QueryableObjectRef";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: QueryableObjectRefHandler> gluon::HandledBy<H> for QueryableObjectRef {}
impl gluon::RefExt for QueryableObjectRef {
    fn from_ref(obj: gluon::Ref) -> QueryableObjectRef {
        QueryableObjectRef { obj }
    }
}
impl QueryableObjectRef {
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> QueryableObjectRef {
        QueryableObjectRef { obj }
    }
}
impl From<QueryableObjectRef> for gluon::Ref {
    fn from(value: QueryableObjectRef) -> Self {
        value.obj
    }
}
impl gluon::ToRef for QueryableObjectRef {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for QueryableObjectRef {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for QueryableObjectRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for QueryableObjectRef {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for QueryableObjectRef {}
pub trait QueryableObjectRefHandler: gluon::Handler + Send + Sync + 'static {
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
pub struct QueryableObject {
    obj: gluon::Ref,
}
impl gluon::Convertable for QueryableObject {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(QueryableObject::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for QueryableObject {
    const ID: &'static str = "org.stardustxr.Query.QueryableObject";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: QueryableObjectHandler> gluon::HandledBy<H> for QueryableObject {}
impl gluon::RefExt for QueryableObject {
    fn from_ref(obj: gluon::Ref) -> QueryableObject {
        QueryableObject { obj }
    }
}
impl QueryableObject {
    pub async fn queryable_ref(&self) -> Result<QueryableObjectRef, gluon::SendError> {
        tracing::trace!(interface = "QueryableObject", method = "queryable_ref", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_queryable = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "QueryableObject", method = "queryable_ref", ? __ret_queryable,
            "←"
        );
        Ok(__ret_queryable)
    }
    pub async fn add_interface(
        &self,
        interface: &impl gluon::ToRef,
        interface_id: impl Into<String>,
    ) -> Result<QueryableInterfaceGuard, gluon::SendError> {
        let interface: gluon::Ref = gluon::ToRef::to_ref(interface);
        let interface_id: String = interface_id.into();
        tracing::trace!(
            interface = "QueryableObject", method = "add_interface", ? interface, ?
            interface_id, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        interface.write(&mut gluon_builder)?;
        interface_id.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_guard = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "QueryableObject", method = "add_interface", ? __ret_guard, "←"
        );
        Ok(__ret_guard)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> QueryableObject {
        QueryableObject { obj }
    }
}
impl From<QueryableObject> for gluon::Ref {
    fn from(value: QueryableObject) -> Self {
        value.obj
    }
}
impl gluon::ToRef for QueryableObject {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for QueryableObject {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for QueryableObject {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for QueryableObject {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for QueryableObject {}
pub trait QueryableObjectHandler: gluon::Handler + Send + Sync + 'static {
    fn queryable_ref(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = QueryableObjectRef> + Send + Sync;
    ///Dispatched instead of [`Self::queryable_ref`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `queryable_ref` and sends the result through `reply`. Override this method instead of `queryable_ref` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn queryable_ref_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<QueryableObjectRef>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let queryable = self.queryable_ref(_ctx).await;
            reply.send(queryable)
        }
    }
    fn add_interface(
        &self,
        _ctx: gluon::Context,
        interface: gluon::Ref,
        interface_id: String,
    ) -> impl Future<Output = QueryableInterfaceGuard> + Send + Sync;
    ///Dispatched instead of [`Self::add_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `add_interface` and sends the result through `reply`. Override this method instead of `add_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn add_interface_oneway(
        &self,
        _ctx: gluon::Context,
        interface: gluon::Ref,
        interface_id: String,
        reply: gluon::ReplySender<QueryableInterfaceGuard>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let guard = self.add_interface(_ctx, interface, interface_id).await;
            reply.send(guard)
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
                    tracing::trace!(
                        interface = "QueryableObject", method = "queryable_ref",
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<QueryableObjectRef> = gluon::ReplySender::new(
                        return_callback,
                        |queryable, gluon_out| {
                            tracing::trace!(
                                interface = "QueryableObject", method = "queryable_ref", ?
                                queryable, "←"
                            );
                            queryable.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.queryable_ref_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "QueryableObject", method =
                                "queryable_ref", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_interface = gluon::Convertable::read(&mut gluon_data)?;
                    let param_interface_id = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "QueryableObject", method = "add_interface", ?
                        param_interface, ? param_interface_id, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<QueryableInterfaceGuard> = gluon::ReplySender::new(
                        return_callback,
                        |guard, gluon_out| {
                            tracing::trace!(
                                interface = "QueryableObject", method = "add_interface", ?
                                guard, "←"
                            );
                            guard.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.add_interface_oneway(
                            ctx,
                            param_interface,
                            param_interface_id,
                            reply,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "QueryableObject", method =
                                "add_interface", method_id = 9u32
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
pub struct QueryableInterfaceGuard {
    obj: gluon::Ref,
}
impl gluon::Convertable for QueryableInterfaceGuard {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(QueryableInterfaceGuard::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for QueryableInterfaceGuard {
    const ID: &'static str = "org.stardustxr.Query.QueryableInterfaceGuard";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: QueryableInterfaceGuardHandler> gluon::HandledBy<H> for QueryableInterfaceGuard {}
impl gluon::RefExt for QueryableInterfaceGuard {
    fn from_ref(obj: gluon::Ref) -> QueryableInterfaceGuard {
        QueryableInterfaceGuard { obj }
    }
}
impl QueryableInterfaceGuard {
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> QueryableInterfaceGuard {
        QueryableInterfaceGuard { obj }
    }
}
impl From<QueryableInterfaceGuard> for gluon::Ref {
    fn from(value: QueryableInterfaceGuard) -> Self {
        value.obj
    }
}
impl gluon::ToRef for QueryableInterfaceGuard {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for QueryableInterfaceGuard {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for QueryableInterfaceGuard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for QueryableInterfaceGuard {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for QueryableInterfaceGuard {}
pub trait QueryableInterfaceGuardHandler: gluon::Handler + Send + Sync + 'static {
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
pub struct QueryInterface {
    obj: gluon::Ref,
}
impl gluon::Convertable for QueryInterface {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(QueryInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for QueryInterface {
    const ID: &'static str = "org.stardustxr.Query.QueryInterface";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: QueryInterfaceHandler> gluon::HandledBy<H> for QueryInterface {}
impl gluon::RefExt for QueryInterface {
    fn from_ref(obj: gluon::Ref) -> QueryInterface {
        QueryInterface { obj }
    }
}
impl QueryInterface {
    pub async fn register_queryable(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        field: impl Into<super::field::Field>,
    ) -> Result<Result<QueryableObject, QueryableError>, gluon::SendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let field: super::field::Field = field.into();
        tracing::trace!(
            interface = "QueryInterface", method = "register_queryable", ? spatial, ?
            field, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_queryable = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "QueryInterface", method = "register_queryable", ?
            __ret_queryable, "←"
        );
        Ok(__ret_queryable)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> QueryInterface {
        QueryInterface { obj }
    }
}
impl From<QueryInterface> for gluon::Ref {
    fn from(value: QueryInterface) -> Self {
        value.obj
    }
}
impl gluon::ToRef for QueryInterface {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for QueryInterface {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for QueryInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for QueryInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for QueryInterface {}
pub trait QueryInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    fn register_queryable(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        field: super::field::Field,
    ) -> impl Future<Output = Result<QueryableObject, QueryableError>> + Send + Sync;
    ///Dispatched instead of [`Self::register_queryable`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `register_queryable` and sends the result through `reply`. Override this method instead of `register_queryable` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn register_queryable_oneway(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        field: super::field::Field,
        reply: gluon::ReplySender<Result<QueryableObject, QueryableError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let queryable = self.register_queryable(_ctx, spatial, field).await;
            reply.send(queryable)
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
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_field = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "QueryInterface", method = "register_queryable", ?
                        param_spatial, ? param_field, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<QueryableObject, QueryableError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |queryable, gluon_out| {
                            tracing::trace!(
                                interface = "QueryInterface", method = "register_queryable",
                                ? queryable, "←"
                            );
                            queryable.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.register_queryable_oneway(
                            ctx,
                            param_spatial,
                            param_field,
                            reply,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "QueryInterface", method =
                                "register_queryable", method_id = 8u32
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
