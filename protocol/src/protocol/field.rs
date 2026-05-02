#![allow(
    unused,
    clippy::single_match,
    clippy::match_single_binding,
    clippy::large_enum_variant
)]
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handle_in.write(gluon_data)?;
        self.anchor.write(gluon_data)?;
        self.handle_out.write(gluon_data)?;
        self.thickness.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let handle_in = gluon_wire::GluonConvertable::read(gluon_data)?;
        let anchor = gluon_wire::GluonConvertable::read(gluon_data)?;
        let handle_out = gluon_wire::GluonConvertable::read(gluon_data)?;
        let thickness = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(CubicBezierControlPoint {
            handle_in,
            anchor,
            handle_out,
            thickness,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.handle_in.write_owned(gluon_data)?;
        self.anchor.write_owned(gluon_data)?;
        self.handle_out.write_owned(gluon_data)?;
        self.thickness.write_owned(gluon_data)?;
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
    pub ray_length: f32,
    ///Number of steps taken
    pub ray_steps: u32,
}
impl gluon_wire::GluonConvertable for RayMarchResult {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.min_distance.write(gluon_data)?;
        self.deepest_point_distance.write(gluon_data)?;
        self.ray_length.write(gluon_data)?;
        self.ray_steps.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let min_distance = gluon_wire::GluonConvertable::read(gluon_data)?;
        let deepest_point_distance = gluon_wire::GluonConvertable::read(gluon_data)?;
        let ray_length = gluon_wire::GluonConvertable::read(gluon_data)?;
        let ray_steps = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(RayMarchResult {
            min_distance,
            deepest_point_distance,
            ray_length,
            ray_steps,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.min_distance.write_owned(gluon_data)?;
        self.deepest_point_distance.write_owned(gluon_data)?;
        self.ray_length.write_owned(gluon_data)?;
        self.ray_steps.write_owned(gluon_data)?;
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
        length: f32,
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            Shape::Box { size } => {
                gluon_data.write_u16(0u16)?;
                size.write(gluon_data)?;
            }
            Shape::Sphere { radius } => {
                gluon_data.write_u16(1u16)?;
                radius.write(gluon_data)?;
            }
            Shape::Cylinder { length, radius } => {
                gluon_data.write_u16(2u16)?;
                length.write(gluon_data)?;
                radius.write(gluon_data)?;
            }
            Shape::Torus { major_radius, minor_radius } => {
                gluon_data.write_u16(3u16)?;
                major_radius.write(gluon_data)?;
                minor_radius.write(gluon_data)?;
            }
            Shape::CubicBezierSpline { points, cyclic } => {
                gluon_data.write_u16(4u16)?;
                points.write(gluon_data)?;
                cyclic.write(gluon_data)?;
            }
        };
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => {
                    let size = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::Box { size }
                }
                1u16 => {
                    let radius = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::Sphere { radius }
                }
                2u16 => {
                    let length = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let radius = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::Cylinder { length, radius }
                }
                3u16 => {
                    let major_radius = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let minor_radius = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::Torus {
                        major_radius,
                        minor_radius,
                    }
                }
                4u16 => {
                    let points = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let cyclic = gluon_wire::GluonConvertable::read(gluon_data)?;
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            Shape::Box { size } => {
                gluon_data.write_u16(0u16)?;
                size.write_owned(gluon_data)?;
            }
            Shape::Sphere { radius } => {
                gluon_data.write_u16(1u16)?;
                radius.write_owned(gluon_data)?;
            }
            Shape::Cylinder { length, radius } => {
                gluon_data.write_u16(2u16)?;
                length.write_owned(gluon_data)?;
                radius.write_owned(gluon_data)?;
            }
            Shape::Torus { major_radius, minor_radius } => {
                gluon_data.write_u16(3u16)?;
                major_radius.write_owned(gluon_data)?;
                minor_radius.write_owned(gluon_data)?;
            }
            Shape::CubicBezierSpline { points, cyclic } => {
                gluon_data.write_u16(4u16)?;
                points.write_owned(gluon_data)?;
                cyclic.write_owned(gluon_data)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct FieldRef {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for FieldRef {
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
        Ok(FieldRef::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl FieldRef {
    pub fn from_handler<H: FieldRefHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        FieldRef { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for FieldRef {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for FieldRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for FieldRef {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for FieldRef {}
pub trait FieldRefHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon_wire::GluonDataReader,
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
#[derive(Debug, Clone)]
pub struct Field {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for Field {
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
        Ok(Field::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Field {
    pub async fn field_ref(&self) -> Result<FieldRef, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn spatial(
        &self,
    ) -> Result<super::spatial::Spatial, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn distance(
        &self,
        reference_space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<f32>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        reference_space.write(&mut gluon_builder)?;
        point.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn normal(
        &self,
        reference_space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        reference_space.write(&mut gluon_builder)?;
        point.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn closest_point(
        &self,
        reference_space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        reference_space.write(&mut gluon_builder)?;
        point.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn ray_march(
        &self,
        reference_space: super::spatial::SpatialRef,
        ray_origin: super::types::Vec3F,
        ray_direction: super::types::Vec3F,
    ) -> Result<Option<RayMarchResult>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        reference_space.write(&mut gluon_builder)?;
        ray_origin.write(&mut gluon_builder)?;
        ray_direction.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 13u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn set_shape(&self, shape: Shape) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        shape.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 14u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: FieldHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        Field { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for Field {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for Field {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Field {}
pub trait FieldHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn field_ref(
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
        reference_space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> impl Future<Output = Option<f32>> + Send + Sync;
    fn normal(
        &self,
        _ctx: gluon_wire::GluonCtx,
        reference_space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> impl Future<Output = Option<super::types::Vec3F>> + Send + Sync;
    fn closest_point(
        &self,
        _ctx: gluon_wire::GluonCtx,
        reference_space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> impl Future<Output = Option<super::types::Vec3F>> + Send + Sync;
    fn ray_march(
        &self,
        _ctx: gluon_wire::GluonCtx,
        reference_space: super::spatial::SpatialRef,
        ray_origin: super::types::Vec3F,
        ray_direction: super::types::Vec3F,
    ) -> impl Future<Output = Option<RayMarchResult>> + Send + Sync;
    fn set_shape(
        &self,
        _ctx: gluon_wire::GluonCtx,
        shape: Shape,
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
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (field) = self.field_ref(ctx).await;
                    drop(gluon_data);
                    field.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (spatial) = self.spatial(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_reference_space = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_point = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (distance) = self
                        .distance(ctx, param_reference_space, param_point)
                        .await;
                    drop(gluon_data);
                    distance.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                11u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_reference_space = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_point = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (normal) = self
                        .normal(ctx, param_reference_space, param_point)
                        .await;
                    drop(gluon_data);
                    normal.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                12u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_reference_space = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_point = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (point) = self
                        .closest_point(ctx, param_reference_space, param_point)
                        .await;
                    drop(gluon_data);
                    point.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                13u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_reference_space = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_ray_origin = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_ray_direction = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (result) = self
                        .ray_march(
                            ctx,
                            param_reference_space,
                            param_ray_origin,
                            param_ray_direction,
                        )
                        .await;
                    drop(gluon_data);
                    result.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                14u32 => {
                    let param_shape = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_shape(ctx, param_shape).await;
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
}
impl gluon_wire::GluonConvertable for FieldInterface {
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
        Ok(FieldInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl FieldInterface {
    pub async fn distance(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<f32>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        field.write(&mut gluon_builder)?;
        space.write(&mut gluon_builder)?;
        point.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn normal(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        field.write(&mut gluon_builder)?;
        space.write(&mut gluon_builder)?;
        point.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn closest_point(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: super::types::Vec3F,
    ) -> Result<Option<super::types::Vec3F>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        field.write(&mut gluon_builder)?;
        space.write(&mut gluon_builder)?;
        point.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn ray_march(
        &self,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        ray_origin: super::types::Vec3F,
        ray_direction: super::types::Vec3F,
    ) -> Result<Option<RayMarchResult>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        field.write(&mut gluon_builder)?;
        space.write(&mut gluon_builder)?;
        ray_origin.write(&mut gluon_builder)?;
        ray_direction.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn create_field(
        &self,
        spatial: super::spatial::Spatial,
        shape: Shape,
    ) -> Result<Field, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        shape.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: FieldInterfaceHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        FieldInterface { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for FieldInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for FieldInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for FieldInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for FieldInterface {}
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
        spatial: super::spatial::Spatial,
        shape: Shape,
    ) -> impl Future<Output = Field> + Send + Sync;
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
                    let param_field = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_space = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_point = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (distance) = self
                        .distance(ctx, param_field, param_space, param_point)
                        .await;
                    drop(gluon_data);
                    distance.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_field = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_space = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_point = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (normal) = self
                        .normal(ctx, param_field, param_space, param_point)
                        .await;
                    drop(gluon_data);
                    normal.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_field = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_space = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_point = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (point) = self
                        .closest_point(ctx, param_field, param_space, param_point)
                        .await;
                    drop(gluon_data);
                    point.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                11u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_field = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_space = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_ray_origin = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_ray_direction = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (result) = self
                        .ray_march(
                            ctx,
                            param_field,
                            param_space,
                            param_ray_origin,
                            param_ray_direction,
                        )
                        .await;
                    drop(gluon_data);
                    result.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                12u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_spatial = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_shape = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (field) = self
                        .create_field(ctx, param_spatial, param_shape)
                        .await;
                    drop(gluon_data);
                    field.write_owned(&mut gluon_out)?;
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
