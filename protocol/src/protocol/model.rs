#![allow(
    unused,
    clippy::single_match,
    clippy::match_single_binding,
    clippy::large_enum_variant
)]
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
    pub translation: super::types::Vec3F,
    pub rotation: super::types::Quatf,
    pub scale: super::types::Vec3F,
}
impl gluon_wire::GluonConvertable for NonUniformTransform {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.translation.write(gluon_data)?;
        self.rotation.write(gluon_data)?;
        self.scale.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let translation = gluon_wire::GluonConvertable::read(gluon_data)?;
        let rotation = gluon_wire::GluonConvertable::read(gluon_data)?;
        let scale = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(NonUniformTransform {
            translation,
            rotation,
            scale,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.translation.write_owned(gluon_data)?;
        self.rotation.write_owned(gluon_data)?;
        self.scale.write_owned(gluon_data)?;
        Ok(())
    }
}
///Partial version of NonUniformTransform
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PartialNonUniformTransform {
    pub translation: Option<super::types::Vec3F>,
    pub rotation: Option<super::types::Quatf>,
    pub scale: Option<super::types::Vec3F>,
}
impl gluon_wire::GluonConvertable for PartialNonUniformTransform {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.translation.write(gluon_data)?;
        self.rotation.write(gluon_data)?;
        self.scale.write(gluon_data)?;
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        let translation = gluon_wire::GluonConvertable::read(gluon_data)?;
        let rotation = gluon_wire::GluonConvertable::read(gluon_data)?;
        let scale = gluon_wire::GluonConvertable::read(gluon_data)?;
        Ok(PartialNonUniformTransform {
            translation,
            rotation,
            scale,
        })
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.translation.write_owned(gluon_data)?;
        self.rotation.write_owned(gluon_data)?;
        self.scale.write_owned(gluon_data)?;
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            ModelLoadError::NotFound => {
                gluon_data.write_u16(0u16)?;
            }
        };
        Ok(())
    }
    fn read(
        gluon_data: &mut gluon_wire::GluonDataReader,
    ) -> Result<Self, gluon_wire::GluonReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => ModelLoadError::NotFound,
                v => return Err(gluon_wire::GluonReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            ModelLoadError::NotFound => {
                gluon_data.write_u16(0u16)?;
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            MaterialParamError::ParamNotFound { known_params } => {
                gluon_data.write_u16(0u16)?;
                known_params.write(gluon_data)?;
            }
            MaterialParamError::IncorrectType { valid_type } => {
                gluon_data.write_u16(1u16)?;
                valid_type.write(gluon_data)?;
            }
            MaterialParamError::Holdout => {
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
                0u16 => {
                    let known_params = gluon_wire::GluonConvertable::read(gluon_data)?;
                    MaterialParamError::ParamNotFound {
                        known_params,
                    }
                }
                1u16 => {
                    let valid_type = gluon_wire::GluonConvertable::read(gluon_data)?;
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            MaterialParamError::ParamNotFound { known_params } => {
                gluon_data.write_u16(0u16)?;
                known_params.write_owned(gluon_data)?;
            }
            MaterialParamError::IncorrectType { valid_type } => {
                gluon_data.write_u16(1u16)?;
                valid_type.write_owned(gluon_data)?;
            }
            MaterialParamError::Holdout => {
                gluon_data.write_u16(2u16)?;
            }
        };
        Ok(())
    }
}
///Material parameter values
#[derive(Debug, Clone)]
pub enum MaterialParameter {
    Bool { value: bool },
    Int { value: i32 },
    Uint { value: u32 },
    Float { value: f32 },
    Vec2 { value: super::types::Vec2F },
    Vec3 { value: super::types::Vec3F },
    Color { value: super::types::Color },
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'a>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            MaterialParameter::Bool { value } => {
                gluon_data.write_u16(0u16)?;
                value.write(gluon_data)?;
            }
            MaterialParameter::Int { value } => {
                gluon_data.write_u16(1u16)?;
                value.write(gluon_data)?;
            }
            MaterialParameter::Uint { value } => {
                gluon_data.write_u16(2u16)?;
                value.write(gluon_data)?;
            }
            MaterialParameter::Float { value } => {
                gluon_data.write_u16(3u16)?;
                value.write(gluon_data)?;
            }
            MaterialParameter::Vec2 { value } => {
                gluon_data.write_u16(4u16)?;
                value.write(gluon_data)?;
            }
            MaterialParameter::Vec3 { value } => {
                gluon_data.write_u16(5u16)?;
                value.write(gluon_data)?;
            }
            MaterialParameter::Color { value } => {
                gluon_data.write_u16(6u16)?;
                value.write(gluon_data)?;
            }
            MaterialParameter::Texture { value } => {
                gluon_data.write_u16(7u16)?;
                value.write(gluon_data)?;
            }
            MaterialParameter::Dmatex { dmatex, acquire_point, release_point } => {
                gluon_data.write_u16(8u16)?;
                dmatex.write(gluon_data)?;
                acquire_point.write(gluon_data)?;
                release_point.write(gluon_data)?;
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
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    MaterialParameter::Bool { value }
                }
                1u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    MaterialParameter::Int { value }
                }
                2u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    MaterialParameter::Uint { value }
                }
                3u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    MaterialParameter::Float { value }
                }
                4u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    MaterialParameter::Vec2 { value }
                }
                5u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    MaterialParameter::Vec3 { value }
                }
                6u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    MaterialParameter::Color { value }
                }
                7u16 => {
                    let value = gluon_wire::GluonConvertable::read(gluon_data)?;
                    MaterialParameter::Texture {
                        value,
                    }
                }
                8u16 => {
                    let dmatex = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let acquire_point = gluon_wire::GluonConvertable::read(gluon_data)?;
                    let release_point = gluon_wire::GluonConvertable::read(gluon_data)?;
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
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        match self {
            MaterialParameter::Bool { value } => {
                gluon_data.write_u16(0u16)?;
                value.write_owned(gluon_data)?;
            }
            MaterialParameter::Int { value } => {
                gluon_data.write_u16(1u16)?;
                value.write_owned(gluon_data)?;
            }
            MaterialParameter::Uint { value } => {
                gluon_data.write_u16(2u16)?;
                value.write_owned(gluon_data)?;
            }
            MaterialParameter::Float { value } => {
                gluon_data.write_u16(3u16)?;
                value.write_owned(gluon_data)?;
            }
            MaterialParameter::Vec2 { value } => {
                gluon_data.write_u16(4u16)?;
                value.write_owned(gluon_data)?;
            }
            MaterialParameter::Vec3 { value } => {
                gluon_data.write_u16(5u16)?;
                value.write_owned(gluon_data)?;
            }
            MaterialParameter::Color { value } => {
                gluon_data.write_u16(6u16)?;
                value.write_owned(gluon_data)?;
            }
            MaterialParameter::Texture { value } => {
                gluon_data.write_u16(7u16)?;
                value.write_owned(gluon_data)?;
            }
            MaterialParameter::Dmatex { dmatex, acquire_point, release_point } => {
                gluon_data.write_u16(8u16)?;
                dmatex.write_owned(gluon_data)?;
                acquire_point.write_owned(gluon_data)?;
                release_point.write_owned(gluon_data)?;
            }
        };
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct ModelInterface {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for ModelInterface {
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
        Ok(ModelInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl ModelInterface {
    ///Load a GLTF model into a Model
    pub async fn load_model(
        &self,
        spatial: super::spatial::Spatial,
        model: super::types::Resource,
        model_scale: super::types::Vec3F,
    ) -> Result<Model, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        model.write(&mut gluon_builder)?;
        model_scale.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub fn from_handler<H: ModelInterfaceHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        ModelInterface { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for ModelInterface {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for ModelInterface {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for ModelInterface {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for ModelInterface {}
pub trait ModelInterfaceHandler: binderbinder::device::TransactionHandler + Send + Sync + 'static {
    ///Load a GLTF model into a Model
    fn load_model(
        &self,
        _ctx: gluon_wire::GluonCtx,
        spatial: super::spatial::Spatial,
        model: super::types::Resource,
        model_scale: super::types::Vec3F,
    ) -> impl Future<Output = Model> + Send + Sync;
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
                    let param_model = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_model_scale = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (model) = self
                        .load_model(ctx, param_spatial, param_model, param_model_scale)
                        .await;
                    drop(gluon_data);
                    model.write_owned(&mut gluon_out)?;
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
pub struct Model {
    obj: binderbinder::binder_object::BinderObjectOrRef,
}
impl gluon_wire::GluonConvertable for Model {
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
        Ok(Model::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Model {
    pub async fn get_spatial(
        &self,
    ) -> Result<super::spatial::Spatial, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn get_part(
        &self,
        path: String,
    ) -> Result<Option<ModelPart>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        path.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    pub async fn enumerate_parts(
        &self,
    ) -> Result<Vec<ModelPart>, gluon_wire::GluonSendError> {
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
    pub fn set_model_scale(
        &self,
        scale: super::types::Vec3F,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        scale.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: ModelHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        Model { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for Model {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for Model {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for Model {}
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
    fn set_model_scale(
        &self,
        _ctx: gluon_wire::GluonCtx,
        scale: super::types::Vec3F,
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
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (spatial) = self.get_spatial(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_path = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (part) = self.get_part(ctx, param_path).await;
                    drop(gluon_data);
                    part.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (parts) = self.enumerate_parts(ctx).await;
                    drop(gluon_data);
                    parts.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                11u32 => {
                    let param_scale = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_model_scale(ctx, param_scale).await;
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
}
impl gluon_wire::GluonConvertable for ModelPart {
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
        Ok(ModelPart::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon_wire::GluonDataBuilder<'_>,
    ) -> Result<(), gluon_wire::GluonWriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl ModelPart {
    pub async fn get_part_path(&self) -> Result<String, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the transform relative to the Model
    pub async fn get_model_transform(
        &self,
    ) -> Result<NonUniformTransform, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Get the transform relative to the parent ModelPart or Model
    pub async fn get_local_transform(
        &self,
    ) -> Result<NonUniformTransform, gluon_wire::GluonSendError> {
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
    ///Get the transform relative to referenced ModelPart
    pub async fn get_relative_transform(
        &self,
        relative_to: ModelPart,
    ) -> Result<NonUniformTransform, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        relative_to.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Set the transform relative to the Model
    pub fn set_model_transform(
        &self,
        transform: PartialNonUniformTransform,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        transform.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 12u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///Set the transform relative to the parent ModelPart or Model
    pub fn set_local_transform(
        &self,
        transform: PartialNonUniformTransform,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        transform.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 13u32, gluon_builder.to_payload())?;
        Ok(())
    }
    ///Set the transform relative to referenced ModelPart
    pub fn set_relative_transform(
        &self,
        relative_to: ModelPart,
        transform: PartialNonUniformTransform,
    ) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        relative_to.write(&mut gluon_builder)?;
        transform.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 14u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub async fn set_material_parameter(
        &self,
        parameter_name: String,
        value: MaterialParameter,
    ) -> Result<Option<MaterialParamError>, gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon_wire::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        parameter_name.write(&mut gluon_builder)?;
        value.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 15u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon_wire::GluonDataReader::from_payload(transaction.payload);
        Ok(gluon_wire::GluonConvertable::read(&mut reader)?)
    }
    ///Set this model part's material to one that cuts a hole in the world. Often used for overlays/passthrough where you want to show the background through an object. This removes the ability to set material parameters and cannot be undone
    pub fn apply_holdout_material(&self) -> Result<(), gluon_wire::GluonSendError> {
        let mut gluon_builder = gluon_wire::GluonDataBuilder::new();
        self.obj
            .device()
            .transact_one_way(&self.obj, 16u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: ModelPartHandler>(
        obj: &impl binderbinder::binder_object::OwnedBinderObjectRefTrait<H>,
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
        ModelPart { obj }
    }
}
impl binderbinder::binder_object::ToBinderObjectOrRef for ModelPart {
    fn to_binder_object_or_ref(&self) -> binderbinder::binder_object::BinderObjectOrRef {
        self.obj.to_binder_object_or_ref()
    }
}
impl std::hash::Hash for ModelPart {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
    }
}
impl PartialEq for ModelPart {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
impl Eq for ModelPart {}
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
    ) -> impl Future<Output = ()> + Send + Sync;
    ///Set the transform relative to the parent ModelPart or Model
    fn set_local_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
        transform: PartialNonUniformTransform,
    ) -> impl Future<Output = ()> + Send + Sync;
    ///Set the transform relative to referenced ModelPart
    fn set_relative_transform(
        &self,
        _ctx: gluon_wire::GluonCtx,
        relative_to: ModelPart,
        transform: PartialNonUniformTransform,
    ) -> impl Future<Output = ()> + Send + Sync;
    fn set_material_parameter(
        &self,
        _ctx: gluon_wire::GluonCtx,
        parameter_name: String,
        value: MaterialParameter,
    ) -> impl Future<Output = Option<MaterialParamError>> + Send + Sync;
    ///Set this model part's material to one that cuts a hole in the world. Often used for overlays/passthrough where you want to show the background through an object. This removes the ability to set material parameters and cannot be undone
    fn apply_holdout_material(
        &self,
        _ctx: gluon_wire::GluonCtx,
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
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (path) = self.get_part_path(ctx).await;
                    drop(gluon_data);
                    path.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (transform) = self.get_model_transform(ctx).await;
                    drop(gluon_data);
                    transform.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let (transform) = self.get_local_transform(ctx).await;
                    drop(gluon_data);
                    transform.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                11u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_relative_to = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (transform) = self
                        .get_relative_transform(ctx, param_relative_to)
                        .await;
                    drop(gluon_data);
                    transform.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                12u32 => {
                    let param_transform = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_model_transform(ctx, param_transform).await;
                }
                13u32 => {
                    let param_transform = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_local_transform(ctx, param_transform).await;
                }
                14u32 => {
                    let param_relative_to = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_transform = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    drop(gluon_data);
                    self.set_relative_transform(ctx, param_relative_to, param_transform)
                        .await;
                }
                15u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon_wire::GluonDataBuilder::new();
                    let param_parameter_name = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let param_value = gluon_wire::GluonConvertable::read(
                        &mut gluon_data,
                    )?;
                    let (error) = self
                        .set_material_parameter(ctx, param_parameter_name, param_value)
                        .await;
                    drop(gluon_data);
                    error.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                16u32 => {
                    drop(gluon_data);
                    self.apply_holdout_material(ctx).await;
                }
                _ => {}
            }
            Ok(())
        }
    }
}
