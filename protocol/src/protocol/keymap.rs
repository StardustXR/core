#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
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
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
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
        gluon_data: &mut gluon::DataBuilder<'_>,
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for KeymapStore {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(KeymapStore::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
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
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        keymap.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
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
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        keymap.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_keymap = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "KeymapStore", method = "get", ? __ret_keymap, "←"
        );
        Ok(__ret_keymap)
    }
    pub fn from_handler<H: KeymapStoreHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> KeymapStore {
        KeymapStore::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> KeymapStore {
        KeymapStore { obj }
    }
}
impl From<KeymapStore> for gluon::ObjectOrRef {
    fn from(value: KeymapStore) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for KeymapStore {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
    fn get(
        &self,
        _ctx: gluon::Context,
        keymap: Keymap,
    ) -> impl Future<Output = Option<XkbcommonKeymapFd>> + Send + Sync;
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
                    let param_keymap = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "KeymapStore", method = "exchange", ? param_keymap,
                        "dispatching"
                    );
                    let (keymap) = self.exchange(ctx, param_keymap).await;
                    drop(gluon_data);
                    tracing::trace!(
                        interface = "KeymapStore", method = "exchange", ? keymap, "←"
                    );
                    keymap.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_keymap = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "KeymapStore", method = "get", ? param_keymap,
                        "dispatching"
                    );
                    let (keymap) = self.get(ctx, param_keymap).await;
                    drop(gluon_data);
                    tracing::trace!(
                        interface = "KeymapStore", method = "get", ? keymap, "←"
                    );
                    keymap.write_owned(&mut gluon_out)?;
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
pub struct Keymap {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for Keymap {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(Keymap::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Keymap {
    pub fn from_handler<H: KeymapHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> Keymap {
        Keymap::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> Keymap {
        Keymap { obj }
    }
}
impl From<Keymap> for gluon::ObjectOrRef {
    fn from(value: Keymap) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for Keymap {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
