pub(crate) const INTERFACE_VERSION: u32 = 1u32;
pub(crate) const INTERFACE_NODE_ID: u64 = 11u64;
#[allow(clippy::all)]
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CameraItem(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl CameraItem {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        CameraItem(node)
    }
    pub fn as_item(self) -> super::Item {
        super::Item(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for CameraItem {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for CameraItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl CameraItemAspect for CameraItem {}
pub(crate) const CAMERA_ITEM_ASPECT_ID: u64 = 15672103361112197430u64;
#[derive(Debug)]
pub enum CameraItemEvent {}
#[allow(clippy::all)]
///
pub trait CameraItemAspect: crate::node::NodeType + super::ItemAspect + std::fmt::Debug {}
#[allow(clippy::all)]
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CameraItemUi(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl CameraItemUi {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<CameraItemUiEvent>(&node);
        CameraItemUi(node)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for CameraItemUi {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for CameraItemUi {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl CameraItemUiAspect for CameraItemUi {}
pub(crate) const CAMERA_ITEM_UI_ASPECT_ID: u64 = 708021061010127172u64;
pub(crate) const CAMERA_ITEM_UI_CREATE_ITEM_CLIENT_OPCODE: u64 = 15524466827491111758u64;
pub(crate) const CAMERA_ITEM_UI_CREATE_ACCEPTOR_CLIENT_OPCODE: u64 = 16628549773568263004u64;
#[derive(Debug)]
pub enum CameraItemUiEvent {
    CreateItem { item: CameraItem },
    CreateAcceptor { acceptor: CameraItemAcceptor, acceptor_field: Field },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for CameraItemUiEvent {
    const ASPECT_ID: u64 = 708021061010127172u64;
    fn serialize_signal(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        signal_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
    ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
        match signal_id {
            15524466827491111758u64 => {
                let (item): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? item, "Got signal from server, {}::{}", "CameraItemUi",
                    "create_item"
                );
                Ok(CameraItemUiEvent::CreateItem {
                    item: CameraItem::from_id(&_client, item, false),
                })
            }
            16628549773568263004u64 => {
                let (acceptor, acceptor_field): (u64, u64) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? acceptor, ? acceptor_field, "Got signal from server, {}::{}",
                    "CameraItemUi", "create_acceptor"
                );
                Ok(CameraItemUiEvent::CreateAcceptor {
                    acceptor: CameraItemAcceptor::from_id(&_client, acceptor, false),
                    acceptor_field: Field::from_id(&_client, acceptor_field, false),
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
///
pub trait CameraItemUiAspect: crate::node::NodeType + std::fmt::Debug {
    fn recv_camera_item_ui_event(&self) -> Option<CameraItemUiEvent> {
        self.node().recv_event(708021061010127172u64)
    }
}
#[allow(clippy::all)]
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct CameraItemAcceptor(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl CameraItemAcceptor {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<CameraItemAcceptorEvent>(&node);
        CameraItemAcceptor(node)
    }
    pub fn as_item_acceptor(self) -> super::ItemAcceptor {
        super::ItemAcceptor(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for CameraItemAcceptor {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for CameraItemAcceptor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl CameraItemAcceptorAspect for CameraItemAcceptor {}
pub(crate) const CAMERA_ITEM_ACCEPTOR_ASPECT_ID: u64 = 5036088114779304421u64;
pub(crate) const CAMERA_ITEM_ACCEPTOR_CAPTURE_ITEM_SERVER_OPCODE: u64 = 1751367302976798762u64;
pub(crate) const CAMERA_ITEM_ACCEPTOR_CAPTURE_ITEM_CLIENT_OPCODE: u64 = 1751367302976798762u64;
#[derive(Debug)]
pub enum CameraItemAcceptorEvent {
    CaptureItem { item: CameraItem },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for CameraItemAcceptorEvent {
    const ASPECT_ID: u64 = 5036088114779304421u64;
    fn serialize_signal(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        signal_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
    ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
        match signal_id {
            1751367302976798762u64 => {
                let (item): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? item, "Got signal from server, {}::{}", "CameraItemAcceptor",
                    "capture_item"
                );
                Ok(CameraItemAcceptorEvent::CaptureItem {
                    item: CameraItem::from_id(&_client, item, false),
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
///
pub trait CameraItemAcceptorAspect: crate::node::NodeType + super::ItemAcceptorAspect + std::fmt::Debug {
    fn recv_camera_item_acceptor_event(&self) -> Option<CameraItemAcceptorEvent> {
        self.node().recv_event(5036088114779304421u64)
    }
    ///
    fn capture_item(&self, item: &impl CameraItemAspect) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (item.node().id());
        self.node()
            .send_remote_signal(
                5036088114779304421u64,
                1751367302976798762u64,
                &data,
                _fds,
            )?;
        let (item) = data;
        tracing::trace!(
            ? item, "Sent signal to server, {}::{}", "CameraItemAcceptor", "capture_item"
        );
        Ok(())
    }
}
pub(crate) const INTERFACE_CREATE_CAMERA_ITEM_SERVER_OPCODE: u64 = 16398826726504952950u64;
///Create a camera item at a specific location
fn create_camera_item(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    proj_matrix: impl Into<stardust_xr::values::Mat4>,
    px_size: impl Into<stardust_xr::values::Vector2<u32>>,
) -> crate::node::NodeResult<CameraItem> {
    {
        let mut _fds = Vec::new();
        let data = (
            id,
            parent.node().id(),
            transform,
            proj_matrix.into(),
            px_size.into(),
        );
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(11u64, 0u64, 16398826726504952950u64, &serialized_data, _fds)?;
        let (id, parent, transform, proj_matrix, px_size) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? proj_matrix, ? px_size,
            "Sent signal to server, {}::{}", "Interface", "create_camera_item"
        );
    }
    Ok(CameraItem::from_id(_client, id, true))
}
pub(crate) const INTERFACE_REGISTER_CAMERA_ITEM_UI_SERVER_OPCODE: u64 = 13470969625663359032u64;
#[allow(clippy::all)]
///Register this client to manage camera items and create default 3D UI for them.
pub fn register_camera_item_ui(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
) -> crate::node::NodeResult<()> {
    let mut _fds = Vec::new();
    let data = ();
    let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
    _client
        .message_sender_handle
        .signal(11u64, 0u64, 13470969625663359032u64, &serialized_data, _fds)?;
    let () = data;
    tracing::trace!(
        "Sent signal to server, {}::{}", "Interface", "register_camera_item_ui"
    );
    Ok(())
}
pub(crate) const INTERFACE_CREATE_CAMERA_ITEM_ACCEPTOR_SERVER_OPCODE: u64 = 13070169044031356364u64;
///Create an item acceptor to allow temporary ownership of a given type of item. Creates a node at `/item/camera/acceptor/<name>`.
fn create_camera_item_acceptor(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    field: &impl FieldAspect,
) -> crate::node::NodeResult<CameraItemAcceptor> {
    {
        let mut _fds = Vec::new();
        let data = (id, parent.node().id(), transform, field.node().id());
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(11u64, 0u64, 13070169044031356364u64, &serialized_data, _fds)?;
        let (id, parent, transform, field) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? field, "Sent signal to server, {}::{}",
            "Interface", "create_camera_item_acceptor"
        );
    }
    Ok(CameraItemAcceptor::from_id(_client, id, true))
}
