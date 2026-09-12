#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_ipc::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon_ipc::ExternalProtocol = gluon_ipc::ExternalProtocol {
    protocol_name: "org.stardustxr.Keymap",
    types: &[
        gluon_ipc::ExternalGluonType {
            name: "XkbcommonKeymapFd",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(0u32),
            proxy: None,
        },
        gluon_ipc::ExternalGluonType {
            name: "KeymapExchangeError",
            supported_derives: gluon_ipc::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
/**A Fd representing an xkbcommon keymap with a null byte
Has the same format as wayland uses*/
#[derive(Debug)]
pub struct XkbcommonKeymapFd {
    pub fd: std::os::fd::OwnedFd,
    pub size: u32,
}
impl gluon_ipc::Convertable for XkbcommonKeymapFd {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.fd.write(gluon_data)?;
        self.size.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        let fd = gluon_ipc::Convertable::read(gluon_data)?;
        let size = gluon_ipc::Convertable::read(gluon_data)?;
        Ok(XkbcommonKeymapFd { fd, size })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.fd.write_owned(gluon_data)?;
        self.size.write_owned(gluon_data)?;
        Ok(())
    }
}
///Error returned by KeymapStore::exchange
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum KeymapExchangeError {
    InvalidKeymap,
}
impl gluon_ipc::Convertable for KeymapExchangeError {
    fn write(
        &self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        match self {
            KeymapExchangeError::InvalidKeymap => {
                gluon_data.write_u16(0u16)?;
            }
        };
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_ipc::DataReader,
    ) -> Result<Self, gluon_ipc::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => KeymapExchangeError::InvalidKeymap,
                v => return Err(gluon_ipc::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        match self {
            KeymapExchangeError::InvalidKeymap => {
                gluon_data.write_u16(0u16)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct KeymapStore {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for KeymapStore {
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
        Ok(KeymapStore::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl KeymapStore {
    const ID: &'static str = "org.stardustxr.Keymap.KeymapStore";
}
impl gluon_ipc::Interface for KeymapStore {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: KeymapStoreHandler> gluon_ipc::HandledBy<H> for KeymapStore {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type KeymapStoreLocal<H> = gluon_ipc::LocalRef<KeymapStore, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: KeymapStoreHandler> From<KeymapStoreLocal<H>> for KeymapStore {
    fn from(value: KeymapStoreLocal<H>) -> KeymapStore {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for KeymapStore {
    fn from_ref(obj: gluon_ipc::Ref) -> KeymapStore {
        KeymapStore { obj }
    }
}
impl KeymapStore {
    ///Register a xkbcommon keymap, deduplicates
    pub async fn exchange(
        &self,
        keymap: impl Into<XkbcommonKeymapFd>,
    ) -> Result<Result<Keymap, KeymapExchangeError>, gluon_ipc::SendError> {
        let keymap: XkbcommonKeymapFd = keymap.into();
        tracing::trace!(interface = "KeymapStore", method = "exchange", ? keymap, "→");
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        keymap.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_keymap = gluon_ipc::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "KeymapStore", method = "exchange", ? __ret_keymap, "←"
        );
        Ok(__ret_keymap)
    }
    pub async fn get(
        &self,
        keymap: impl Into<Keymap>,
    ) -> Result<Option<XkbcommonKeymapFd>, gluon_ipc::SendError> {
        let keymap: Keymap = keymap.into();
        tracing::trace!(interface = "KeymapStore", method = "get", ? keymap, "→");
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        keymap.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_keymap = gluon_ipc::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "KeymapStore", method = "get", ? __ret_keymap, "←"
        );
        Ok(__ret_keymap)
    }
    ///returns a unique and opaque id for the keymap
    pub async fn get_keymap_id(
        &self,
        keymap: impl Into<Keymap>,
    ) -> Result<Option<u64>, gluon_ipc::SendError> {
        let keymap: Keymap = keymap.into();
        tracing::trace!(
            interface = "KeymapStore", method = "get_keymap_id", ? keymap, "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        keymap.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 10u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_id = gluon_ipc::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "KeymapStore", method = "get_keymap_id", ? __ret_id, "←"
        );
        Ok(__ret_id)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> KeymapStore {
        KeymapStore { obj }
    }
}
impl From<KeymapStore> for gluon_ipc::Ref {
    fn from(value: KeymapStore) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for KeymapStore {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for KeymapStore {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for KeymapStore {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for KeymapStore {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for KeymapStore {}
pub trait KeymapStoreHandler: gluon_ipc::Handler + Send + Sync + 'static {
    ///Register a xkbcommon keymap, deduplicates
    fn exchange(
        &self,
        _ctx: gluon_ipc::Context,
        keymap: XkbcommonKeymapFd,
    ) -> impl Future<Output = Result<Keymap, KeymapExchangeError>> + Send + Sync;
    ///Dispatched instead of [`Self::exchange`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `exchange` and sends the result through `reply`. Override this method instead of `exchange` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn exchange_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        keymap: XkbcommonKeymapFd,
        reply: gluon_ipc::ReplySender<Result<Keymap, KeymapExchangeError>>,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            let keymap = self.exchange(_ctx, keymap).await;
            reply.send(keymap)
        }
    }
    fn get(
        &self,
        _ctx: gluon_ipc::Context,
        keymap: Keymap,
    ) -> impl Future<Output = Option<XkbcommonKeymapFd>> + Send + Sync;
    ///Dispatched instead of [`Self::get`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get` and sends the result through `reply`. Override this method instead of `get` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        keymap: Keymap,
        reply: gluon_ipc::ReplySender<Option<XkbcommonKeymapFd>>,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            let keymap = self.get(_ctx, keymap).await;
            reply.send(keymap)
        }
    }
    ///returns a unique and opaque id for the keymap
    fn get_keymap_id(
        &self,
        _ctx: gluon_ipc::Context,
        keymap: Keymap,
    ) -> impl Future<Output = Option<u64>> + Send + Sync;
    ///Dispatched instead of [`Self::get_keymap_id`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_keymap_id` and sends the result through `reply`. Override this method instead of `get_keymap_id` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_keymap_id_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        keymap: Keymap,
        reply: gluon_ipc::ReplySender<Option<u64>>,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            let id = self.get_keymap_id(_ctx, keymap).await;
            reply.send(id)
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
                    let param_keymap = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "KeymapStore", method = "exchange", ? param_keymap,
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<
                        Result<Keymap, KeymapExchangeError>,
                    > = gluon_ipc::ReplySender::new(
                        return_callback,
                        |keymap, gluon_out| {
                            tracing::trace!(
                                interface = "KeymapStore", method = "exchange", ? keymap,
                                "←"
                            );
                            keymap.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.exchange_oneway(ctx, param_keymap, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "KeymapStore", method =
                                "exchange", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_keymap = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "KeymapStore", method = "get", ? param_keymap,
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<Option<XkbcommonKeymapFd>> = gluon_ipc::ReplySender::new(
                        return_callback,
                        |keymap, gluon_out| {
                            tracing::trace!(
                                interface = "KeymapStore", method = "get", ? keymap, "←"
                            );
                            keymap.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_oneway(ctx, param_keymap, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "KeymapStore", method = "get",
                                method_id = 9u32
                            ),
                        )
                        .await?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_keymap = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "KeymapStore", method = "get_keymap_id", ?
                        param_keymap, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<Option<u64>> = gluon_ipc::ReplySender::new(
                        return_callback,
                        |id, gluon_out| {
                            tracing::trace!(
                                interface = "KeymapStore", method = "get_keymap_id", ? id,
                                "←"
                            );
                            id.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_keymap_id_oneway(ctx, param_keymap, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "KeymapStore", method =
                                "get_keymap_id", method_id = 10u32
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
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<KeymapStore, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        KeymapStore::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<KeymapStore, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        KeymapStore::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct Keymap {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for Keymap {
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
        Ok(Keymap::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Keymap {
    const ID: &'static str = "org.stardustxr.Keymap.Keymap";
}
impl gluon_ipc::Interface for Keymap {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: KeymapHandler> gluon_ipc::HandledBy<H> for Keymap {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type KeymapLocal<H> = gluon_ipc::LocalRef<Keymap, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: KeymapHandler> From<KeymapLocal<H>> for Keymap {
    fn from(value: KeymapLocal<H>) -> Keymap {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for Keymap {
    fn from_ref(obj: gluon_ipc::Ref) -> Keymap {
        Keymap { obj }
    }
}
impl Keymap {
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> Keymap {
        Keymap { obj }
    }
}
impl From<Keymap> for gluon_ipc::Ref {
    fn from(value: Keymap) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for Keymap {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for Keymap {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for Keymap {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Keymap {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Keymap {}
pub trait KeymapHandler: gluon_ipc::Handler + Send + Sync + 'static {
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
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<Keymap, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        Keymap::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<Keymap, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        Keymap::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
