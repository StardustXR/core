#![allow(
    unused,
    clippy::single_match,
    clippy::match_single_binding,
    clippy::large_enum_variant
)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Sky",
    types: &[],
};
#[derive(Debug, Clone)]
pub struct SkyGuard {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for SkyGuard {
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
        Ok(SkyGuard::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl SkyGuard {
    pub fn from_handler<H: SkyGuardHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> SkyGuard {
        SkyGuard::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> SkyGuard {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new(&obj));
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        SkyGuard { obj, drop_notification }
    }
    pub fn death_or_drop(&self) -> impl Future<Output = ()> + Send + Sync + 'static {
        let death_notification_future = match &self.obj {
            binderbinder::binder_object::BinderObjectOrRef::Ref(r) => {
                Some(r.death_notification())
            }
            binderbinder::binder_object::BinderObjectOrRef::WeakRef(r) => {
                Some(r.death_notification())
            }
            _ => None,
        };
        let drop_notification = self.drop_notification.clone();
        async move {
            if let Some(death) = death_notification_future {
                tokio::select! {
                    _ = death => {} _ = drop_notification.wait() => {}
                }
            } else {
                drop_notification.wait().await;
            }
        }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for SkyGuard {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait SkyGuardHandler: binderbinder::device::TransactionHandler<
        ObjectResource = tokio::sync::RwLock<
            std::collections::HashMap<u64, gluon_wire::drop_tracking::DropNotifier>,
        >,
    > + Send + Sync + 'static {
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
        obj_res: &Self::ObjectResource,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                4 => {
                    use std::hash::BuildHasher as _;
                    let Ok(obj) = gluon_data.read_binder() else {
                        return Ok(out);
                    };
                    let hash = std::hash::RandomState::new().hash_one(obj.clone());
                    if out.write_u64(hash).is_err() {
                        return Ok(out);
                    }
                    obj_res
                        .write()
                        .await
                        .insert(
                            hash,
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        );
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
        obj_res: &Self::ObjectResource,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(id) = gluon_data.read_u64() else {
                        return Ok(());
                    };
                    if let Some(mut obj) = obj_res.write().await.remove(&id) {
                        obj.abort();
                    }
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct SkyInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for SkyInterface {
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
        Ok(SkyInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl SkyInterface {
    /**Set the sky texture to a given equirectagular texture.
Returns None if the sky texture is already set.*/
    pub async fn set_sky_tex(
        &self,
        tex: super::types::Resource,
    ) -> Result<Option<SkyGuard>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.set_sky_tex_blocking(tex))
            .await
            .unwrap()
    }
    pub fn set_sky_tex_blocking(
        &self,
        tex: super::types::Resource,
    ) -> Result<Option<SkyGuard>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        tex.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    /**Set the sky lighting to a given equirectagular texture, supports HDRI images.
Returns None if the sky lighting is already set.*/
    pub async fn set_sky_light(
        &self,
        tex: super::types::Resource,
    ) -> Result<Option<SkyGuard>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.set_sky_light_blocking(tex))
            .await
            .unwrap()
    }
    pub fn set_sky_light_blocking(
        &self,
        tex: super::types::Resource,
    ) -> Result<Option<SkyGuard>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        tex.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: SkyInterfaceHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> SkyInterface {
        SkyInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> SkyInterface {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new(&obj));
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        SkyInterface {
            obj,
            drop_notification,
        }
    }
    pub fn death_or_drop(&self) -> impl Future<Output = ()> + Send + Sync + 'static {
        let death_notification_future = match &self.obj {
            binderbinder::binder_object::BinderObjectOrRef::Ref(r) => {
                Some(r.death_notification())
            }
            binderbinder::binder_object::BinderObjectOrRef::WeakRef(r) => {
                Some(r.death_notification())
            }
            _ => None,
        };
        let drop_notification = self.drop_notification.clone();
        async move {
            if let Some(death) = death_notification_future {
                tokio::select! {
                    _ = death => {} _ = drop_notification.wait() => {}
                }
            } else {
                drop_notification.wait().await;
            }
        }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for SkyInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
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
pub trait SkyInterfaceHandler: binderbinder::device::TransactionHandler<
        ObjectResource = tokio::sync::RwLock<
            std::collections::HashMap<u64, gluon_wire::drop_tracking::DropNotifier>,
        >,
    > + Send + Sync + 'static {
    /**Set the sky texture to a given equirectagular texture.
Returns None if the sky texture is already set.*/
    fn set_sky_tex(
        &self,
        _ctx: gluon_wire::GluonCtx,
        tex: super::types::Resource,
    ) -> impl Future<Output = Option<SkyGuard>> + Send + Sync;
    /**Set the sky lighting to a given equirectagular texture, supports HDRI images.
Returns None if the sky lighting is already set.*/
    fn set_sky_light(
        &self,
        _ctx: gluon_wire::GluonCtx,
        tex: super::types::Resource,
    ) -> impl Future<Output = Option<SkyGuard>> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
        obj_res: &Self::ObjectResource,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                4 => {
                    use std::hash::BuildHasher as _;
                    let Ok(obj) = gluon_data.read_binder() else {
                        return Ok(out);
                    };
                    let hash = std::hash::RandomState::new().hash_one(obj.clone());
                    if out.write_u64(hash).is_err() {
                        return Ok(out);
                    }
                    obj_res
                        .write()
                        .await
                        .insert(
                            hash,
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        );
                }
                8u32 => {
                    let (guard) = self
                        .set_sky_tex(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    guard.write_owned(&mut out)?;
                }
                9u32 => {
                    let (guard) = self
                        .set_sky_light(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    guard.write_owned(&mut out)?;
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
        obj_res: &Self::ObjectResource,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(id) = gluon_data.read_u64() else {
                        return Ok(());
                    };
                    if let Some(mut obj) = obj_res.write().await.remove(&id) {
                        obj.abort();
                    }
                }
                _ => {}
            }
            Ok(())
        }
    }
}
