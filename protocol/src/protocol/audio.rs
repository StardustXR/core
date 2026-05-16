#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Audio",
    types: &[],
};
#[derive(Debug, Clone)]
pub struct Sound {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for Sound {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
        Ok(Sound::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Sound {
    ///Play sound effect
    pub fn play(&self) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///Stop sound effect
    pub fn stop(&self) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: SoundHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> Sound {
        Sound::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Sound {
        Sound { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for Sound {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait SoundHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Play sound effect
    fn play(&self, _ctx: gluon_wire::GluonCtx) -> impl Future<Output = ()> + Send + Sync;
    ///Stop sound effect
    fn stop(&self, _ctx: gluon_wire::GluonCtx) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
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
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for AudioInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(gluon_data)
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(gluon_data)?;
        Ok(AudioInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl AudioInterface {
    pub async fn create_sound(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        sound: impl Into<super::types::Resource>,
    ) -> Result<Sound, gluon_wire::GluonSendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let sound: super::types::Resource = sound.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        sound.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: AudioInterfaceHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> AudioInterface {
        AudioInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> AudioInterface {
        AudioInterface { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for AudioInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait AudioInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn create_sound(
        &self,
        _ctx: gluon_wire::GluonCtx,
        spatial: super::spatial::Spatial,
        sound: super::types::Resource,
    ) -> impl Future<Output = Sound> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_spatial = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_sound = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
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
