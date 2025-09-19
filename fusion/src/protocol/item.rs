pub(crate) const INTERFACE_VERSION: u32 = 1u32;
pub(crate) const INTERFACE_NODE_ID: u64 = 10u64;
#[allow(clippy::all)]
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Item(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl Item {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        Item(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for Item {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for Item {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl ItemAspect for Item {}
pub(crate) const ITEM_ASPECT_ID: u64 = 18318655529277677339u64;
pub(crate) const ITEM_RELEASE_SERVER_OPCODE: u64 = 11905596878821798323u64;
#[derive(Debug)]
pub enum ItemEvent {}
#[allow(clippy::all)]
///
pub trait ItemAspect: crate::node::NodeType + super::SpatialAspect + std::fmt::Debug {
    ///
    fn release(&self) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = ();
        self.node()
            .send_remote_signal(
                18318655529277677339u64,
                11905596878821798323u64,
                &data,
                _fds,
            )?;
        let () = data;
        tracing::trace!("Sent signal to server, {}::{}", "Item", "release");
        Ok(())
    }
}
#[allow(clippy::all)]
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ItemAcceptor(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl ItemAcceptor {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<ItemAcceptorEvent>(&node);
        ItemAcceptor(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for ItemAcceptor {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for ItemAcceptor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl ItemAcceptorAspect for ItemAcceptor {}
pub(crate) const ITEM_ACCEPTOR_ASPECT_ID: u64 = 10274055739447304636u64;
pub(crate) const ITEM_ACCEPTOR_RELEASE_ITEM_CLIENT_OPCODE: u64 = 14821884892980204849u64;
#[derive(Debug)]
pub enum ItemAcceptorEvent {
    ReleaseItem { item_id: u64 },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for ItemAcceptorEvent {
    const ASPECT_ID: u64 = 10274055739447304636u64;
    fn serialize_signal(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        signal_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
    ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
        match signal_id {
            14821884892980204849u64 => {
                let (item_id): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? item_id, "Got signal from server, {}::{}", "ItemAcceptor",
                    "release_item"
                );
                Ok(ItemAcceptorEvent::ReleaseItem {
                    item_id: item_id,
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
pub trait ItemAcceptorAspect: crate::node::NodeType + super::SpatialAspect + std::fmt::Debug {
    fn recv_item_acceptor_event(&self) -> Option<ItemAcceptorEvent> {
        self.node().recv_event(10274055739447304636u64)
    }
}
#[allow(clippy::all)]
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ItemUi(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl ItemUi {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<ItemUiEvent>(&node);
        ItemUi(node)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for ItemUi {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for ItemUi {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl ItemUiAspect for ItemUi {}
pub(crate) const ITEM_UI_ASPECT_ID: u64 = 7265392688253796589u64;
pub(crate) const ITEM_UI_CAPTURE_ITEM_CLIENT_OPCODE: u64 = 1751367302976798762u64;
pub(crate) const ITEM_UI_RELEASE_ITEM_CLIENT_OPCODE: u64 = 14821884892980204849u64;
pub(crate) const ITEM_UI_DESTROY_ITEM_CLIENT_OPCODE: u64 = 11215449886948753686u64;
pub(crate) const ITEM_UI_DESTROY_ACCEPTOR_CLIENT_OPCODE: u64 = 3521554848760623636u64;
#[derive(Debug)]
pub enum ItemUiEvent {
    CaptureItem { item_id: u64, acceptor_id: u64 },
    ReleaseItem { item_id: u64, acceptor_id: u64 },
    DestroyItem { id: u64 },
    DestroyAcceptor { id: u64 },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for ItemUiEvent {
    const ASPECT_ID: u64 = 7265392688253796589u64;
    fn serialize_signal(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        signal_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
    ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
        match signal_id {
            1751367302976798762u64 => {
                let (item_id, acceptor_id): (u64, u64) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? item_id, ? acceptor_id, "Got signal from server, {}::{}", "ItemUi",
                    "capture_item"
                );
                Ok(ItemUiEvent::CaptureItem {
                    item_id: item_id,
                    acceptor_id: acceptor_id,
                })
            }
            14821884892980204849u64 => {
                let (item_id, acceptor_id): (u64, u64) = stardust_xr::schemas::flex::deserialize(
                    _data,
                )?;
                tracing::trace!(
                    ? item_id, ? acceptor_id, "Got signal from server, {}::{}", "ItemUi",
                    "release_item"
                );
                Ok(ItemUiEvent::ReleaseItem {
                    item_id: item_id,
                    acceptor_id: acceptor_id,
                })
            }
            11215449886948753686u64 => {
                let (id): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? id, "Got signal from server, {}::{}", "ItemUi", "destroy_item"
                );
                Ok(ItemUiEvent::DestroyItem { id: id })
            }
            3521554848760623636u64 => {
                let (id): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? id, "Got signal from server, {}::{}", "ItemUi", "destroy_acceptor"
                );
                Ok(ItemUiEvent::DestroyAcceptor {
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
///
pub trait ItemUiAspect: crate::node::NodeType + std::fmt::Debug {
    fn recv_item_ui_event(&self) -> Option<ItemUiEvent> {
        self.node().recv_event(7265392688253796589u64)
    }
}
