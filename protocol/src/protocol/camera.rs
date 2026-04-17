#![allow(
    unused,
    clippy::single_match,
    clippy::match_single_binding,
    clippy::large_enum_variant
)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Camera",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "View",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
    ],
};
///A single viewpoint for a camera
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct View {
    ///Right-handed colum major projection matrix with a 1..0 (Reversed Z) depth range, where the Y axis == Up
    pub projection_matrix: super::types::Mat4F,
    ///Transform applied to the view, relative to the camera
    pub camera_relative_transform: super::spatial::Transform,
}
impl gluon_wire::GluonConvertable for View {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.projection_matrix.write(gluon_data)?;
        self.camera_relative_transform.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let projection_matrix = gluon_wire::GluonConvertable::read(gluon_data)?;
        let camera_relative_transform = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(View {
            projection_matrix,
            camera_relative_transform,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.projection_matrix.write_owned(gluon_data)?;
        self.camera_relative_transform.write_owned(gluon_data)?;
        Ok(())
    }
}
#[derive(Debug)]
pub struct CameraInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: binderbinder::binder_object::BinderObject<
        gluon_wire::drop_tracking::DropNotifiedHandler,
    >,
    drop_handler: std::sync::Arc<gluon_wire::drop_tracking::DropNotifiedHandler>,
}
impl Clone for CameraInterface {
    fn clone(&self) -> Self {
        CameraInterface::from_object_or_ref(self.obj.clone())
    }
}
impl gluon_wire::GluonConvertable for CameraInterface {
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
        Ok(CameraInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl CameraInterface {
    pub async fn create_camera(
        &self,
        spatial: super::spatial::Spatial,
    ) -> Result<Camera, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.create_camera_blocking(spatial))
            .await
            .unwrap()
    }
    pub fn create_camera_blocking(
        &self,
        spatial: super::spatial::Spatial,
    ) -> Result<Camera, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        spatial.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: CameraInterfaceHandler>(
        obj: &binderbinder::binder_object::BinderObject<H>,
    ) -> CameraInterface {
        CameraInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> CameraInterface {
        let drop_handler = gluon_wire::drop_tracking::DropNotifiedHandler::new(
            obj.clone(),
        );
        let drop_notification = obj.device().register_object(drop_handler.clone());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        CameraInterface {
            obj,
            drop_notification,
            drop_handler,
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
        let drop_handler = self.drop_handler.clone();
        async move {
            if let Some(death) = death_notification_future {
                tokio::select! {
                    _ = death => {} _ = drop_handler.wait() => {}
                }
            } else {
                drop_handler.wait().await;
            }
        }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for CameraInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for CameraInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for CameraInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for CameraInterface {}
pub trait CameraInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn create_camera(
        &self,
        _ctx: gluon_wire::GluonCtx,
        spatial: super::spatial::Spatial,
    ) -> impl Future<Output = Camera> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
                8u32 => {
                    let (camera) = self
                        .create_camera(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    camera.write_owned(&mut out)?;
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
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug)]
pub struct Camera {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: binderbinder::binder_object::BinderObject<
        gluon_wire::drop_tracking::DropNotifiedHandler,
    >,
    drop_handler: std::sync::Arc<gluon_wire::drop_tracking::DropNotifiedHandler>,
}
impl Clone for Camera {
    fn clone(&self) -> Self {
        Camera::from_object_or_ref(self.obj.clone())
    }
}
impl gluon_wire::GluonConvertable for Camera {
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
        Ok(Camera::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Camera {
    ///Request that the server renders this camera, the number of views has to match the array layer count in the dmatex, or one view if the dmatex has no array layers
    pub fn request_draw(
        &self,
        render_target: super::dmatex::DmatexRef,
        acquire_point: u64,
        release_point: u64,
        views: Vec<View>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        render_target.write(&mut gluon_builder)?;
        acquire_point.write(&mut gluon_builder)?;
        release_point.write(&mut gluon_builder)?;
        views.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: CameraHandler>(
        obj: &binderbinder::binder_object::BinderObject<H>,
    ) -> Camera {
        Camera::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Camera {
        let drop_handler = gluon_wire::drop_tracking::DropNotifiedHandler::new(
            obj.clone(),
        );
        let drop_notification = obj.device().register_object(drop_handler.clone());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        Camera {
            obj,
            drop_notification,
            drop_handler,
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
        let drop_handler = self.drop_handler.clone();
        async move {
            if let Some(death) = death_notification_future {
                tokio::select! {
                    _ = death => {} _ = drop_handler.wait() => {}
                }
            } else {
                drop_handler.wait().await;
            }
        }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for Camera {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for Camera {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Camera {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Camera {}
pub trait CameraHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Request that the server renders this camera, the number of views has to match the array layer count in the dmatex, or one view if the dmatex has no array layers
    fn request_draw(
        &self,
        _ctx: gluon_wire::GluonCtx,
        render_target: super::dmatex::DmatexRef,
        acquire_point: u64,
        release_point: u64,
        views: Vec<View>,
    );
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<
        Output = Result<
            gluon_wire::GluonDataBuilder<'static>,
            gluon_wire::GluonSendError,
        >,
    > + Send + Sync {
        async move {
            let mut out = gluon_wire::GluonDataBuilder::new();
            match transaction_code {
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
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    self.request_draw(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                _ => {}
            }
            Ok(())
        }
    }
}
