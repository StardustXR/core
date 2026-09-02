#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Camera",
    types: &[
        gluon::ExternalGluonType {
            name: "View",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///A single viewpoint for a camera
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct View {
    ///Right-handed colum major projection matrix with a 1..0 (Reversed Z) depth range, where the Y axis == Up and the X axis == Right
    pub projection_matrix: crate::types::Mat4F,
    ///Transform applied to the view, relative to the camera
    pub camera_relative_transform: super::spatial::Transform,
}
impl gluon::Convertable for View {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Mat4F = self
                .projection_matrix
                .clone()
                .into();
            __w.write_owned(gluon_data)?;
        }
        self.camera_relative_transform.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let projection_matrix: crate::types::Mat4F = {
            let __w: super::types::proxied::Mat4F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let camera_relative_transform = gluon::Convertable::read(gluon_data)?;
        Ok(View {
            projection_matrix,
            camera_relative_transform,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Mat4F = self.projection_matrix.into();
            __w.write_owned(gluon_data)?;
        }
        self.camera_relative_transform.write_owned(gluon_data)?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct CameraInterface {
    obj: gluon::Ref,
}
impl gluon::Convertable for CameraInterface {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(CameraInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl CameraInterface {
    const ID: &'static str = "org.stardustxr.Camera.CameraInterface";
}
impl gluon::Interface for CameraInterface {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: CameraInterfaceHandler> gluon::HandledBy<H> for CameraInterface {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type CameraInterfaceLocal<H> = gluon::LocalRef<CameraInterface, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: CameraInterfaceHandler> From<CameraInterfaceLocal<H>> for CameraInterface {
    fn from(value: CameraInterfaceLocal<H>) -> CameraInterface {
        value.into_proxy()
    }
}
impl gluon::RefExt for CameraInterface {
    fn from_ref(obj: gluon::Ref) -> CameraInterface {
        CameraInterface { obj }
    }
}
impl CameraInterface {
    pub async fn create_camera(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
    ) -> Result<Result<Camera, super::types::CreateError>, gluon::SendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        tracing::trace!(
            interface = "CameraInterface", method = "create_camera", ? spatial, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_camera = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "CameraInterface", method = "create_camera", ? __ret_camera,
            "←"
        );
        Ok(__ret_camera)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> CameraInterface {
        CameraInterface { obj }
    }
}
impl From<CameraInterface> for gluon::Ref {
    fn from(value: CameraInterface) -> Self {
        value.obj
    }
}
impl gluon::ToRef for CameraInterface {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for CameraInterface {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
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
pub trait CameraInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    fn create_camera(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
    ) -> impl Future<Output = Result<Camera, super::types::CreateError>> + Send + Sync;
    ///Dispatched instead of [`Self::create_camera`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `create_camera` and sends the result through `reply`. Override this method instead of `create_camera` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn create_camera_oneway(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        reply: gluon::ReplySender<Result<Camera, super::types::CreateError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let camera = self.create_camera(_ctx, spatial).await;
            reply.send(camera)
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
                    let return_callback = gluon_data.read_ref()?;
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "CameraInterface", method = "create_camera", ?
                        param_spatial, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<Camera, super::types::CreateError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |camera, gluon_out| {
                            tracing::trace!(
                                interface = "CameraInterface", method = "create_camera", ?
                                camera, "←"
                            );
                            camera.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.create_camera_oneway(ctx, param_spatial, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "CameraInterface", method =
                                "create_camera", method_id = 8u32
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
        (gluon::Node<Self>, gluon::LocalRef<CameraInterface, Self>),
        gluon::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon::RefExt;
        CameraInterface::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon::LocalRef<CameraInterface, Self>, gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        CameraInterface::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct Camera {
    obj: gluon::Ref,
}
impl gluon::Convertable for Camera {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(Camera::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Camera {
    const ID: &'static str = "org.stardustxr.Camera.Camera";
}
impl gluon::Interface for Camera {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: CameraHandler> gluon::HandledBy<H> for Camera {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type CameraLocal<H> = gluon::LocalRef<Camera, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: CameraHandler> From<CameraLocal<H>> for Camera {
    fn from(value: CameraLocal<H>) -> Camera {
        value.into_proxy()
    }
}
impl gluon::RefExt for Camera {
    fn from_ref(obj: gluon::Ref) -> Camera {
        Camera { obj }
    }
}
impl Camera {
    ///Request that the server renders this camera, the number of views has to match the array layer count in the dmatex, or one view if the dmatex has no array layers
    pub fn request_draw(
        &self,
        render_target: impl Into<super::dmatex::DmatexRef>,
        acquire_point: impl Into<u64>,
        release_point: impl Into<super::dmatex::DmatexSubmitRelease>,
        views: impl Into<Vec<View>>,
    ) -> Result<(), gluon::SendError> {
        let render_target: super::dmatex::DmatexRef = render_target.into();
        let acquire_point: u64 = acquire_point.into();
        let release_point: super::dmatex::DmatexSubmitRelease = release_point.into();
        let views: Vec<View> = views.into();
        tracing::trace!(
            interface = "Camera", method = "request_draw", ? render_target, ?
            acquire_point, ? release_point, ? views, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        render_target.write(&mut gluon_builder)?;
        acquire_point.write(&mut gluon_builder)?;
        release_point.write(&mut gluon_builder)?;
        views.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> Camera {
        Camera { obj }
    }
}
impl From<Camera> for gluon::Ref {
    fn from(value: Camera) -> Self {
        value.obj
    }
}
impl gluon::ToRef for Camera {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for Camera {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
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
pub trait CameraHandler: gluon::Handler + Send + Sync + 'static {
    ///Request that the server renders this camera, the number of views has to match the array layer count in the dmatex, or one view if the dmatex has no array layers
    fn request_draw(
        &self,
        _ctx: gluon::Context,
        render_target: super::dmatex::DmatexRef,
        acquire_point: u64,
        release_point: super::dmatex::DmatexSubmitRelease,
        views: Vec<View>,
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
                    let param_render_target = gluon::Convertable::read(&mut gluon_data)?;
                    let param_acquire_point = gluon::Convertable::read(&mut gluon_data)?;
                    let param_release_point = gluon::Convertable::read(&mut gluon_data)?;
                    let param_views = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Camera", method = "request_draw", ?
                        param_render_target, ? param_acquire_point, ?
                        param_release_point, ? param_views, "dispatching"
                    );
                    drop(gluon_data);
                    self.request_draw(
                            ctx,
                            param_render_target,
                            param_acquire_point,
                            param_release_point,
                            param_views,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Camera", method =
                                "request_draw", method_id = 8u32
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
    ) -> Result<(gluon::Node<Self>, gluon::LocalRef<Camera, Self>), gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        Camera::new_node(self)
    }
    fn to_service(self) -> Result<gluon::LocalRef<Camera, Self>, gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        Camera::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
