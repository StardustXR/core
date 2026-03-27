#![allow(unused, clippy::single_match, clippy::match_single_binding)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.DmaTex",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "DmaTexFormatInfo",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
        gluon_wire::ExternalGluonType {
            name: "DmaTexPlane",
            supported_derives: gluon_wire::Derives::from_bits_truncate(0u32),
        },
        gluon_wire::ExternalGluonType {
            name: "DmaTexSize",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
    ],
};
///Information about a DMA texture format.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DmaTexFormatInfo {
    pub format: u32,
    pub drm_modifier: u64,
    pub is_srgb: bool,
    pub planes: u32,
}
impl gluon_wire::GluonConvertable for DmaTexFormatInfo {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.format.write(data)?;
        self.drm_modifier.write(data)?;
        self.is_srgb.write(data)?;
        self.planes.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let format = gluon_wire::GluonConvertable::read(data)?;
        let drm_modifier = gluon_wire::GluonConvertable::read(data)?;
        let is_srgb = gluon_wire::GluonConvertable::read(data)?;
        let planes = gluon_wire::GluonConvertable::read(data)?;
        Ok(DmaTexFormatInfo {
            format,
            drm_modifier,
            is_srgb,
            planes,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.format.write_owned(data)?;
        self.drm_modifier.write_owned(data)?;
        self.is_srgb.write_owned(data)?;
        self.planes.write_owned(data)?;
        Ok(())
    }
}
///Information about a DMA texture plane.
#[derive(Debug)]
pub struct DmaTexPlane {
    pub dmabuf_fd: std::os::fd::OwnedFd,
    pub offset: u64,
    pub row_size: u64,
    pub array_element_size: u64,
    pub depth_slice_size: u64,
}
impl gluon_wire::GluonConvertable for DmaTexPlane {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.dmabuf_fd.write(data)?;
        self.offset.write(data)?;
        self.row_size.write(data)?;
        self.array_element_size.write(data)?;
        self.depth_slice_size.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let dmabuf_fd = gluon_wire::GluonConvertable::read(data)?;
        let offset = gluon_wire::GluonConvertable::read(data)?;
        let row_size = gluon_wire::GluonConvertable::read(data)?;
        let array_element_size = gluon_wire::GluonConvertable::read(data)?;
        let depth_slice_size = gluon_wire::GluonConvertable::read(data)?;
        Ok(DmaTexPlane {
            dmabuf_fd,
            offset,
            row_size,
            array_element_size,
            depth_slice_size,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.dmabuf_fd.write_owned(data)?;
        self.offset.write_owned(data)?;
        self.row_size.write_owned(data)?;
        self.array_element_size.write_owned(data)?;
        self.depth_slice_size.write_owned(data)?;
        Ok(())
    }
}
///Size of a DMA texture.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DmaTexSize {
    Size1D { size: u32 },
    Size2D { size: super::types::Size2 },
    Size3D { size: super::types::Size3 },
}
impl gluon_wire::GluonConvertable for DmaTexSize {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            DmaTexSize::Size1D { size } => {
                data.write_u16(0u16)?;
                size.write(data)?;
            }
            DmaTexSize::Size2D { size } => {
                data.write_u16(1u16)?;
                size.write(data)?;
            }
            DmaTexSize::Size3D { size } => {
                data.write_u16(2u16)?;
                size.write(data)?;
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
                    DmaTexSize::Size1D { size }
                }
                1u16 => {
                    let size = gluon_wire::GluonConvertable::read(data)?;
                    DmaTexSize::Size2D { size }
                }
                2u16 => {
                    let size = gluon_wire::GluonConvertable::read(data)?;
                    DmaTexSize::Size3D { size }
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
            DmaTexSize::Size1D { size } => {
                data.write_u16(0u16)?;
                size.write_owned(data)?;
            }
            DmaTexSize::Size2D { size } => {
                data.write_u16(1u16)?;
                size.write_owned(data)?;
            }
            DmaTexSize::Size3D { size } => {
                data.write_u16(2u16)?;
                size.write_owned(data)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct DmaTexRef {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for DmaTexRef {
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
        Ok(DmaTexRef::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl DmaTexRef {
    pub fn from_handler<H: DmaTexRefHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> DmaTexRef {
        DmaTexRef::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> DmaTexRef {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        DmaTexRef {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for DmaTexRef {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait DmaTexRefHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
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
pub struct DmaTexInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for DmaTexInterface {
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
        Ok(DmaTexInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl DmaTexInterface {
    pub async fn import_dmatex(
        &self,
        size: DmaTexSize,
        format: u32,
        drm_format_modifier: u64,
        srgb: bool,
        array_layers: u32,
        planes: Vec<DmaTexPlane>,
        timeline_syncobj_fd: std::os::fd::OwnedFd,
    ) -> Result<DmaTexRef, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this
                    .import_dmatex_blocking(
                        size,
                        format,
                        drm_format_modifier,
                        srgb,
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
        size: DmaTexSize,
        format: u32,
        drm_format_modifier: u64,
        srgb: bool,
        array_layers: u32,
        planes: Vec<DmaTexPlane>,
        timeline_syncobj_fd: std::os::fd::OwnedFd,
    ) -> Result<DmaTexRef, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        size.write(&mut builder)?;
        format.write(&mut builder)?;
        drm_format_modifier.write(&mut builder)?;
        srgb.write(&mut builder)?;
        array_layers.write(&mut builder)?;
        planes.write(&mut builder)?;
        timeline_syncobj_fd.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: DmaTexInterfaceHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> DmaTexInterface {
        DmaTexInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> DmaTexInterface {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        DmaTexInterface {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for DmaTexInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait DmaTexInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn import_dmatex(
        &self,
        _ctx: gluon_wire::GluonCtx,
        size: DmaTexSize,
        format: u32,
        drm_format_modifier: u64,
        srgb: bool,
        array_layers: u32,
        planes: Vec<DmaTexPlane>,
        timeline_syncobj_fd: std::os::fd::OwnedFd,
    ) -> impl Future<Output = DmaTexRef> + Send + Sync;
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
                    let (dmatex) = self
                        .import_dmatex(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    dmatex.write_owned(&mut out)?;
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
