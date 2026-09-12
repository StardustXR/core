#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_ipc::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon_ipc::ExternalProtocol = gluon_ipc::ExternalProtocol {
    protocol_name: "org.stardustxr.Client",
    types: &[
        gluon_ipc::ExternalGluonType {
            name: "FrameInfo",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(779u32),
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
impl gluon_ipc::Convertable for FrameInfo {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.delta.write(gluon_data)?;
        self.predicted_display_time.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let delta = gluon_ipc::Convertable::read(gluon_data)?;
        let predicted_display_time = gluon_ipc::Convertable::read(gluon_data)?;
        Ok(FrameInfo {
            delta,
            predicted_display_time,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.delta.write_owned(gluon_data)?;
        self.predicted_display_time.write_owned(gluon_data)?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Client {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for Client {
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
        Ok(Client::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Client {
    const ID: &'static str = "org.stardustxr.Client.Client";
}
impl gluon_ipc::Interface for Client {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: ClientHandler> gluon_ipc::HandledBy<H> for Client {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type ClientLocal<H> = gluon_ipc::LocalRef<Client, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: ClientHandler> From<ClientLocal<H>> for Client {
    fn from(value: ClientLocal<H>) -> Client {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for Client {
    fn from_ref(obj: gluon_ipc::Ref) -> Client {
        Client { obj }
    }
}
impl Client {
    pub fn frame(&self, info: impl Into<FrameInfo>) -> Result<(), gluon_ipc::SendError> {
        let info: FrameInfo = info.into();
        tracing::trace!(interface = "Client", method = "frame", ? info, "→");
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        info.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> Client {
        Client { obj }
    }
}
impl From<Client> for gluon_ipc::Ref {
    fn from(value: Client) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for Client {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for Client {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
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
pub trait ClientHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn frame(
        &self,
        _ctx: gluon_ipc::Context,
        info: FrameInfo,
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
                    let param_info = gluon_ipc::Convertable::read(&mut gluon_data)?;
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
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<Client, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        Client::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<Client, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        Client::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
