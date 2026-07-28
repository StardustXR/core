#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Tracked",
    types: &[],
};
pub mod proxies {
    use super::*;
}
#[derive(Debug, Clone)]
pub struct Tracked {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for Tracked {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(Tracked::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Tracked {
    pub async fn get(
        &self,
        handler: impl Into<TrackedStateReceiver>,
    ) -> Result<(super::spatial::SpatialRef, TrackedGuard, bool), gluon::SendError> {
        let handler: TrackedStateReceiver = handler.into();
        tracing::trace!(interface = "Tracked", method = "get", ? handler, "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        handler.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        let __ret_guard = gluon::Convertable::read(&mut reader)?;
        let __ret_tracked = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Tracked", method = "get", ? __ret_spatial, ? __ret_guard, ?
            __ret_tracked, "←"
        );
        Ok((__ret_spatial, __ret_guard, __ret_tracked))
    }
    pub async fn get_pose(
        &self,
        at: impl Into<super::types::Timestamp>,
        relative_to: impl Into<super::spatial::SpatialRef>,
    ) -> Result<(Option<super::types::Posef>, bool), gluon::SendError> {
        let at: super::types::Timestamp = at.into();
        let relative_to: super::spatial::SpatialRef = relative_to.into();
        tracing::trace!(
            interface = "Tracked", method = "get_pose", ? at, ? relative_to, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        at.write(&mut gluon_builder)?;
        relative_to.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        let __ret_pose = gluon::Convertable::read(&mut reader)?;
        let __ret_tracked = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Tracked", method = "get_pose", ? __ret_pose, ? __ret_tracked,
            "←"
        );
        Ok((__ret_pose, __ret_tracked))
    }
    pub fn from_handler<H: TrackedHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> Tracked {
        Tracked::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> Tracked {
        Tracked { obj }
    }
}
impl From<Tracked> for gluon::ObjectOrRef {
    fn from(value: Tracked) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for Tracked {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl gluon::Liveness for Tracked {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for Tracked {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Tracked {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Tracked {}
pub trait TrackedHandler: gluon::Handler + Send + Sync + 'static {
    fn get(
        &self,
        _ctx: gluon::Context,
        handler: TrackedStateReceiver,
    ) -> impl Future<
        Output = (super::spatial::SpatialRef, TrackedGuard, bool),
    > + Send + Sync;
    ///Dispatched instead of [`Self::get`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get` and sends the result through `reply`. Override this method instead of `get` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_oneway(
        &self,
        _ctx: gluon::Context,
        handler: TrackedStateReceiver,
        reply: gluon::ReplySender<(super::spatial::SpatialRef, TrackedGuard, bool)>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let (spatial, guard, tracked) = self.get(_ctx, handler).await;
            reply.send((spatial, guard, tracked))
        }
    }
    fn get_pose(
        &self,
        _ctx: gluon::Context,
        at: super::types::Timestamp,
        relative_to: super::spatial::SpatialRef,
    ) -> impl Future<Output = (Option<super::types::Posef>, bool)> + Send + Sync;
    ///Dispatched instead of [`Self::get_pose`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_pose` and sends the result through `reply`. Override this method instead of `get_pose` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_pose_oneway(
        &self,
        _ctx: gluon::Context,
        at: super::types::Timestamp,
        relative_to: super::spatial::SpatialRef,
        reply: gluon::ReplySender<(Option<super::types::Posef>, bool)>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let (pose, tracked) = self.get_pose(_ctx, at, relative_to).await;
            reply.send((pose, tracked))
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
                    let param_handler = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Tracked", method = "get", ? param_handler,
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        (super::spatial::SpatialRef, TrackedGuard, bool),
                    > = gluon::ReplySender::new(
                        return_callback,
                        |(spatial, guard, tracked), gluon_out| {
                            tracing::trace!(
                                interface = "Tracked", method = "get", ? spatial, ? guard, ?
                                tracked, "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            guard.write_owned(gluon_out)?;
                            tracked.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_oneway(ctx, param_handler, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Tracked", method = "get",
                                method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let param_at = gluon::Convertable::read(&mut gluon_data)?;
                    let param_relative_to = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Tracked", method = "get_pose", ? param_at, ?
                        param_relative_to, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<(Option<super::types::Posef>, bool)> = gluon::ReplySender::new(
                        return_callback,
                        |(pose, tracked), gluon_out| {
                            tracing::trace!(
                                interface = "Tracked", method = "get_pose", ? pose, ?
                                tracked, "←"
                            );
                            pose.write_owned(gluon_out)?;
                            tracked.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_pose_oneway(ctx, param_at, param_relative_to, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Tracked", method = "get_pose",
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
pub struct TrackedGuard {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for TrackedGuard {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(TrackedGuard::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl TrackedGuard {
    pub fn from_handler<H: TrackedGuardHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> TrackedGuard {
        TrackedGuard::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> TrackedGuard {
        TrackedGuard { obj }
    }
}
impl From<TrackedGuard> for gluon::ObjectOrRef {
    fn from(value: TrackedGuard) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for TrackedGuard {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl gluon::Liveness for TrackedGuard {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for TrackedGuard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for TrackedGuard {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for TrackedGuard {}
pub trait TrackedGuardHandler: gluon::Handler + Send + Sync + 'static {
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
pub struct TrackedStateReceiver {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for TrackedStateReceiver {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(TrackedStateReceiver::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl TrackedStateReceiver {
    pub fn tracked(&self, tracked: impl Into<bool>) -> Result<(), gluon::SendError> {
        let tracked: bool = tracked.into();
        tracing::trace!(
            interface = "TrackedStateReceiver", method = "tracked", ? tracked, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        tracked.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: TrackedStateReceiverHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> TrackedStateReceiver {
        TrackedStateReceiver::from_object_or_ref(
            gluon::OwnedObjectRef::to_object_or_ref(obj),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> TrackedStateReceiver {
        TrackedStateReceiver { obj }
    }
}
impl From<TrackedStateReceiver> for gluon::ObjectOrRef {
    fn from(value: TrackedStateReceiver) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for TrackedStateReceiver {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
    }
}
impl gluon::Liveness for TrackedStateReceiver {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
    }
}
impl std::hash::Hash for TrackedStateReceiver {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for TrackedStateReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for TrackedStateReceiver {}
pub trait TrackedStateReceiverHandler: gluon::Handler + Send + Sync + 'static {
    fn tracked(
        &self,
        _ctx: gluon::Context,
        tracked: bool,
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
                    let param_tracked = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "TrackedStateReceiver", method = "tracked", ?
                        param_tracked, "dispatching"
                    );
                    drop(gluon_data);
                    self.tracked(ctx, param_tracked)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "TrackedStateReceiver", method =
                                "tracked", method_id = 8u32
                            ),
                        )
                        .await;
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
