#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Sky",
    types: &[],
};
pub mod proxies {
    use super::*;
}
#[derive(Debug, Clone)]
pub struct SkyGuard {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for SkyGuard {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(SkyGuard::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for SkyGuard {
    const ID: &'static str = "org.stardustxr.Sky.SkyGuard";
}
impl SkyGuard {
    pub fn from_handler<H: SkyGuardHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> SkyGuard {
        SkyGuard::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> SkyGuard {
        SkyGuard { obj }
    }
}
impl From<SkyGuard> for gluon::ObjectOrRef {
    fn from(value: SkyGuard) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for SkyGuard {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl gluon::Liveness for SkyGuard {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for SkyGuard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for SkyGuard {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for SkyGuard {}
pub trait SkyGuardHandler: gluon::Handler + Send + Sync + 'static {
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
#[derive(Debug, Clone)]
pub struct SkyInterface {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for SkyInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(SkyInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for SkyInterface {
    const ID: &'static str = "org.stardustxr.Sky.SkyInterface";
}
impl SkyInterface {
    /**Set the sky texture to a given equirectagular texture.
Returns None if the sky texture is already set.*/
    pub async fn set_sky_tex(
        &self,
        tex: impl Into<super::types::Resource>,
        opaque: impl Into<bool>,
    ) -> Result<Option<SkyGuard>, gluon::SendError> {
        let tex: super::types::Resource = tex.into();
        let opaque: bool = opaque.into();
        tracing::trace!(
            interface = "SkyInterface", method = "set_sky_tex", ? tex, ? opaque, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        tex.write(&mut gluon_builder)?;
        opaque.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_guard = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "SkyInterface", method = "set_sky_tex", ? __ret_guard, "←"
        );
        Ok(__ret_guard)
    }
    /**Set the sky lighting to a given equirectagular texture, supports HDRI images.
Returns None if the sky lighting is already set.*/
    pub async fn set_sky_light(
        &self,
        tex: impl Into<super::types::Resource>,
    ) -> Result<Option<SkyGuard>, gluon::SendError> {
        let tex: super::types::Resource = tex.into();
        tracing::trace!(
            interface = "SkyInterface", method = "set_sky_light", ? tex, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        tex.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_guard = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "SkyInterface", method = "set_sky_light", ? __ret_guard, "←"
        );
        Ok(__ret_guard)
    }
    pub fn from_handler<H: SkyInterfaceHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> SkyInterface {
        SkyInterface::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> SkyInterface {
        SkyInterface { obj }
    }
}
impl From<SkyInterface> for gluon::ObjectOrRef {
    fn from(value: SkyInterface) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for SkyInterface {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl gluon::Liveness for SkyInterface {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for SkyInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for SkyInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for SkyInterface {}
pub trait SkyInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    /**Set the sky texture to a given equirectagular texture.
Returns None if the sky texture is already set.*/
    fn set_sky_tex(
        &self,
        _ctx: gluon::Context,
        tex: super::types::Resource,
        opaque: bool,
    ) -> impl Future<Output = Option<SkyGuard>> + Send + Sync;
    ///Dispatched instead of [`Self::set_sky_tex`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `set_sky_tex` and sends the result through `reply`. Override this method instead of `set_sky_tex` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn set_sky_tex_oneway(
        &self,
        _ctx: gluon::Context,
        tex: super::types::Resource,
        opaque: bool,
        reply: gluon::ReplySender<Option<SkyGuard>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let guard = self.set_sky_tex(_ctx, tex, opaque).await;
            reply.send(guard)
        }
    }
    /**Set the sky lighting to a given equirectagular texture, supports HDRI images.
Returns None if the sky lighting is already set.*/
    fn set_sky_light(
        &self,
        _ctx: gluon::Context,
        tex: super::types::Resource,
    ) -> impl Future<Output = Option<SkyGuard>> + Send + Sync;
    ///Dispatched instead of [`Self::set_sky_light`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `set_sky_light` and sends the result through `reply`. Override this method instead of `set_sky_light` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn set_sky_light_oneway(
        &self,
        _ctx: gluon::Context,
        tex: super::types::Resource,
        reply: gluon::ReplySender<Option<SkyGuard>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let guard = self.set_sky_light(_ctx, tex).await;
            reply.send(guard)
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
                    let param_tex = gluon::Convertable::read(&mut gluon_data)?;
                    let param_opaque = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SkyInterface", method = "set_sky_tex", ? param_tex,
                        ? param_opaque, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Option<SkyGuard>> = gluon::ReplySender::new(
                        return_callback,
                        |guard, gluon_out| {
                            tracing::trace!(
                                interface = "SkyInterface", method = "set_sky_tex", ? guard,
                                "←"
                            );
                            guard.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.set_sky_tex_oneway(ctx, param_tex, param_opaque, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "SkyInterface", method =
                                "set_sky_tex", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let param_tex = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "SkyInterface", method = "set_sky_light", ?
                        param_tex, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Option<SkyGuard>> = gluon::ReplySender::new(
                        return_callback,
                        |guard, gluon_out| {
                            tracing::trace!(
                                interface = "SkyInterface", method = "set_sky_light", ?
                                guard, "←"
                            );
                            guard.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.set_sky_light_oneway(ctx, param_tex, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "SkyInterface", method =
                                "set_sky_light", method_id = 9u32
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
