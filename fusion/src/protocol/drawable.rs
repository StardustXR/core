pub(crate) const INTERFACE_VERSION: u32 = 1u32;
pub(crate) const INTERFACE_NODE_ID: u64 = 4u64;
///X alignment
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    serde_repr::Deserialize_repr,
    serde_repr::Serialize_repr
)]
#[repr(u32)]
pub enum XAlign {
    Left,
    Center,
    Right,
}
///Y alignment
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    serde_repr::Deserialize_repr,
    serde_repr::Serialize_repr
)]
#[repr(u32)]
pub enum YAlign {
    Top,
    Center,
    Bottom,
}
///How the text fits in a box of any size
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialEq,
    serde_repr::Deserialize_repr,
    serde_repr::Serialize_repr
)]
#[repr(u32)]
pub enum TextFit {
    Wrap,
    Clip,
    Squeeze,
    Exact,
    Overflow,
}
///
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(tag = "t", content = "c")]
pub enum MaterialParameter {
    Bool(bool),
    Int(i32),
    UInt(u32),
    Float(f32),
    Vec2(stardust_xr::values::Vector2<f32>),
    Vec3(stardust_xr::values::Vector3<f32>),
    Color(stardust_xr::values::Color),
    Texture(stardust_xr::values::ResourceID),
}
///A single point on a line
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct LinePoint {
    pub point: stardust_xr::values::Vector3<f32>,
    pub thickness: f32,
    pub color: stardust_xr::values::Color,
}
///A single continuous polyline
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Line {
    pub points: Vec<LinePoint>,
    pub cyclic: bool,
}
///
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TextBounds {
    pub bounds: stardust_xr::values::Vector2<f32>,
    pub fit: TextFit,
    pub anchor_align_x: XAlign,
    pub anchor_align_y: YAlign,
}
///
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TextStyle {
    pub character_height: f32,
    pub color: stardust_xr::values::Color,
    pub font: Option<stardust_xr::values::ResourceID>,
    pub text_align_x: XAlign,
    pub text_align_y: YAlign,
    pub bounds: Option<TextBounds>,
}
#[allow(clippy::all)]
///A collection of polylines drawn by the server. Makes prototyping UI and drawing gizmos easier as well as just looks sci-fi
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Lines(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl Lines {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        Lines(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for Lines {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for Lines {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl LinesAspect for Lines {}
pub(crate) const LINES_ASPECT_ID: u64 = 16705186951373789081u64;
pub(crate) const LINES_SET_LINES_SERVER_OPCODE: u64 = 17689001183742889136u64;
#[derive(Debug)]
pub enum LinesEvent {}
#[allow(clippy::all)]
///A collection of polylines drawn by the server. Makes prototyping UI and drawing gizmos easier as well as just looks sci-fi
pub trait LinesAspect: crate::node::NodeType + super::SpatialAspect + std::fmt::Debug {
    ///Replace all polylines with the given lines
    fn set_lines(&self, lines: &[Line]) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (lines
            .iter()
            .map(|a| Ok(a))
            .collect::<crate::node::NodeResult<Vec<_>>>()?);
        self.node()
            .send_remote_signal(
                16705186951373789081u64,
                17689001183742889136u64,
                &data,
                _fds,
            )?;
        let (lines) = data;
        tracing::trace!(? lines, "Sent signal to server, {}::{}", "Lines", "set_lines");
        Ok(())
    }
}
#[allow(clippy::all)]
///A GLTF model loaded by the server.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Model(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl Model {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        Model(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for Model {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for Model {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl ModelAspect for Model {}
pub(crate) const MODEL_ASPECT_ID: u64 = 11775342128130118047u64;
pub(crate) const MODEL_BIND_MODEL_PART_SERVER_OPCODE: u64 = 18406803564448475833u64;
#[derive(Debug)]
pub enum ModelEvent {}
#[allow(clippy::all)]
///A GLTF model loaded by the server.
pub trait ModelAspect: crate::node::NodeType + super::SpatialAspect + std::fmt::Debug {
    ///Bind a model part to the node with the ID input.
    fn bind_model_part(
        &self,
        id: u64,
        part_path: &str,
    ) -> crate::node::NodeResult<ModelPart> {
        {
            let mut _fds = Vec::new();
            let data = (id, part_path);
            self.node()
                .send_remote_signal(
                    11775342128130118047u64,
                    18406803564448475833u64,
                    &data,
                    _fds,
                )?;
            let (id, part_path) = data;
            tracing::trace!(
                ? id, ? part_path, "Sent signal to server, {}::{}", "Model",
                "bind_model_part"
            );
        }
        Ok(ModelPart::from_id(&self.node().client()?, id, true))
    }
}
#[allow(clippy::all)]
///A graphical node in the GLTF hierarchy for the given model. Can be reparented and have material parameters set on.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ModelPart(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl ModelPart {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        ModelPart(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for ModelPart {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for ModelPart {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl ModelPartAspect for ModelPart {}
pub(crate) const MODEL_PART_ASPECT_ID: u64 = 7912164431074553740u64;
pub(crate) const MODEL_PART_APPLY_HOLDOUT_MATERIAL_SERVER_OPCODE: u64 = 13817793452575402942u64;
pub(crate) const MODEL_PART_SET_MATERIAL_PARAMETER_SERVER_OPCODE: u64 = 12609900228877593594u64;
#[derive(Debug)]
pub enum ModelPartEvent {}
#[allow(clippy::all)]
///A graphical node in the GLTF hierarchy for the given model. Can be reparented and have material parameters set on.
pub trait ModelPartAspect: crate::node::NodeType + super::SpatialAspect + std::fmt::Debug {
    ///Set this model part's material to one that cuts a hole in the world. Often used for overlays/passthrough where you want to show the background through an object.
    fn apply_holdout_material(&self) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = ();
        self.node()
            .send_remote_signal(
                7912164431074553740u64,
                13817793452575402942u64,
                &data,
                _fds,
            )?;
        let () = data;
        tracing::trace!(
            "Sent signal to server, {}::{}", "ModelPart", "apply_holdout_material"
        );
        Ok(())
    }
    ///Set the material parameter with `parameter_name` to `value`
    fn set_material_parameter(
        &self,
        parameter_name: &str,
        value: MaterialParameter,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (parameter_name, value);
        self.node()
            .send_remote_signal(
                7912164431074553740u64,
                12609900228877593594u64,
                &data,
                _fds,
            )?;
        let (parameter_name, value) = data;
        tracing::trace!(
            ? parameter_name, ? value, "Sent signal to server, {}::{}", "ModelPart",
            "set_material_parameter"
        );
        Ok(())
    }
}
#[allow(clippy::all)]
///Text rendered to work best in XR
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Text(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl Text {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        Text(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for Text {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for Text {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl TextAspect for Text {}
pub(crate) const TEXT_ASPECT_ID: u64 = 3129045917168168339u64;
pub(crate) const TEXT_SET_CHARACTER_HEIGHT_SERVER_OPCODE: u64 = 1124886941794143568u64;
pub(crate) const TEXT_SET_TEXT_SERVER_OPCODE: u64 = 395974856293277940u64;
#[derive(Debug)]
pub enum TextEvent {}
#[allow(clippy::all)]
///Text rendered to work best in XR
pub trait TextAspect: crate::node::NodeType + super::SpatialAspect + std::fmt::Debug {
    ///Set the character height in meters
    fn set_character_height(&self, height: f32) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (height);
        self.node()
            .send_remote_signal(
                3129045917168168339u64,
                1124886941794143568u64,
                &data,
                _fds,
            )?;
        let (height) = data;
        tracing::trace!(
            ? height, "Sent signal to server, {}::{}", "Text", "set_character_height"
        );
        Ok(())
    }
    ///Set the text content
    fn set_text(&self, text: &str) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (text);
        self.node()
            .send_remote_signal(
                3129045917168168339u64,
                395974856293277940u64,
                &data,
                _fds,
            )?;
        let (text) = data;
        tracing::trace!(? text, "Sent signal to server, {}::{}", "Text", "set_text");
        Ok(())
    }
}
pub(crate) const INTERFACE_SET_SKY_TEX_SERVER_OPCODE: u64 = 4424860741442403592u64;
#[allow(clippy::all)]
///Set the sky texture to a given HDRI file.
pub fn set_sky_tex(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    tex: Option<&stardust_xr::values::ResourceID>,
) -> crate::node::NodeResult<()> {
    let mut _fds = Vec::new();
    let data = (tex.map(|o| Ok::<_, crate::node::NodeError>(o)).transpose()?);
    let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
    _client
        .message_sender_handle
        .signal(4u64, 0u64, 4424860741442403592u64, &serialized_data, _fds)?;
    let (tex) = data;
    tracing::trace!(? tex, "Sent signal to server, {}::{}", "Interface", "set_sky_tex");
    Ok(())
}
pub(crate) const INTERFACE_SET_SKY_LIGHT_SERVER_OPCODE: u64 = 6210987039553590011u64;
#[allow(clippy::all)]
///Set the sky lighting to a given HDRI file.
pub fn set_sky_light(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    light: Option<&stardust_xr::values::ResourceID>,
) -> crate::node::NodeResult<()> {
    let mut _fds = Vec::new();
    let data = (light.map(|o| Ok::<_, crate::node::NodeError>(o)).transpose()?);
    let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
    _client
        .message_sender_handle
        .signal(4u64, 0u64, 6210987039553590011u64, &serialized_data, _fds)?;
    let (light) = data;
    tracing::trace!(
        ? light, "Sent signal to server, {}::{}", "Interface", "set_sky_light"
    );
    Ok(())
}
pub(crate) const INTERFACE_CREATE_LINES_SERVER_OPCODE: u64 = 17691651736865216822u64;
///Create a lines node
fn create_lines(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    lines: &[Line],
) -> crate::node::NodeResult<Lines> {
    {
        let mut _fds = Vec::new();
        let data = (
            id,
            parent.node().id(),
            transform,
            lines.iter().map(|a| Ok(a)).collect::<crate::node::NodeResult<Vec<_>>>()?,
        );
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(4u64, 0u64, 17691651736865216822u64, &serialized_data, _fds)?;
        let (id, parent, transform, lines) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? lines, "Sent signal to server, {}::{}",
            "Interface", "create_lines"
        );
    }
    Ok(Lines::from_id(_client, id, true))
}
pub(crate) const INTERFACE_LOAD_MODEL_SERVER_OPCODE: u64 = 8647852218278439936u64;
///Load a GLTF model into a Model node
fn load_model(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    model: &stardust_xr::values::ResourceID,
) -> crate::node::NodeResult<Model> {
    {
        let mut _fds = Vec::new();
        let data = (id, parent.node().id(), transform, model);
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(4u64, 0u64, 8647852218278439936u64, &serialized_data, _fds)?;
        let (id, parent, transform, model) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? model, "Sent signal to server, {}::{}",
            "Interface", "load_model"
        );
    }
    Ok(Model::from_id(_client, id, true))
}
pub(crate) const INTERFACE_CREATE_TEXT_SERVER_OPCODE: u64 = 11386227176670607870u64;
///Create a text node
fn create_text(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    text: &str,
    style: TextStyle,
) -> crate::node::NodeResult<Text> {
    {
        let mut _fds = Vec::new();
        let data = (id, parent.node().id(), transform, text, style);
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(4u64, 0u64, 11386227176670607870u64, &serialized_data, _fds)?;
        let (id, parent, transform, text, style) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? text, ? style,
            "Sent signal to server, {}::{}", "Interface", "create_text"
        );
    }
    Ok(Text::from_id(_client, id, true))
}
