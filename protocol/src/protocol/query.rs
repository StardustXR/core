#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_ipc::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon_ipc::ExternalProtocol = gluon_ipc::ExternalProtocol {
    protocol_name: "org.stardustxr.Query",
    types: &[
        gluon_ipc::ExternalGluonType {
            name: "QueryableId",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon_ipc::ExternalGluonType {
            name: "InterfaceDependency",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(798u32),
            proxy: None,
        },
        gluon_ipc::ExternalGluonType {
            name: "QueriedInterface",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(30u32),
            proxy: None,
        },
        gluon_ipc::ExternalGluonType {
            name: "QueryableError",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(799u32),
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
impl gluon_ipc::Convertable for QueryableId {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.id.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let id = gluon_ipc::Convertable::read(gluon_data)?;
        Ok(QueryableId { id })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
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
impl gluon_ipc::Convertable for InterfaceDependency {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.id.write(gluon_data)?;
        self.optional.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let id = gluon_ipc::Convertable::read(gluon_data)?;
        let optional = gluon_ipc::Convertable::read(gluon_data)?;
        Ok(InterfaceDependency {
            id,
            optional,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.id.write_owned(gluon_data)?;
        self.optional.write_owned(gluon_data)?;
        Ok(())
    }
}
///A successfully queried interface
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct QueriedInterface {
    pub interface_id: String,
    pub interface: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for QueriedInterface {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.interface_id.write(gluon_data)?;
        self.interface.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let interface_id = gluon_ipc::Convertable::read(gluon_data)?;
        let interface = gluon_ipc::Convertable::read(gluon_data)?;
        Ok(QueriedInterface {
            interface_id,
            interface,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
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
impl gluon_ipc::Convertable for QueryableError {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
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
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => QueryableError::NotOwnedSpatial,
                1u16 => QueryableError::NotOwnedField,
                2u16 => QueryableError::DuplicateInterface,
                v => return Err(gluon_ipc::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
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
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for QueryableObject {
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
        Ok(QueryableObject::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl QueryableObject {
    const ID: &'static str = "org.stardustxr.Query.QueryableObject";
}
impl gluon_ipc::Interface for QueryableObject {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: QueryableObjectHandler> gluon_ipc::HandledBy<H> for QueryableObject {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type QueryableObjectLocal<H> = gluon_ipc::LocalRef<QueryableObject, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: QueryableObjectHandler> From<QueryableObjectLocal<H>> for QueryableObject {
    fn from(value: QueryableObjectLocal<H>) -> QueryableObject {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for QueryableObject {
    fn from_ref(obj: gluon_ipc::Ref) -> QueryableObject {
        QueryableObject { obj }
    }
}
impl QueryableObject {
    ///this queryable's id, as querying clients see it
    pub async fn id(&self) -> Result<QueryableId, gluon_ipc::SendError> {
        tracing::trace!(interface = "QueryableObject", method = "id", "→");
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_id = gluon_ipc::Convertable::read(&mut reader)?;
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
        interface: &impl gluon_ipc::ToRef,
        interface_id: impl Into<String>,
    ) -> Result<Result<QueryableInterface, QueryableError>, gluon_ipc::SendError> {
        let interface: gluon_ipc::Ref = gluon_ipc::ToRef::to_ref(interface);
        let interface_id: String = interface_id.into();
        tracing::trace!(
            interface = "QueryableObject", method = "add_interface", ? interface, ?
            interface_id, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        interface.write(&mut gluon_builder)?;
        interface_id.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_advertisement = gluon_ipc::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "QueryableObject", method = "add_interface", ?
            __ret_advertisement, "←"
        );
        Ok(__ret_advertisement)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> QueryableObject {
        QueryableObject { obj }
    }
}
impl From<QueryableObject> for gluon_ipc::Ref {
    fn from(value: QueryableObject) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for QueryableObject {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for QueryableObject {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
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
pub trait QueryableObjectHandler: gluon_ipc::Handler + Send + Sync + 'static {
    ///this queryable's id, as querying clients see it
    fn id(
        &self,
        _ctx: gluon_ipc::Context,
    ) -> impl Future<Output = QueryableId> + Send + Sync;
    ///Dispatched instead of [`Self::id`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `id` and sends the result through `reply`. Override this method instead of `id` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn id_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        reply: gluon_ipc::ReplySender<QueryableId>,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
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
        _ctx: gluon_ipc::Context,
        interface: gluon_ipc::Ref,
        interface_id: String,
    ) -> impl Future<Output = Result<QueryableInterface, QueryableError>> + Send + Sync;
    ///Dispatched instead of [`Self::add_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `add_interface` and sends the result through `reply`. Override this method instead of `add_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn add_interface_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        interface: gluon_ipc::Ref,
        interface_id: String,
        reply: gluon_ipc::ReplySender<Result<QueryableInterface, QueryableError>>,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            let advertisement = self.add_interface(_ctx, interface, interface_id).await;
            reply.send(advertisement)
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
                    tracing::trace!(
                        interface = "QueryableObject", method = "id", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<QueryableId> = gluon_ipc::ReplySender::new(
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
                    let param_interface = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_interface_id = gluon_ipc::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "QueryableObject", method = "add_interface", ?
                        param_interface, ? param_interface_id, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<
                        Result<QueryableInterface, QueryableError>,
                    > = gluon_ipc::ReplySender::new(
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
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<QueryableObject, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        QueryableObject::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<QueryableObject, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        QueryableObject::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct QueryableInterface {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for QueryableInterface {
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
        Ok(QueryableInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl QueryableInterface {
    const ID: &'static str = "org.stardustxr.Query.QueryableInterface";
}
impl gluon_ipc::Interface for QueryableInterface {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: QueryableInterfaceHandler> gluon_ipc::HandledBy<H> for QueryableInterface {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type QueryableInterfaceLocal<H> = gluon_ipc::LocalRef<QueryableInterface, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: QueryableInterfaceHandler> From<QueryableInterfaceLocal<H>>
for QueryableInterface {
    fn from(value: QueryableInterfaceLocal<H>) -> QueryableInterface {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for QueryableInterface {
    fn from_ref(obj: gluon_ipc::Ref) -> QueryableInterface {
        QueryableInterface { obj }
    }
}
impl QueryableInterface {
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> QueryableInterface {
        QueryableInterface { obj }
    }
}
impl From<QueryableInterface> for gluon_ipc::Ref {
    fn from(value: QueryableInterface) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for QueryableInterface {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for QueryableInterface {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
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
pub trait QueryableInterfaceHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_ipc::DataReader,
        ctx: gluon_ipc::Context,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<QueryableInterface, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        QueryableInterface::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<QueryableInterface, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        QueryableInterface::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct QueryInterface {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for QueryInterface {
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
        Ok(QueryInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl QueryInterface {
    const ID: &'static str = "org.stardustxr.Query.QueryInterface";
}
impl gluon_ipc::Interface for QueryInterface {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: QueryInterfaceHandler> gluon_ipc::HandledBy<H> for QueryInterface {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type QueryInterfaceLocal<H> = gluon_ipc::LocalRef<QueryInterface, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: QueryInterfaceHandler> From<QueryInterfaceLocal<H>> for QueryInterface {
    fn from(value: QueryInterfaceLocal<H>) -> QueryInterface {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for QueryInterface {
    fn from_ref(obj: gluon_ipc::Ref) -> QueryInterface {
        QueryInterface { obj }
    }
}
impl QueryInterface {
    pub async fn register_queryable(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        field: impl Into<super::field::Field>,
    ) -> Result<Result<QueryableObject, QueryableError>, gluon_ipc::SendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let field: super::field::Field = field.into();
        tracing::trace!(
            interface = "QueryInterface", method = "register_queryable", ? spatial, ?
            field, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        field.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_queryable = gluon_ipc::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "QueryInterface", method = "register_queryable", ?
            __ret_queryable, "←"
        );
        Ok(__ret_queryable)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> QueryInterface {
        QueryInterface { obj }
    }
}
impl From<QueryInterface> for gluon_ipc::Ref {
    fn from(value: QueryInterface) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for QueryInterface {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for QueryInterface {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
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
pub trait QueryInterfaceHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn register_queryable(
        &self,
        _ctx: gluon_ipc::Context,
        spatial: super::spatial::Spatial,
        field: super::field::Field,
    ) -> impl Future<Output = Result<QueryableObject, QueryableError>> + Send + Sync;
    ///Dispatched instead of [`Self::register_queryable`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `register_queryable` and sends the result through `reply`. Override this method instead of `register_queryable` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn register_queryable_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        spatial: super::spatial::Spatial,
        field: super::field::Field,
        reply: gluon_ipc::ReplySender<Result<QueryableObject, QueryableError>>,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            let queryable = self.register_queryable(_ctx, spatial, field).await;
            reply.send(queryable)
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
                    let param_spatial = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_field = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "QueryInterface", method = "register_queryable", ?
                        param_spatial, ? param_field, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<
                        Result<QueryableObject, QueryableError>,
                    > = gluon_ipc::ReplySender::new(
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
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<QueryInterface, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        QueryInterface::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<QueryInterface, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        QueryInterface::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
