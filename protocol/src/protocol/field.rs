#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Field",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "FieldSample",
            supported_derives: gluon_wire::Derives::from_bits_truncate(3u32),
        },
        gluon_wire::ExternalGluonType {
            name: "RayMarchResult",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "CubicBezierControlPoint",
            supported_derives: gluon_wire::Derives::from_bits_truncate(3u32),
        },
        gluon_wire::ExternalGluonType {
            name: "Shape",
            supported_derives: gluon_wire::Derives::from_bits_truncate(2u32),
        },
    ],
};
///Information about the field at a sample point in space.
#[derive(Debug, Copy, Clone)]
pub struct FieldSample {
    ///Signed Euclidean distance: negative inside, positive outside.
    pub distance: f32,
    /**Unit outward surface normal at the closest point.
Always points away from the shape interior, regardless of whether the
query point is inside or outside.*/
    pub gradient: crate::types::Vec3F,
    ///Closest point on the shape boundary, in the same space as the query.
    pub closest_point: crate::types::Vec3F,
}
impl gluon_wire::GluonConvertable for FieldSample {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.distance.write(gluon_data)?;
        {
            let __w: super::types::Vec3F = self.gradient.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Vec3F = self.closest_point.clone().into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let distance = gluon_wire::GluonConvertable::read(gluon_data)?;
        let gradient: crate::types::Vec3F = {
            let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let closest_point: crate::types::Vec3F = {
            let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        Ok(FieldSample {
            distance,
            gradient,
            closest_point,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.distance.write_owned(gluon_data)?;
        {
            let __w: super::types::Vec3F = self.gradient.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Vec3F = self.closest_point.into();
            __w.write_owned(gluon_data)?;
        }
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
///Control point for cubic bezier spline
#[derive(Debug, Copy, Clone)]
pub struct CubicBezierControlPoint {
    pub handle_in: crate::types::Vec3F,
    pub anchor: crate::types::Vec3F,
    pub handle_out: crate::types::Vec3F,
    ///Thickness of the spline tube at the point
    pub thickness: f32,
}
impl gluon_wire::GluonConvertable for CubicBezierControlPoint {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: super::types::Vec3F = self.handle_in.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Vec3F = self.anchor.clone().into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Vec3F = self.handle_out.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.thickness.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let handle_in: crate::types::Vec3F = {
            let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let anchor: crate::types::Vec3F = {
            let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let handle_out: crate::types::Vec3F = {
            let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
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
        {
            let __w: super::types::Vec3F = self.handle_in.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Vec3F = self.anchor.into();
            __w.write_owned(gluon_data)?;
        }
        {
            let __w: super::types::Vec3F = self.handle_out.into();
            __w.write_owned(gluon_data)?;
        }
        self.thickness.write_owned(gluon_data)?;
        Ok(())
    }
}
///Shape for a signed distance field.
#[derive(Debug, Clone)]
pub enum Shape {
    ///Axis-aligned box.  `size` = full extents in metres.
    Box { size: crate::types::Vec3F },
    ///Sphere.
    Sphere { radius: f32 },
    /**Capsule aligned to the **Y** axis.
`length` = full length of the cylindrical section; hemispherical caps add `radius`.*/
    Capsule { length: f32, radius: f32 },
    /**Flat-capped cylinder aligned to the **Y** axis.
`length` = full length; `radius` in the XZ plane.*/
    Cylinder { length: f32, radius: f32 },
    ///Torus in the **XZ** plane.
    Torus { major_radius: f32, minor_radius: f32 },
    ///Variable-radius cubic Bézier tube.
    CubicBezierSpline {
        points: Vec<CubicBezierControlPoint>,
        ///Whether the spline is a closed loop
        cyclic: bool,
    },
    ///Affine transform wrapper: child is evaluated in local space.
    Transform { shape: Box<Shape>, transform: crate::types::Mat4F },
    ///Hard union (min) with routed-VDF interior.
    Union { shapes: Vec<Shape> },
    ///Smooth union (smooth-min).  `smoothing` = blend radius k.
    SmoothUnion { shapes: Vec<Shape>, smoothing: f32 },
    /**Minkowski sum: `surface ⊕ sweeper`.
Typical use: `Sweep { surface: Box, sweeper: Sphere }` = rounded box.*/
    Sweep { surface: Box<Shape>, sweeper: Box<Shape> },
}
impl gluon_wire::GluonConvertable for Shape {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            Shape::Box { size } => {
                gluon_data.write_u16(0u16)?;
                {
                    let __w: super::types::Vec3F = size.clone().into();
                    __w.write_owned(gluon_data)?;
                }
            }
            Shape::Sphere { radius } => {
                gluon_data.write_u16(1u16)?;
                radius.write(gluon_data)?;
            }
            Shape::Capsule { length, radius } => {
                gluon_data.write_u16(2u16)?;
                length.write(gluon_data)?;
                radius.write(gluon_data)?;
            }
            Shape::Cylinder { length, radius } => {
                gluon_data.write_u16(3u16)?;
                length.write(gluon_data)?;
                radius.write(gluon_data)?;
            }
            Shape::Torus { major_radius, minor_radius } => {
                gluon_data.write_u16(4u16)?;
                major_radius.write(gluon_data)?;
                minor_radius.write(gluon_data)?;
            }
            Shape::CubicBezierSpline { points, cyclic } => {
                gluon_data.write_u16(5u16)?;
                points.write(gluon_data)?;
                cyclic.write(gluon_data)?;
            }
            Shape::Transform { shape, transform } => {
                gluon_data.write_u16(6u16)?;
                shape.write(gluon_data)?;
                {
                    let __w: super::types::Mat4F = transform.clone().into();
                    __w.write_owned(gluon_data)?;
                }
            }
            Shape::Union { shapes } => {
                gluon_data.write_u16(7u16)?;
                shapes.write(gluon_data)?;
            }
            Shape::SmoothUnion { shapes, smoothing } => {
                gluon_data.write_u16(8u16)?;
                shapes.write(gluon_data)?;
                smoothing.write(gluon_data)?;
            }
            Shape::Sweep { surface, sweeper } => {
                gluon_data.write_u16(9u16)?;
                surface.write(gluon_data)?;
                sweeper.write(gluon_data)?;
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
                    let size: crate::types::Vec3F = {
                        let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                            gluon_data,
                        )?;
                        __w.into()
                    };
                    Shape::Box { size }
                }
                1u16 => {
                    let radius = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::Sphere { radius }
                }
                2u16 => {
                    let length = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let radius = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::Capsule { length, radius }
                }
                3u16 => {
                    let length = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let radius = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::Cylinder { length, radius }
                }
                4u16 => {
                    let major_radius = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let minor_radius = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::Torus {
                        major_radius,
                        minor_radius,
                    }
                }
                5u16 => {
                    let points = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let cyclic = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::CubicBezierSpline {
                        points,
                        cyclic,
                    }
                }
                6u16 => {
                    let shape = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let transform: crate::types::Mat4F = {
                        let __w: super::types::Mat4F = gluon_wire::GluonConvertable::read(
                            gluon_data,
                        )?;
                        __w.into()
                    };
                    Shape::Transform {
                        shape,
                        transform,
                    }
                }
                7u16 => {
                    let shapes = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::Union { shapes }
                }
                8u16 => {
                    let shapes = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let smoothing = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::SmoothUnion {
                        shapes,
                        smoothing,
                    }
                }
                9u16 => {
                    let surface = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let sweeper = gluon_wire::GluonConvertable::read(gluon_data)?;
                    Shape::Sweep { surface, sweeper }
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
                {
                    let __w: super::types::Vec3F = size.into();
                    __w.write_owned(gluon_data)?;
                }
            }
            Shape::Sphere { radius } => {
                gluon_data.write_u16(1u16)?;
                radius.write_owned(gluon_data)?;
            }
            Shape::Capsule { length, radius } => {
                gluon_data.write_u16(2u16)?;
                length.write_owned(gluon_data)?;
                radius.write_owned(gluon_data)?;
            }
            Shape::Cylinder { length, radius } => {
                gluon_data.write_u16(3u16)?;
                length.write_owned(gluon_data)?;
                radius.write_owned(gluon_data)?;
            }
            Shape::Torus { major_radius, minor_radius } => {
                gluon_data.write_u16(4u16)?;
                major_radius.write_owned(gluon_data)?;
                minor_radius.write_owned(gluon_data)?;
            }
            Shape::CubicBezierSpline { points, cyclic } => {
                gluon_data.write_u16(5u16)?;
                points.write_owned(gluon_data)?;
                cyclic.write_owned(gluon_data)?;
            }
            Shape::Transform { shape, transform } => {
                gluon_data.write_u16(6u16)?;
                shape.write_owned(gluon_data)?;
                {
                    let __w: super::types::Mat4F = transform.into();
                    __w.write_owned(gluon_data)?;
                }
            }
            Shape::Union { shapes } => {
                gluon_data.write_u16(7u16)?;
                shapes.write_owned(gluon_data)?;
            }
            Shape::SmoothUnion { shapes, smoothing } => {
                gluon_data.write_u16(8u16)?;
                shapes.write_owned(gluon_data)?;
                smoothing.write_owned(gluon_data)?;
            }
            Shape::Sweep { surface, sweeper } => {
                gluon_data.write_u16(9u16)?;
                surface.write_owned(gluon_data)?;
                sweeper.write_owned(gluon_data)?;
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
    pub async fn sample(
        &self,
        reference_space: impl Into<super::spatial::SpatialRef>,
        point: crate::types::Vec3F,
    ) -> Result<FieldSample, gluon_wire::GluonSendError> {
        let reference_space: super::spatial::SpatialRef = reference_space.into();
        let point: super::types::Vec3F = point.into();
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
    pub async fn ray_march(
        &self,
        reference_space: impl Into<super::spatial::SpatialRef>,
        ray_origin: crate::types::Vec3F,
        ray_direction: crate::types::Vec3F,
    ) -> Result<Option<RayMarchResult>, gluon_wire::GluonSendError> {
        let reference_space: super::spatial::SpatialRef = reference_space.into();
        let ray_origin: super::types::Vec3F = ray_origin.into();
        let ray_direction: super::types::Vec3F = ray_direction.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        reference_space.write(&mut gluon_builder)?;
        ray_origin.write(&mut gluon_builder)?;
        ray_direction.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn set_shape(
        &self,
        shape: impl Into<Shape>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let shape: Shape = shape.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        shape.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
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
    fn sample(
        &self,
        _ctx: gluon_wire::GluonCtx,
        reference_space: super::spatial::SpatialRef,
        point: crate::types::Vec3F,
    ) -> impl Future<Output = FieldSample> + Send + Sync;
    fn ray_march(
        &self,
        _ctx: gluon_wire::GluonCtx,
        reference_space: super::spatial::SpatialRef,
        ray_origin: crate::types::Vec3F,
        ray_direction: crate::types::Vec3F,
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
                    let param_point: crate::types::Vec3F = {
                        let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                            &mut gluon_data,
                        )?;
                        __w.into()
                    };
                    let (result) = self
                        .sample(ctx, param_reference_space, param_point)
                        .await;
                    drop(gluon_data);
                    result.write_owned(&mut gluon_out)?;
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
                    let param_ray_origin: crate::types::Vec3F = {
                        let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                            &mut gluon_data,
                        )?;
                        __w.into()
                    };
                    let param_ray_direction: crate::types::Vec3F = {
                        let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                            &mut gluon_data,
                        )?;
                        __w.into()
                    };
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
                12u32 => {
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
    pub async fn sample(
        &self,
        field: impl Into<FieldRef>,
        space: impl Into<super::spatial::SpatialRef>,
        point: crate::types::Vec3F,
    ) -> Result<FieldSample, gluon_wire::GluonSendError> {
        let field: FieldRef = field.into();
        let space: super::spatial::SpatialRef = space.into();
        let point: super::types::Vec3F = point.into();
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
    pub async fn ray_march(
        &self,
        field: impl Into<FieldRef>,
        space: impl Into<super::spatial::SpatialRef>,
        ray_origin: crate::types::Vec3F,
        ray_direction: crate::types::Vec3F,
    ) -> Result<Option<RayMarchResult>, gluon_wire::GluonSendError> {
        let field: FieldRef = field.into();
        let space: super::spatial::SpatialRef = space.into();
        let ray_origin: super::types::Vec3F = ray_origin.into();
        let ray_direction: super::types::Vec3F = ray_direction.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        field.write(&mut gluon_builder)?;
        space.write(&mut gluon_builder)?;
        ray_origin.write(&mut gluon_builder)?;
        ray_direction.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn create_field(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        shape: impl Into<Shape>,
    ) -> Result<Field, gluon_wire::GluonSendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let shape: Shape = shape.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        shape.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
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
    fn sample(
        &self,
        _ctx: gluon_wire::GluonCtx,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        point: crate::types::Vec3F,
    ) -> impl Future<Output = FieldSample> + Send + Sync;
    fn ray_march(
        &self,
        _ctx: gluon_wire::GluonCtx,
        field: FieldRef,
        space: super::spatial::SpatialRef,
        ray_origin: crate::types::Vec3F,
        ray_direction: crate::types::Vec3F,
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
                    let param_point: crate::types::Vec3F = {
                        let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                            &mut gluon_data,
                        )?;
                        __w.into()
                    };
                    let (result) = self
                        .sample(ctx, param_field, param_space, param_point)
                        .await;
                    drop(gluon_data);
                    result.write_owned(&mut gluon_out)?;
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
                    let param_ray_origin: crate::types::Vec3F = {
                        let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                            &mut gluon_data,
                        )?;
                        __w.into()
                    };
                    let param_ray_direction: crate::types::Vec3F = {
                        let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                            &mut gluon_data,
                        )?;
                        __w.into()
                    };
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
                10u32 => {
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
