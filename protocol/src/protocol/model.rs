#![allow(unused, clippy::single_match, clippy::match_single_binding)]
use gluon_wire::GluonConvertable;
pub const EXTERNAL_PROTOCOL: gluon_wire::ExternalGluonProtocol = gluon_wire::ExternalGluonProtocol {
    protocol_name: "org.stardustxr.Model",
    types: &[
        gluon_wire::ExternalGluonType {
            name: "NonUniformTransform",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "PartialNonUniformTransform",
            supported_derives: gluon_wire::Derives::from_bits_truncate(11u32),
        },
        gluon_wire::ExternalGluonType {
            name: "ModelLoadError",
            supported_derives: gluon_wire::Derives::from_bits_truncate(31u32),
        },
        gluon_wire::ExternalGluonType {
            name: "MaterialParamError",
            supported_derives: gluon_wire::Derives::from_bits_truncate(30u32),
        },
        gluon_wire::ExternalGluonType {
            name: "MaterialParameter",
            supported_derives: gluon_wire::Derives::from_bits_truncate(2u32),
        },
    ],
};
///Transform with non-uniform scale
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NonUniformTransform {
    pub position: super::types::Vec3F,
    pub rotation: super::types::Quatf,
    pub scale: super::types::Vec3F,
}
impl gluon_wire::GluonConvertable for NonUniformTransform {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.position.write(data)?;
        self.rotation.write(data)?;
        self.scale.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let position = gluon_wire::GluonConvertable::read(data)?;
        let rotation = gluon_wire::GluonConvertable::read(data)?;
        let scale = gluon_wire::GluonConvertable::read(data)?;
        Ok(NonUniformTransform {
            position,
            rotation,
            scale,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.position.write_owned(data)?;
        self.rotation.write_owned(data)?;
        self.scale.write_owned(data)?;
        Ok(())
    }
}
///Partial version of NonUniformTransform
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PartialNonUniformTransform {
    pub position: Option<super::types::Vec3F>,
    pub rotation: Option<super::types::Quatf>,
    pub scale: Option<super::types::Vec3F>,
}
impl gluon_wire::GluonConvertable for PartialNonUniformTransform {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.position.write(data)?;
        self.rotation.write(data)?;
        self.scale.write(data)?;
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let position = gluon_wire::GluonConvertable::read(data)?;
        let rotation = gluon_wire::GluonConvertable::read(data)?;
        let scale = gluon_wire::GluonConvertable::read(data)?;
        Ok(PartialNonUniformTransform {
            position,
            rotation,
            scale,
        })
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.position.write_owned(data)?;
        self.rotation.write_owned(data)?;
        self.scale.write_owned(data)?;
        Ok(())
    }
}
///Error potentially produced when loading a model
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ModelLoadError {
    NotFound,
}
impl gluon_wire::GluonConvertable for ModelLoadError {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            ModelLoadError::NotFound => {
                data.write_u16(0u16)?;
            }
        };
        Ok(())
    }
    fn read(
        data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match data.read_u16()? {
                0u16 => ModelLoadError::NotFound,
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            ModelLoadError::NotFound => {
                data.write_u16(0u16)?;
            }
        };
        Ok(())
    }
}
///Error potentially produced when trying to set a material paramterer
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum MaterialParamError {
    ParamNotFound { known_params: Vec<String> },
    IncorrectType { valid_type: String },
    ///ModelPart had apply_holdout_material called before
    Holdout,
}
impl gluon_wire::GluonConvertable for MaterialParamError {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            MaterialParamError::ParamNotFound { known_params } => {
                data.write_u16(0u16)?;
                known_params.write(data)?;
            }
            MaterialParamError::IncorrectType { valid_type } => {
                data.write_u16(1u16)?;
                valid_type.write(data)?;
            }
            MaterialParamError::Holdout => {
                data.write_u16(2u16)?;
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
                    let known_params = gluon_wire::GluonConvertable::read(data)?;
                    MaterialParamError::ParamNotFound {
                        known_params,
                    }
                }
                1u16 => {
                    let valid_type = gluon_wire::GluonConvertable::read(data)?;
                    MaterialParamError::IncorrectType {
                        valid_type,
                    }
                }
                2u16 => MaterialParamError::Holdout,
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            MaterialParamError::ParamNotFound { known_params } => {
                data.write_u16(0u16)?;
                known_params.write_owned(data)?;
            }
            MaterialParamError::IncorrectType { valid_type } => {
                data.write_u16(1u16)?;
                valid_type.write_owned(data)?;
            }
            MaterialParamError::Holdout => {
                data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
}
///Material parameter values
#[derive(Debug, Clone)]
pub enum MaterialParameter {
    Boolean { value: bool },
    Int { value: i32 },
    Uint { value: u32 },
    Float { value: f32 },
    Vec2 { value: super::types::Vec2F },
    Vec3 { value: super::types::Vec3F },
    Texture { value: super::types::Resource },
    Dmatex {
        dmatex: super::dmatex::DmatexRef,
        ///After this point is reached the Server may access the texture
        acquire_point: u64,
        ///This point is reached once the Server is done accessing the texture
        release_point: u64,
    },
}
impl gluon_wire::GluonConvertable for MaterialParameter {
    fn write<'a, 'b: 'a>(
        &'b self,
        data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            MaterialParameter::Boolean { value } => {
                data.write_u16(0u16)?;
                value.write(data)?;
            }
            MaterialParameter::Int { value } => {
                data.write_u16(1u16)?;
                value.write(data)?;
            }
            MaterialParameter::Uint { value } => {
                data.write_u16(2u16)?;
                value.write(data)?;
            }
            MaterialParameter::Float { value } => {
                data.write_u16(3u16)?;
                value.write(data)?;
            }
            MaterialParameter::Vec2 { value } => {
                data.write_u16(4u16)?;
                value.write(data)?;
            }
            MaterialParameter::Vec3 { value } => {
                data.write_u16(5u16)?;
                value.write(data)?;
            }
            MaterialParameter::Texture { value } => {
                data.write_u16(6u16)?;
                value.write(data)?;
            }
            MaterialParameter::Dmatex { dmatex, acquire_point, release_point } => {
                data.write_u16(7u16)?;
                dmatex.write(data)?;
                acquire_point.write(data)?;
                release_point.write(data)?;
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
                    let value = gluon_wire::GluonConvertable::read(data)?;
                    MaterialParameter::Boolean {
                        value,
                    }
                }
                1u16 => {
                    let value = gluon_wire::GluonConvertable::read(data)?;
                    MaterialParameter::Int { value }
                }
                2u16 => {
                    let value = gluon_wire::GluonConvertable::read(data)?;
                    MaterialParameter::Uint { value }
                }
                3u16 => {
                    let value = gluon_wire::GluonConvertable::read(data)?;
                    MaterialParameter::Float { value }
                }
                4u16 => {
                    let value = gluon_wire::GluonConvertable::read(data)?;
                    MaterialParameter::Vec2 { value }
                }
                5u16 => {
                    let value = gluon_wire::GluonConvertable::read(data)?;
                    MaterialParameter::Vec3 { value }
                }
                6u16 => {
                    let value = gluon_wire::GluonConvertable::read(data)?;
                    MaterialParameter::Texture {
                        value,
                    }
                }
                7u16 => {
                    let dmatex = gluon_wire::GluonConvertable::read(data)?;
                    let acquire_point = gluon_wire::GluonConvertable::read(data)?;
                    let release_point = gluon_wire::GluonConvertable::read(data)?;
                    MaterialParameter::Dmatex {
                        dmatex,
                        acquire_point,
                        release_point,
                    }
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
            MaterialParameter::Boolean { value } => {
                data.write_u16(0u16)?;
                value.write_owned(data)?;
            }
            MaterialParameter::Int { value } => {
                data.write_u16(1u16)?;
                value.write_owned(data)?;
            }
            MaterialParameter::Uint { value } => {
                data.write_u16(2u16)?;
                value.write_owned(data)?;
            }
            MaterialParameter::Float { value } => {
                data.write_u16(3u16)?;
                value.write_owned(data)?;
            }
            MaterialParameter::Vec2 { value } => {
                data.write_u16(4u16)?;
                value.write_owned(data)?;
            }
            MaterialParameter::Vec3 { value } => {
                data.write_u16(5u16)?;
                value.write_owned(data)?;
            }
            MaterialParameter::Texture { value } => {
                data.write_u16(6u16)?;
                value.write_owned(data)?;
            }
            MaterialParameter::Dmatex { dmatex, acquire_point, release_point } => {
                data.write_u16(7u16)?;
                dmatex.write_owned(data)?;
                acquire_point.write_owned(data)?;
                release_point.write_owned(data)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct ModelInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for ModelInterface {
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
        Ok(ModelInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl ModelInterface {
    ///Load a GLTF model into a Model
    pub async fn load_model(
        &self,
        model: super::types::Resource,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        model_scale: super::types::Vec3F,
    ) -> Result<Model, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.load_model_blocking(model, parent, transform, model_scale)
            })
            .await
            .unwrap()
    }
    pub fn load_model_blocking(
        &self,
        model: super::types::Resource,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        model_scale: super::types::Vec3F,
    ) -> Result<Model, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        model.write(&mut builder)?;
        parent.write(&mut builder)?;
        transform.write(&mut builder)?;
        model_scale.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: ModelInterfaceHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> ModelInterface {
        ModelInterface::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> ModelInterface {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        ModelInterface {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for ModelInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait ModelInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Load a GLTF model into a Model
    fn load_model(
        &self,
        _ctx: gluon_wire::GluonCtx,
        model: super::types::Resource,
        parent: super::spatial::SpatialRef,
        transform: super::spatial::Transform,
        model_scale: super::types::Vec3F,
    ) -> impl Future<Output = Model> + Send + Sync;
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
                    let (model) = self
                        .load_model(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    model.write_owned(&mut out)?;
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
#[derive(Debug, Clone)]
pub struct Model {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for Model {
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
        Ok(Model::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl Model {
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
    pub async fn get_part(
        &self,
        path: String,
    ) -> Result<Option<ModelPart>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_part_blocking(path)).await.unwrap()
    }
    pub fn get_part_blocking(
        &self,
        path: String,
    ) -> Result<Option<ModelPart>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        path.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn enumerate_parts(
        &self,
    ) -> Result<Vec<ModelPart>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.enumerate_parts_blocking())
            .await
            .unwrap()
    }
    pub fn enumerate_parts_blocking(
        &self,
    ) -> Result<Vec<ModelPart>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn set_model_scale(
        &self,
        scale: super::types::Vec3F,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        scale.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 11u32, builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: ModelHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> Model {
        Model::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> Model {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        Model { obj, drop_notification }
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
impl binderbinder::binder_object::ToBinderObjectOrRef for Model {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait ModelHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn get_spatial(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = super::spatial::Spatial> + Send + Sync;
    fn get_part(
        &self,
        _ctx: gluon_wire::GluonCtx,
        path: String,
    ) -> impl Future<Output = Option<ModelPart>> + Send + Sync;
    fn enumerate_parts(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = Vec<ModelPart>> + Send + Sync;
    fn set_model_scale(&self, _ctx: gluon_wire::GluonCtx, scale: super::types::Vec3F);
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
                9u32 => {
                    let (part) = self
                        .get_part(ctx, gluon_wire::GluonConvertable::read(data)?)
                        .await;
                    part.write_owned(&mut out)?;
                }
                10u32 => {
                    let (parts) = self.enumerate_parts(ctx).await;
                    parts.write_owned(&mut out)?;
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
                11u32 => {
                    self.set_model_scale(ctx, gluon_wire::GluonConvertable::read(data)?);
                }
                _ => {}
            }
            Ok(())
        }
    }
}
#[derive(Debug, Clone)]
pub struct ModelPart {
    obj: binderbinder::binder_object::BinderObjectOrRef,
    drop_notification: std::sync::Arc<
        binderbinder::binder_object::BinderObject<
            gluon_wire::drop_tracking::DropNotifiedHandler,
        >,
    >,
}
impl gluon_wire::GluonConvertable for ModelPart {
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
        Ok(ModelPart::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(data)
    }
}
impl ModelPart {
    pub async fn get_part_path(&self) -> Result<String, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_part_path_blocking()).await.unwrap()
    }
    pub fn get_part_path_blocking(&self) -> Result<String, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 8u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the transform relative to the Model
    pub async fn get_model_transform(
        &self,
    ) -> Result<NonUniformTransform, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_model_transform_blocking())
            .await
            .unwrap()
    }
    pub fn get_model_transform_blocking(
        &self,
    ) -> Result<NonUniformTransform, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 9u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the transform relative to the parent ModelPart or Model
    pub async fn get_local_transform(
        &self,
    ) -> Result<NonUniformTransform, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || this.get_local_transform_blocking())
            .await
            .unwrap()
    }
    pub fn get_local_transform_blocking(
        &self,
    ) -> Result<NonUniformTransform, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 10u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the transform relative to referenced ModelPart
    pub async fn get_relative_transform(
        &self,
        relative_to: ModelPart,
    ) -> Result<NonUniformTransform, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.get_relative_transform_blocking(relative_to)
            })
            .await
            .unwrap()
    }
    pub fn get_relative_transform_blocking(
        &self,
        relative_to: ModelPart,
    ) -> Result<NonUniformTransform, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        relative_to.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 11u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Set the transform relative to the Model
    pub fn set_model_transform(
        &self,
        transform: PartialNonUniformTransform,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        transform.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 12u32, builder.to_payload())?;
        Ok(())
    }
    ///Set the transform relative to the parent ModelPart or Model
    pub fn set_local_transform(
        &self,
        transform: PartialNonUniformTransform,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        transform.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 13u32, builder.to_payload())?;
        Ok(())
    }
    ///Set the transform relative to referenced ModelPart
    pub fn set_relative_transform(
        &self,
        relative_to: ModelPart,
        transform: PartialNonUniformTransform,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        relative_to.write(&mut builder)?;
        transform.write(&mut builder)?;
        self.obj.device().transact_one_way(&self.obj, 14u32, builder.to_payload())?;
        Ok(())
    }
    pub async fn set_material_parameter(
        &self,
        parameter_name: String,
        value: MaterialParameter,
    ) -> Result<Option<MaterialParamError>, gluon_wire::GluonSendError> {
        let this = self.clone();
        tokio::task::spawn_blocking(move || {
                this.set_material_parameter_blocking(parameter_name, value)
            })
            .await
            .unwrap()
    }
    pub fn set_material_parameter_blocking(
        &self,
        parameter_name: String,
        value: MaterialParameter,
    ) -> Result<Option<MaterialParamError>, gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        parameter_name.write(&mut builder)?;
        value.write(&mut builder)?;
        let reader = self
            .obj
            .device()
            .transact_blocking(&self.obj, 15u32, builder.to_payload())?
            .1;
        let mut reader = gluon_wire::GluonDataReader::from_payload(reader);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Set this model part's material to one that cuts a hole in the world. Often used for overlays/passthrough where you want to show the background through an object. This removes the ability to set material parameters and cannot be undone
    pub fn apply_holdout_material(&self) -> Result<(), gluon_wire::GluonSendError> {
        let mut builder = gluon_wire::GluonDataBuilder::new();
        self.obj.device().transact_one_way(&self.obj, 16u32, builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: ModelPartHandler>(
        obj: &std::sync::Arc<binderbinder::binder_object::BinderObject<H>>,
    ) -> ModelPart {
        ModelPart::from_object_or_ref(
            binderbinder::binder_object::ToBinderObjectOrRef::to_binder_object_or_ref(
                obj,
            ),
        )
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(
        obj: binderbinder::binder_object::BinderObjectOrRef,
    ) -> ModelPart {
        let drop_notification = obj
            .device()
            .register_object(gluon_wire::drop_tracking::DropNotifiedHandler::new());
        let mut builder = gluon_wire::GluonDataBuilder::new();
        builder.write_binder(&drop_notification);
        _ = obj.device().transact_one_way(&obj, 4, builder.to_payload());
        ModelPart {
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
impl binderbinder::binder_object::ToBinderObjectOrRef for ModelPart {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
pub trait ModelPartHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    fn get_part_path(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = String> + Send + Sync;
    ///Get the transform relative to the Model
    fn get_model_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = NonUniformTransform> + Send + Sync;
    ///Get the transform relative to the parent ModelPart or Model
    fn get_local_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
    ) -> impl Future<Output = NonUniformTransform> + Send + Sync;
    ///Get the transform relative to referenced ModelPart
    fn get_relative_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
        relative_to: ModelPart,
    ) -> impl Future<Output = NonUniformTransform> + Send + Sync;
    ///Set the transform relative to the Model
    fn set_model_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
        transform: PartialNonUniformTransform,
    );
    ///Set the transform relative to the parent ModelPart or Model
    fn set_local_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
        transform: PartialNonUniformTransform,
    );
    ///Set the transform relative to referenced ModelPart
    fn set_relative_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
        relative_to: ModelPart,
        transform: PartialNonUniformTransform,
    );
    fn set_material_parameter(
        &self,
        _ctx: gluon_wire::GluonCtx,
        parameter_name: String,
        value: MaterialParameter,
    ) -> impl Future<Output = Option<MaterialParamError>> + Send + Sync;
    ///Set this model part's material to one that cuts a hole in the world. Often used for overlays/passthrough where you want to show the background through an object. This removes the ability to set material parameters and cannot be undone
    fn apply_holdout_material(&self, _ctx: gluon_wire::GluonCtx);
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
                    let (path) = self.get_part_path(ctx).await;
                    path.write_owned(&mut out)?;
                }
                9u32 => {
                    let (transform) = self.get_model_transform(ctx).await;
                    transform.write_owned(&mut out)?;
                }
                10u32 => {
                    let (transform) = self.get_local_transform(ctx).await;
                    transform.write_owned(&mut out)?;
                }
                11u32 => {
                    let (transform) = self
                        .get_relative_transform(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    transform.write_owned(&mut out)?;
                }
                15u32 => {
                    let (error) = self
                        .set_material_parameter(
                            ctx,
                            gluon_wire::GluonConvertable::read(data)?,
                            gluon_wire::GluonConvertable::read(data)?,
                        )
                        .await;
                    error.write_owned(&mut out)?;
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
                12u32 => {
                    self.set_model_transform(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                13u32 => {
                    self.set_local_transform(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                14u32 => {
                    self.set_relative_transform(
                        ctx,
                        gluon_wire::GluonConvertable::read(data)?,
                        gluon_wire::GluonConvertable::read(data)?,
                    );
                }
                16u32 => {
                    self.apply_holdout_material(ctx);
                }
                _ => {}
            }
            Ok(())
        }
    }
}
