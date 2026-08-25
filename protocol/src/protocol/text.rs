#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Text",
    types: &[
        gluon::ExternalGluonType {
            name: "TextBounds",
            supported_derives: gluon::Derives::from_bits_truncate(779u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "TextStyle",
            supported_derives: gluon::Derives::from_bits_truncate(778u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "XAlign",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "YAlign",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "TextFit",
            supported_derives: gluon::Derives::from_bits_truncate(799u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///Bounds for text
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextBounds {
    ///Bounds in meters
    pub bounds: crate::types::Vec2F,
    pub fit: TextFit,
    pub anchor_align_x: XAlign,
    pub anchor_align_y: YAlign,
}
impl gluon::Convertable for TextBounds {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Vec2F = self.bounds.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.fit.write(gluon_data)?;
        self.anchor_align_x.write(gluon_data)?;
        self.anchor_align_y.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let bounds: crate::types::Vec2F = {
            let __w: super::types::proxied::Vec2F = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let fit = gluon::Convertable::read(gluon_data)?;
        let anchor_align_x = gluon::Convertable::read(gluon_data)?;
        let anchor_align_y = gluon::Convertable::read(gluon_data)?;
        Ok(TextBounds {
            bounds,
            fit,
            anchor_align_x,
            anchor_align_y,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        {
            let __w: super::types::proxied::Vec2F = self.bounds.into();
            __w.write_owned(gluon_data)?;
        }
        self.fit.write_owned(gluon_data)?;
        self.anchor_align_x.write_owned(gluon_data)?;
        self.anchor_align_y.write_owned(gluon_data)?;
        Ok(())
    }
}
///Styling info for text
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextStyle {
    ///Height of a character in meters
    pub character_height: f32,
    pub color: crate::types::Color,
    pub text_align_x: XAlign,
    pub text_align_y: YAlign,
    pub font: Option<super::types::Resource>,
    pub bounds: Option<TextBounds>,
}
impl gluon::Convertable for TextStyle {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.character_height.write(gluon_data)?;
        {
            let __w: super::types::proxied::Color = self.color.clone().into();
            __w.write_owned(gluon_data)?;
        }
        self.text_align_x.write(gluon_data)?;
        self.text_align_y.write(gluon_data)?;
        self.font.write(gluon_data)?;
        self.bounds.write(gluon_data)?;
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let character_height = gluon::Convertable::read(gluon_data)?;
        let color: crate::types::Color = {
            let __w: super::types::proxied::Color = gluon::Convertable::read(
                gluon_data,
            )?;
            __w.into()
        };
        let text_align_x = gluon::Convertable::read(gluon_data)?;
        let text_align_y = gluon::Convertable::read(gluon_data)?;
        let font = gluon::Convertable::read(gluon_data)?;
        let bounds = gluon::Convertable::read(gluon_data)?;
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
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.character_height.write_owned(gluon_data)?;
        {
            let __w: super::types::proxied::Color = self.color.into();
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum XAlign {
    Left,
    Center,
    Right,
}
impl gluon::Convertable for XAlign {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
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
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => XAlign::Left,
                1u16 => XAlign::Center,
                2u16 => XAlign::Right,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum YAlign {
    Top,
    Center,
    Bottom,
}
impl gluon::Convertable for YAlign {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
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
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => YAlign::Top,
                1u16 => YAlign::Center,
                2u16 => YAlign::Bottom,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TextFit {
    Wrap,
    Clip,
    Squeeze,
    Exact,
    Overflow,
}
impl gluon::Convertable for TextFit {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
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
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => TextFit::Wrap,
                1u16 => TextFit::Clip,
                2u16 => TextFit::Squeeze,
                3u16 => TextFit::Exact,
                4u16 => TextFit::Overflow,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
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
    obj: gluon::Ref,
}
impl gluon::Convertable for TextInterface {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(TextInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for TextInterface {
    const ID: &'static str = "org.stardustxr.Text.TextInterface";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: TextInterfaceHandler> gluon::HandledBy<H> for TextInterface {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type TextInterfaceLocal<H> = gluon::LocalRef<TextInterface, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: TextInterfaceHandler> From<TextInterfaceLocal<H>> for TextInterface {
    fn from(value: TextInterfaceLocal<H>) -> TextInterface {
        value.into_proxy()
    }
}
impl gluon::RefExt for TextInterface {
    fn from_ref(obj: gluon::Ref) -> TextInterface {
        TextInterface { obj }
    }
}
impl TextInterface {
    pub async fn create_text(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        text: impl Into<String>,
        style: impl Into<TextStyle>,
    ) -> Result<Result<Text, super::types::ResourceLoadError>, gluon::SendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let text: String = text.into();
        let style: TextStyle = style.into();
        tracing::trace!(
            interface = "TextInterface", method = "create_text", ? spatial, ? text, ?
            style, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        text.write(&mut gluon_builder)?;
        style.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_text = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "TextInterface", method = "create_text", ? __ret_text, "←"
        );
        Ok(__ret_text)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> TextInterface {
        TextInterface { obj }
    }
}
impl From<TextInterface> for gluon::Ref {
    fn from(value: TextInterface) -> Self {
        value.obj
    }
}
impl gluon::ToRef for TextInterface {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for TextInterface {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
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
pub trait TextInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    fn create_text(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        text: String,
        style: TextStyle,
    ) -> impl Future<
        Output = Result<Text, super::types::ResourceLoadError>,
    > + Send + Sync;
    ///Dispatched instead of [`Self::create_text`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `create_text` and sends the result through `reply`. Override this method instead of `create_text` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn create_text_oneway(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        text: String,
        style: TextStyle,
        reply: gluon::ReplySender<Result<Text, super::types::ResourceLoadError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let text = self.create_text(_ctx, spatial, text, style).await;
            reply.send(text)
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
                    let param_text = gluon::Convertable::read(&mut gluon_data)?;
                    let param_style = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "TextInterface", method = "create_text", ?
                        param_spatial, ? param_text, ? param_style, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<Text, super::types::ResourceLoadError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |text, gluon_out| {
                            tracing::trace!(
                                interface = "TextInterface", method = "create_text", ? text,
                                "←"
                            );
                            text.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.create_text_oneway(
                            ctx,
                            param_spatial,
                            param_text,
                            param_style,
                            reply,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "TextInterface", method =
                                "create_text", method_id = 8u32
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
#[derive(Debug, Clone)]
pub struct Text {
    obj: gluon::Ref,
}
impl gluon::Convertable for Text {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(Text::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for Text {
    const ID: &'static str = "org.stardustxr.Text.Text";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: TextHandler> gluon::HandledBy<H> for Text {}
///A proxy this process made, carrying the handler behind it — see [`gluon::LocalRef`]. Handed back by [`gluon::RefExt::new_node`] and [`gluon::RefExt::new_service`].
pub type TextLocal<H> = gluon::LocalRef<Text, H>;
///Drops the handler share and keeps the proxy, so a [`gluon::LocalRef`] goes anywhere this proxy does — including the `impl Into<Self>` parameters generated for typed refs.
impl<H: TextHandler> From<TextLocal<H>> for Text {
    fn from(value: TextLocal<H>) -> Text {
        value.into_proxy()
    }
}
impl gluon::RefExt for Text {
    fn from_ref(obj: gluon::Ref) -> Text {
        Text { obj }
    }
}
impl Text {
    ///Set the character height in meters
    pub fn set_character_height(
        &self,
        height: impl Into<f32>,
    ) -> Result<(), gluon::SendError> {
        let height: f32 = height.into();
        tracing::trace!(
            interface = "Text", method = "set_character_height", ? height, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        height.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        Ok(())
    }
    ///Set the text content
    pub fn set_text(&self, text: impl Into<String>) -> Result<(), gluon::SendError> {
        let text: String = text.into();
        tracing::trace!(interface = "Text", method = "set_text", ? text, "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        text.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> Text {
        Text { obj }
    }
}
impl From<Text> for gluon::Ref {
    fn from(value: Text) -> Self {
        value.obj
    }
}
impl gluon::ToRef for Text {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for Text {
    fn death_notifier(&self) -> gluon::DeathNotifier {
        gluon::Liveness::death_notifier(&self.obj)
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
pub trait TextHandler: gluon::Handler + Send + Sync + 'static {
    ///Set the character height in meters
    fn set_character_height(
        &self,
        _ctx: gluon::Context,
        height: f32,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///Set the text content
    fn set_text(
        &self,
        _ctx: gluon::Context,
        text: String,
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
                    let param_height = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Text", method = "set_character_height", ?
                        param_height, "dispatching"
                    );
                    drop(gluon_data);
                    self.set_character_height(ctx, param_height)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Text", method =
                                "set_character_height", method_id = 8u32
                            ),
                        )
                        .await;
                }
                9u32 => {
                    let param_text = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Text", method = "set_text", ? param_text,
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.set_text(ctx, param_text)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Text", method = "set_text",
                                method_id = 9u32
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
pub mod proxied {
    use super::*;
}
