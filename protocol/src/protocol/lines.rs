#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Lines",
    types: &[
        gluon::ExternalGluonType {
            name: "Line",
            supported_derives: gluon::Derives::from_bits_truncate(778u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "LinePoint",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///A single continuous polyline
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Line {
    pub points: Vec<LinePoint>,
    ///Whether this line is a closed loop
    pub cyclic: bool,
}
impl gluon::Convertable for Line {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.points.write(gluon_data)?;
        self.cyclic.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let points = gluon::Convertable::read(gluon_data)?;
        let cyclic = gluon::Convertable::read(gluon_data)?;
        Ok(Line { points, cyclic })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.points.write_owned(gluon_data)?;
        self.cyclic.write_owned(gluon_data)?;
        Ok(())
    }
}
///A single point on a line
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LinePoint {
    ///The position of the point relative to the Lines Spatial
    pub point: crate::types::Vec3F,
    ///Thickness in meters, world space
    pub thickness: f32,
    ///Color of the point
    pub color: crate::types::Color,
}
impl gluon::Convertable for LinePoint {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Vec3F = self.point.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.thickness.write(gluon_data)?;
        {
            let __w: super::types::proxied::Color = self.color.clone().into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let point: crate::types::Vec3F = {
            let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let thickness = gluon::Convertable::read(gluon_data)?;
        let color: crate::types::Color = {
            let __w: super::types::proxied::Color = gluon::Convertable::read(
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
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Vec3F = self.point.into();
            __w.write_owned(gluon_data)?;
        }
        self.thickness.write_owned(gluon_data)?;
        {
            let __w: super::types::proxied::Color = self.color.into();
            __w.write_owned(gluon_data)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Lines {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for Lines {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(Lines::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Lines {
    pub fn set_lines(
        &self,
        lines: impl Into<Vec<Line>>,
    ) -> Result<(), gluon::SendError> {
        let lines: Vec<Line> = lines.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        lines.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: LinesHandler>(obj: &impl gluon::OwnedObjectRef<H>) -> Lines {
        Lines::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> Lines {
        Lines { obj }
    }
}
impl From<Lines> for gluon::ObjectOrRef {
    fn from(value: Lines) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for Lines {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait LinesHandler: gluon::Handler + Send + Sync + 'static {
    fn set_lines(
        &self,
        _ctx: gluon::Context,
        lines: Vec<Line>,
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
                    let param_lines = gluon::Convertable::read(&mut gluon_data)?;
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for LinesInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(LinesInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl LinesInterface {
    pub async fn create_lines(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        lines: impl Into<Vec<Line>>,
    ) -> Result<Result<Lines, super::types::CreateError>, gluon::SendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let lines: Vec<Line> = lines.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        lines.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn from_handler<H: LinesInterfaceHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> LinesInterface {
        LinesInterface::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> LinesInterface {
        LinesInterface { obj }
    }
}
impl From<LinesInterface> for gluon::ObjectOrRef {
    fn from(value: LinesInterface) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for LinesInterface {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait LinesInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    fn create_lines(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        lines: Vec<Line>,
    ) -> impl Future<Output = Result<Lines, super::types::CreateError>> + Send + Sync;
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
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_lines = gluon::Convertable::read(&mut gluon_data)?;
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
pub mod proxied {
    use super::*;
}
