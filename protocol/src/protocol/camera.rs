#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Camera",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "View",
            supported_derives: gluon_wire::Derives::from_bits_truncate(3u32),
        },
    ],
};
///A single viewpoint for a camera
#[derive(Debug, Copy, Clone)]
pub struct View {
    ///Right-handed colum major projection matrix with a 1..0 (Reversed Z) depth range, where the Y axis == Up
    pub projection_matrix: crate::types::Mat4F,
    ///Transform applied to the view, relative to the camera
    pub camera_relative_transform: super::spatial::Transform,
}
impl gluon_wire::GluonConvertable for View {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: super::types::Mat4F = self.projection_matrix.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.camera_relative_transform.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let projection_matrix: crate::types::Mat4F = {
            let __w: super::types::Mat4F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
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
        {
            let __w: super::types::Mat4F = self.projection_matrix.into();
            __w.write_owned(gluon_data)?;
        }
        self.camera_relative_transform.write_owned(gluon_data)?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct CameraInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
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
        spatial: impl Into<super::spatial::Spatial>,
    ) -> Result<Camera, gluon_wire::GluonSendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: CameraInterfaceHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        CameraInterface { obj }
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
                    let (camera) = self.create_camera(ctx, param_spatial).await;
                    drop(gluon_data);
                    camera.write_owned(&mut gluon_out)?;
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
pub struct Camera {
    obj: binderbinder::binder_object::BinderObjectOrRef,
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
        render_target: impl Into<super::dmatex::DmatexRef>,
        acquire_point: impl Into<u64>,
        release_point: impl Into<u64>,
        views: impl Into<Vec<View>>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let render_target: super::dmatex::DmatexRef = render_target.into();
        let acquire_point: u64 = acquire_point.into();
        let release_point: u64 = release_point.into();
        let views: Vec<View> = views.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        render_target.write(&mut gluon_builder)?;
        acquire_point.write(&mut gluon_builder)?;
        release_point.write(&mut gluon_builder)?;
        views.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: CameraHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        Camera { obj }
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
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let param_render_target = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_acquire_point = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_release_point = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_views = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.request_draw(
                            ctx,
                            param_render_target,
                            param_acquire_point,
                            param_release_point,
                            param_views,
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
