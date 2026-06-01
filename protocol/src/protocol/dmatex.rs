#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Dmatex",
    types: &[
        gluon::ExternalGluonType {
            name: "DmatexFormat",
            supported_derives: gluon::Derives::from_bits_truncate(31u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DmatexPlane",
            supported_derives: gluon::Derives::from_bits_truncate(0u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DmatexSize",
            supported_derives: gluon::Derives::from_bits_truncate(31u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DmatexImportError",
            supported_derives: gluon::Derives::from_bits_truncate(31u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///Information about a DMA texture format.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct DmatexFormat {
    pub drm_fourcc: u32,
    pub drm_modifier: u64,
    pub is_srgb: bool,
}
impl gluon::Convertable for DmatexFormat {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.drm_fourcc.write(gluon_data)?;
        self.drm_modifier.write(gluon_data)?;
        self.is_srgb.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let drm_fourcc = gluon::Convertable::read(gluon_data)?;
        let drm_modifier = gluon::Convertable::read(gluon_data)?;
        let is_srgb = gluon::Convertable::read(gluon_data)?;
        Ok(DmatexFormat {
            drm_fourcc,
            drm_modifier,
            is_srgb,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
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
impl gluon::Convertable for DmatexPlane {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.dmabuf_fd.write(gluon_data)?;
        self.offset.write(gluon_data)?;
        self.row_size.write(gluon_data)?;
        self.array_element_size.write(gluon_data)?;
        self.depth_slice_size.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let dmabuf_fd = gluon::Convertable::read(gluon_data)?;
        let offset = gluon::Convertable::read(gluon_data)?;
        let row_size = gluon::Convertable::read(gluon_data)?;
        let array_element_size = gluon::Convertable::read(gluon_data)?;
        let depth_slice_size = gluon::Convertable::read(gluon_data)?;
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
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
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
    Size2D { size: crate::types::Size2 },
    Size3D { size: crate::types::Size3 },
}
impl gluon::Convertable for DmatexSize {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            DmatexSize::Size1D { size } => {
                gluon_data.write_u16(0u16)?;
                size.write(gluon_data)?;
            }
            DmatexSize::Size2D { size } => {
                gluon_data.write_u16(1u16)?;
                {
                    let __w: super::types::proxied::Size2 = size.clone().into();
                    __w.write_owned(gluon_data)?;
                }
            }
            DmatexSize::Size3D { size } => {
                gluon_data.write_u16(2u16)?;
                {
                    let __w: super::types::proxied::Size3 = size.clone().into();
                    __w.write_owned(gluon_data)?;
                }
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => {
                    let size = gluon::Convertable::read(gluon_data)?;
                    DmatexSize::Size1D { size }
                }
                1u16 => {
                    let size: crate::types::Size2 = {
                        let __w: super::types::proxied::Size2 = gluon::Convertable::read(
                            gluon_data,
                        )?;
                        __w.into()
                    };
                    DmatexSize::Size2D { size }
                }
                2u16 => {
                    let size: crate::types::Size3 = {
                        let __w: super::types::proxied::Size3 = gluon::Convertable::read(
                            gluon_data,
                        )?;
                        __w.into()
                    };
                    DmatexSize::Size3D { size }
                }
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            DmatexSize::Size1D { size } => {
                gluon_data.write_u16(0u16)?;
                size.write_owned(gluon_data)?;
            }
            DmatexSize::Size2D { size } => {
                gluon_data.write_u16(1u16)?;
                {
                    let __w: super::types::proxied::Size2 = size.into();
                    __w.write_owned(gluon_data)?;
                }
            }
            DmatexSize::Size3D { size } => {
                gluon_data.write_u16(2u16)?;
                {
                    let __w: super::types::proxied::Size3 = size.into();
                    __w.write_owned(gluon_data)?;
                }
            }
        };
        Ok(())
    }
}
///Error potentially produced when loading a model
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DmatexImportError {
    InvalidSize,
    InvalidFormat,
    UnsupportedArrayLayers { max_supported_layers: u32 },
    InvalidPlanes,
    InvalidTimelineFd,
    InternalImportError,
}
impl gluon::Convertable for DmatexImportError {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            DmatexImportError::InvalidSize => {
                gluon_data.write_u16(0u16)?;
            }
            DmatexImportError::InvalidFormat => {
                gluon_data.write_u16(1u16)?;
            }
            DmatexImportError::UnsupportedArrayLayers { max_supported_layers } => {
                gluon_data.write_u16(2u16)?;
                max_supported_layers.write(gluon_data)?;
            }
            DmatexImportError::InvalidPlanes => {
                gluon_data.write_u16(3u16)?;
            }
            DmatexImportError::InvalidTimelineFd => {
                gluon_data.write_u16(4u16)?;
            }
            DmatexImportError::InternalImportError => {
                gluon_data.write_u16(5u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => DmatexImportError::InvalidSize,
                1u16 => DmatexImportError::InvalidFormat,
                2u16 => {
                    let max_supported_layers = gluon::Convertable::read(gluon_data)?;
                    DmatexImportError::UnsupportedArrayLayers {
                        max_supported_layers,
                    }
                }
                3u16 => DmatexImportError::InvalidPlanes,
                4u16 => DmatexImportError::InvalidTimelineFd,
                5u16 => DmatexImportError::InternalImportError,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        match self {
            DmatexImportError::InvalidSize => {
                gluon_data.write_u16(0u16)?;
            }
            DmatexImportError::InvalidFormat => {
                gluon_data.write_u16(1u16)?;
            }
            DmatexImportError::UnsupportedArrayLayers { max_supported_layers } => {
                gluon_data.write_u16(2u16)?;
                max_supported_layers.write_owned(gluon_data)?;
            }
            DmatexImportError::InvalidPlanes => {
                gluon_data.write_u16(3u16)?;
            }
            DmatexImportError::InvalidTimelineFd => {
                gluon_data.write_u16(4u16)?;
            }
            DmatexImportError::InternalImportError => {
                gluon_data.write_u16(5u16)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct DmatexRef {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for DmatexRef {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(DmatexRef::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl DmatexRef {
    pub fn from_handler<H: DmatexRefHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> DmatexRef {
        DmatexRef::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> DmatexRef {
        DmatexRef { obj }
    }
}
impl From<DmatexRef> for gluon::ObjectOrRef {
    fn from(value: DmatexRef) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for DmatexRef {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait DmatexRefHandler: gluon::Handler + Send + Sync + 'static {
    fn dispatch_one_way(
        &self,
        transaction_code: u32,
        mut gluon_data: gluon::DataReader,
        ctx: gluon::Context,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for DmatexInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(DmatexInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl DmatexInterface {
    pub async fn import_dmatex(
        &self,
        size: impl Into<DmatexSize>,
        format: impl Into<DmatexFormat>,
        array_layers: impl Into<u32>,
        planes: impl Into<Vec<DmatexPlane>>,
        timeline_syncobj_fd: impl Into<std::os::fd::OwnedFd>,
    ) -> Result<Result<DmatexRef, DmatexImportError>, gluon::SendError> {
        let size: DmatexSize = size.into();
        let format: DmatexFormat = format.into();
        let array_layers: u32 = array_layers.into();
        let planes: Vec<DmatexPlane> = planes.into();
        let timeline_syncobj_fd: std::os::fd::OwnedFd = timeline_syncobj_fd.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        size.write(&mut gluon_builder)?;
        format.write(&mut gluon_builder)?;
        array_layers.write(&mut gluon_builder)?;
        planes.write(&mut gluon_builder)?;
        timeline_syncobj_fd.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn enumerate_formats(
        &self,
        render_node: impl Into<u64>,
    ) -> Result<Option<Vec<DmatexFormat>>, gluon::SendError> {
        let render_node: u64 = render_node.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        render_node.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn primary_render_node_id(&self) -> Result<u64, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn from_handler<H: DmatexInterfaceHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> DmatexInterface {
        DmatexInterface::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> DmatexInterface {
        DmatexInterface { obj }
    }
}
impl From<DmatexInterface> for gluon::ObjectOrRef {
    fn from(value: DmatexInterface) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for DmatexInterface {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait DmatexInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    fn import_dmatex(
        &self,
        _ctx: gluon::Context,
        size: DmatexSize,
        format: DmatexFormat,
        array_layers: u32,
        planes: Vec<DmatexPlane>,
        timeline_syncobj_fd: std::os::fd::OwnedFd,
    ) -> impl Future<Output = Result<DmatexRef, DmatexImportError>> + Send + Sync;
    fn enumerate_formats(
        &self,
        _ctx: gluon::Context,
        render_node: u64,
    ) -> impl Future<Output = Option<Vec<DmatexFormat>>> + Send + Sync;
    fn primary_render_node_id(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = u64> + Send + Sync;
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
                    let param_size = gluon::Convertable::read(&mut gluon_data)?;
                    let param_format = gluon::Convertable::read(&mut gluon_data)?;
                    let param_array_layers = gluon::Convertable::read(&mut gluon_data)?;
                    let param_planes = gluon::Convertable::read(&mut gluon_data)?;
                    let param_timeline_syncobj_fd = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let (dmatex) = self
                        .import_dmatex(
                            ctx,
                            param_size,
                            param_format,
                            param_array_layers,
                            param_planes,
                            param_timeline_syncobj_fd,
                        )
                        .await;
                    drop(gluon_data);
                    dmatex.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_render_node = gluon::Convertable::read(&mut gluon_data)?;
                    let (formats) = self.enumerate_formats(ctx, param_render_node).await;
                    drop(gluon_data);
                    formats.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (drm_render_node_id) = self.primary_render_node_id(ctx).await;
                    drop(gluon_data);
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
pub mod proxied {
    use super::*;
}
