#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Audio",
    types: &[],
};
pub mod proxies {
    use super::*;
}
#[derive(Debug, Clone)]
pub struct Sound {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for Sound {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(Sound::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Sound {
    ///Play sound effect
    pub fn play(&self) -> Result<(), gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///Stop sound effect
    pub fn stop(&self) -> Result<(), gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: SoundHandler>(obj: &impl gluon::OwnedObjectRef<H>) -> Sound {
        Sound::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> Sound {
        Sound { obj }
    }
}
impl From<Sound> for gluon::ObjectOrRef {
    fn from(value: Sound) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for Sound {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait SoundHandler: gluon::Handler + Send + Sync + 'static {
    ///Play sound effect
    fn play(&self, _ctx: gluon::Context) -> impl Future<Output = ()> + Send + Sync;
    ///Stop sound effect
    fn stop(&self, _ctx: gluon::Context) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    drop(gluon_data);
                    self.play(ctx).await;
                }
                9u32 => {
                    drop(gluon_data);
                    self.stop(ctx).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct AudioInterface {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for AudioInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(AudioInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl AudioInterface {
    pub async fn create_sound(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        sound: impl Into<super::types::Resource>,
    ) -> Result<Sound, gluon::SendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let sound: super::types::Resource = sound.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        sound.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn from_handler<H: AudioInterfaceHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> AudioInterface {
        AudioInterface::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> AudioInterface {
        AudioInterface { obj }
    }
}
impl From<AudioInterface> for gluon::ObjectOrRef {
    fn from(value: AudioInterface) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for AudioInterface {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait AudioInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    fn create_sound(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        sound: super::types::Resource,
    ) -> impl Future<Output = Sound> + Send + Sync;
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
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_sound = gluon::Convertable::read(&mut gluon_data)?;
                    let (sound) = self
                        .create_sound(ctx, param_spatial, param_sound)
                        .await;
                    drop(gluon_data);
                    sound.write_owned(&mut gluon_out)?;
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
pub mod proxied {
    use super::*;
}
