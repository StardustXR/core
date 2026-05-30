#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
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
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn field_interface(
        &self,
    ) -> Result<super::field::FieldInterface, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    ///Get the dmatex interface node.
    pub async fn dmatex_interface(
        &self,
    ) -> Result<super::dmatex::DmatexInterface, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn text_interface(
        &self,
    ) -> Result<super::text::TextInterface, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn model_interface(
        &self,
    ) -> Result<super::model::ModelInterface, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn lines_interface(
        &self,
    ) -> Result<super::lines::LinesInterface, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 13u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn sky_interface(
        &self,
    ) -> Result<super::sky::SkyInterface, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 14u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn audio_interface(
        &self,
    ) -> Result<super::audio::AudioInterface, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 15u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn query_interface(
        &self,
    ) -> Result<super::query::QueryInterface, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 16u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn spatial_query_interface(
        &self,
    ) -> Result<super::spatial_query::SpatialQueryInterface, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 17u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    /**Generate a client state token and return it back.

When launching a new client, set the environment variable `STARDUST_STARTUP_TOKEN` to the returned string.*/
    pub async fn generate_startup_token(
        &self,
        root: impl Into<super::spatial::SpatialRef>,
    ) -> Result<Result<String, super::types::CreateError>, gluon::SendError> {
        let root: super::spatial::SpatialRef = root.into();
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
        Ok(gluon::Convertable::read(&mut reader)?)
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
    fn field_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::field::FieldInterface> + Send + Sync;
    ///Get the dmatex interface node.
    fn dmatex_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::dmatex::DmatexInterface> + Send + Sync;
    fn text_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::text::TextInterface> + Send + Sync;
    fn model_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::model::ModelInterface> + Send + Sync;
    fn lines_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::lines::LinesInterface> + Send + Sync;
    fn sky_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::sky::SkyInterface> + Send + Sync;
    fn audio_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::audio::AudioInterface> + Send + Sync;
    fn query_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::query::QueryInterface> + Send + Sync;
    fn spatial_query_interface(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::spatial_query::SpatialQueryInterface> + Send + Sync;
    /**Generate a client state token and return it back.

When launching a new client, set the environment variable `STARDUST_STARTUP_TOKEN` to the returned string.*/
    fn generate_startup_token(
        &self,
        _ctx: gluon::Context,
        root: super::spatial::SpatialRef,
    ) -> impl Future<Output = Result<String, super::types::CreateError>> + Send + Sync;
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
                    let (spatial) = self.spatial_interface(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (spatial) = self.field_interface(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (spatial) = self.dmatex_interface(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                11u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (spatial) = self.text_interface(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                12u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (spatial) = self.model_interface(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                13u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (spatial) = self.lines_interface(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                14u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (spatial) = self.sky_interface(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                15u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (spatial) = self.audio_interface(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                16u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (interface) = self.query_interface(ctx).await;
                    drop(gluon_data);
                    interface.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                17u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (interface) = self.spatial_query_interface(ctx).await;
                    drop(gluon_data);
                    interface.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                18u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_root = gluon::Convertable::read(&mut gluon_data)?;
                    let (token) = self.generate_startup_token(ctx, param_root).await;
                    drop(gluon_data);
                    token.write_owned(&mut gluon_out)?;
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
        Ok((
            gluon::Convertable::read(&mut reader)?,
            gluon::Convertable::read(&mut reader)?,
        ))
    }
    pub async fn startup_spatial(
        &self,
        startup_token: impl Into<String>,
    ) -> Result<Option<super::spatial::SpatialRef>, gluon::SendError> {
        let startup_token: String = startup_token.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        startup_token.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
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
    fn startup_spatial(
        &self,
        _ctx: gluon::Context,
        startup_token: String,
    ) -> impl Future<Output = Option<super::spatial::SpatialRef>> + Send + Sync;
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
                    let param_client = gluon::Convertable::read(&mut gluon_data)?;
                    let param_startup_token = gluon::Convertable::read(&mut gluon_data)?;
                    let param_resource_prefixes = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let (server, root) = self
                        .connect(
                            ctx,
                            param_client,
                            param_startup_token,
                            param_resource_prefixes,
                        )
                        .await;
                    drop(gluon_data);
                    server.write_owned(&mut gluon_out)?;
                    root.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_startup_token = gluon::Convertable::read(&mut gluon_data)?;
                    let (spatial_ref) = self
                        .startup_spatial(ctx, param_startup_token)
                        .await;
                    drop(gluon_data);
                    spatial_ref.write_owned(&mut gluon_out)?;
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
pub mod proxied {
    use super::*;
}
