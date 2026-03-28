#![allow(unused, clippy::single_match, clippy::match_single_binding)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Dmatex",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "DmatexFormat",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
        gluon_wire::ExternalGluonType {
            name: "DmatexPlane",
            supported_derives: gluon_wire::Derives::from_bits_truncate(0u32),
        },
        gluon_wire::ExternalGluonType {
            name: "DmatexSize",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
    ],
};
///Information about a DMA texture format.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DmatexFormat {
    pub drm_fourcc: u32,
    pub drm_modifier: u64,
    pub is_srgb: bool,
}
impl gluon_wire::GluonConvertable for DmatexFormat {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.drm_fourcc.write(gluon_data)?;
        self.drm_modifier.write(gluon_data)?;
        self.is_srgb.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let drm_fourcc = gluon_wire::GluonConvertable::read(gluon_data)?;
        let drm_modifier = gluon_wire::GluonConvertable::read(gluon_data)?;
        let is_srgb = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(DmatexFormat {
            drm_fourcc,
            drm_modifier,
            is_srgb,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.drm_fourcc.write_owned(gluon_data)?;
        self.drm_modifier.write_owned(gluon_data)?;
        self.is_srgb.write_owned(gluon_data)?;
        Ok(())
    }
}
///Information about a DMA texture plane.
#[derive(Debug)]
pub struct DmatexPlane {
    pub dmabuf_fd: std::os::fd::OwnedFd,
    pub offset: u64,
    pub row_size: u64,
    pub array_element_size: u64,
    pub depth_slice_size: u64,
}
impl gluon_wire::GluonConvertable for DmatexPlane {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.dmabuf_fd.write(gluon_data)?;
        self.offset.write(gluon_data)?;
        self.row_size.write(gluon_data)?;
        self.array_element_size.write(gluon_data)?;
        self.depth_slice_size.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let dmabuf_fd = gluon_wire::GluonConvertable::read(gluon_data)?;
        let offset = gluon_wire::GluonConvertable::read(gluon_data)?;
        let row_size = gluon_wire::GluonConvertable::read(gluon_data)?;
        let array_element_size = gluon_wire::GluonConvertable::read(gluon_data)?;
        let depth_slice_size = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(DmatexPlane {
            dmabuf_fd,
            offset,
            row_size,
            array_element_size,
            depth_slice_size,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.dmabuf_fd.write_owned(gluon_data)?;
        self.offset.write_owned(gluon_data)?;
        self.row_size.write_owned(gluon_data)?;
        self.array_element_size.write_owned(gluon_data)?;
        self.depth_slice_size.write_owned(gluon_data)?;
        Ok(())
    }
}
///Size of a DMA texture.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DmatexSize {
    Size1D { size: u32 },
    Size2D { size: super::types::Size2 },
    Size3D { size: super::types::Size3 },
}
impl gluon_wire::GluonConvertable for DmatexSize {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            DmatexSize::Size1D { size } => {
                gluon_data.write_u16(0u16)?;
                size.write(gluon_data)?;
            }
            DmatexSize::Size2D { size } => {
                gluon_data.write_u16(1u16)?;
                size.write(gluon_data)?;
            }
            DmatexSize::Size3D { size } => {
                gluon_data.write_u16(2u16)?;
                size.write(gluon_data)?;
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
                    DmatexSize::Size1D { size }
                }
                1u16 => {
                    let size = gluon_wire::GluonConvertable::read(gluon_data)?;
                    DmatexSize::Size2D { size }
                }
                2u16 => {
                    let size = gluon_wire::GluonConvertable::read(gluon_data)?;
                    DmatexSize::Size3D { size }
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
            DmatexSize::Size1D { size } => {
                gluon_data.write_u16(0u16)?;
                size.write_owned(gluon_data)?;
            }
            DmatexSize::Size2D { size } => {
                gluon_data.write_u16(1u16)?;
                size.write_owned(gluon_data)?;
            }
            DmatexSize::Size3D { size } => {
                gluon_data.write_u16(2u16)?;
                size.write_owned(gluon_data)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct DmatexRef {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for DmatexRef {
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
        Ok(DmatexRef::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl DmatexRef {
    pub fn from_handler<H: DmatexRefHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> DmatexRef {
        DmatexRef::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> DmatexRef {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        DmatexRef {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for DmatexRef {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait DmatexRefHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
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
                4 => {
                    let Ok(obj) = gluon_data.read_binder() else {
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
pub struct DmatexInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for DmatexInterface {
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
        Ok(DmatexInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl DmatexInterface {
    pub async fn import_dmatex(
        &self,
        size: DmatexSize,
        format: DmatexFormat,
        array_layers: u32,
        planes: Vec<DmatexPlane>,
        timeline_syncobj_fd: std::os::fd::OwnedFd,
    ) -> Result<DmatexRef, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this
                    .import_dmatex_blocking(
                        size,
                        format,
                        array_layers,
                        planes,
                        timeline_syncobj_fd,
                    )
            })
            .await
            .unwrap()
    }
    pub fn import_dmatex_blocking(
        &self,
        size: DmatexSize,
        format: DmatexFormat,
        array_layers: u32,
        planes: Vec<DmatexPlane>,
        timeline_syncobj_fd: std::os::fd::OwnedFd,
    ) -> Result<DmatexRef, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        size.write(&mut gluon_builder)?;
        format.write(&mut gluon_builder)?;
        array_layers.write(&mut gluon_builder)?;
        planes.write(&mut gluon_builder)?;
        timeline_syncobj_fd.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn enumerate_formats(
        &self,
        render_node: u64,
    ) -> Result<Vec<DmatexFormat>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.enumerate_formats_blocking(render_node))
            .await
            .unwrap()
    }
    pub fn enumerate_formats_blocking(
        &self,
        render_node: u64,
    ) -> Result<Vec<DmatexFormat>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        render_node.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn primary_render_node_id(
        &self,
    ) -> Result<u64, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.primary_render_node_id_blocking())
            .await
            .unwrap()
    }
    pub fn primary_render_node_id_blocking(
        &self,
    ) -> Result<u64, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: DmatexInterfaceHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> DmatexInterface {
        DmatexInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> DmatexInterface {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        DmatexInterface {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for DmatexInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait DmatexInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn import_dmatex(
        &self,
        _ctx: gluon_wire::GluonCtx,
        size: DmatexSize,
        format: DmatexFormat,
        array_layers: u32,
        planes: Vec<DmatexPlane>,
        timeline_syncobj_fd: std::os::fd::OwnedFd,
    ) -> impl Future<Output = DmatexRef> + Send + Sync;
    fn enumerate_formats(
        &self,
        _ctx: gluon_wire::GluonCtx,
        render_node: u64,
    ) -> impl Future<Output = Vec<DmatexFormat>> + Send + Sync;
    fn primary_render_node_id(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = u64> + Send + Sync;
    fn drop_notification_requested(
        &self,
        notifier: gluon_wire::drop_tracking::DropNotifier,
    ) -> impl Future<Output = ()> + Send + Sync;
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
                    let (dmatex) = self
                        .import_dmatex(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    dmatex.write_owned(&mut out)?;
                }
                9u32 => {
                    let (formats) = self
                        .enumerate_formats(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    formats.write_owned(&mut out)?;
                }
                10u32 => {
                    let (drm_render_node_id) = self.primary_render_node_id(ctx).await;
                    drm_render_node_id.write_owned(&mut out)?;
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
                4 => {
                    let Ok(obj) = gluon_data.read_binder() else {
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
