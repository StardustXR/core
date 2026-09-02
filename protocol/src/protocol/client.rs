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
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.delta.write_owned(gluon_data)?;
        self.predicted_display_time.write_owned(gluon_data)?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Client {
    obj: gluon::Ref,
}
impl gluon::Convertable for Client {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(Client::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Client {
    const ID: &'static str = "org.stardustxr.Client.Client";
}
impl gluon::Interface for Client {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: ClientHandler> gluon::HandledBy<H> for Client {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type ClientLocal<H> = gluon::LocalRef<Client, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: ClientHandler> From<ClientLocal<H>> for Client {
    fn from(value: ClientLocal<H>) -> Client {
        value.into_proxy()
    }
}
impl gluon::RefExt for Client {
    fn from_ref(obj: gluon::Ref) -> Client {
        Client { obj }
    }
}
impl Client {
    pub fn frame(&self, info: impl Into<FrameInfo>) -> Result<(), gluon::SendError> {
        let info: FrameInfo = info.into();
        tracing::trace!(interface = "Client", method = "frame", ? info, "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        info.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> Client {
        Client { obj }
    }
}
impl From<Client> for gluon::Ref {
    fn from(value: Client) -> Self {
        value.obj
    }
}
impl gluon::ToRef for Client {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for Client {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
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
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<(gluon::Node<Self>, gluon::LocalRef<Client, Self>), gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        Client::new_node(self)
    }
    fn to_service(self) -> Result<gluon::LocalRef<Client, Self>, gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        Client::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
