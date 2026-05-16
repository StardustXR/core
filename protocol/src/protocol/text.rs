#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Text",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "TextBounds",
            supported_derives: gluon_wire::Derives::from_bits_truncate(3u32),
        },
        gluon_wire::ExternalGluonType {
            name: "TextStyle",
            supported_derives: gluon_wire::Derives::from_bits_truncate(2u32),
        },
        gluon_wire::ExternalGluonType {
            name: "XAlign",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
        gluon_wire::ExternalGluonType {
            name: "YAlign",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
        gluon_wire::ExternalGluonType {
            name: "TextFit",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
    ],
};
///Bounds for text
#[derive(Debug, Copy, Clone)]
pub struct TextBounds {
    ///Bounds in meters
    pub bounds: mint::Vector2<f32>,
    pub fit: TextFit,
    pub anchor_align_x: XAlign,
    pub anchor_align_y: YAlign,
}
impl gluon_wire::GluonConvertable for TextBounds {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: super::types::Vec2F = self.bounds.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.fit.write(gluon_data)?;
        self.anchor_align_x.write(gluon_data)?;
        self.anchor_align_y.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let bounds: mint::Vector2<f32> = {
            let __w: super::types::Vec2F = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let fit = gluon_wire::GluonConvertable::read(gluon_data)?;
        let anchor_align_x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let anchor_align_y = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(TextBounds {
            bounds,
            fit,
            anchor_align_x,
            anchor_align_y,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        {
            let __w: super::types::Vec2F = self.bounds.into();
            __w.write_owned(gluon_data)?;
        }
        self.fit.write_owned(gluon_data)?;
        self.anchor_align_x.write_owned(gluon_data)?;
        self.anchor_align_y.write_owned(gluon_data)?;
        Ok(())
    }
}
///Styling info for text
#[derive(Debug, Clone)]
pub struct TextStyle {
    ///Height of a character in meters
    pub character_height: f32,
    pub color: crate::Color,
    pub text_align_x: XAlign,
    pub text_align_y: YAlign,
    pub font: Option<super::types::Resource>,
    pub bounds: Option<TextBounds>,
}
impl gluon_wire::GluonConvertable for TextStyle {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.character_height.write(gluon_data)?;
        {
            let __w: super::types::Color = self.color.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.text_align_x.write(gluon_data)?;
        self.text_align_y.write(gluon_data)?;
        self.font.write(gluon_data)?;
        self.bounds.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let character_height = gluon_wire::GluonConvertable::read(gluon_data)?;
        let color: crate::Color = {
            let __w: super::types::Color = gluon_wire::GluonConvertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let text_align_x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let text_align_y = gluon_wire::GluonConvertable::read(gluon_data)?;
        let font = gluon_wire::GluonConvertable::read(gluon_data)?;
        let bounds = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(TextStyle {
            character_height,
            color,
            text_align_x,
            text_align_y,
            font,
            bounds,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.character_height.write_owned(gluon_data)?;
        {
            let __w: super::types::Color = self.color.into();
            __w.write_owned(gluon_data)?;
        }
        self.text_align_x.write_owned(gluon_data)?;
        self.text_align_y.write_owned(gluon_data)?;
        self.font.write_owned(gluon_data)?;
        self.bounds.write_owned(gluon_data)?;
        Ok(())
    }
}
///X alignment
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum XAlign {
    Left,
    Center,
    Right,
}
impl gluon_wire::GluonConvertable for XAlign {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            XAlign::Left => {
                gluon_data.write_u16(0u16)?;
            }
            XAlign::Center => {
                gluon_data.write_u16(1u16)?;
            }
            XAlign::Right => {
                gluon_data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => XAlign::Left,
                1u16 => XAlign::Center,
                2u16 => XAlign::Right,
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            XAlign::Left => {
                gluon_data.write_u16(0u16)?;
            }
            XAlign::Center => {
                gluon_data.write_u16(1u16)?;
            }
            XAlign::Right => {
                gluon_data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
}
///Y alignment
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum YAlign {
    Top,
    Center,
    Bottom,
}
impl gluon_wire::GluonConvertable for YAlign {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            YAlign::Top => {
                gluon_data.write_u16(0u16)?;
            }
            YAlign::Center => {
                gluon_data.write_u16(1u16)?;
            }
            YAlign::Bottom => {
                gluon_data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => YAlign::Top,
                1u16 => YAlign::Center,
                2u16 => YAlign::Bottom,
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            YAlign::Top => {
                gluon_data.write_u16(0u16)?;
            }
            YAlign::Center => {
                gluon_data.write_u16(1u16)?;
            }
            YAlign::Bottom => {
                gluon_data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
}
///How the text fits in a box of any size
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TextFit {
    Wrap,
    Clip,
    Squeeze,
    Exact,
    Overflow,
}
impl gluon_wire::GluonConvertable for TextFit {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            TextFit::Wrap => {
                gluon_data.write_u16(0u16)?;
            }
            TextFit::Clip => {
                gluon_data.write_u16(1u16)?;
            }
            TextFit::Squeeze => {
                gluon_data.write_u16(2u16)?;
            }
            TextFit::Exact => {
                gluon_data.write_u16(3u16)?;
            }
            TextFit::Overflow => {
                gluon_data.write_u16(4u16)?;
            }
        };
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => TextFit::Wrap,
                1u16 => TextFit::Clip,
                2u16 => TextFit::Squeeze,
                3u16 => TextFit::Exact,
                4u16 => TextFit::Overflow,
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            TextFit::Wrap => {
                gluon_data.write_u16(0u16)?;
            }
            TextFit::Clip => {
                gluon_data.write_u16(1u16)?;
            }
            TextFit::Squeeze => {
                gluon_data.write_u16(2u16)?;
            }
            TextFit::Exact => {
                gluon_data.write_u16(3u16)?;
            }
            TextFit::Overflow => {
                gluon_data.write_u16(4u16)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct TextInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for TextInterface {
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
        Ok(TextInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl TextInterface {
    pub async fn create_text(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        text: impl Into<String>,
        style: impl Into<TextStyle>,
    ) -> Result<Text, gluon_wire::GluonSendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let text: String = text.into();
        let style: TextStyle = style.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        text.write(&mut gluon_builder)?;
        style.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: TextInterfaceHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> TextInterface {
        TextInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> TextInterface {
        TextInterface { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for TextInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for TextInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for TextInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for TextInterface {}
pub trait TextInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn create_text(
        &self,
        _ctx: gluon_wire::GluonCtx,
        spatial: super::spatial::Spatial,
        text: String,
        style: TextStyle,
    ) -> impl Future<Output = Text> + Send + Sync;
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
                    let param_text = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_style = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (text) = self
                        .create_text(ctx, param_spatial, param_text, param_style)
                        .await;
                    drop(gluon_data);
                    text.write_owned(&mut gluon_out)?;
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
#[derive(Debug, Clone)]
pub struct Text {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for Text {
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
        Ok(Text::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Text {
    ///Set the character height in meters
    pub fn set_character_height(
        &self,
        height: impl Into<f32>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let height: f32 = height.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        height.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///Set the text content
    pub fn set_text(
        &self,
        text: impl Into<String>,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let text: String = text.into();
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        text.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: TextHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
    ) -> Text {
        Text::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Text {
        Text { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for Text {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for Text {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Text {}
pub trait TextHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Set the character height in meters
    fn set_character_height(
        &self,
        _ctx: gluon_wire::GluonCtx,
        height: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///Set the text content
    fn set_text(
        &self,
        _ctx: gluon_wire::GluonCtx,
        text: String,
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
                    let param_height = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_character_height(ctx, param_height).await;
                }
                9u32 => {
                    let param_text = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_text(ctx, param_text).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
