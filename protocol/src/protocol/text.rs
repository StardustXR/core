#![allow(unused, clippy::single_match, clippy::match_single_binding)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Text",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "TextBounds",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "TextStyle",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextBounds {
    ///Bounds in meters
    pub bounds: super::types::Vec2F,
    pub fit: TextFit,
    pub anchor_align_x: XAlign,
    pub anchor_align_y: YAlign,
}
impl gluon_wire::GluonConvertable for TextBounds {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.bounds.write(gluon_data)?;
        self.fit.write(gluon_data)?;
        self.anchor_align_x.write(gluon_data)?;
        self.anchor_align_y.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let bounds = gluon_wire::GluonConvertable::read(gluon_data)?;
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
        self.bounds.write_owned(gluon_data)?;
        self.fit.write_owned(gluon_data)?;
        self.anchor_align_x.write_owned(gluon_data)?;
        self.anchor_align_y.write_owned(gluon_data)?;
        Ok(())
    }
}
///Styling info for text
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TextStyle {
    ///Height of a character in meters
    pub character_height: f32,
    pub color: super::types::Color,
    pub text_align_x: XAlign,
    pub text_align_y: YAlign,
    pub bounds: Option<TextBounds>,
}
impl gluon_wire::GluonConvertable for TextStyle {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.character_height.write(gluon_data)?;
        self.color.write(gluon_data)?;
        self.text_align_x.write(gluon_data)?;
        self.text_align_y.write(gluon_data)?;
        self.bounds.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let character_height = gluon_wire::GluonConvertable::read(gluon_data)?;
        let color = gluon_wire::GluonConvertable::read(gluon_data)?;
        let text_align_x = gluon_wire::GluonConvertable::read(gluon_data)?;
        let text_align_y = gluon_wire::GluonConvertable::read(gluon_data)?;
        let bounds = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(TextStyle {
            character_height,
            color,
            text_align_x,
            text_align_y,
            bounds,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.character_height.write_owned(gluon_data)?;
        self.color.write_owned(gluon_data)?;
        self.text_align_x.write_owned(gluon_data)?;
        self.text_align_y.write_owned(gluon_data)?;
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
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
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
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        text: String,
        style: TextStyle,
    ) -> Result<Text, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.create_text_blocking(parent, transform, text, style)
            })
            .await
            .unwrap()
    }
    pub fn create_text_blocking(
        &self,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        text: String,
        style: TextStyle,
    ) -> Result<Text, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        parent.write(&mut gluon_builder)?;
        transform.write(&mut gluon_builder)?;
        text.write(&mut gluon_builder)?;
        style.write(&mut gluon_builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: TextInterfaceHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
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
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        TextInterface {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for TextInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait TextInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn create_text(
        &self,
        _ctx: gluon_wire::GluonCtx,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        text: String,
        style: TextStyle,
    ) -> impl Future<Output = Text> + Send + Sync;
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
                    let (text) = self
                        .create_text(
                            ctx,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                            gluon_wire::GluonConvertable::read(gluon_data)?,
                        )
                        .await;
                    text.write_owned(&mut out)?;
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
#[derive(Debug, Clone)]
pub struct Text {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
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
    pub async fn get_spatial(
        &self,
    ) -> Result<super::spatial::Spatial, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_spatial_blocking()).await.unwrap()
    }
    pub fn get_spatial_blocking(
        &self,
    ) -> Result<super::spatial::Spatial, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, gluon_builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Set the character height in meters
    pub fn set_character_height(
        &self,
        height: f32,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        height.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///Set the text content
    pub fn set_text(&self, text: String) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        text.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: TextHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
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
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        gluon_builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, gluon_builder.to_payload());
        Text { obj, drop_notification }
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
impl binderbinder::binder_object::ToBinderObjectOrRef for Text {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait TextHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn get_spatial(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::spatial::Spatial> + Send + Sync;
    ///Set the character height in meters
    fn set_character_height(&self, _ctx: gluon_wire::GluonCtx, height: f32);
    ///Set the text content
    fn set_text(&self, _ctx: gluon_wire::GluonCtx, text: String);
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
                9u32 => {
                    self.set_character_height(
                        ctx,
                        gluon_wire::GluonConvertable::read(gluon_data)?,
                    );
                }
                10u32 => {
                    self.set_text(ctx, gluon_wire::GluonConvertable::read(gluon_data)?);
                }
                _ => {}
            }
            Ok(())
        }
    }
}
