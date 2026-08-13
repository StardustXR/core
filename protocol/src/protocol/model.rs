#![allow(unused, clippy::all, private_bounds, private_interfaces)]
use gluon::Convertable as _;
use tracing::Instrument as _;
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
            supported_derives: gluon::Derives::from_bits_truncate(10u32),
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
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
#[derive(Debug, Clone, PartialEq)]
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
        release_point: super::dmatex::DmatexSubmitRelease,
    },
}
impl gluon::Convertable for MaterialParameter {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
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
        gluon_data: &mut gluon::DataBuilder,
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
    obj: gluon::Ref,
}
impl gluon::Convertable for ModelInterface {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(ModelInterface::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for ModelInterface {
    const ID: &'static str = "org.stardustxr.Model.ModelInterface";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: ModelInterfaceHandler> gluon::HandledBy<H> for ModelInterface {}
impl gluon::RefExt for ModelInterface {
    fn from_ref(obj: gluon::Ref) -> ModelInterface {
        ModelInterface { obj }
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
        tracing::trace!(
            interface = "ModelInterface", method = "load_model", ? spatial, ? model,
            "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        spatial.write(&mut gluon_builder)?;
        model.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_model = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "ModelInterface", method = "load_model", ? __ret_model, "←"
        );
        Ok(__ret_model)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> ModelInterface {
        ModelInterface { obj }
    }
}
impl From<ModelInterface> for gluon::Ref {
    fn from(value: ModelInterface) -> Self {
        value.obj
    }
}
impl gluon::ToRef for ModelInterface {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for ModelInterface {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
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
    ///Dispatched instead of [`Self::load_model`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `load_model` and sends the result through `reply`. Override this method instead of `load_model` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn load_model_oneway(
        &self,
        _ctx: gluon::Context,
        spatial: super::spatial::Spatial,
        model: super::types::Resource,
        reply: gluon::ReplySender<Result<Model, super::types::ResourceLoadError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let model = self.load_model(_ctx, spatial, model).await;
            reply.send(model)
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
                    let param_model = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "ModelInterface", method = "load_model", ?
                        param_spatial, ? param_model, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<
                        Result<Model, super::types::ResourceLoadError>,
                    > = gluon::ReplySender::new(
                        return_callback,
                        |model, gluon_out| {
                            tracing::trace!(
                                interface = "ModelInterface", method = "load_model", ?
                                model, "←"
                            );
                            model.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.load_model_oneway(ctx, param_spatial, param_model, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ModelInterface", method =
                                "load_model", method_id = 8u32
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
pub struct Model {
    obj: gluon::Ref,
}
impl gluon::Convertable for Model {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(Model::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for Model {
    const ID: &'static str = "org.stardustxr.Model.Model";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: ModelHandler> gluon::HandledBy<H> for Model {}
impl gluon::RefExt for Model {
    fn from_ref(obj: gluon::Ref) -> Model {
        Model { obj }
    }
}
impl Model {
    pub async fn get_part(
        &self,
        path: impl Into<String>,
    ) -> Result<Option<ModelPart>, gluon::SendError> {
        let path: String = path.into();
        tracing::trace!(interface = "Model", method = "get_part", ? path, "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        path.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_part = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(interface = "Model", method = "get_part", ? __ret_part, "←");
        Ok(__ret_part)
    }
    pub async fn enumerate_parts(&self) -> Result<Vec<ModelPart>, gluon::SendError> {
        tracing::trace!(interface = "Model", method = "enumerate_parts", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_parts = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "Model", method = "enumerate_parts", ? __ret_parts, "←"
        );
        Ok(__ret_parts)
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> Model {
        Model { obj }
    }
}
impl From<Model> for gluon::Ref {
    fn from(value: Model) -> Self {
        value.obj
    }
}
impl gluon::ToRef for Model {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for Model {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
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
    ///Dispatched instead of [`Self::get_part`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_part` and sends the result through `reply`. Override this method instead of `get_part` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_part_oneway(
        &self,
        _ctx: gluon::Context,
        path: String,
        reply: gluon::ReplySender<Option<ModelPart>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let part = self.get_part(_ctx, path).await;
            reply.send(part)
        }
    }
    fn enumerate_parts(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = Vec<ModelPart>> + Send + Sync;
    ///Dispatched instead of [`Self::enumerate_parts`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `enumerate_parts` and sends the result through `reply`. Override this method instead of `enumerate_parts` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn enumerate_parts_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<Vec<ModelPart>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let parts = self.enumerate_parts(_ctx).await;
            reply.send(parts)
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
                    let param_path = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "Model", method = "get_part", ? param_path,
                        "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Option<ModelPart>> = gluon::ReplySender::new(
                        return_callback,
                        |part, gluon_out| {
                            tracing::trace!(
                                interface = "Model", method = "get_part", ? part, "←"
                            );
                            part.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_part_oneway(ctx, param_path, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Model", method = "get_part",
                                method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    tracing::trace!(
                        interface = "Model", method = "enumerate_parts", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Vec<ModelPart>> = gluon::ReplySender::new(
                        return_callback,
                        |parts, gluon_out| {
                            tracing::trace!(
                                interface = "Model", method = "enumerate_parts", ? parts,
                                "←"
                            );
                            parts.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.enumerate_parts_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "Model", method =
                                "enumerate_parts", method_id = 9u32
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
pub struct ModelPart {
    obj: gluon::Ref,
}
impl gluon::Convertable for ModelPart {
    fn write(
        &self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write(gluon_data)
    }
    fn read(gluon_data: &mut gluon::DataReader) -> Result<Self, gluon::ReadError> {
        let obj = gluon::Ref::read(gluon_data)?;
        Ok(ModelPart::from_ref(obj))
    }
    fn write_owned(
        self,
        gluon_data: &mut gluon::DataBuilder,
    ) -> Result<(), gluon::WriteError> {
        self.obj.write_owned(gluon_data)
    }
}
impl gluon::Interface for ModelPart {
    const ID: &'static str = "org.stardustxr.Model.ModelPart";
}
///Carries the per-interface bound for [`gluon::RefExt`]'s handler constructors: only a handler implementing this interface's handler trait can be passed to them.
impl<H: ModelPartHandler> gluon::HandledBy<H> for ModelPart {}
impl gluon::RefExt for ModelPart {
    fn from_ref(obj: gluon::Ref) -> ModelPart {
        ModelPart { obj }
    }
}
impl ModelPart {
    pub async fn get_part_path(&self) -> Result<String, gluon::SendError> {
        tracing::trace!(interface = "ModelPart", method = "get_part_path", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 8u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_path = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "ModelPart", method = "get_part_path", ? __ret_path, "←"
        );
        Ok(__ret_path)
    }
    pub async fn get_spatial(
        &self,
    ) -> Result<super::spatial::Spatial, gluon::SendError> {
        tracing::trace!(interface = "ModelPart", method = "get_spatial", "→");
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        gluon::transact(&self.obj, 9u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_spatial = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "ModelPart", method = "get_spatial", ? __ret_spatial, "←"
        );
        Ok(__ret_spatial)
    }
    pub async fn set_material_parameter(
        &self,
        parameter_name: impl Into<String>,
        value: impl Into<MaterialParameter>,
    ) -> Result<Option<MaterialParamError>, gluon::SendError> {
        let parameter_name: String = parameter_name.into();
        let value: MaterialParameter = value.into();
        tracing::trace!(
            interface = "ModelPart", method = "set_material_parameter", ? parameter_name,
            ? value, "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        let (gluon_ret_handler, mut gluon_recv) = gluon::ReturnHandler::new();
        let (gluon_ret_node, gluon_ret) = gluon::Node::new(gluon_ret_handler)?;
        gluon_builder.write_ref(&gluon_ret)?;
        parameter_name.write(&mut gluon_builder)?;
        value.write(&mut gluon_builder)?;
        gluon::transact(&self.obj, 10u32, gluon_builder)?;
        let mut reader = gluon_recv.recv().await.unwrap();
        drop(gluon_ret_node);
        let __ret_error = gluon::Convertable::read(&mut reader)?;
        tracing::trace!(
            interface = "ModelPart", method = "set_material_parameter", ? __ret_error,
            "←"
        );
        Ok(__ret_error)
    }
    ///Set this model part's material to one that cuts a hole in the world. Often used for overlays/passthrough where you want to show the background through an object. This removes the ability to set material parameters and cannot be undone
    pub fn apply_holdout_material(&self) -> Result<(), gluon::SendError> {
        tracing::trace!(
            interface = "ModelPart", method = "apply_holdout_material", "→"
        );
        let mut gluon_builder = gluon::DataBuilder::new();
        gluon::transact(&self.obj, 11u32, gluon_builder)?;
        Ok(())
    }
    ///only use this when you know the ref leads to something implementing this interface, else the consquences are for you to find out
    pub fn from_ref(obj: gluon::Ref) -> ModelPart {
        ModelPart { obj }
    }
}
impl From<ModelPart> for gluon::Ref {
    fn from(value: ModelPart) -> Self {
        value.obj
    }
}
impl gluon::ToRef for ModelPart {
    fn to_ref(&self) -> gluon::Ref {
        self.obj.clone()
    }
}
impl gluon::Liveness for ModelPart {
    fn alive(&self) -> bool {
        gluon::Liveness::alive(&self.obj)
    }
    fn death_notification(
        &self,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        gluon::Liveness::death_notification(&self.obj)
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
    ///Dispatched instead of [`Self::get_part_path`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_part_path` and sends the result through `reply`. Override this method instead of `get_part_path` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_part_path_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<String>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let path = self.get_part_path(_ctx).await;
            reply.send(path)
        }
    }
    fn get_spatial(
        &self,
        _ctx: gluon::Context,
    ) -> impl Future<Output = super::spatial::Spatial> + Send + Sync;
    ///Dispatched instead of [`Self::get_spatial`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `get_spatial` and sends the result through `reply`. Override this method instead of `get_spatial` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn get_spatial_oneway(
        &self,
        _ctx: gluon::Context,
        reply: gluon::ReplySender<super::spatial::Spatial>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let spatial = self.get_spatial(_ctx).await;
            reply.send(spatial)
        }
    }
    fn set_material_parameter(
        &self,
        _ctx: gluon::Context,
        parameter_name: String,
        value: MaterialParameter,
    ) -> impl Future<Output = Option<MaterialParamError>> + Send + Sync;
    ///Dispatched instead of [`Self::set_material_parameter`] so a slow reply doesn't hold up dispatch of the next transaction. The default implementation just awaits `set_material_parameter` and sends the result through `reply`. Override this method instead of `set_material_parameter` to defer the reply: stash `reply` (it's `Send + Sync + 'static`) somewhere else — a channel, a queue, another task — and return as soon as this method's future is done, without waiting for the reply to actually be sent.
    fn set_material_parameter_oneway(
        &self,
        _ctx: gluon::Context,
        parameter_name: String,
        value: MaterialParameter,
        reply: gluon::ReplySender<Option<MaterialParamError>>,
    ) -> impl Future<Output = Result<(), gluon::SendError>> + Send + Sync {
        async move {
            let error = self.set_material_parameter(_ctx, parameter_name, value).await;
            reply.send(error)
        }
    }
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
                    let return_callback = gluon_data.read_ref()?;
                    tracing::trace!(
                        interface = "ModelPart", method = "get_part_path", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<String> = gluon::ReplySender::new(
                        return_callback,
                        |path, gluon_out| {
                            tracing::trace!(
                                interface = "ModelPart", method = "get_part_path", ? path,
                                "←"
                            );
                            path.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_part_path_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ModelPart", method =
                                "get_part_path", method_id = 8u32
                            ),
                        )
                        .await?;
                }
                9u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    tracing::trace!(
                        interface = "ModelPart", method = "get_spatial", "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<super::spatial::Spatial> = gluon::ReplySender::new(
                        return_callback,
                        |spatial, gluon_out| {
                            tracing::trace!(
                                interface = "ModelPart", method = "get_spatial", ? spatial,
                                "←"
                            );
                            spatial.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.get_spatial_oneway(ctx, reply)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ModelPart", method =
                                "get_spatial", method_id = 9u32
                            ),
                        )
                        .await?;
                }
                10u32 => {
                    let return_callback = gluon_data.read_ref()?;
                    let param_parameter_name = gluon::Convertable::read(
                        &mut gluon_data,
                    )?;
                    let param_value = gluon::Convertable::read(&mut gluon_data)?;
                    tracing::trace!(
                        interface = "ModelPart", method = "set_material_parameter", ?
                        param_parameter_name, ? param_value, "dispatching"
                    );
                    drop(gluon_data);
                    let reply: gluon::ReplySender<Option<MaterialParamError>> = gluon::ReplySender::new(
                        return_callback,
                        |error, gluon_out| {
                            tracing::trace!(
                                interface = "ModelPart", method = "set_material_parameter",
                                ? error, "←"
                            );
                            error.write_owned(gluon_out)?;
                            Ok(())
                        },
                    );
                    self.set_material_parameter_oneway(
                            ctx,
                            param_parameter_name,
                            param_value,
                            reply,
                        )
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ModelPart", method =
                                "set_material_parameter", method_id = 10u32
                            ),
                        )
                        .await?;
                }
                11u32 => {
                    tracing::trace!(
                        interface = "ModelPart", method = "apply_holdout_material",
                        "dispatching"
                    );
                    drop(gluon_data);
                    self.apply_holdout_material(ctx)
                        .instrument(
                            tracing::trace_span!(
                                "dispatching", interface = "ModelPart", method =
                                "apply_holdout_material", method_id = 11u32
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
