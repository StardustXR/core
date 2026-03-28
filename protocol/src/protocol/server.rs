#![allow(unused, clippy::single_match, clippy::match_single_binding)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Server",
    types: &[],
};
#[derive(Debug, Clone)]
pub struct Server {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for Server {
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
        Ok(Server::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Server {
    ///Get the spatial interface node.
    pub async fn spatial_interface(
        &self,
    ) -> Result<super::spatial::SpatialInterface, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.spatial_interface_blocking())
            .await
            .unwrap()
    }
    pub fn spatial_interface_blocking(
        &self,
    ) -> Result<super::spatial::SpatialInterface, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn field_interface(
        &self,
    ) -> Result<super::field::FieldInterface, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.field_interface_blocking())
            .await
            .unwrap()
    }
    pub fn field_interface_blocking(
        &self,
    ) -> Result<super::field::FieldInterface, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the dmatex interface node.
    pub async fn dmatex_interface(
        &self,
    ) -> Result<super::dmatex::DmatexInterface, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.dmatex_interface_blocking())
            .await
            .unwrap()
    }
    pub fn dmatex_interface_blocking(
        &self,
    ) -> Result<super::dmatex::DmatexInterface, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn text_interface(
        &self,
    ) -> Result<super::text::TextInterface, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.text_interface_blocking())
            .await
            .unwrap()
    }
    pub fn text_interface_blocking(
        &self,
    ) -> Result<super::text::TextInterface, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 11u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn model_interface(
        &self,
    ) -> Result<super::model::ModelInterface, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.model_interface_blocking())
            .await
            .unwrap()
    }
    pub fn model_interface_blocking(
        &self,
    ) -> Result<super::model::ModelInterface, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 12u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn lines_interface(
        &self,
    ) -> Result<super::lines::LinesInterface, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.lines_interface_blocking())
            .await
            .unwrap()
    }
    pub fn lines_interface_blocking(
        &self,
    ) -> Result<super::lines::LinesInterface, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 13u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn sky_interface(
        &self,
    ) -> Result<super::sky::SkyInterface, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.sky_interface_blocking()).await.unwrap()
    }
    pub fn sky_interface_blocking(
        &self,
    ) -> Result<super::sky::SkyInterface, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 14u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    /**Generate a client state token and return it back.

When launching a new client, set the environment variable `STARDUST_STARTUP_TOKEN` to the returned string.
Make sure the environment variable shows in `/proc/{pid}/environ` as that's the only reliable way to pass the value to the server (suggestions welcome).*/
    pub async fn generate_state_token(
        &self,
        state: super::client::ClientState,
    ) -> Result<String, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.generate_state_token_blocking(state))
            .await
            .unwrap()
    }
    pub fn generate_state_token_blocking(
        &self,
        state: super::client::ClientState,
    ) -> Result<String, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        state.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 15u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: ServerHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> Server {
        Server::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Server {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        Server { obj, drop_notification }
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
impl binderbinder::binder_object::ToBinderObjectOrRef for Server {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait ServerHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Get the spatial interface node.
    fn spatial_interface(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::spatial::SpatialInterface> + Send + Sync;
    fn field_interface(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::field::FieldInterface> + Send + Sync;
    ///Get the dmatex interface node.
    fn dmatex_interface(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::dmatex::DmatexInterface> + Send + Sync;
    fn text_interface(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::text::TextInterface> + Send + Sync;
    fn model_interface(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::model::ModelInterface> + Send + Sync;
    fn lines_interface(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::lines::LinesInterface> + Send + Sync;
    fn sky_interface(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::sky::SkyInterface> + Send + Sync;
    /**Generate a client state token and return it back.

When launching a new client, set the environment variable `STARDUST_STARTUP_TOKEN` to the returned string.
Make sure the environment variable shows in `/proc/{pid}/environ` as that's the only reliable way to pass the value to the server (suggestions welcome).*/
    fn generate_state_token(
        &self,
        _ctx: gluon_wire::GluonCtx,
        state: super::client::ClientState,
    ) -> impl Future<Output = String> + Send + Sync;
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                8u32 => {
                    let (spatial) = self.spatial_interface(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                9u32 => {
                    let (spatial) = self.field_interface(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                10u32 => {
                    let (spatial) = self.dmatex_interface(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                11u32 => {
                    let (spatial) = self.text_interface(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                12u32 => {
                    let (spatial) = self.model_interface(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                13u32 => {
                    let (spatial) = self.lines_interface(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                14u32 => {
                    let (spatial) = self.sky_interface(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                15u32 => {
                    let (token) = self
                        .generate_state_token(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    token.write_owned(&mut out)?;
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
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = gluon_data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
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
pub struct ServerInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for ServerInterface {
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
        Ok(ServerInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl ServerInterface {
    pub async fn connect(
        &self,
        client: super::client::Client,
        resource_prefixes: Vec<String>,
    ) -> Result<(Server, super::client::ClientState), gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.connect_blocking(client, resource_prefixes)
            })
            .await
            .unwrap()
    }
    pub fn connect_blocking(
        &self,
        client: super::client::Client,
        resource_prefixes: Vec<String>,
    ) -> Result<(Server, super::client::ClientState), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        client.write(&mut gluon_builder)?;
        resource_prefixes.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok((
            gluon_wire::GluonConvertable::read(&mut reader)?,
            gluon_wire::GluonConvertable::read(&mut reader)?,
        ))
    }
    pub fn from_handler<H: ServerInterfaceHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> ServerInterface {
        ServerInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> ServerInterface {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        ServerInterface {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for ServerInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait ServerInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn connect(
        &self,
        _ctx: gluon_wire::GluonCtx,
        client: super::client::Client,
        resource_prefixes: Vec<String>,
    ) -> impl Future<Output = (Server, super::client::ClientState)> + Send + Sync;
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                8u32 => {
                    let (server, state) = self
                        .connect(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    server.write_owned(&mut out)?;
                    state.write_owned(&mut out)?;
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
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = gluon_data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
