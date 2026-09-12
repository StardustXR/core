#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_ipc::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon_ipc::ExternalProtocol = gluon_ipc::ExternalProtocol {
    protocol_name: "org.stardustxr.Audio",
    types: &[],
};
pub mod proxies {
    use super::*;
}
#[derive(Debug, Clone)]
pub struct Sound {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for Sound {
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
        Ok(Sound::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Sound {
    const ID: &'static str = "org.stardustxr.Audio.Sound";
}
impl gluon_ipc::Interface for Sound {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: SoundHandler> gluon_ipc::HandledBy<H> for Sound {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type SoundLocal<H> = gluon_ipc::LocalRef<Sound, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: SoundHandler> From<SoundLocal<H>> for Sound {
    fn from(value: SoundLocal<H>) -> Sound {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for Sound {
    fn from_ref(obj: gluon_ipc::Ref) -> Sound {
        Sound { obj }
    }
}
impl Sound {
    ///Play sound effect
    pub fn play(&self) -> Result<(), gluon_ipc::SendError> {
        tracing::trace!(interface = "Sound", method = "play", "→");
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    ///Stop sound effect
    pub fn stop(&self) -> Result<(), gluon_ipc::SendError> {
        tracing::trace!(interface = "Sound", method = "stop", "→");
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        gluon_ipc::transact(&self.obj, 9u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> Sound {
        Sound { obj }
    }
}
impl From<Sound> for gluon_ipc::Ref {
    fn from(value: Sound) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for Sound {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for Sound {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for Sound {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Sound {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Sound {}
pub trait SoundHandler: gluon_ipc::Handler + Send + Sync + 'static {
    ///Play sound effect
    fn play(&self, _ctx: gluon_ipc::Context) -> impl Future<Output = ()> + Send + Sync;
    ///Stop sound effect
    fn stop(&self, _ctx: gluon_ipc::Context) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_ipc::DataReader,
        ctx: gluon_ipc::Context,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    tracing::trace!(interface = "Sound", method = "play", "dispatching");
                    drop(gluon_data);
                    self.play(ctx)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Sound", method = "play",
                                method_id = 8u32
                            ),
                        )
                        .await;
                }
                9u32 => {
                    tracing::trace!(interface = "Sound", method = "stop", "dispatching");
                    drop(gluon_data);
                    self.stop(ctx)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Sound", method = "stop",
                                method_id = 9u32
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
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<Sound, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        Sound::new_node(self)
    }
    fn to_service(self) -> Result<gluon_ipc::LocalRef<Sound, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        Sound::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct AudioInterface {
    obj: gluon_ipc::Ref,
}
impl gluon_ipc::Convertable for AudioInterface {
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
        Ok(AudioInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_ipc::DataBuilder,
    ) -> Result<(), gluon_ipc::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl AudioInterface {
    const ID: &'static str = "org.stardustxr.Audio.AudioInterface";
}
impl gluon_ipc::Interface for AudioInterface {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon_ipc::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: AudioInterfaceHandler> gluon_ipc::HandledBy<H> for AudioInterface {}
///A proxy this process made, carrying the handler behind it — see [`gluon_ipc::LocalRef`]. Handed back by [`gluon_ipc::RefExt::new_node`] and [`gluon_ipc::RefExt::new_service`].
pub type AudioInterfaceLocal<H> = gluon_ipc::LocalRef<AudioInterface, H>;
///Drops the handler share and keeps the proxy, so a [`gluon_ipc::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: AudioInterfaceHandler> From<AudioInterfaceLocal<H>> for AudioInterface {
    fn from(value: AudioInterfaceLocal<H>) -> AudioInterface {
        value.into_proxy()
    }
}
impl gluon_ipc::RefExt for AudioInterface {
    fn from_ref(obj: gluon_ipc::Ref) -> AudioInterface {
        AudioInterface { obj }
    }
}
impl AudioInterface {
    pub async fn create_sound(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        sound: impl Into<super::types::Resource>,
    ) -> Result<Result<Sound, super::types::ResourceLoadError>, gluon_ipc::SendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let sound: super::types::Resource = sound.into();
        tracing::trace!(
            interface = "AudioInterface", method = "create_sound", ? spatial, ? sound,
            "→"
        );
        let mut gluon_builder = gluon_ipc::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon_ipc::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        sound.write(&mut gluon_builder)?;
        gluon_ipc::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_sound = gluon_ipc::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "AudioInterface", method = "create_sound", ? __ret_sound, "←"
        );
        Ok(__ret_sound)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon_ipc::Ref) -> AudioInterface {
        AudioInterface { obj }
    }
}
impl From<AudioInterface> for gluon_ipc::Ref {
    fn from(value: AudioInterface) -> Self {
        value.obj
    }
}
impl gluon_ipc::ToRef for AudioInterface {
    fn to_ref(&self) -> gluon_ipc::Ref {
        self.obj.clone()
    }
}
impl gluon_ipc::Liveness for AudioInterface {
    fn death_notifier(&self) -> gluon_ipc::DeathNotifier {
        gluon_ipc::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for AudioInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for AudioInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for AudioInterface {}
pub trait AudioInterfaceHandler: gluon_ipc::Handler + Send + Sync + 'static {
    fn create_sound(
        &self,
        _ctx: gluon_ipc::Context,
        spatial: super::spatial::Spatial,
        sound: super::types::Resource,
    ) -> impl Future<
        Output = Result<Sound, super::types::ResourceLoadError>,
    > + Send + Sync;
    ///Dispatched instead of [`Self::create_sound`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `create_sound` and sends the result through `reply`. Override this method instead of `create_sound` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn create_sound_oneway(
        &self,
        _ctx: gluon_ipc::Context,
        spatial: super::spatial::Spatial,
        sound: super::types::Resource,
        reply: gluon_ipc::ReplySender<Result<Sound, super::types::ResourceLoadError>>,
    ) -> impl Future<Output = Result<(), gluon_ipc::SendError>> + Send + Sync {
        async move {
            let sound = self.create_sound(_ctx, spatial, sound).await;
            reply.send(sound)
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
                    let param_spatial = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    let param_sound = gluon_ipc::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "AudioInterface", method = "create_sound", ?
                        param_spatial, ? param_sound, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon_ipc::ReplySender<
                        Result<Sound, super::types::ResourceLoadError>,
                    > = gluon_ipc::ReplySender::new(
                        return_callback,
                        |sound, gluon_out| {
                            tracing::trace!(
                                interface = "AudioInterface", method = "create_sound", ?
                                sound, "←"
                            );
                            sound.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.create_sound_oneway(ctx, param_spatial, param_sound, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "AudioInterface", method =
                                "create_sound", method_id = 8u32
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
        (gluon_ipc::Node<Self>, gluon_ipc::LocalRef<AudioInterface, Self>),
        gluon_ipc::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        AudioInterface::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon_ipc::LocalRef<AudioInterface, Self>, gluon_ipc::NodeError>
    where
        Self: Sized,
    {
        use gluon_ipc::RefExt;
        AudioInterface::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
