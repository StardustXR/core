#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Query",
    types: &[
        gluon::ExternalGluonType {
            name: "QueryableId",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
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
/**identifies one queryable across a stream of query events

correlation, not authority: `moved`, `interfaces_changed` and `left` arrive after the
`entered` that introduced the object, and this is what ties them to the entry you made
then. Unique per server and never reused, so it is safe as a map key for as long as you
care about the object; meaningless to anyone but the query stream it came from.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QueryableId {
    pub id: u64,
}
impl gluon::Convertable for QueryableId {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.id.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let id = gluon::Convertable::read(gluon_data)?;
        Ok(QueryableId { id })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.id.write_owned(gluon_data)?;
        Ok(())
    }
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
///error returned from QueryInterface::register_queryable and QueryableObject::add_interface
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum QueryableError {
    ///You don't own this spatial or it didn't come from the right stardust server!
    NotOwnedSpatial,
    ///You don't own this field or it didn't come from the right stardust server!
    NotOwnedField,
    ///This queryable already advertises an interface with that id, drop that advertisement first!
    DuplicateInterface,
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
            QueryableError::DuplicateInterface => {
                gluon_data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => QueryableError::NotOwnedSpatial,
                1u16 => QueryableError::NotOwnedField,
                2u16 => QueryableError::DuplicateInterface,
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
            QueryableError::DuplicateInterface => {
                gluon_data.write_u16(2u16)?;
            }
        };
        Ok(())
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
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type QueryableObjectLocal<H> = gluon::LocalRef<QueryableObject, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: QueryableObjectHandler> From<QueryableObjectLocal<H>> for QueryableObject {
    fn from(value: QueryableObjectLocal<H>) -> QueryableObject {
        value.into_proxy()
    }
}
impl gluon::RefExt for QueryableObject {
    fn from_ref(obj: gluon::Ref) -> QueryableObject {
        QueryableObject { obj }
    }
}
impl QueryableObject {
    ///this queryable's id, as querying clients see it
    pub async fn id(&self) -> Result<QueryableId, gluon::SendError> {
        tracing::trace!(interface = "QueryableObject", method = "id", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_id = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(interface = "QueryableObject", method = "id", ? __ret_id, "←");
        Ok(__ret_id)
    }
    /**advertise `interface` under `interface_id` so queries depending on that id find it

the returned advertisement is what keeps it listed: drop it to stop advertising
without taking down the node that serves the interface. That node dying withdraws
the advertisement too — a ref going dead always means the object behind it is gone,
never that someone revoked it.*/
    pub async fn add_interface(
        &self,
        interface: &impl gluon::ToRef,
        interface_id: impl Into<String>,
    ) -> Result<Result<QueryableInterface, QueryableError>, gluon::SendError> {
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
        let __ret_advertisement = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "QueryableObject", method = "add_interface", ?
            __ret_advertisement, "←"
        );
        Ok(__ret_advertisement)
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
    ///this queryable's id, as querying clients see it
    fn id(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = QueryableId> + Send + Sync;
    ///Dispatched instead of [`Self::id`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `id` and sends the result through `reply`. Override this method instead of `id` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn id_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<QueryableId>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let id = self.id(_ctx).await;
            reply.send(id)
        }
    }
    /**advertise `interface` under `interface_id` so queries depending on that id find it

the returned advertisement is what keeps it listed: drop it to stop advertising
without taking down the node that serves the interface. That node dying withdraws
the advertisement too — a ref going dead always means the object behind it is gone,
never that someone revoked it.*/
    fn add_interface(
        &self,
        _ctx: gluon::Context,
        interface: gluon::Ref,
        interface_id: String,
    ) -> impl Future<Output = Result<QueryableInterface, QueryableError>> + Send + Sync;
    ///Dispatched instead of [`Self::add_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `add_interface` and sends the result through `reply`. Override this method instead of `add_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn add_interface_oneway(
        &self,
        _ctx: gluon::Context,
        interface: gluon::Ref,
        interface_id: String,
        reply: gluon::ReplySender<Result<QueryableInterface, QueryableError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let advertisement = self.add_interface(_ctx, interface, interface_id).await;
            reply.send(advertisement)
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
                        interface = "QueryableObject", method = "id", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<QueryableId> = gluon::ReplySender::new(
                        return_callback,
                        |id, gluon_out| {
                            tracing::trace!(
                                interface = "QueryableObject", method = "id", ? id, "←"
                            );
                            id.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.id_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "QueryableObject", method = "id",
                                method_id = 8u32
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
                    let reply: gluon::ReplySender<
                        Result<QueryableInterface, QueryableError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |advertisement, gluon_out| {
                            tracing::trace!(
                                interface = "QueryableObject", method = "add_interface", ?
                                advertisement, "←"
                            );
                            advertisement.write_owned(gluon_out)?;
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
pub struct QueryableInterface {
    obj: gluon::Ref,
}
impl gluon::Convertable for QueryableInterface {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(QueryableInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for QueryableInterface {
    const ID: &'static str = "org.stardustxr.Query.QueryableInterface";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: QueryableInterfaceHandler> gluon::HandledBy<H> for QueryableInterface {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type QueryableInterfaceLocal<H> = gluon::LocalRef<QueryableInterface, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: QueryableInterfaceHandler> From<QueryableInterfaceLocal<H>>
for QueryableInterface {
    fn from(value: QueryableInterfaceLocal<H>) -> QueryableInterface {
        value.into_proxy()
    }
}
impl gluon::RefExt for QueryableInterface {
    fn from_ref(obj: gluon::Ref) -> QueryableInterface {
        QueryableInterface { obj }
    }
}
impl QueryableInterface {
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> QueryableInterface {
        QueryableInterface { obj }
    }
}
impl From<QueryableInterface> for gluon::Ref {
    fn from(value: QueryableInterface) -> Self {
        value.obj
    }
}
impl gluon::ToRef for QueryableInterface {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for QueryableInterface {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for QueryableInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for QueryableInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for QueryableInterface {}
pub trait QueryableInterfaceHandler: gluon::Handler + Send + Sync + 'static {
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
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type QueryInterfaceLocal<H> = gluon::LocalRef<QueryInterface, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: QueryInterfaceHandler> From<QueryInterfaceLocal<H>> for QueryInterface {
    fn from(value: QueryInterfaceLocal<H>) -> QueryInterface {
        value.into_proxy()
    }
}
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
