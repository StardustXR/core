#![allow(unused, clippy::single_match, clippy::match_single_binding)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Field",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "CubicBezierControlPoint",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "RayMarchResult",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Shape",
            supported_derives: gluon_wire::Derives::from_bits_truncate(10u32),
        },
    ],
};
///Control point for cubic bezier spline
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CubicBezierControlPoint {
    pub handle_in: super::types::Vec3F,
    pub anchor: super::types::Vec3F,
    pub handle_out: super::types::Vec3F,
    ///Thickness of the spline tube at the point
    pub thickness: f32,
}
impl gluon_wire::GluonConvertable for CubicBezierControlPoint {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handle_in.write(data)?;
        self.anchor.write(data)?;
        self.handle_out.write(data)?;
        self.thickness.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let handle_in = gluon_wire::GluonConvertable::read(data)?;
        let anchor = gluon_wire::GluonConvertable::read(data)?;
        let handle_out = gluon_wire::GluonConvertable::read(data)?;
        let thickness = gluon_wire::GluonConvertable::read(data)?;
        Ok(CubicBezierControlPoint {
            handle_in,
            anchor,
            handle_out,
            thickness,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handle_in.write_owned(data)?;
        self.anchor.write_owned(data)?;
        self.handle_out.write_owned(data)?;
        self.thickness.write_owned(data)?;
        Ok(())
    }
}
///Results for a ray march against a signed distance field
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RayMarchResult {
    ///How close to or far inside the field the ray got. If less than zero, the ray intersected the field.
    pub min_distance: f32,
    ///The distance to the point on the ray that has the least distance to the field/most distance inside it. Useful for finding a "near miss" point or how close to the core of the field you're pointing.
    pub deepest_point_distance: f32,
    ///Maximum length of the ray
    pub ray_lenght: f32,
    ///Number of steps taken
    pub ray_steps: u32,
}
impl gluon_wire::GluonConvertable for RayMarchResult {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.min_distance.write(data)?;
        self.deepest_point_distance.write(data)?;
        self.ray_lenght.write(data)?;
        self.ray_steps.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let min_distance = gluon_wire::GluonConvertable::read(data)?;
        let deepest_point_distance = gluon_wire::GluonConvertable::read(data)?;
        let ray_lenght = gluon_wire::GluonConvertable::read(data)?;
        let ray_steps = gluon_wire::GluonConvertable::read(data)?;
        Ok(RayMarchResult {
            min_distance,
            deepest_point_distance,
            ray_lenght,
            ray_steps,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.min_distance.write_owned(data)?;
        self.deepest_point_distance.write_owned(data)?;
        self.ray_lenght.write_owned(data)?;
        self.ray_steps.write_owned(data)?;
        Ok(())
    }
}
///Shape for a signed distance field
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Box {
        ///Box size in meters
        size: super::types::Vec3F,
    },
    Sphere {
        ///Sphere radius in meters
        radius: f32,
    },
    ///Cylinder aligned to the XZ plane
    Cylinder {
        ///Length of the cylinder along the Y axis
        lenght: f32,
        ///Radius of the cylinder along the XZ plane
        radius: f32,
    },
    ///Torus aligned to the XZ plane
    Torus {
        ///Radius of the ring along the XZ plane
        major_radius: f32,
        ///Radius of the tube
        minor_radius: f32,
    },
    CubicBezierSpline {
        points: Vec<CubicBezierControlPoint>,
        ///Whether the spline is a closed loop
        cyclic: bool,
    },
}
impl gluon_wire::GluonConvertable for Shape {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            Shape::Box { size } => {
                data.write_u16(0u16)?;
                size.write(data)?;
            }
            Shape::Sphere { radius } => {
                data.write_u16(1u16)?;
                radius.write(data)?;
            }
            Shape::Cylinder { lenght, radius } => {
                data.write_u16(2u16)?;
                lenght.write(data)?;
                radius.write(data)?;
            }
            Shape::Torus { major_radius, minor_radius } => {
                data.write_u16(3u16)?;
                major_radius.write(data)?;
                minor_radius.write(data)?;
            }
            Shape::CubicBezierSpline { points, cyclic } => {
                data.write_u16(4u16)?;
                points.write(data)?;
                cyclic.write(data)?;
            }
        };
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match data.read_u16()? {
                0u16 => {
                    let size = gluon_wire::GluonConvertable::read(data)?;
                    Shape::Box { size }
                }
                1u16 => {
                    let radius = gluon_wire::GluonConvertable::read(data)?;
                    Shape::Sphere { radius }
                }
                2u16 => {
                    let lenght = gluon_wire::GluonConvertable::read(data)?;
                    let radius = gluon_wire::GluonConvertable::read(data)?;
                    Shape::Cylinder { lenght, radius }
                }
                3u16 => {
                    let major_radius = gluon_wire::GluonConvertable::read(data)?;
                    let minor_radius = gluon_wire::GluonConvertable::read(data)?;
                    Shape::Torus {
                        major_radius,
                        minor_radius,
                    }
                }
                4u16 => {
                    let points = gluon_wire::GluonConvertable::read(data)?;
                    let cyclic = gluon_wire::GluonConvertable::read(data)?;
                    Shape::CubicBezierSpline {
                        points,
                        cyclic,
                    }
                }
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            Shape::Box { size } => {
                data.write_u16(0u16)?;
                size.write_owned(data)?;
            }
            Shape::Sphere { radius } => {
                data.write_u16(1u16)?;
                radius.write_owned(data)?;
            }
            Shape::Cylinder { lenght, radius } => {
                data.write_u16(2u16)?;
                lenght.write_owned(data)?;
                radius.write_owned(data)?;
            }
            Shape::Torus { major_radius, minor_radius } => {
                data.write_u16(3u16)?;
                major_radius.write_owned(data)?;
                minor_radius.write_owned(data)?;
            }
            Shape::CubicBezierSpline { points, cyclic } => {
                data.write_u16(4u16)?;
                points.write_owned(data)?;
                cyclic.write_owned(data)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct FieldRef {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for FieldRef {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(data)
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(data)?;
        Ok(FieldRef::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl FieldRef {
    pub async fn spatial_ref(
        &self,
    ) -> Result<super::spatial::SpatialRef, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.spatial_ref_blocking()).await.unwrap()
    }
    pub fn spatial_ref_blocking(
        &self,
    ) -> Result<super::spatial::SpatialRef, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: FieldRefHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> FieldRef {
        FieldRef::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> FieldRef {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        FieldRef { obj, drop_notification }
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
impl binderbinder::binder_object::ToBinderObjectOrRef for FieldRef {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait FieldRefHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn spatial_ref(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::spatial::SpatialRef> + Send + Sync;
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
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
                    let (spatial_ref) = self.spatial_ref(ctx).await;
                    spatial_ref.write_owned(&mut out)?;
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct Field {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for Field {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(data)
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(data)?;
        Ok(Field::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl Field {
    pub async fn get_ref(&self) -> Result<FieldRef, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_ref_blocking()).await.unwrap()
    }
    pub fn get_ref_blocking(&self) -> Result<FieldRef, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn spatial(
        &self,
    ) -> Result<super::spatial::Spatial, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.spatial_blocking()).await.unwrap()
    }
    pub fn spatial_blocking(
        &self,
    ) -> Result<super::spatial::Spatial, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn distance(
        &self,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<f32>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.distance_blocking(space, point))
            .await
            .unwrap()
    }
    pub fn distance_blocking(
        &self,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<f32>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        space.write(&mut builder)?;
        point.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn normal(
        &self,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.normal_blocking(space, point))
            .await
            .unwrap()
    }
    pub fn normal_blocking(
        &self,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        space.write(&mut builder)?;
        point.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 11u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn closest_point(
        &self,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.closest_point_blocking(space, point))
            .await
            .unwrap()
    }
    pub fn closest_point_blocking(
        &self,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        space.write(&mut builder)?;
        point.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 12u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn ray_march(
        &self,
        space: super::spatial::SpatialRef,
        ray_origin: super::types::Vec3F,
        ray_direction: super::types::Vec3F,
    ) -> Result<Option<RayMarchResult>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.ray_march_blocking(space, ray_origin, ray_direction)
            })
            .await
            .unwrap()
    }
    pub fn ray_march_blocking(
        &self,
        space: super::spatial::SpatialRef,
        ray_origin: super::types::Vec3F,
        ray_direction: super::types::Vec3F,
    ) -> Result<Option<RayMarchResult>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        space.write(&mut builder)?;
        ray_origin.write(&mut builder)?;
        ray_direction.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 13u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn set_shape(&self, shape: Shape) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        shape.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 14u32, builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: FieldHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> Field {
        Field::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Field {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        Field { obj, drop_notification }
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
impl binderbinder::binder_object::ToBinderObjectOrRef for Field {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait FieldHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn get_ref(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = FieldRef> + Send + Sync;
    fn spatial(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::spatial::Spatial> + Send + Sync;
    fn distance(
        &self,
        _ctx: gluon_wire::GluonCtx,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> impl Future<Output = Option<f32>> + Send + Sync;
    fn normal(
        &self,
        _ctx: gluon_wire::GluonCtx,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> impl Future<Output = Option<super::types::Vec3F>> + Send + Sync;
    fn closest_point(
        &self,
        _ctx: gluon_wire::GluonCtx,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> impl Future<Output = Option<super::types::Vec3F>> + Send + Sync;
    fn ray_march(
        &self,
        _ctx: gluon_wire::GluonCtx,
        space: super::spatial::SpatialRef,
        ray_origin: super::types::Vec3F,
        ray_direction: super::types::Vec3F,
    ) -> impl Future<Output = Option<RayMarchResult>> + Send + Sync;
    fn set_shape(&self, _ctx: gluon_wire::GluonCtx, shape: Shape);
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
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
                    let (field) = self.get_ref(ctx).await;
                    field.write_owned(&mut out)?;
                }
                9u32 => {
                    let (spatial) = self.spatial(ctx).await;
                    spatial.write_owned(&mut out)?;
                }
                10u32 => {
                    let (distance) = self
                        .distance(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    distance.write_owned(&mut out)?;
                }
                11u32 => {
                    let (normal) = self
                        .normal(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    normal.write_owned(&mut out)?;
                }
                12u32 => {
                    let (point) = self
                        .closest_point(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    point.write_owned(&mut out)?;
                }
                13u32 => {
                    let (result) = self
                        .ray_march(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    result.write_owned(&mut out)?;
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                14u32 => {
                    self.set_shape(ctx, gluon_wire::GluonConvertable::read(data)?);
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct FieldInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for FieldInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write(data)
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let obj = binderbinder::binder_object::BinderObjectOrRef::read(data)?;
        Ok(FieldInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl FieldInterface {
    pub async fn distance(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<f32>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.distance_blocking(field, space, point))
            .await
            .unwrap()
    }
    pub fn distance_blocking(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<f32>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        field.write(&mut builder)?;
        space.write(&mut builder)?;
        point.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn normal(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.normal_blocking(field, space, point))
            .await
            .unwrap()
    }
    pub fn normal_blocking(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        field.write(&mut builder)?;
        space.write(&mut builder)?;
        point.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn closest_point(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.closest_point_blocking(field, space, point)
            })
            .await
            .unwrap()
    }
    pub fn closest_point_blocking(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        field.write(&mut builder)?;
        space.write(&mut builder)?;
        point.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn ray_march(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        ray_origin: super::types::Vec3F,
        ray_direction: super::types::Vec3F,
    ) -> Result<Option<RayMarchResult>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.ray_march_blocking(field, space, ray_origin, ray_direction)
            })
            .await
            .unwrap()
    }
    pub fn ray_march_blocking(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        ray_origin: super::types::Vec3F,
        ray_direction: super::types::Vec3F,
    ) -> Result<Option<RayMarchResult>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        field.write(&mut builder)?;
        space.write(&mut builder)?;
        ray_origin.write(&mut builder)?;
        ray_direction.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 11u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn create_field(
        &self,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        shape: Shape,
    ) -> Result<Field, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.create_field_blocking(parent, transform, shape)
            })
            .await
            .unwrap()
    }
    pub fn create_field_blocking(
        &self,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        shape: Shape,
    ) -> Result<Field, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        parent.write(&mut builder)?;
        transform.write(&mut builder)?;
        shape.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 12u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: FieldInterfaceHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> FieldInterface {
        FieldInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> FieldInterface {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        FieldInterface {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for FieldInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait FieldInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn distance(
        &self,
        _ctx: gluon_wire::GluonCtx,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> impl Future<Output = Option<f32>> + Send + Sync;
    fn normal(
        &self,
        _ctx: gluon_wire::GluonCtx,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> impl Future<Output = Option<super::types::Vec3F>> + Send + Sync;
    fn closest_point(
        &self,
        _ctx: gluon_wire::GluonCtx,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> impl Future<Output = Option<super::types::Vec3F>> + Send + Sync;
    fn ray_march(
        &self,
        _ctx: gluon_wire::GluonCtx,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        ray_origin: super::types::Vec3F,
        ray_direction: super::types::Vec3F,
    ) -> impl Future<Output = Option<RayMarchResult>> + Send + Sync;
    fn create_field(
        &self,
        _ctx: gluon_wire::GluonCtx,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        shape: Shape,
    ) -> impl Future<Output = Field> + Send + Sync;
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn dispatch_two_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
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
                    let (distance) = self
                        .distance(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    distance.write_owned(&mut out)?;
                }
                9u32 => {
                    let (normal) = self
                        .normal(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    normal.write_owned(&mut out)?;
                }
                10u32 => {
                    let (point) = self
                        .closest_point(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    point.write_owned(&mut out)?;
                }
                11u32 => {
                    let (result) = self
                        .ray_march(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    result.write_owned(&mut out)?;
                }
                12u32 => {
                    let (field) = self
                        .create_field(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    field.write_owned(&mut out)?;
                }
                _ => {}
            }
            Ok(out)
        }
    }
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                4 => {
                    let Ok(obj) = data.read_binder() else {
                        return Ok(());
                    };
                    self.drop_notification_requested(
                            gluon_wire::drop_tracking::DropNotifier::new(&obj),
                        )
                        .await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
