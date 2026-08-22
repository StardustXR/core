#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
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
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
    obj: gluon::Ref,
}
impl gluon::Convertable for Lines {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(Lines::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for Lines {
    const ID: &'static str = "org.stardustxr.Lines.Lines";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: LinesHandler> gluon::HandledBy<H> for Lines {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type LinesLocal<H> = gluon::LocalRef<Lines, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: LinesHandler> From<LinesLocal<H>> for Lines {
    fn from(value: LinesLocal<H>) -> Lines {
        value.into_proxy()
    }
}
impl gluon::RefExt for Lines {
    fn from_ref(obj: gluon::Ref) -> Lines {
        Lines { obj }
    }
}
impl Lines {
    pub fn set_lines(
        &self,
        lines: impl Into<Vec<Line>>,
    ) -> Result<(), gluon::SendError> {
        let lines: Vec<Line> = lines.into();
        tracing::trace!(interface = "Lines", method = "set_lines", ? lines, "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        lines.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> Lines {
        Lines { obj }
    }
}
impl From<Lines> for gluon::Ref {
    fn from(value: Lines) -> Self {
        value.obj
    }
}
impl gluon::ToRef for Lines {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for Lines {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
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
                    tracing::trace!(
                        interface = "Lines", method = "set_lines", ? param_lines,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.set_lines(ctx, param_lines)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Lines", method = "set_lines",
                                method_id = 8u32
                            ),
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
pub struct LinesInterface {
    obj: gluon::Ref,
}
impl gluon::Convertable for LinesInterface {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(LinesInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for LinesInterface {
    const ID: &'static str = "org.stardustxr.Lines.LinesInterface";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: LinesInterfaceHandler> gluon::HandledBy<H> for LinesInterface {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type LinesInterfaceLocal<H> = gluon::LocalRef<LinesInterface, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: LinesInterfaceHandler> From<LinesInterfaceLocal<H>> for LinesInterface {
    fn from(value: LinesInterfaceLocal<H>) -> LinesInterface {
        value.into_proxy()
    }
}
impl gluon::RefExt for LinesInterface {
    fn from_ref(obj: gluon::Ref) -> LinesInterface {
        LinesInterface { obj }
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
        tracing::trace!(
            interface = "LinesInterface", method = "create_lines", ? spatial, ? lines,
            "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        lines.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_lines = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "LinesInterface", method = "create_lines", ? __ret_lines, "←"
        );
        Ok(__ret_lines)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> LinesInterface {
        LinesInterface { obj }
    }
}
impl From<LinesInterface> for gluon::Ref {
    fn from(value: LinesInterface) -> Self {
        value.obj
    }
}
impl gluon::ToRef for LinesInterface {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for LinesInterface {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
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
    ///Dispatched instead of [`Self::create_lines`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `create_lines` and sends the result through `reply`. Override this method instead of `create_lines` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn create_lines_oneway(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        lines: Vec<Line>,
        reply: gluon::ReplySender<Result<Lines, super::types::CreateError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let lines = self.create_lines(_ctx, spatial, lines).await;
            reply.send(lines)
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
                    let param_lines = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "LinesInterface", method = "create_lines", ?
                        param_spatial, ? param_lines, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<Lines, super::types::CreateError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |lines, gluon_out| {
                            tracing::trace!(
                                interface = "LinesInterface", method = "create_lines", ?
                                lines, "←"
                            );
                            lines.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.create_lines_oneway(ctx, param_spatial, param_lines, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "LinesInterface", method =
                                "create_lines", method_id = 8u32
                            ),
                        )
                        .await?;
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
