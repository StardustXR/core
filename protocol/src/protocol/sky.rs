#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Sky",
    types: &[],
};
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
impl SkyGuard {
    pub fn from_handler(obj: &impl gluon::OwnedObjectRef) -> SkyGuard {
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
impl SkyInterface {
    /**Set the sky texture to a given equirectagular texture.
Returns None if the sky texture is already set.*/
    pub async fn set_sky_tex(
        &self,
        tex: impl Into<super::types::Resource>,
    ) -> Result<Option<SkyGuard>, gluon::SendError> {
        let tex: super::types::Resource = tex.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        tex.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    /**Set the sky lighting to a given equirectagular texture, supports HDRI images.
Returns None if the sky lighting is already set.*/
    pub async fn set_sky_light(
        &self,
        tex: impl Into<super::types::Resource>,
    ) -> Result<Option<SkyGuard>, gluon::SendError> {
        let tex: super::types::Resource = tex.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        tex.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn from_handler(obj: &impl gluon::OwnedObjectRef) -> SkyInterface {
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
    ) -> impl Future<Output = Option<SkyGuard>> + Send + Sync;
    /**Set the sky lighting to a given equirectagular texture, supports HDRI images.
Returns None if the sky lighting is already set.*/
    fn set_sky_light(
        &self,
        _ctx: gluon::Context,
        tex: super::types::Resource,
    ) -> impl Future<Output = Option<SkyGuard>> + Send + Sync;
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
                    let param_tex = gluon::Convertable::read(&mut gluon_data)?;
                    let (guard) = self.set_sky_tex(ctx, param_tex).await;
                    drop(gluon_data);
                    guard.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_tex = gluon::Convertable::read(&mut gluon_data)?;
                    let (guard) = self.set_sky_light(ctx, param_tex).await;
                    drop(gluon_data);
                    guard.write_owned(&mut gluon_out)?;
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
