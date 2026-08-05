#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Client",
    types: &[
        gluon::ExternalGluonType {
            name: "FrameInfo",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///Information for a specific frame
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrameInfo {
    pub delta: f32,
    pub predicted_display_time: super::types::Timestamp,
}
impl gluon::Convertable for FrameInfo {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.delta.write(gluon_data)?;
        self.predicted_display_time.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let delta = gluon::Convertable::read(gluon_data)?;
        let predicted_display_time = gluon::Convertable::read(gluon_data)?;
        Ok(FrameInfo {
            delta,
            predicted_display_time,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.delta.write_owned(gluon_data)?;
        self.predicted_display_time.write_owned(gluon_data)?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Client {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for Client {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(Client::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for Client {
    const ID: &'static str = "org.stardustxr.Client.Client";
}
impl Client {
    pub fn frame(&self, info: impl Into<FrameInfo>) -> gluon::OnewayFuture {
        use gluon::ToObjectOrRef as _;
        let info: FrameInfo = info.into();
        tracing::trace!(interface = "Client", method = "frame", ? info, "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        let gluon_ret: Option<gluon::ObjectOrRef> = Some(
            gluon_ret.to_binder_object_or_ref(),
        );
        if let Err(err) = gluon_ret.write(&mut gluon_builder) {
            return err.into();
        }
        if let Err(err) = info.write(&mut gluon_builder) {
            return err.into();
        }
        if let Err(err) = self
            .obj
            .device()
            .transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())
        {
            return err.into();
        }
        gluon_recv.into()
    }
    ///Fire and Forget, events sent to different objects may not be handled in order
    pub fn frame_event(
        &self,
        info: impl Into<FrameInfo>,
    ) -> Result<(), gluon::SendError> {
        let info: FrameInfo = info.into();
        tracing::trace!(interface = "Client", method = "frame", ? info, "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let gluon_ret: Option<gluon::ObjectOrRef> = None;
        gluon_ret.write(&mut gluon_builder)?;
        info.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: ClientHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> Client {
        Client::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> Client {
        Client { obj }
    }
}
impl From<Client> for gluon::ObjectOrRef {
    fn from(value: Client) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for Client {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl gluon::Liveness for Client {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for Client {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Client {}
pub trait ClientHandler: gluon::Handler + Send + Sync + 'static {
    fn frame(
        &self,
        _ctx: gluon::Context,
        info: FrameInfo,
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
                    let gluon_ret: Option<gluon::ObjectOrRef> = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_info = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Client", method = "frame", ? param_info,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.frame(ctx, param_info)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Client", method = "frame",
                                method_id = 8u32
                            ),
                        )
                        .await;
                    if let Some(obj) = gluon_ret {
                        obj.device()
                            .transact_one_way(
                                &obj,
                                0,
                                gluon::DataBuilder::new().to_payload(),
                            )?;
                    }
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
