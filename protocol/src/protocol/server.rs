#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Server",
    types: &[],
};
pub mod proxies {
    use super::*;
}
#[derive(Debug, Clone)]
pub struct Server {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for Server {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(Server::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Server {
    ///Get the spatial interface node.
    pub async fn spatial_interface(
        &self,
    ) -> Result<super::spatial::SpatialInterface, gluon::SendError> {
        tracing::trace!(interface = "Server", method = "spatial_interface", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "spatial_interface", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    pub async fn field_interface(
        &self,
    ) -> Result<super::field::FieldInterface, gluon::SendError> {
        tracing::trace!(interface = "Server", method = "field_interface", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "field_interface", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    ///Get the dmatex interface node.
    pub async fn dmatex_interface(
        &self,
    ) -> Result<super::dmatex::DmatexInterface, gluon::SendError> {
        tracing::trace!(interface = "Server", method = "dmatex_interface", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "dmatex_interface", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    pub async fn text_interface(
        &self,
    ) -> Result<super::text::TextInterface, gluon::SendError> {
        tracing::trace!(interface = "Server", method = "text_interface", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "text_interface", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    pub async fn model_interface(
        &self,
    ) -> Result<super::model::ModelInterface, gluon::SendError> {
        tracing::trace!(interface = "Server", method = "model_interface", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "model_interface", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    pub async fn lines_interface(
        &self,
    ) -> Result<super::lines::LinesInterface, gluon::SendError> {
        tracing::trace!(interface = "Server", method = "lines_interface", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 13u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "lines_interface", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    pub async fn sky_interface(
        &self,
    ) -> Result<super::sky::SkyInterface, gluon::SendError> {
        tracing::trace!(interface = "Server", method = "sky_interface", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 14u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "sky_interface", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    pub async fn audio_interface(
        &self,
    ) -> Result<super::audio::AudioInterface, gluon::SendError> {
        tracing::trace!(interface = "Server", method = "audio_interface", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 15u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "audio_interface", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    pub async fn query_interface(
        &self,
    ) -> Result<super::query::QueryInterface, gluon::SendError> {
        tracing::trace!(interface = "Server", method = "query_interface", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 16u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_interface = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "query_interface", ? __ret_interface, "←"
        );
        Ok(__ret_interface)
    }
    pub async fn spatial_query_interface(
        &self,
    ) -> Result<super::spatial_query::SpatialQueryInterface, gluon::SendError> {
        tracing::trace!(interface = "Server", method = "spatial_query_interface", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 17u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_interface = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "spatial_query_interface", ? __ret_interface,
            "←"
        );
        Ok(__ret_interface)
    }
    /**Generate a client state token and return it back.

When launching a new client, set the environment variable `STARDUST_STARTUP_TOKEN` to the returned string.*/
    pub async fn generate_startup_token(
        &self,
        root: impl Into<super::spatial::SpatialRef>,
    ) -> Result<Result<String, super::types::CreateError>, gluon::SendError> {
        let root: super::spatial::SpatialRef = root.into();
        tracing::trace!(
            interface = "Server", method = "generate_startup_token", ? root, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        root.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 18u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_token = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Server", method = "generate_startup_token", ? __ret_token, "←"
        );
        Ok(__ret_token)
    }
    pub fn from_handler<H: ServerHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> Server {
        Server::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> Server {
        Server { obj }
    }
}
impl From<Server> for gluon::ObjectOrRef {
    fn from(value: Server) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for Server {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl std::hash::Hash for Server {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Server {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Server {}
pub trait ServerHandler: gluon::Handler + Send + Sync + 'static {
    ///Get the spatial interface node.
    fn spatial_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::spatial::SpatialInterface> + Send + Sync;
    ///Dispatched instead of [`Self::spatial_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `spatial_interface` and sends the result through `reply`. Override this method instead of `spatial_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn spatial_interface_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::spatial::SpatialInterface>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.spatial_interface(_ctx).await;
            reply.send(spatial)
        }
    }
    fn field_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::field::FieldInterface> + Send + Sync;
    ///Dispatched instead of [`Self::field_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `field_interface` and sends the result through `reply`. Override this method instead of `field_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn field_interface_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::field::FieldInterface>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.field_interface(_ctx).await;
            reply.send(spatial)
        }
    }
    ///Get the dmatex interface node.
    fn dmatex_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::dmatex::DmatexInterface> + Send + Sync;
    ///Dispatched instead of [`Self::dmatex_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `dmatex_interface` and sends the result through `reply`. Override this method instead of `dmatex_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn dmatex_interface_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::dmatex::DmatexInterface>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.dmatex_interface(_ctx).await;
            reply.send(spatial)
        }
    }
    fn text_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::text::TextInterface> + Send + Sync;
    ///Dispatched instead of [`Self::text_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `text_interface` and sends the result through `reply`. Override this method instead of `text_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn text_interface_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::text::TextInterface>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.text_interface(_ctx).await;
            reply.send(spatial)
        }
    }
    fn model_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::model::ModelInterface> + Send + Sync;
    ///Dispatched instead of [`Self::model_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `model_interface` and sends the result through `reply`. Override this method instead of `model_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn model_interface_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::model::ModelInterface>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.model_interface(_ctx).await;
            reply.send(spatial)
        }
    }
    fn lines_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::lines::LinesInterface> + Send + Sync;
    ///Dispatched instead of [`Self::lines_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `lines_interface` and sends the result through `reply`. Override this method instead of `lines_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn lines_interface_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::lines::LinesInterface>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.lines_interface(_ctx).await;
            reply.send(spatial)
        }
    }
    fn sky_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::sky::SkyInterface> + Send + Sync;
    ///Dispatched instead of [`Self::sky_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `sky_interface` and sends the result through `reply`. Override this method instead of `sky_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn sky_interface_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::sky::SkyInterface>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.sky_interface(_ctx).await;
            reply.send(spatial)
        }
    }
    fn audio_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::audio::AudioInterface> + Send + Sync;
    ///Dispatched instead of [`Self::audio_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `audio_interface` and sends the result through `reply`. Override this method instead of `audio_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn audio_interface_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::audio::AudioInterface>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.audio_interface(_ctx).await;
            reply.send(spatial)
        }
    }
    fn query_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::query::QueryInterface> + Send + Sync;
    ///Dispatched instead of [`Self::query_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `query_interface` and sends the result through `reply`. Override this method instead of `query_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn query_interface_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::query::QueryInterface>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let interface = self.query_interface(_ctx).await;
            reply.send(interface)
        }
    }
    fn spatial_query_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::spatial_query::SpatialQueryInterface> + Send + Sync;
    ///Dispatched instead of [`Self::spatial_query_interface`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `spatial_query_interface` and sends the result through `reply`. Override this method instead of `spatial_query_interface` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn spatial_query_interface_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::spatial_query::SpatialQueryInterface>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let interface = self.spatial_query_interface(_ctx).await;
            reply.send(interface)
        }
    }
    /**Generate a client state token and return it back.

When launching a new client, set the environment variable `STARDUST_STARTUP_TOKEN` to the returned string.*/
    fn generate_startup_token(
        &self,
        _ctx: gluon::Context,
        root: super::spatial::SpatialRef,
    ) -> impl Future<Output = Result<String, super::types::CreateError>> + Send + Sync;
    ///Dispatched instead of [`Self::generate_startup_token`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `generate_startup_token` and sends the result through `reply`. Override this method instead of `generate_startup_token` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn generate_startup_token_oneway(
        &self,
        _ctx: gluon::Context,
        root: super::spatial::SpatialRef,
        reply: gluon::ReplySender<Result<String, super::types::CreateError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let token = self.generate_startup_token(_ctx, root).await;
            reply.send(token)
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
                    let return_callback = gluon_data.read_binder()?;
                    tracing::trace!(
                        interface = "Server", method = "spatial_interface", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::spatial::SpatialInterface> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "spatial_interface", ?
                                spatial, "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.spatial_interface_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "spatial_interface", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    tracing::trace!(
                        interface = "Server", method = "field_interface", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::field::FieldInterface> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "field_interface", ? spatial,
                                "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.field_interface_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "field_interface", method_id = 9u32
                            ),
                        )
                        .await?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    tracing::trace!(
                        interface = "Server", method = "dmatex_interface", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::dmatex::DmatexInterface> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "dmatex_interface", ?
                                spatial, "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.dmatex_interface_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "dmatex_interface", method_id = 10u32
                            ),
                        )
                        .await?;
                }
                11u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    tracing::trace!(
                        interface = "Server", method = "text_interface", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::text::TextInterface> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "text_interface", ? spatial,
                                "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.text_interface_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "text_interface", method_id = 11u32
                            ),
                        )
                        .await?;
                }
                12u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    tracing::trace!(
                        interface = "Server", method = "model_interface", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::model::ModelInterface> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "model_interface", ? spatial,
                                "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.model_interface_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "model_interface", method_id = 12u32
                            ),
                        )
                        .await?;
                }
                13u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    tracing::trace!(
                        interface = "Server", method = "lines_interface", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::lines::LinesInterface> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "lines_interface", ? spatial,
                                "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.lines_interface_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "lines_interface", method_id = 13u32
                            ),
                        )
                        .await?;
                }
                14u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    tracing::trace!(
                        interface = "Server", method = "sky_interface", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::sky::SkyInterface> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "sky_interface", ? spatial,
                                "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.sky_interface_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "sky_interface", method_id = 14u32
                            ),
                        )
                        .await?;
                }
                15u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    tracing::trace!(
                        interface = "Server", method = "audio_interface", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::audio::AudioInterface> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "audio_interface", ? spatial,
                                "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.audio_interface_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "audio_interface", method_id = 15u32
                            ),
                        )
                        .await?;
                }
                16u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    tracing::trace!(
                        interface = "Server", method = "query_interface", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::query::QueryInterface> = gluon::ReplySender::new(
                        return_callback,
                        |interface, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "query_interface", ?
                                interface, "←"
                            );
                            interface.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.query_interface_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "query_interface", method_id = 16u32
                            ),
                        )
                        .await?;
                }
                17u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    tracing::trace!(
                        interface = "Server", method = "spatial_query_interface",
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        super::spatial_query::SpatialQueryInterface,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |interface, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "spatial_query_interface", ?
                                interface, "←"
                            );
                            interface.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.spatial_query_interface_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "spatial_query_interface", method_id = 17u32
                            ),
                        )
                        .await?;
                }
                18u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let param_root = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Server", method = "generate_startup_token", ?
                        param_root, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<String, super::types::CreateError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |token, gluon_out| {
                            tracing::trace!(
                                interface = "Server", method = "generate_startup_token", ?
                                token, "←"
                            );
                            token.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.generate_startup_token_oneway(ctx, param_root, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Server", method =
                                "generate_startup_token", method_id = 18u32
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
pub struct ServerInterface {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for ServerInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(ServerInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl ServerInterface {
    ///The startup_token should be read from the `STARDUST_STARTUP_TOKEN`environment variable.
    pub async fn connect(
        &self,
        client: impl Into<super::client::Client>,
        startup_token: impl Into<Option<String>>,
        resource_prefixes: impl Into<Vec<String>>,
    ) -> Result<(Server, super::spatial::SpatialRef), gluon::SendError> {
        let client: super::client::Client = client.into();
        let startup_token: Option<String> = startup_token.into();
        let resource_prefixes: Vec<String> = resource_prefixes.into();
        tracing::trace!(
            interface = "ServerInterface", method = "connect", ? client, ? startup_token,
            ? resource_prefixes, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        client.write(&mut gluon_builder)?;
        startup_token.write(&mut gluon_builder)?;
        resource_prefixes.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_server = gluon::Convertable::read(&mut reader)?;
        let __ret_root = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "ServerInterface", method = "connect", ? __ret_server, ?
            __ret_root, "←"
        );
        Ok((__ret_server, __ret_root))
    }
    pub async fn startup_spatial(
        &self,
        startup_token: impl Into<String>,
    ) -> Result<Option<super::spatial::SpatialRef>, gluon::SendError> {
        let startup_token: String = startup_token.into();
        tracing::trace!(
            interface = "ServerInterface", method = "startup_spatial", ? startup_token,
            "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        startup_token.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_spatial_ref = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "ServerInterface", method = "startup_spatial", ?
            __ret_spatial_ref, "←"
        );
        Ok(__ret_spatial_ref)
    }
    pub fn from_handler<H: ServerInterfaceHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> ServerInterface {
        ServerInterface::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> ServerInterface {
        ServerInterface { obj }
    }
}
impl From<ServerInterface> for gluon::ObjectOrRef {
    fn from(value: ServerInterface) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for ServerInterface {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl std::hash::Hash for ServerInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for ServerInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for ServerInterface {}
pub trait ServerInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    ///The startup_token should be read from the `STARDUST_STARTUP_TOKEN`environment variable.
    fn connect(
        &self,
        _ctx: gluon::Context,
        client: super::client::Client,
        startup_token: Option<String>,
        resource_prefixes: Vec<String>,
    ) -> impl Future<Output = (Server, super::spatial::SpatialRef)> + Send + Sync;
    ///Dispatched instead of [`Self::connect`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `connect` and sends the result through `reply`. Override this method instead of `connect` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn connect_oneway(
        &self,
        _ctx: gluon::Context,
        client: super::client::Client,
        startup_token: Option<String>,
        resource_prefixes: Vec<String>,
        reply: gluon::ReplySender<(Server, super::spatial::SpatialRef)>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let (server, root) = self
                .connect(_ctx, client, startup_token, resource_prefixes)
                .await;
            reply.send((server, root))
        }
    }
    fn startup_spatial(
        &self,
        _ctx: gluon::Context,
        startup_token: String,
    ) -> impl Future<Output = Option<super::spatial::SpatialRef>> + Send + Sync;
    ///Dispatched instead of [`Self::startup_spatial`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `startup_spatial` and sends the result through `reply`. Override this method instead of `startup_spatial` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn startup_spatial_oneway(
        &self,
        _ctx: gluon::Context,
        startup_token: String,
        reply: gluon::ReplySender<Option<super::spatial::SpatialRef>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial_ref = self.startup_spatial(_ctx, startup_token).await;
            reply.send(spatial_ref)
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
                    let return_callback = gluon_data.read_binder()?;
                    let param_client = gluon::Convertable::read(&mut gluon_data)?;
                    let param_startup_token = gluon::Convertable::read(&mut gluon_data)?;
                    let param_resource_prefixes = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "ServerInterface", method = "connect", ?
                        param_client, ? param_startup_token, ? param_resource_prefixes,
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        (Server, super::spatial::SpatialRef),
                    > = gluon::ReplySender::new(
                        return_callback,
                        |(server, root), gluon_out| {
                            tracing::trace!(
                                interface = "ServerInterface", method = "connect", ? server,
                                ? root, "←"
                            );
                            server.write_owned(gluon_out)?;
                            root.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.connect_oneway(
                            ctx,
                            param_client,
                            param_startup_token,
                            param_resource_prefixes,
                            reply,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ServerInterface", method =
                                "connect", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let param_startup_token = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "ServerInterface", method = "startup_spatial", ?
                        param_startup_token, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Option<super::spatial::SpatialRef>> = gluon::ReplySender::new(
                        return_callback,
                        |spatial_ref, gluon_out| {
                            tracing::trace!(
                                interface = "ServerInterface", method = "startup_spatial", ?
                                spatial_ref, "←"
                            );
                            spatial_ref.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.startup_spatial_oneway(ctx, param_startup_token, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ServerInterface", method =
                                "startup_spatial", method_id = 9u32
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
