#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Dmatex",
    types: &[
        gluon::ExternalGluonType {
            name: "DmatexFormat",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DmatexFormatInfo",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DmatexPlane",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DisjointDmatexPlane",
            supported_derives: gluon::Derives::from_bits_truncate(0u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "YcbcrFormat",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DmatexSize",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DmatexPlanes",
            supported_derives: gluon::Derives::from_bits_truncate(0u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "AlphaMode",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "YcbcrCoefficients",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "YcbcrChromaLocation",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "YcbcrRange",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "DmatexImportError",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///Representing DMA texture format.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DmatexFormat {
    pub drm_fourcc: u32,
    pub drm_modifier: u64,
    pub is_srgb: bool,
    pub alpha_mode: AlphaMode,
    ///Must be Some when using a YCbCr format
    pub ycbcr_info: Option<YcbcrFormat>,
}
impl gluon::Convertable for DmatexFormat {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.drm_fourcc.write(gluon_data)?;
        self.drm_modifier.write(gluon_data)?;
        self.is_srgb.write(gluon_data)?;
        self.alpha_mode.write(gluon_data)?;
        self.ycbcr_info.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let drm_fourcc = gluon::Convertable::read(gluon_data)?;
        let drm_modifier = gluon::Convertable::read(gluon_data)?;
        let is_srgb = gluon::Convertable::read(gluon_data)?;
        let alpha_mode = gluon::Convertable::read(gluon_data)?;
        let ycbcr_info = gluon::Convertable::read(gluon_data)?;
        Ok(DmatexFormat {
            drm_fourcc,
            drm_modifier,
            is_srgb,
            alpha_mode,
            ycbcr_info,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.drm_fourcc.write_owned(gluon_data)?;
        self.drm_modifier.write_owned(gluon_data)?;
        self.is_srgb.write_owned(gluon_data)?;
        self.alpha_mode.write_owned(gluon_data)?;
        self.ycbcr_info.write_owned(gluon_data)?;
        Ok(())
    }
}
///Queried information about a DMA texture format.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DmatexFormatInfo {
    pub drm_fourcc: u32,
    pub drm_modifier: u64,
    pub supports_srgb: bool,
    ///Allows the dmatex to use multiple underlying memory objects
    pub supports_disjoint: bool,
    ///A dmatex created with this format may be applied as a texture
    pub supports_sampling: bool,
    ///A dmatex created with this format may be used as a render attachment
    pub supports_rendering: bool,
}
impl gluon::Convertable for DmatexFormatInfo {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.drm_fourcc.write(gluon_data)?;
        self.drm_modifier.write(gluon_data)?;
        self.supports_srgb.write(gluon_data)?;
        self.supports_disjoint.write(gluon_data)?;
        self.supports_sampling.write(gluon_data)?;
        self.supports_rendering.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let drm_fourcc = gluon::Convertable::read(gluon_data)?;
        let drm_modifier = gluon::Convertable::read(gluon_data)?;
        let supports_srgb = gluon::Convertable::read(gluon_data)?;
        let supports_disjoint = gluon::Convertable::read(gluon_data)?;
        let supports_sampling = gluon::Convertable::read(gluon_data)?;
        let supports_rendering = gluon::Convertable::read(gluon_data)?;
        Ok(DmatexFormatInfo {
            drm_fourcc,
            drm_modifier,
            supports_srgb,
            supports_disjoint,
            supports_sampling,
            supports_rendering,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.drm_fourcc.write_owned(gluon_data)?;
        self.drm_modifier.write_owned(gluon_data)?;
        self.supports_srgb.write_owned(gluon_data)?;
        self.supports_disjoint.write_owned(gluon_data)?;
        self.supports_sampling.write_owned(gluon_data)?;
        self.supports_rendering.write_owned(gluon_data)?;
        Ok(())
    }
}
///Information about a DMA texture plane.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DmatexPlane {
    pub offset: u64,
    pub row_size: u64,
    pub array_element_size: u64,
    pub depth_slice_size: u64,
}
impl gluon::Convertable for DmatexPlane {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.offset.write(gluon_data)?;
        self.row_size.write(gluon_data)?;
        self.array_element_size.write(gluon_data)?;
        self.depth_slice_size.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let offset = gluon::Convertable::read(gluon_data)?;
        let row_size = gluon::Convertable::read(gluon_data)?;
        let array_element_size = gluon::Convertable::read(gluon_data)?;
        let depth_slice_size = gluon::Convertable::read(gluon_data)?;
        Ok(DmatexPlane {
            offset,
            row_size,
            array_element_size,
            depth_slice_size,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.offset.write_owned(gluon_data)?;
        self.row_size.write_owned(gluon_data)?;
        self.array_element_size.write_owned(gluon_data)?;
        self.depth_slice_size.write_owned(gluon_data)?;
        Ok(())
    }
}
///Basically just a tuple of a DmatexPlane and its corresponding dmabuf fd
#[derive(Debug)]
pub struct DisjointDmatexPlane {
    pub dmabuf_fd: std::os::fd::OwnedFd,
    pub plane: DmatexPlane,
}
impl gluon::Convertable for DisjointDmatexPlane {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.dmabuf_fd.write(gluon_data)?;
        self.plane.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let dmabuf_fd = gluon::Convertable::read(gluon_data)?;
        let plane = gluon::Convertable::read(gluon_data)?;
        Ok(DisjointDmatexPlane {
            dmabuf_fd,
            plane,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.dmabuf_fd.write_owned(gluon_data)?;
        self.plane.write_owned(gluon_data)?;
        Ok(())
    }
}
///YCbCr specific format information
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct YcbcrFormat {
    pub coefficients: YcbcrCoefficients,
    pub range: YcbcrRange,
    ///needs to be Some when using a format with subsampling on the x axis
    pub chroma_location_x: Option<YcbcrChromaLocation>,
    ///needs to be Some when using a format with subsampling on the y axis
    pub chroma_location_y: Option<YcbcrChromaLocation>,
}
impl gluon::Convertable for YcbcrFormat {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.coefficients.write(gluon_data)?;
        self.range.write(gluon_data)?;
        self.chroma_location_x.write(gluon_data)?;
        self.chroma_location_y.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let coefficients = gluon::Convertable::read(gluon_data)?;
        let range = gluon::Convertable::read(gluon_data)?;
        let chroma_location_x = gluon::Convertable::read(gluon_data)?;
        let chroma_location_y = gluon::Convertable::read(gluon_data)?;
        Ok(YcbcrFormat {
            coefficients,
            range,
            chroma_location_x,
            chroma_location_y,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.coefficients.write_owned(gluon_data)?;
        self.range.write_owned(gluon_data)?;
        self.chroma_location_x.write_owned(gluon_data)?;
        self.chroma_location_y.write_owned(gluon_data)?;
        Ok(())
    }
}
///Size of a DMA texture.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DmatexSize {
    Size1D { size: u32 },
    Size2D { size: crate::types::Size2 },
    Size3D { size: crate::types::Size3 },
}
impl gluon::Convertable for DmatexSize {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
///List of dmatex planes
#[derive(Debug)]
pub enum DmatexPlanes {
    Simple { dmabuf_fd: std::os::fd::OwnedFd, planes: Vec<DmatexPlane> },
    ///Only allowed if the format advertises disjoint support
    Disjoint { planes: Vec<DisjointDmatexPlane> },
}
impl gluon::Convertable for DmatexPlanes {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            DmatexPlanes::Simple { dmabuf_fd, planes } => {
                gluon_data.write_u16(0u16)?;
                dmabuf_fd.write(gluon_data)?;
                planes.write(gluon_data)?;
            }
            DmatexPlanes::Disjoint { planes } => {
                gluon_data.write_u16(1u16)?;
                planes.write(gluon_data)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => {
                    let dmabuf_fd = gluon::Convertable::read(gluon_data)?;
                    let planes = gluon::Convertable::read(gluon_data)?;
                    DmatexPlanes::Simple {
                        dmabuf_fd,
                        planes,
                    }
                }
                1u16 => {
                    let planes = gluon::Convertable::read(gluon_data)?;
                    DmatexPlanes::Disjoint { planes }
                }
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            DmatexPlanes::Simple { dmabuf_fd, planes } => {
                gluon_data.write_u16(0u16)?;
                dmabuf_fd.write_owned(gluon_data)?;
                planes.write_owned(gluon_data)?;
            }
            DmatexPlanes::Disjoint { planes } => {
                gluon_data.write_u16(1u16)?;
                planes.write_owned(gluon_data)?;
            }
        };
        Ok(())
    }
}
///Alpha mode used by this Dmatex
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AlphaMode {
    /**The stored color values are premultiplied instead of being premuliplied in linear light.
Wayland uses this by default, consider using PremultipliedOptical instead if you can,
as its the correct solution for blending.*/
    PremultipliedElectrical,
    /**The colors where premultiplied in linear light and then encoded into their storage values.
This is the correct way to handle premultiplication, but may be more expensive if you do everything in electrical values (but blending is only correct in linear anyway)*/
    PremultipliedOptical,
}
impl gluon::Convertable for AlphaMode {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            AlphaMode::PremultipliedElectrical => {
                gluon_data.write_u16(0u16)?;
            }
            AlphaMode::PremultipliedOptical => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => AlphaMode::PremultipliedElectrical,
                1u16 => AlphaMode::PremultipliedOptical,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            AlphaMode::PremultipliedElectrical => {
                gluon_data.write_u16(0u16)?;
            }
            AlphaMode::PremultipliedOptical => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
}
///YCbCr Coeficients or YCbCr model in vulkan terms
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum YcbcrCoefficients {
    ///Maps onto vulkans YCbCr identity
    Identity,
    /**The input values are converted according to the
[ITU-R BT.709](https://en.wikipedia.org/wiki/Rec._709) standard.*/
    Bt709,
    /**The input values are converted according to the
[ITU-R BT.601](https://en.wikipedia.org/wiki/Rec._601) standard.*/
    Bt601,
    /**The input values are converted according to the
[ITU-R BT.2020](https://en.wikipedia.org/wiki/Rec._2020) standard.*/
    Bt2020,
}
impl gluon::Convertable for YcbcrCoefficients {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            YcbcrCoefficients::Identity => {
                gluon_data.write_u16(0u16)?;
            }
            YcbcrCoefficients::Bt709 => {
                gluon_data.write_u16(1u16)?;
            }
            YcbcrCoefficients::Bt601 => {
                gluon_data.write_u16(2u16)?;
            }
            YcbcrCoefficients::Bt2020 => {
                gluon_data.write_u16(3u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => YcbcrCoefficients::Identity,
                1u16 => YcbcrCoefficients::Bt709,
                2u16 => YcbcrCoefficients::Bt601,
                3u16 => YcbcrCoefficients::Bt2020,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            YcbcrCoefficients::Identity => {
                gluon_data.write_u16(0u16)?;
            }
            YcbcrCoefficients::Bt709 => {
                gluon_data.write_u16(1u16)?;
            }
            YcbcrCoefficients::Bt601 => {
                gluon_data.write_u16(2u16)?;
            }
            YcbcrCoefficients::Bt2020 => {
                gluon_data.write_u16(3u16)?;
            }
        };
        Ok(())
    }
}
///Only relevant for subsampled YCbCr
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum YcbcrChromaLocation {
    ///The chroma components are sampled at the even luma coordinate.
    CositedEven,
    /**The chroma components are sampled at the midpoint between the even luma coordinate and
the next higher odd luma coordinate.*/
    Midpoint,
}
impl gluon::Convertable for YcbcrChromaLocation {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            YcbcrChromaLocation::CositedEven => {
                gluon_data.write_u16(0u16)?;
            }
            YcbcrChromaLocation::Midpoint => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => YcbcrChromaLocation::CositedEven,
                1u16 => YcbcrChromaLocation::Midpoint,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            YcbcrChromaLocation::CositedEven => {
                gluon_data.write_u16(0u16)?;
            }
            YcbcrChromaLocation::Midpoint => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
}
///The used range of values in a YCbCr Dmatex
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum YcbcrRange {
    Full,
    Limited,
}
impl gluon::Convertable for YcbcrRange {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            YcbcrRange::Full => {
                gluon_data.write_u16(0u16)?;
            }
            YcbcrRange::Limited => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => YcbcrRange::Full,
                1u16 => YcbcrRange::Limited,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        match self {
            YcbcrRange::Full => {
                gluon_data.write_u16(0u16)?;
            }
            YcbcrRange::Limited => {
                gluon_data.write_u16(1u16)?;
            }
        };
        Ok(())
    }
}
///Error potentially produced when loading a model
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DmatexImportError {
    InvalidSize,
    InvalidFormat,
    UnsupportedArrayLayers { max_supported_layers: u32 },
    InvalidPlanes,
    InvalidTimelineFd,
    InternalImportError,
}
impl gluon::Convertable for DmatexImportError {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
    obj: gluon::Ref,
}
impl gluon::Convertable for DmatexRef {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(DmatexRef::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl DmatexRef {
    const ID: &'static str = "org.stardustxr.Dmatex.DmatexRef";
}
impl gluon::Interface for DmatexRef {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: DmatexRefHandler> gluon::HandledBy<H> for DmatexRef {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type DmatexRefLocal<H> = gluon::LocalRef<DmatexRef, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: DmatexRefHandler> From<DmatexRefLocal<H>> for DmatexRef {
    fn from(value: DmatexRefLocal<H>) -> DmatexRef {
        value.into_proxy()
    }
}
impl gluon::RefExt for DmatexRef {
    fn from_ref(obj: gluon::Ref) -> DmatexRef {
        DmatexRef { obj }
    }
}
impl DmatexRef {
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> DmatexRef {
        DmatexRef { obj }
    }
}
impl From<DmatexRef> for gluon::Ref {
    fn from(value: DmatexRef) -> Self {
        value.obj
    }
}
impl gluon::ToRef for DmatexRef {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for DmatexRef {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
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
    fn to_node(
        self,
    ) -> Result<(gluon::Node<Self>, gluon::LocalRef<DmatexRef, Self>), gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        DmatexRef::new_node(self)
    }
    fn to_service(self) -> Result<gluon::LocalRef<DmatexRef, Self>, gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        DmatexRef::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct DmatexInterface {
    obj: gluon::Ref,
}
impl gluon::Convertable for DmatexInterface {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(DmatexInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl DmatexInterface {
    const ID: &'static str = "org.stardustxr.Dmatex.DmatexInterface";
}
impl gluon::Interface for DmatexInterface {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: DmatexInterfaceHandler> gluon::HandledBy<H> for DmatexInterface {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type DmatexInterfaceLocal<H> = gluon::LocalRef<DmatexInterface, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: DmatexInterfaceHandler> From<DmatexInterfaceLocal<H>> for DmatexInterface {
    fn from(value: DmatexInterfaceLocal<H>) -> DmatexInterface {
        value.into_proxy()
    }
}
impl gluon::RefExt for DmatexInterface {
    fn from_ref(obj: gluon::Ref) -> DmatexInterface {
        DmatexInterface { obj }
    }
}
impl DmatexInterface {
    pub async fn import_dmatex(
        &self,
        size: impl Into<DmatexSize>,
        format: impl Into<DmatexFormat>,
        array_layers: impl Into<u32>,
        planes: impl Into<DmatexPlanes>,
        timeline_syncobj_fd: impl Into<std::os::fd::OwnedFd>,
    ) -> Result<Result<DmatexRef, DmatexImportError>, gluon::SendError> {
        let size: DmatexSize = size.into();
        let format: DmatexFormat = format.into();
        let array_layers: u32 = array_layers.into();
        let planes: DmatexPlanes = planes.into();
        let timeline_syncobj_fd: std::os::fd::OwnedFd = timeline_syncobj_fd.into();
        tracing::trace!(
            interface = "DmatexInterface", method = "import_dmatex", ? size, ? format, ?
            array_layers, ? planes, ? timeline_syncobj_fd, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        size.write(&mut gluon_builder)?;
        format.write(&mut gluon_builder)?;
        array_layers.write(&mut gluon_builder)?;
        planes.write(&mut gluon_builder)?;
        timeline_syncobj_fd.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_dmatex = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "DmatexInterface", method = "import_dmatex", ? __ret_dmatex,
            "←"
        );
        Ok(__ret_dmatex)
    }
    pub async fn enumerate_formats(
        &self,
        render_node: impl Into<u64>,
    ) -> Result<Option<Vec<DmatexFormatInfo>>, gluon::SendError> {
        let render_node: u64 = render_node.into();
        tracing::trace!(
            interface = "DmatexInterface", method = "enumerate_formats", ? render_node,
            "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        render_node.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_formats = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "DmatexInterface", method = "enumerate_formats", ? __ret_formats,
            "←"
        );
        Ok(__ret_formats)
    }
    pub async fn primary_render_node_id(&self) -> Result<u64, gluon::SendError> {
        tracing::trace!(
            interface = "DmatexInterface", method = "primary_render_node_id", "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 10u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_drm_render_node_id = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "DmatexInterface", method = "primary_render_node_id", ?
            __ret_drm_render_node_id, "←"
        );
        Ok(__ret_drm_render_node_id)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> DmatexInterface {
        DmatexInterface { obj }
    }
}
impl From<DmatexInterface> for gluon::Ref {
    fn from(value: DmatexInterface) -> Self {
        value.obj
    }
}
impl gluon::ToRef for DmatexInterface {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for DmatexInterface {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
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
        planes: DmatexPlanes,
        timeline_syncobj_fd: std::os::fd::OwnedFd,
    ) -> impl Future<Output = Result<DmatexRef, DmatexImportError>> + Send + Sync;
    ///Dispatched instead of [`Self::import_dmatex`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `import_dmatex` and sends the result through `reply`. Override this method instead of `import_dmatex` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn import_dmatex_oneway(
        &self,
        _ctx: gluon::Context,
        size: DmatexSize,
        format: DmatexFormat,
        array_layers: u32,
        planes: DmatexPlanes,
        timeline_syncobj_fd: std::os::fd::OwnedFd,
        reply: gluon::ReplySender<Result<DmatexRef, DmatexImportError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let dmatex = self
                .import_dmatex(
                    _ctx,
                    size,
                    format,
                    array_layers,
                    planes,
                    timeline_syncobj_fd,
                )
                .await;
            reply.send(dmatex)
        }
    }
    fn enumerate_formats(
        &self,
        _ctx: gluon::Context,
        render_node: u64,
    ) -> impl Future<Output = Option<Vec<DmatexFormatInfo>>> + Send + Sync;
    ///Dispatched instead of [`Self::enumerate_formats`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `enumerate_formats` and sends the result through `reply`. Override this method instead of `enumerate_formats` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn enumerate_formats_oneway(
        &self,
        _ctx: gluon::Context,
        render_node: u64,
        reply: gluon::ReplySender<Option<Vec<DmatexFormatInfo>>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let formats = self.enumerate_formats(_ctx, render_node).await;
            reply.send(formats)
        }
    }
    fn primary_render_node_id(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = u64> + Send + Sync;
    ///Dispatched instead of [`Self::primary_render_node_id`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `primary_render_node_id` and sends the result through `reply`. Override this method instead of `primary_render_node_id` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn primary_render_node_id_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<u64>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let drm_render_node_id = self.primary_render_node_id(_ctx).await;
            reply.send(drm_render_node_id)
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
                    let param_size = gluon::Convertable::read(&mut gluon_data)?;
                    let param_format = gluon::Convertable::read(&mut gluon_data)?;
                    let param_array_layers = gluon::Convertable::read(&mut gluon_data)?;
                    let param_planes = gluon::Convertable::read(&mut gluon_data)?;
                    let param_timeline_syncobj_fd = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    tracing::trace!(
                        interface = "DmatexInterface", method = "import_dmatex", ?
                        param_size, ? param_format, ? param_array_layers, ? param_planes,
                        ? param_timeline_syncobj_fd, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<DmatexRef, DmatexImportError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |dmatex, gluon_out| {
                            tracing::trace!(
                                interface = "DmatexInterface", method = "import_dmatex", ?
                                dmatex, "←"
                            );
                            dmatex.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.import_dmatex_oneway(
                            ctx,
                            param_size,
                            param_format,
                            param_array_layers,
                            param_planes,
                            param_timeline_syncobj_fd,
                            reply,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "DmatexInterface", method =
                                "import_dmatex", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_render_node = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "DmatexInterface", method = "enumerate_formats", ?
                        param_render_node, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Option<Vec<DmatexFormatInfo>>> = gluon::ReplySender::new(
                        return_callback,
                        |formats, gluon_out| {
                            tracing::trace!(
                                interface = "DmatexInterface", method = "enumerate_formats",
                                ? formats, "←"
                            );
                            formats.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.enumerate_formats_oneway(ctx, param_render_node, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "DmatexInterface", method =
                                "enumerate_formats", method_id = 9u32
                            ),
                        )
                        .await?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    tracing::trace!(
                        interface = "DmatexInterface", method = "primary_render_node_id",
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<u64> = gluon::ReplySender::new(
                        return_callback,
                        |drm_render_node_id, gluon_out| {
                            tracing::trace!(
                                interface = "DmatexInterface", method =
                                "primary_render_node_id", ? drm_render_node_id, "←"
                            );
                            drm_render_node_id.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.primary_render_node_id_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "DmatexInterface", method =
                                "primary_render_node_id", method_id = 10u32
                            ),
                        )
                        .await?;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon::Node<Self>, gluon::LocalRef<DmatexInterface, Self>),
        gluon::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon::RefExt;
        DmatexInterface::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon::LocalRef<DmatexInterface, Self>, gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        DmatexInterface::new_service(self)
    }
}
#[derive(Debug, Clone)]
pub struct DmatexSubmitRelease {
    obj: gluon::Ref,
}
impl gluon::Convertable for DmatexSubmitRelease {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(DmatexSubmitRelease::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl DmatexSubmitRelease {
    const ID: &'static str = "org.stardustxr.Dmatex.DmatexSubmitRelease";
}
impl gluon::Interface for DmatexSubmitRelease {
    const ID: &'static str = Self::ID;
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: DmatexSubmitReleaseHandler> gluon::HandledBy<H> for DmatexSubmitRelease {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type DmatexSubmitReleaseLocal<H> = gluon::LocalRef<DmatexSubmitRelease, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: DmatexSubmitReleaseHandler> From<DmatexSubmitReleaseLocal<H>>
for DmatexSubmitRelease {
    fn from(value: DmatexSubmitReleaseLocal<H>) -> DmatexSubmitRelease {
        value.into_proxy()
    }
}
impl gluon::RefExt for DmatexSubmitRelease {
    fn from_ref(obj: gluon::Ref) -> DmatexSubmitRelease {
        DmatexSubmitRelease { obj }
    }
}
impl DmatexSubmitRelease {
    ///Consume the release point, after you get the release point you have to signal it at some point!
    pub async fn consume(&self) -> Result<u64, gluon::SendError> {
        tracing::trace!(interface = "DmatexSubmitRelease", method = "consume", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (mut gluon_recv, gluon_ret) = gluon::ReturnReceiver::new()?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        let __ret_release_point = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "DmatexSubmitRelease", method = "consume", ? __ret_release_point,
            "←"
        );
        Ok(__ret_release_point)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> DmatexSubmitRelease {
        DmatexSubmitRelease { obj }
    }
}
impl From<DmatexSubmitRelease> for gluon::Ref {
    fn from(value: DmatexSubmitRelease) -> Self {
        value.obj
    }
}
impl gluon::ToRef for DmatexSubmitRelease {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for DmatexSubmitRelease {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
    }
}
impl std::hash::Hash for DmatexSubmitRelease {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for DmatexSubmitRelease {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for DmatexSubmitRelease {}
pub trait DmatexSubmitReleaseHandler: gluon::Handler + Send + Sync + 'static {
    ///Consume the release point, after you get the release point you have to signal it at some point!
    fn consume(&self, _ctx: gluon::Context) -> impl Future<Output = u64> + Send + Sync;
    ///Dispatched instead of [`Self::consume`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `consume` and sends the result through `reply`. Override this method instead of `consume` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn consume_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<u64>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let release_point = self.consume(_ctx).await;
            reply.send(release_point)
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
                    tracing::trace!(
                        interface = "DmatexSubmitRelease", method = "consume",
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<u64> = gluon::ReplySender::new(
                        return_callback,
                        |release_point, gluon_out| {
                            tracing::trace!(
                                interface = "DmatexSubmitRelease", method = "consume", ?
                                release_point, "←"
                            );
                            release_point.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.consume_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "DmatexSubmitRelease", method =
                                "consume", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                _ => {}
            }
            Ok(())
        }
    }
    fn to_node(
        self,
    ) -> Result<
        (gluon::Node<Self>, gluon::LocalRef<DmatexSubmitRelease, Self>),
        gluon::NodeError,
    >
    where
        Self: Sized,
    {
        use gluon::RefExt;
        DmatexSubmitRelease::new_node(self)
    }
    fn to_service(
        self,
    ) -> Result<gluon::LocalRef<DmatexSubmitRelease, Self>, gluon::NodeError>
    where
        Self: Sized,
    {
        use gluon::RefExt;
        DmatexSubmitRelease::new_service(self)
    }
}
pub mod proxied {
    use super::*;
}
