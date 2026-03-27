#![allow(unused, clippy::single_match, clippy::match_single_binding)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Lines",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "Line",
            supported_derives: gluon_wire::Derives::from_bits_truncate(10u32),
        },
        gluon_wire::ExternalGluonType {
            name: "LinePoint",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
    ],
};
///A single continuous polyline
#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub points: Vec<LinePoint>,
    ///Whether this line is a closed loop
    pub cyclic: bool,
}
impl gluon_wire::GluonConvertable for Line {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.points.write(data)?;
        self.cyclic.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let points = gluon_wire::GluonConvertable::read(data)?;
        let cyclic = gluon_wire::GluonConvertable::read(data)?;
        Ok(Line { points, cyclic })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.points.write_owned(data)?;
        self.cyclic.write_owned(data)?;
        Ok(())
    }
}
///A single point on a line
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LinePoint {
    ///The position of the point relative to the Lines Spatial
    pub point: super::types::Vec3F,
    ///Thickness in meters, world space
    pub thickness: f32,
    ///Color of the point
    pub color: super::types::Color,
}
impl gluon_wire::GluonConvertable for LinePoint {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.point.write(data)?;
        self.thickness.write(data)?;
        self.color.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let point = gluon_wire::GluonConvertable::read(data)?;
        let thickness = gluon_wire::GluonConvertable::read(data)?;
        let color = gluon_wire::GluonConvertable::read(data)?;
        Ok(LinePoint {
            point,
            thickness,
            color,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.point.write_owned(data)?;
        self.thickness.write_owned(data)?;
        self.color.write_owned(data)?;
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Lines {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for Lines {
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
        Ok(Lines::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl Lines {
    pub async fn get_spatial(
        &self,
    ) -> Result<super::spatial::Spatial, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_spatial_blocking()).await.unwrap()
    }
    pub fn get_spatial_blocking(
        &self,
    ) -> Result<super::spatial::Spatial, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn set_lines(&self, lines: Vec<Line>) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        lines.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: LinesHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
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
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        Lines { obj, drop_notification }
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
impl binderbinder::binder_object::ToBinderObjectOrRef for Lines {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait LinesHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn get_spatial(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::spatial::Spatial> + Send + Sync;
    fn set_lines(&self, _ctx: gluon_wire::GluonCtx, lines: Vec<Line>);
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
                    let (spatial) = self.get_spatial(ctx).await;
                    spatial.write_owned(&mut out)?;
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
                9u32 => {
                    self.set_lines(ctx, gluon_wire::GluonConvertable::read(data)?);
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
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for LinesInterface {
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
        Ok(LinesInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl LinesInterface {
    pub async fn create_lines(
        &self,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        lines: Vec<Line>,
    ) -> Result<Lines, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.create_lines_blocking(parent, transform, lines)
            })
            .await
            .unwrap()
    }
    pub fn create_lines_blocking(
        &self,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        lines: Vec<Line>,
    ) -> Result<Lines, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        parent.write(&mut builder)?;
        transform.write(&mut builder)?;
        lines.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: LinesInterfaceHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
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
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        LinesInterface {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for LinesInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait LinesInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn create_lines(
        &self,
        _ctx: gluon_wire::GluonCtx,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        lines: Vec<Line>,
    ) -> impl Future<Output = Lines> + Send + Sync;
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
                    let (lines) = self
                        .create_lines(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    lines.write_owned(&mut out)?;
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
