#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Lines",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "Line",
            supported_derives: gluon_wire::Derives::from_bits_truncate(2u32),
        },
        gluon_wire::ExternalGluonType {
            name: "LinePoint",
            supported_derives: gluon_wire::Derives::from_bits_truncate(3u32),
        },
    ],
};
///A single continuous polyline
#[derive(Debug, Clone)]
pub struct Line {
    pub points: Vec<LinePoint>,
    ///Whether this line is a closed loop
    pub cyclic: bool,
}
impl gluon_wire::GluonConvertable for Line {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.points.write(gluon_data)?;
        self.cyclic.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let points = gluon_wire::GluonConvertable::read(gluon_data)?;
        let cyclic = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(Line { points, cyclic })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.points.write_owned(gluon_data)?;
        self.cyclic.write_owned(gluon_data)?;
        Ok(())
    }
}
///A single point on a line
#[derive(Debug, Copy, Clone)]
pub struct LinePoint {
    ///The position of the point relative to the Lines Spatial
    pub point: crate::types::Vec3F,
    ///Thickness in meters, world space
    pub thickness: f32,
    ///Color of the point
    pub color: crate::types::Color,
}
impl gluon_wire::GluonConvertable for LinePoint {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: super::types::Vec3F = self.point.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.thickness.write(gluon_data)?;
        {
            let __w: super::types::Color = self.color.clone().into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let point: crate::types::Vec3F = {
            let __w: super::types::Vec3F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let thickness = gluon_wire::GluonConvertable::read(gluon_data)?;
        let color: crate::types::Color = {
            let __w: super::types::Color = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        Ok(LinePoint {
            point,
            thickness,
            color,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: super::types::Vec3F = self.point.into();
            __w.write_owned(gluon_data)?;
        }
        self.thickness.write_owned(gluon_data)?;
        {
            let __w: super::types::Color = self.color.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Lines {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for Lines {
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
        Ok(Lines::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Lines {
    pub fn set_lines(
        &self,
        lines: impl Into<Vec<Line>>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let lines: Vec<Line> = lines.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        lines.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: LinesHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> Lines {
        Lines::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Lines {
        Lines { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for Lines {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for Lines {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Lines {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Lines {}
pub trait LinesHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn set_lines(
        &self,
        _ctx: gluon_wire::GluonCtx,
        lines: Vec<Line>,
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
                    let param_lines = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_lines(ctx, param_lines).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct LinesInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for LinesInterface {
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
        Ok(LinesInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl LinesInterface {
    pub async fn create_lines(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        lines: impl Into<Vec<Line>>,
    ) -> Result<Lines, gluon_wire::GluonSendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let lines: Vec<Line> = lines.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        lines.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: LinesInterfaceHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> LinesInterface {
        LinesInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> LinesInterface {
        LinesInterface { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for LinesInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for LinesInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for LinesInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for LinesInterface {}
pub trait LinesInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn create_lines(
        &self,
        _ctx: gluon_wire::GluonCtx,
        spatial: super::spatial::Spatial,
        lines: Vec<Line>,
    ) -> impl Future<Output = Lines> + Send + Sync;
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
                    let param_lines = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (lines) = self
                        .create_lines(ctx, param_spatial, param_lines)
                        .await;
                    drop(gluon_data);
                    lines.write_owned(&mut gluon_out)?;
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
