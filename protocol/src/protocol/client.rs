#![allow(unused, clippy::single_match, clippy::match_single_binding)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Client",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "FrameInfo",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "ClientState",
            supported_derives: gluon_wire::Derives::from_bits_truncate(0u32),
        },
    ],
};
///Information for a specific frame
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FrameInfo {
    pub delta: f32,
    pub predicted_display_time: Option<super::types::Timestamp>,
}
impl gluon_wire::GluonConvertable for FrameInfo {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.delta.write(gluon_data)?;
        self.predicted_display_time.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let delta = gluon_wire::GluonConvertable::read(gluon_data)?;
        let predicted_display_time = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(FrameInfo {
            delta,
            predicted_display_time,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.delta.write_owned(gluon_data)?;
        self.predicted_display_time.write_owned(gluon_data)?;
        Ok(())
    }
}
///The persistent state of a Stardust client.
#[derive(Debug)]
pub struct ClientState {
    ///Data specific to your client, put anything you like here and it'll be saved/restored intact.
    pub data: Vec<u8>,
    ///Where the client's root should be positioned on reload.
    pub root: super::spatial::SpatialRef,
    ///Spatials that will be in the same place you left them.
    pub spatial_anchors: std::collections::HashMap<u64, super::spatial::SpatialRef>,
}
impl gluon_wire::GluonConvertable for ClientState {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.data.write(gluon_data)?;
        self.root.write(gluon_data)?;
        self.spatial_anchors.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let data = gluon_wire::GluonConvertable::read(gluon_data)?;
        let root = gluon_wire::GluonConvertable::read(gluon_data)?;
        let spatial_anchors = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(ClientState {
            data,
            root,
            spatial_anchors,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.data.write_owned(gluon_data)?;
        self.root.write_owned(gluon_data)?;
        self.spatial_anchors.write_owned(gluon_data)?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Client {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for Client {
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
        Ok(Client::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Client {
    pub async fn ping(&self) -> Result<(), gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.ping_blocking()).await.unwrap()
    }
    pub fn ping_blocking(&self) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(())
    }
    pub fn frame(&self, info: FrameInfo) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        info.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub async fn safe_state(&self) -> Result<ClientState, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.safe_state_blocking()).await.unwrap()
    }
    pub fn safe_state_blocking(
        &self,
    ) -> Result<ClientState, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: ClientHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> Client {
        Client::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Client {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        Client { obj, drop_notification }
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
impl binderbinder::binder_object::ToBinderObjectOrRef for Client {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait ClientHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn ping(&self, _ctx: gluon_wire::GluonCtx) -> impl Future<Output = ()> + Send + Sync;
    fn frame(&self, _ctx: gluon_wire::GluonCtx, info: FrameInfo);
    fn safe_state(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = ClientState> + Send + Sync;
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
                    let () = self.ping(ctx).await;
                }
                10u32 => {
                    let (state) = self.safe_state(ctx).await;
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
                9u32 => {
                    self.frame(ctx, gluon_wire::GluonConvertable::read(gluon_data)?);
                }
                _ => {}
            }
            Ok(())
        }
    }
}
