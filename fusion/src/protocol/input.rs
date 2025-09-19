pub(crate) const INTERFACE_VERSION: u32 = 1u32;
pub(crate) const INTERFACE_NODE_ID: u64 = 3u64;
///The special type of an InputMethod.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "t", content = "c")]
pub enum InputDataType {
    Pointer(Pointer),
    Hand(Hand),
    Tip(Tip),
}
///A hand joint. Distance from input handler's field is given because it's cheap to calculate and laggy to request from the server.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Joint {
    pub position: stardust_xr::values::Vector3<f32>,
    pub rotation: stardust_xr::values::Quaternion,
    pub radius: f32,
    pub distance: f32,
}
///
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Finger {
    pub tip: Joint,
    pub distal: Joint,
    pub intermediate: Joint,
    pub proximal: Joint,
    pub metacarpal: Joint,
}
///Different than finger to be explicit about number of joints.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Thumb {
    pub tip: Joint,
    pub distal: Joint,
    pub proximal: Joint,
    pub metacarpal: Joint,
}
///A 3D pointer, such as a gaze pointer for eye tracking or a mouse or a ray from a controller.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Pointer {
    pub origin: stardust_xr::values::Vector3<f32>,
    pub orientation: stardust_xr::values::Quaternion,
    pub deepest_point: stardust_xr::values::Vector3<f32>,
}
///A fully articulated and tracked hand.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Hand {
    pub right: bool,
    pub thumb: Thumb,
    pub index: Finger,
    pub middle: Finger,
    pub ring: Finger,
    pub little: Finger,
    pub palm: Joint,
    pub wrist: Joint,
    pub elbow: Option<Joint>,
}
///Represents a controller, pen tip, spatial cursor, etc. that is just a single point.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Tip {
    pub origin: stardust_xr::values::Vector3<f32>,
    pub orientation: stardust_xr::values::Quaternion,
}
///Information about a given input method's state relative to an input handler. All coordinates are relative to the InputHandler.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct InputData {
    pub id: u64,
    pub input: InputDataType,
    pub distance: f32,
    pub datamap: stardust_xr::values::Datamap,
    pub order: u32,
    pub captured: bool,
}
#[allow(clippy::all)]
///Node representing a spatial input device
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct InputMethodRef(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl InputMethodRef {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        InputMethodRef(node)
    }
    pub fn as_spatial_ref(self) -> super::SpatialRef {
        super::SpatialRef(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for InputMethodRef {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for InputMethodRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl InputMethodRefAspect for InputMethodRef {}
pub(crate) const INPUT_METHOD_REF_ASPECT_ID: u64 = 2611007814387963428u64;
pub(crate) const INPUT_METHOD_REF_TRY_CAPTURE_SERVER_OPCODE: u64 = 12158986667525139020u64;
pub(crate) const INPUT_METHOD_REF_RELEASE_SERVER_OPCODE: u64 = 11905596878821798323u64;
#[derive(Debug)]
pub enum InputMethodRefEvent {}
#[allow(clippy::all)]
///Node representing a spatial input device
pub trait InputMethodRefAspect: crate::node::NodeType + super::SpatialRefAspect + std::fmt::Debug {
    ///Try to capture the input method with the given handler. When the handler does not get input from the method, it will be released.
    fn try_capture(
        &self,
        handler: &impl InputHandlerAspect,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (handler.node().id());
        self.node()
            .send_remote_signal(
                2611007814387963428u64,
                12158986667525139020u64,
                &data,
                _fds,
            )?;
        let (handler) = data;
        tracing::trace!(
            ? handler, "Sent signal to server, {}::{}", "InputMethodRef", "try_capture"
        );
        Ok(())
    }
    ///If captured by this handler, release it (e.g. the object is let go of after grabbing).
    fn release(&self, handler: &impl InputHandlerAspect) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (handler.node().id());
        self.node()
            .send_remote_signal(
                2611007814387963428u64,
                11905596878821798323u64,
                &data,
                _fds,
            )?;
        let (handler) = data;
        tracing::trace!(
            ? handler, "Sent signal to server, {}::{}", "InputMethodRef", "release"
        );
        Ok(())
    }
}
#[allow(clippy::all)]
///Node representing a spatial input device
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct InputMethod(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl InputMethod {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<InputMethodEvent>(&node);
        InputMethod(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
    pub fn as_input_method_ref(self) -> super::InputMethodRef {
        super::InputMethodRef(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for InputMethod {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for InputMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl InputMethodAspect for InputMethod {}
pub(crate) const INPUT_METHOD_ASPECT_ID: u64 = 14883688361483968991u64;
pub(crate) const INPUT_METHOD_SET_INPUT_SERVER_OPCODE: u64 = 17348904196349853573u64;
pub(crate) const INPUT_METHOD_SET_DATAMAP_SERVER_OPCODE: u64 = 9666763984937627751u64;
pub(crate) const INPUT_METHOD_SET_HANDLER_ORDER_SERVER_OPCODE: u64 = 4447101880184876824u64;
pub(crate) const INPUT_METHOD_SET_CAPTURES_SERVER_OPCODE: u64 = 4141712352465076448u64;
pub(crate) const INPUT_METHOD_CREATE_HANDLER_CLIENT_OPCODE: u64 = 6944316585732678571u64;
pub(crate) const INPUT_METHOD_REQUEST_CAPTURE_HANDLER_CLIENT_OPCODE: u64 = 11807638350036597049u64;
pub(crate) const INPUT_METHOD_RELEASE_HANDLER_CLIENT_OPCODE: u64 = 9300665394087171854u64;
pub(crate) const INPUT_METHOD_DESTROY_HANDLER_CLIENT_OPCODE: u64 = 7635230773176050803u64;
#[derive(Debug)]
pub enum InputMethodEvent {
    CreateHandler { handler: InputHandler, field: Field },
    RequestCaptureHandler { id: u64 },
    ReleaseHandler { id: u64 },
    DestroyHandler { id: u64 },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for InputMethodEvent {
    const ASPECT_ID: u64 = 14883688361483968991u64;
    fn serialize_signal(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        signal_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
    ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
        match signal_id {
            6944316585732678571u64 => {
                let (handler, field): (u64, u64) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? handler, ? field, "Got signal from server, {}::{}", "InputMethod",
                    "create_handler"
                );
                Ok(InputMethodEvent::CreateHandler {
                    handler: InputHandler::from_id(&_client, handler, false),
                    field: Field::from_id(&_client, field, false),
                })
            }
            11807638350036597049u64 => {
                let (id): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? id, "Got signal from server, {}::{}", "InputMethod",
                    "request_capture_handler"
                );
                Ok(InputMethodEvent::RequestCaptureHandler {
                    id: id,
                })
            }
            9300665394087171854u64 => {
                let (id): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? id, "Got signal from server, {}::{}", "InputMethod",
                    "release_handler"
                );
                Ok(InputMethodEvent::ReleaseHandler {
                    id: id,
                })
            }
            7635230773176050803u64 => {
                let (id): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? id, "Got signal from server, {}::{}", "InputMethod",
                    "destroy_handler"
                );
                Ok(InputMethodEvent::DestroyHandler {
                    id: id,
                })
            }
            _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
        }
    }
    fn serialize_method(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        method_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
        response: stardust_xr::messenger::MethodResponse,
    ) -> Option<Self> {
        let response = std::rc::Rc::new(std::cell::RefCell::new(Some(response)));
        let response2 = response.clone();
        let result = || match method_id {
            _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
        };
        match (result)() {
            Ok(event) => Some(event),
            Err(e) => {
                let _ = response2.borrow_mut().take().unwrap().send(Err(e));
                None
            }
        }
    }
}
#[allow(clippy::all)]
///Node representing a spatial input device
pub trait InputMethodAspect: crate::node::NodeType + super::SpatialAspect + super::InputMethodRefAspect + std::fmt::Debug {
    fn recv_input_method_event(&self) -> Option<InputMethodEvent> {
        self.node().recv_event(14883688361483968991u64)
    }
    ///Set the spatial input component of this input method. You must keep the same input data type throughout the entire thing.
    fn set_input(&self, input: InputDataType) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (input);
        self.node()
            .send_remote_signal(
                14883688361483968991u64,
                17348904196349853573u64,
                &data,
                _fds,
            )?;
        let (input) = data;
        tracing::trace!(
            ? input, "Sent signal to server, {}::{}", "InputMethod", "set_input"
        );
        Ok(())
    }
    ///Set the datamap of this input method
    fn set_datamap(
        &self,
        datamap: &stardust_xr::values::Datamap,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (datamap);
        self.node()
            .send_remote_signal(
                14883688361483968991u64,
                9666763984937627751u64,
                &data,
                _fds,
            )?;
        let (datamap) = data;
        tracing::trace!(
            ? datamap, "Sent signal to server, {}::{}", "InputMethod", "set_datamap"
        );
        Ok(())
    }
    ///Set the order of handlers to propagate input to.
    fn set_handler_order(
        &self,
        handlers: &[InputHandler],
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (handlers
            .iter()
            .map(|a| Ok(a.node().id()))
            .collect::<crate::node::NodeResult<Vec<_>>>()?);
        self.node()
            .send_remote_signal(
                14883688361483968991u64,
                4447101880184876824u64,
                &data,
                _fds,
            )?;
        let (handlers) = data;
        tracing::trace!(
            ? handlers, "Sent signal to server, {}::{}", "InputMethod",
            "set_handler_order"
        );
        Ok(())
    }
    ///Set which handlers are captured.
    fn set_captures(&self, handlers: &[InputHandler]) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (handlers
            .iter()
            .map(|a| Ok(a.node().id()))
            .collect::<crate::node::NodeResult<Vec<_>>>()?);
        self.node()
            .send_remote_signal(
                14883688361483968991u64,
                4141712352465076448u64,
                &data,
                _fds,
            )?;
        let (handlers) = data;
        tracing::trace!(
            ? handlers, "Sent signal to server, {}::{}", "InputMethod", "set_captures"
        );
        Ok(())
    }
}
#[allow(clippy::all)]
///Handle raw input events.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct InputHandler(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl InputHandler {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<InputHandlerEvent>(&node);
        InputHandler(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for InputHandler {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for InputHandler {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl InputHandlerAspect for InputHandler {}
pub(crate) const INPUT_HANDLER_ASPECT_ID: u64 = 537028132086008694u64;
pub(crate) const INPUT_HANDLER_INPUT_CLIENT_OPCODE: u64 = 5305312459121645740u64;
#[derive(Debug)]
pub enum InputHandlerEvent {
    Input { methods: Vec<InputMethodRef>, data: Vec<InputData> },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for InputHandlerEvent {
    const ASPECT_ID: u64 = 537028132086008694u64;
    fn serialize_signal(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        signal_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
    ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
        match signal_id {
            5305312459121645740u64 => {
                let (methods, data): (Vec<u64>, Vec<InputData>) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? methods, ? data, "Got signal from server, {}::{}", "InputHandler",
                    "input"
                );
                Ok(InputHandlerEvent::Input {
                    methods: methods
                        .into_iter()
                        .map(|a| Ok(InputMethodRef::from_id(&_client, a, false)))
                        .collect::<Result<Vec<_>, crate::node::NodeError>>()?,
                    data: data
                        .into_iter()
                        .map(|a| Ok(a))
                        .collect::<Result<Vec<_>, crate::node::NodeError>>()?,
                })
            }
            _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
        }
    }
    fn serialize_method(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        method_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
        response: stardust_xr::messenger::MethodResponse,
    ) -> Option<Self> {
        let response = std::rc::Rc::new(std::cell::RefCell::new(Some(response)));
        let response2 = response.clone();
        let result = || match method_id {
            _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
        };
        match (result)() {
            Ok(event) => Some(event),
            Err(e) => {
                let _ = response2.borrow_mut().take().unwrap().send(Err(e));
                None
            }
        }
    }
}
#[allow(clippy::all)]
///Handle raw input events.
pub trait InputHandlerAspect: crate::node::NodeType + super::SpatialAspect + std::fmt::Debug {
    fn recv_input_handler_event(&self) -> Option<InputHandlerEvent> {
        self.node().recv_event(537028132086008694u64)
    }
}
pub(crate) const INTERFACE_CREATE_INPUT_METHOD_SERVER_OPCODE: u64 = 11977582531774730283u64;
///Create an input method node
fn create_input_method(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    initial_data: InputDataType,
    datamap: &stardust_xr::values::Datamap,
) -> crate::node::NodeResult<InputMethod> {
    {
        let mut _fds = Vec::new();
        let data = (id, parent.node().id(), transform, initial_data, datamap);
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(3u64, 0u64, 11977582531774730283u64, &serialized_data, _fds)?;
        let (id, parent, transform, initial_data, datamap) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? initial_data, ? datamap,
            "Sent signal to server, {}::{}", "Interface", "create_input_method"
        );
    }
    Ok(InputMethod::from_id(_client, id, true))
}
pub(crate) const INTERFACE_CREATE_INPUT_HANDLER_SERVER_OPCODE: u64 = 1654491336591158898u64;
///Create an input handler node
fn create_input_handler(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    field: &impl FieldAspect,
) -> crate::node::NodeResult<InputHandler> {
    {
        let mut _fds = Vec::new();
        let data = (id, parent.node().id(), transform, field.node().id());
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(3u64, 0u64, 1654491336591158898u64, &serialized_data, _fds)?;
        let (id, parent, transform, field) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? field, "Sent signal to server, {}::{}",
            "Interface", "create_input_handler"
        );
    }
    Ok(InputHandler::from_id(_client, id, true))
}
