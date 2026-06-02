#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable;
pub const EXTERNAL_PROTOCOL: gluon::ExternalProtocol = gluon::ExternalProtocol {
    protocol_name: "org.stardustxr.Model",
    types: &[
        gluon::ExternalGluonType {
            name: "MaterialParamError",
            supported_derives: gluon::Derives::from_bits_truncate(798u32),
            proxy: None,
        },
        gluon::ExternalGluonType {
            name: "MaterialParameter",
            supported_derives: gluon::Derives::from_bits_truncate(2u32),
            proxy: None,
        },
    ],
};
pub mod proxies {
    use super::*;
}
///Error potentially produced when trying to set a material paramterer
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MaterialParamError {
    ParamNotFound { known_params: Vec<String> },
    IncorrectType { valid_type: String },
    ///ModelPart had apply_holdout_material called before
    Holdout,
    InvalidValue,
}
impl gluon::Convertable for MaterialParamError {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
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
            MaterialParamError::InvalidValue => {
                gluon_data.write_u16(3u16)?;
            }
        };
        Ok(())
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => {
                    let known_params = gluon::Convertable::read(gluon_data)?;
                    MaterialParamError::ParamNotFound {
                        known_params,
                    }
                }
                1u16 => {
                    let valid_type = gluon::Convertable::read(gluon_data)?;
                    MaterialParamError::IncorrectType {
                        valid_type,
                    }
                }
                2u16 => MaterialParamError::Holdout,
                3u16 => MaterialParamError::InvalidValue,
                v => return Err(gluon::ReadError::UnknownEnumVariant(v)),
            },
        )
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
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
            MaterialParamError::InvalidValue => {
                gluon_data.write_u16(3u16)?;
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
    Vec2 { value: crate::types::Vec2F },
    Vec3 { value: crate::types::Vec3F },
    Color { value: crate::types::Color },
    Texture { value: super::types::Resource },
    Dmatex {
        dmatex: super::dmatex::DmatexRef,
        ///After this point is reached the Server may access the texture
        acquire_point: u64,
        ///This point is reached once the Server is done accessing the texture
        release_point: u64,
    },
}
impl gluon::Convertable for MaterialParameter {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
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
                {
                    let __w: super::types::proxied::Vec2F = value.clone().into();
                    __w.write_owned(gluon_data)?;
                }
            }
            MaterialParameter::Vec3 { value } => {
                gluon_data.write_u16(5u16)?;
                {
                    let __w: super::types::proxied::Vec3F = value.clone().into();
                    __w.write_owned(gluon_data)?;
                }
            }
            MaterialParameter::Color { value } => {
                gluon_data.write_u16(6u16)?;
                {
                    let __w: super::types::proxied::Color = value.clone().into();
                    __w.write_owned(gluon_data)?;
                }
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
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        Ok(
            match gluon_data.read_u16()? {
                0u16 => {
                    let value = gluon::Convertable::read(gluon_data)?;
                    MaterialParameter::Bool { value }
                }
                1u16 => {
                    let value = gluon::Convertable::read(gluon_data)?;
                    MaterialParameter::Int { value }
                }
                2u16 => {
                    let value = gluon::Convertable::read(gluon_data)?;
                    MaterialParameter::Uint { value }
                }
                3u16 => {
                    let value = gluon::Convertable::read(gluon_data)?;
                    MaterialParameter::Float { value }
                }
                4u16 => {
                    let value: crate::types::Vec2F = {
                        let __w: super::types::proxied::Vec2F = gluon::Convertable::read(
                            gluon_data,
                        )?;
                        __w.into()
                    };
                    MaterialParameter::Vec2 { value }
                }
                5u16 => {
                    let value: crate::types::Vec3F = {
                        let __w: super::types::proxied::Vec3F = gluon::Convertable::read(
                            gluon_data,
                        )?;
                        __w.into()
                    };
                    MaterialParameter::Vec3 { value }
                }
                6u16 => {
                    let value: crate::types::Color = {
                        let __w: super::types::proxied::Color = gluon::Convertable::read(
                            gluon_data,
                        )?;
                        __w.into()
                    };
                    MaterialParameter::Color { value }
                }
                7u16 => {
                    let value = gluon::Convertable::read(gluon_data)?;
                    MaterialParameter::Texture {
                        value,
                    }
                }
                8u16 => {
                    let dmatex = gluon::Convertable::read(gluon_data)?;
                    let acquire_point = gluon::Convertable::read(gluon_data)?;
                    let release_point = gluon::Convertable::read(gluon_data)?;
                    MaterialParameter::Dmatex {
                        dmatex,
                        acquire_point,
                        release_point,
                    }
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
                {
                    let __w: super::types::proxied::Vec2F = value.into();
                    __w.write_owned(gluon_data)?;
                }
            }
            MaterialParameter::Vec3 { value } => {
                gluon_data.write_u16(5u16)?;
                {
                    let __w: super::types::proxied::Vec3F = value.into();
                    __w.write_owned(gluon_data)?;
                }
            }
            MaterialParameter::Color { value } => {
                gluon_data.write_u16(6u16)?;
                {
                    let __w: super::types::proxied::Color = value.into();
                    __w.write_owned(gluon_data)?;
                }
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for ModelInterface {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(ModelInterface::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl ModelInterface {
    ///Load a GLTF model into a Model
    pub async fn load_model(
        &self,
        spatial: impl Into<super::spatial::Spatial>,
        model: impl Into<super::types::Resource>,
    ) -> Result<Result<Model, super::types::ResourceLoadError>, gluon::SendError> {
        let spatial: super::spatial::Spatial = spatial.into();
        let model: super::types::Resource = model.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        model.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn from_handler<H: ModelInterfaceHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> ModelInterface {
        ModelInterface::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> ModelInterface {
        ModelInterface { obj }
    }
}
impl From<ModelInterface> for gluon::ObjectOrRef {
    fn from(value: ModelInterface) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for ModelInterface {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait ModelInterfaceHandler: gluon::Handler + Send + Sync + 'static {
    ///Load a GLTF model into a Model
    fn load_model(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        model: super::types::Resource,
    ) -> impl Future<
        Output = Result<Model, super::types::ResourceLoadError>,
    > + Send + Sync;
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
                    let param_spatial = gluon::Convertable::read(&mut gluon_data)?;
                    let param_model = gluon::Convertable::read(&mut gluon_data)?;
                    let (model) = self.load_model(ctx, param_spatial, param_model).await;
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
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for Model {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(Model::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl Model {
    pub async fn get_part(
        &self,
        path: impl Into<String>,
    ) -> Result<Option<ModelPart>, gluon::SendError> {
        let path: String = path.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        path.write(&mut gluon_builder)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn enumerate_parts(&self) -> Result<Vec<ModelPart>, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub fn from_handler<H: ModelHandler>(obj: &impl gluon::OwnedObjectRef<H>) -> Model {
        Model::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> Model {
        Model { obj }
    }
}
impl From<Model> for gluon::ObjectOrRef {
    fn from(value: Model) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for Model {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait ModelHandler: gluon::Handler + Send + Sync + 'static {
    fn get_part(
        &self,
        _ctx: gluon::Context,
        path: String,
    ) -> impl Future<Output = Option<ModelPart>> + Send + Sync;
    fn enumerate_parts(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = Vec<ModelPart>> + Send + Sync;
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
                    let param_path = gluon::Convertable::read(&mut gluon_data)?;
                    let (part) = self.get_part(ctx, param_path).await;
                    drop(gluon_data);
                    part.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (parts) = self.enumerate_parts(ctx).await;
                    drop(gluon_data);
                    parts.write_owned(&mut gluon_out)?;
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
pub struct ModelPart {
    obj: gluon::ObjectOrRef,
}
impl gluon::Convertable for ModelPart {
    fn write<'a, 'b: 'a>(
        &'b self,
        gluon_data: &mut gluon::DataBuilder<'a>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::ObjectOrRef::read(gluon_data)?;
        Ok(ModelPart::from_object_or_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder<'_>,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl ModelPart {
    pub async fn get_part_path(&self) -> Result<String, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 8u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn get_spatial(
        &self,
    ) -> Result<super::spatial::Spatial, gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        self.obj.device().transact_one_way(&self.obj, 9u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    pub async fn set_material_parameter(
        &self,
        parameter_name: impl Into<String>,
        value: impl Into<MaterialParameter>,
    ) -> Result<Option<MaterialParamError>, gluon::SendError> {
        let parameter_name: String = parameter_name.into();
        let value: MaterialParameter = value.into();
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let gluon_ret = self.obj.device().register_object(gluon_ret_handler);
        gluon_builder.write_binder(&gluon_ret)?;
        parameter_name.write(&mut gluon_builder)?;
        value.write(&mut gluon_builder)?;
        self.obj
            .device()
            .transact_one_way(&self.obj, 10u32, gluon_builder.to_payload())?;
        let transaction = gluon_recv.recv().await.unwrap();
        let mut reader = gluon::DataReader::from_payload(transaction.payload);
        Ok(gluon::Convertable::read(&mut reader)?)
    }
    ///Set this model part's material to one that cuts a hole in the world. Often used for overlays/passthrough where you want to show the background through an object. This removes the ability to set material parameters and cannot be undone
    pub fn apply_holdout_material(&self) -> Result<(), gluon::SendError> {
        let mut gluon_builder = gluon::DataBuilder::new();
        self.obj
            .device()
            .transact_one_way(&self.obj, 11u32, gluon_builder.to_payload())?;
        Ok(())
    }
    pub fn from_handler<H: ModelPartHandler>(
        obj: &impl gluon::OwnedObjectRef<H>,
    ) -> ModelPart {
        ModelPart::from_object_or_ref(gluon::OwnedObjectRef::to_object_or_ref(obj))
    }
    ///only use this when you know the binder ref implements this interface, else the consquences are for you to find out
    pub fn from_object_or_ref(obj: gluon::ObjectOrRef) -> ModelPart {
        ModelPart { obj }
    }
}
impl From<ModelPart> for gluon::ObjectOrRef {
    fn from(value: ModelPart) -> Self {
        value.obj
    }
}
impl gluon::ToObjectOrRef for ModelPart {
    fn to_binder_object_or_ref(&self) -> gluon::ObjectOrRef {
        self.obj.clone()
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
pub trait ModelPartHandler: gluon::Handler + Send + Sync + 'static {
    fn get_part_path(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = String> + Send + Sync;
    fn get_spatial(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::spatial::Spatial> + Send + Sync;
    fn set_material_parameter(
        &self,
        _ctx: gluon::Context,
        parameter_name: String,
        value: MaterialParameter,
    ) -> impl Future<Output = Option<MaterialParamError>> + Send + Sync;
    ///Set this model part's material to one that cuts a hole in the world. Often used for overlays/passthrough where you want to show the background through an object. This removes the ability to set material parameters and cannot be undone
    fn apply_holdout_material(
        &self,
        _ctx: gluon::Context,
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
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (path) = self.get_part_path(ctx).await;
                    drop(gluon_data);
                    path.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let (spatial) = self.get_spatial(ctx).await;
                    drop(gluon_data);
                    spatial.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_binder()?;
                    let mut gluon_out = gluon::DataBuilder::new();
                    let param_parameter_name = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_value = gluon::Convertable::read(&mut gluon_data)?;
                    let (error) = self
                        .set_material_parameter(ctx, param_parameter_name, param_value)
                        .await;
                    drop(gluon_data);
                    error.write_owned(&mut gluon_out)?;
                    return_callback
                        .device()
                        .transact_one_way(&return_callback, 0, gluon_out.to_payload())?;
                }
                11u32 => {
                    drop(gluon_data);
                    self.apply_holdout_material(ctx).await;
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
