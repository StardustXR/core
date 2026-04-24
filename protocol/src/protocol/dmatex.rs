#![allow(
    unused,
    clippy::single_match,
    clippy::match_single_binding,
    clippy::large_enum_variant
)]
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
        obj: impl AsRef<binderbinder::binder_object::BinderObjectRef<H>>,
    ) -> DmatexRef {
        DmatexRef::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj.as_ref(),
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> DmatexRef {
        DmatexRef { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for DmatexRef {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for DmatexRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for DmatexRef {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for DmatexRef {}
pub trait DmatexRefHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
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
pub struct DmatexInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
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
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        size.write(&mut gluon_builder)?;
        format.write(&mut gluon_builder)?;
        array_layers.write(&mut gluon_builder)?;
        planes.write(&mut gluon_builder)?;
        timeline_syncobj_fd.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn enumerate_formats(
        &self,
        render_node: u64,
    ) -> Result<Vec<DmatexFormat>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        render_node.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn primary_render_node_id(
        &self,
    ) -> Result<u64, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: DmatexInterfaceHandler>(
        obj: impl AsRef<binderbinder::binder_object::BinderObjectRef<H>>,
    ) -> DmatexInterface {
        DmatexInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj.as_ref(),
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> DmatexInterface {
        DmatexInterface { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for DmatexInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for DmatexInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for DmatexInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for DmatexInterface {}
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
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        gluon_data: &mut gluon_wire::GluonDataReader,
        ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Result<(), gluon_wire::GluonSendError>> + Send + Sync {
        async move {
            match transaction_code {
                8u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
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
                    dmatex.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (formats) = self
                        .enumerate_formats(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    formats.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (drm_render_node_id) = self.primary_render_node_id(ctx).await;
                    drm_render_node_id.write_owned(&mut gluon_out)?;
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
