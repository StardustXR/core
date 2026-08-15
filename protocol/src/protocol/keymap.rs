#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Keymap",
    types: &[
        gluon::ExternalGluonType {
            name: "XkbcommonKeymapFd",
            supported_derives: gluon::Derives::from_bits_truncate(0u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "KeymapExchangeError",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
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
impl gluon::Convertable for XkbcommonKeymapFd {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.fd.write(gluon_data)?;
        self.size.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let fd = gluon::Convertable::read(gluon_data)?;
        let size = gluon::Convertable::read(gluon_data)?;
        Ok(XkbcommonKeymapFd { fd, size })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
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
impl gluon::Convertable for KeymapExchangeError {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            KeymapExchangeError::InvalidKeymap => {
                gluon_data.write_u16(0u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => KeymapExchangeError::InvalidKeymap,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
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
    obj: gluon::Ref,
}
impl gluon::Convertable for KeymapStore {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(KeymapStore::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for KeymapStore {
    const ID: &'static str = "org.stardustxr.Keymap.KeymapStore";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: KeymapStoreHandler> gluon::HandledBy<H> for KeymapStore {}
impl gluon::RefExt for KeymapStore {
    fn from_ref(obj: gluon::Ref) -> KeymapStore {
        KeymapStore { obj }
    }
}
impl KeymapStore {
    ///Register a xkbcommon keymap, deduplicates
    pub async fn exchange(
        &self,
        keymap: impl Into<XkbcommonKeymapFd>,
    ) -> Result<Result<Keymap, KeymapExchangeError>, gluon::SendError> {
        let keymap: XkbcommonKeymapFd = keymap.into();
        tracing::trace!(interface = "KeymapStore", method = "exchange", ? keymap, "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        keymap.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_keymap = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "KeymapStore", method = "exchange", ? __ret_keymap, "←"
        );
        Ok(__ret_keymap)
    }
    pub async fn get(
        &self,
        keymap: impl Into<Keymap>,
    ) -> Result<Option<XkbcommonKeymapFd>, gluon::SendError> {
        let keymap: Keymap = keymap.into();
        tracing::trace!(interface = "KeymapStore", method = "get", ? keymap, "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        keymap.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_keymap = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "KeymapStore", method = "get", ? __ret_keymap, "←"
        );
        Ok(__ret_keymap)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> KeymapStore {
        KeymapStore { obj }
    }
}
impl From<KeymapStore> for gluon::Ref {
    fn from(value: KeymapStore) -> Self {
        value.obj
    }
}
impl gluon::ToRef for KeymapStore {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for KeymapStore {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
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
pub trait KeymapStoreHandler: gluon::Handler + Send + Sync + 'static {
    ///Register a xkbcommon keymap, deduplicates
    fn exchange(
        &self,
        _ctx: gluon::Context,
        keymap: XkbcommonKeymapFd,
    ) -> impl Future<Output = Result<Keymap, KeymapExchangeError>> + Send + Sync;
    ///Dispatched instead of [`Self::exchange`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `exchange` and sends the result through `reply`. Override this method instead of `exchange` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn exchange_oneway(
        &self,
        _ctx: gluon::Context,
        keymap: XkbcommonKeymapFd,
        reply: gluon::ReplySender<Result<Keymap, KeymapExchangeError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let keymap = self.exchange(_ctx, keymap).await;
            reply.send(keymap)
        }
    }
    fn get(
        &self,
        _ctx: gluon::Context,
        keymap: Keymap,
    ) -> impl Future<Output = Option<XkbcommonKeymapFd>> + Send + Sync;
    ///Dispatched instead of [`Self::get`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get` and sends the result through `reply`. Override this method instead of `get` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_oneway(
        &self,
        _ctx: gluon::Context,
        keymap: Keymap,
        reply: gluon::ReplySender<Option<XkbcommonKeymapFd>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let keymap = self.get(_ctx, keymap).await;
            reply.send(keymap)
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
                    let param_keymap = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "KeymapStore", method = "exchange", ? param_keymap,
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Result<Keymap, KeymapExchangeError>> = gluon::ReplySender::new(
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
                    let param_keymap = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "KeymapStore", method = "get", ? param_keymap,
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Option<XkbcommonKeymapFd>> = gluon::ReplySender::new(
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
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct Keymap {
    obj: gluon::Ref,
}
impl gluon::Convertable for Keymap {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(Keymap::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for Keymap {
    const ID: &'static str = "org.stardustxr.Keymap.Keymap";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: KeymapHandler> gluon::HandledBy<H> for Keymap {}
impl gluon::RefExt for Keymap {
    fn from_ref(obj: gluon::Ref) -> Keymap {
        Keymap { obj }
    }
}
impl Keymap {
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> Keymap {
        Keymap { obj }
    }
}
impl From<Keymap> for gluon::Ref {
    fn from(value: Keymap) -> Self {
        value.obj
    }
}
impl gluon::ToRef for Keymap {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for Keymap {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
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
pub trait KeymapHandler: gluon::Handler + Send + Sync + 'static {
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
pub mod proxied {
    use super::*;
}
