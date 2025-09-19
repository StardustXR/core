pub(crate) const INTERFACE_VERSION: u32 = 1u32;
pub(crate) const INTERFACE_NODE_ID: u64 = 1u64;
///
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BoundingBox {
    pub center: stardust_xr::values::Vector3<f32>,
    pub size: stardust_xr::values::Vector3<f32>,
}
///
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Transform {
    pub translation: Option<stardust_xr::values::Vector3<f32>>,
    pub rotation: Option<stardust_xr::values::Quaternion>,
    pub scale: Option<stardust_xr::values::Vector3<f32>>,
}
#[allow(clippy::all)]
/**
		A reference to a node with spatial attributes (position, rotation, scale).

		Equivalent to a Transform in Unity, Spatial in Godot, etc.
	*/
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct SpatialRef(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl SpatialRef {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        SpatialRef(node)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for SpatialRef {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for SpatialRef {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl SpatialRefAspect for SpatialRef {}
pub(crate) const SPATIAL_REF_ASPECT_ID: u64 = 14774096707642646617u64;
pub(crate) const SPATIAL_REF_GET_LOCAL_BOUNDING_BOX_SERVER_OPCODE: u64 = 15184457389419466387u64;
pub(crate) const SPATIAL_REF_GET_RELATIVE_BOUNDING_BOX_SERVER_OPCODE: u64 = 8077745023404307052u64;
pub(crate) const SPATIAL_REF_GET_TRANSFORM_SERVER_OPCODE: u64 = 6982810219028106561u64;
#[derive(Debug)]
pub enum SpatialRefEvent {}
#[allow(clippy::all)]
/**
		A reference to a node with spatial attributes (position, rotation, scale).

		Equivalent to a Transform in Unity, Spatial in Godot, etc.
	*/
pub trait SpatialRefAspect: crate::node::NodeType + std::fmt::Debug {
    ///Get the bounding box of this spatial and its children relative to another spatial
    async fn get_local_bounding_box(&self) -> crate::node::NodeResult<BoundingBox> {
        {
            let mut _fds = Vec::new();
            let data = ();
            {
                let () = &data;
                tracing::trace!(
                    "Called method on server, {}::{}", "SpatialRef",
                    "get_local_bounding_box"
                );
            }
            let result: BoundingBox = self
                .node()
                .execute_remote_method(
                    14774096707642646617u64,
                    15184457389419466387u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}",
                "SpatialRef", "get_local_bounding_box"
            );
            Ok(deserialized)
        }
    }
    ///Get the bounding box of this spatial and its children relative to itself
    async fn get_relative_bounding_box(
        &self,
        relative_to: &impl SpatialRefAspect,
    ) -> crate::node::NodeResult<BoundingBox> {
        {
            let mut _fds = Vec::new();
            let data = (relative_to.node().id());
            {
                let (relative_to) = &data;
                tracing::trace!(
                    ? relative_to, "Called method on server, {}::{}", "SpatialRef",
                    "get_relative_bounding_box"
                );
            }
            let result: BoundingBox = self
                .node()
                .execute_remote_method(
                    14774096707642646617u64,
                    8077745023404307052u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}",
                "SpatialRef", "get_relative_bounding_box"
            );
            Ok(deserialized)
        }
    }
    ///Get the transform relative to some other spatial node.
    async fn get_transform(
        &self,
        relative_to: &impl SpatialRefAspect,
    ) -> crate::node::NodeResult<Transform> {
        {
            let mut _fds = Vec::new();
            let data = (relative_to.node().id());
            {
                let (relative_to) = &data;
                tracing::trace!(
                    ? relative_to, "Called method on server, {}::{}", "SpatialRef",
                    "get_transform"
                );
            }
            let result: Transform = self
                .node()
                .execute_remote_method(
                    14774096707642646617u64,
                    6982810219028106561u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}",
                "SpatialRef", "get_transform"
            );
            Ok(deserialized)
        }
    }
}
#[allow(clippy::all)]
/**
		A node with spatial attributes (position, rotation, scale) that can be manipulated by zones if zoneable.

		Equivalent to a Transform in Unity, Spatial in Godot, etc.
	*/
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Spatial(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl Spatial {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        Spatial(node)
    }
    pub fn as_spatial_ref(self) -> super::SpatialRef {
        super::SpatialRef(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for Spatial {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for Spatial {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl SpatialAspect for Spatial {}
pub(crate) const SPATIAL_ASPECT_ID: u64 = 17785849468685298036u64;
pub(crate) const SPATIAL_SET_LOCAL_TRANSFORM_SERVER_OPCODE: u64 = 5092462149256736585u64;
pub(crate) const SPATIAL_SET_RELATIVE_TRANSFORM_SERVER_OPCODE: u64 = 15020422542376308840u64;
pub(crate) const SPATIAL_SET_SPATIAL_PARENT_SERVER_OPCODE: u64 = 12472379656662040034u64;
pub(crate) const SPATIAL_SET_SPATIAL_PARENT_IN_PLACE_SERVER_OPCODE: u64 = 1386737540675144626u64;
pub(crate) const SPATIAL_SET_ZONEABLE_SERVER_OPCODE: u64 = 14580454097816778715u64;
pub(crate) const SPATIAL_EXPORT_SPATIAL_SERVER_OPCODE: u64 = 3600225297814947977u64;
#[derive(Debug)]
pub enum SpatialEvent {}
#[allow(clippy::all)]
/**
		A node with spatial attributes (position, rotation, scale) that can be manipulated by zones if zoneable.

		Equivalent to a Transform in Unity, Spatial in Godot, etc.
	*/
pub trait SpatialAspect: crate::node::NodeType + super::OwnedAspect + super::SpatialRefAspect + std::fmt::Debug {
    ///Set the transform of this spatial relative to its spatial parent.
    fn set_local_transform(&self, transform: Transform) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (transform);
        self.node()
            .send_remote_signal(
                17785849468685298036u64,
                5092462149256736585u64,
                &data,
                _fds,
            )?;
        let (transform) = data;
        tracing::trace!(
            ? transform, "Sent signal to server, {}::{}", "Spatial",
            "set_local_transform"
        );
        Ok(())
    }
    ///Set the transform of this spatial relative to another node.
    fn set_relative_transform(
        &self,
        relative_to: &impl SpatialRefAspect,
        transform: Transform,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (relative_to.node().id(), transform);
        self.node()
            .send_remote_signal(
                17785849468685298036u64,
                15020422542376308840u64,
                &data,
                _fds,
            )?;
        let (relative_to, transform) = data;
        tracing::trace!(
            ? relative_to, ? transform, "Sent signal to server, {}::{}", "Spatial",
            "set_relative_transform"
        );
        Ok(())
    }
    /**
			Set the spatial parent with its local transform remaining the same.
			It will silently error and not set the spatial parent if it is to a child of itself.
		*/
    fn set_spatial_parent(
        &self,
        parent: &impl SpatialRefAspect,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (parent.node().id());
        self.node()
            .send_remote_signal(
                17785849468685298036u64,
                12472379656662040034u64,
                &data,
                _fds,
            )?;
        let (parent) = data;
        tracing::trace!(
            ? parent, "Sent signal to server, {}::{}", "Spatial", "set_spatial_parent"
        );
        Ok(())
    }
    /**
			Set the spatial parent with its "global" transform remaining the same.
			It will silently error and not set the spatial parent if it is to a child of itself.
		*/
    fn set_spatial_parent_in_place(
        &self,
        parent: &impl SpatialRefAspect,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (parent.node().id());
        self.node()
            .send_remote_signal(
                17785849468685298036u64,
                1386737540675144626u64,
                &data,
                _fds,
            )?;
        let (parent) = data;
        tracing::trace!(
            ? parent, "Sent signal to server, {}::{}", "Spatial",
            "set_spatial_parent_in_place"
        );
        Ok(())
    }
    /**
			Set if this spatial is zoneable or not.
			You may want to set this to false when being grabbed or interacted with, then back to true when it's floating inert in space.
		*/
    fn set_zoneable(&self, zoneable: bool) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (zoneable);
        self.node()
            .send_remote_signal(
                17785849468685298036u64,
                14580454097816778715u64,
                &data,
                _fds,
            )?;
        let (zoneable) = data;
        tracing::trace!(
            ? zoneable, "Sent signal to server, {}::{}", "Spatial", "set_zoneable"
        );
        Ok(())
    }
    ///Return a UUID representing this node's SpatialRef that you can send to other clients
    async fn export_spatial(&self) -> crate::node::NodeResult<u64> {
        {
            let mut _fds = Vec::new();
            let data = ();
            {
                let () = &data;
                tracing::trace!(
                    "Called method on server, {}::{}", "Spatial", "export_spatial"
                );
            }
            let result: u64 = self
                .node()
                .execute_remote_method(
                    17785849468685298036u64,
                    3600225297814947977u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}",
                "Spatial", "export_spatial"
            );
            Ok(deserialized)
        }
    }
}
#[allow(clippy::all)]
/**
		Node to manipulate spatial nodes across clients.
	*/
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Zone(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl Zone {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<ZoneEvent>(&node);
        Zone(node)
    }
    pub fn as_spatial(self) -> super::Spatial {
        super::Spatial(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for Zone {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for Zone {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl ZoneAspect for Zone {}
pub(crate) const ZONE_ASPECT_ID: u64 = 8505905936867072296u64;
pub(crate) const ZONE_UPDATE_SERVER_OPCODE: u64 = 4876473203673722513u64;
pub(crate) const ZONE_CAPTURE_SERVER_OPCODE: u64 = 11313548931929469818u64;
pub(crate) const ZONE_RELEASE_SERVER_OPCODE: u64 = 11905596878821798323u64;
pub(crate) const ZONE_ENTER_CLIENT_OPCODE: u64 = 17714309963407960406u64;
pub(crate) const ZONE_CAPTURE_CLIENT_OPCODE: u64 = 11313548931929469818u64;
pub(crate) const ZONE_RELEASE_CLIENT_OPCODE: u64 = 11905596878821798323u64;
pub(crate) const ZONE_LEAVE_CLIENT_OPCODE: u64 = 2707764513383459725u64;
#[derive(Debug)]
pub enum ZoneEvent {
    Enter { spatial: SpatialRef },
    Capture { spatial: Spatial },
    Release { id: u64 },
    Leave { id: u64 },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for ZoneEvent {
    const ASPECT_ID: u64 = 8505905936867072296u64;
    fn serialize_signal(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        signal_id: u64,
        _data: &[u8],
        _fds: Vec<std::os::fd::OwnedFd>,
    ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
        match signal_id {
            17714309963407960406u64 => {
                let (spatial): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? spatial, "Got signal from server, {}::{}", "Zone", "enter"
                );
                Ok(ZoneEvent::Enter {
                    spatial: SpatialRef::from_id(&_client, spatial, false),
                })
            }
            11313548931929469818u64 => {
                let (spatial): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? spatial, "Got signal from server, {}::{}", "Zone", "capture"
                );
                Ok(ZoneEvent::Capture {
                    spatial: Spatial::from_id(&_client, spatial, false),
                })
            }
            11905596878821798323u64 => {
                let (id): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(
                    ? id, "Got signal from server, {}::{}", "Zone", "release"
                );
                Ok(ZoneEvent::Release { id: id })
            }
            2707764513383459725u64 => {
                let (id): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!(? id, "Got signal from server, {}::{}", "Zone", "leave");
                Ok(ZoneEvent::Leave { id: id })
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
/**
		Node to manipulate spatial nodes across clients.
	*/
pub trait ZoneAspect: crate::node::NodeType + super::SpatialAspect + std::fmt::Debug {
    fn recv_zone_event(&self) -> Option<ZoneEvent> {
        self.node().recv_event(8505905936867072296u64)
    }
    ///
    fn update(&self) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = ();
        self.node()
            .send_remote_signal(
                8505905936867072296u64,
                4876473203673722513u64,
                &data,
                _fds,
            )?;
        let () = data;
        tracing::trace!("Sent signal to server, {}::{}", "Zone", "update");
        Ok(())
    }
    ///
    fn capture(&self, spatial: &impl SpatialRefAspect) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (spatial.node().id());
        self.node()
            .send_remote_signal(
                8505905936867072296u64,
                11313548931929469818u64,
                &data,
                _fds,
            )?;
        let (spatial) = data;
        tracing::trace!(? spatial, "Sent signal to server, {}::{}", "Zone", "capture");
        Ok(())
    }
    ///
    fn release(&self, spatial: &impl SpatialRefAspect) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = (spatial.node().id());
        self.node()
            .send_remote_signal(
                8505905936867072296u64,
                11905596878821798323u64,
                &data,
                _fds,
            )?;
        let (spatial) = data;
        tracing::trace!(? spatial, "Sent signal to server, {}::{}", "Zone", "release");
        Ok(())
    }
}
pub(crate) const INTERFACE_IMPORT_SPATIAL_REF_SERVER_OPCODE: u64 = 7309812661610962094u64;
///Import a spatial ref from a UUID generated by Spatial::export_spatial
pub async fn import_spatial_ref(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    uid: u64,
) -> crate::node::NodeResult<SpatialRef> {
    let mut _fds = Vec::new();
    let data = (uid);
    {
        let (uid) = &data;
        tracing::trace!(
            ? uid, "Called method on server, {}::{}", "Interface", "import_spatial_ref"
        );
    }
    let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
    let message = _client
        .message_sender_handle
        .method(1u64, 0u64, 7309812661610962094u64, &serialized_data, _fds)
        .await?
        .map_err(|e| crate::node::NodeError::ReturnedError {
            e,
        })?
        .into_message();
    let result: u64 = stardust_xr::schemas::flex::deserialize(&message)?;
    let deserialized = SpatialRef::from_id(&_client, result, false);
    tracing::trace!(
        "return" = ? deserialized, "Method return from server, {}::{}", "Interface",
        "import_spatial_ref"
    );
    Ok(deserialized)
}
pub(crate) const INTERFACE_CREATE_SPATIAL_SERVER_OPCODE: u64 = 3949276749019911643u64;
///Create a spatial relative to another spatial
fn create_spatial(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    zoneable: bool,
) -> crate::node::NodeResult<Spatial> {
    {
        let mut _fds = Vec::new();
        let data = (id, parent.node().id(), transform, zoneable);
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(1u64, 0u64, 3949276749019911643u64, &serialized_data, _fds)?;
        let (id, parent, transform, zoneable) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? zoneable, "Sent signal to server, {}::{}",
            "Interface", "create_spatial"
        );
    }
    Ok(Spatial::from_id(_client, id, true))
}
pub(crate) const INTERFACE_CREATE_ZONE_SERVER_OPCODE: u64 = 7282214243246353525u64;
/**
	    Create a zone given a field, this zone will become inactive if the field is dropped.
        Keep in mind the zone and its field are different spatials, they can move independently.
    */
fn create_zone(
    _client: &std::sync::Arc<crate::client::ClientHandle>,
    id: u64,
    parent: &impl SpatialRefAspect,
    transform: Transform,
    field: &impl FieldAspect,
) -> crate::node::NodeResult<Zone> {
    {
        let mut _fds = Vec::new();
        let data = (id, parent.node().id(), transform, field.node().id());
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(1u64, 0u64, 7282214243246353525u64, &serialized_data, _fds)?;
        let (id, parent, transform, field) = data;
        tracing::trace!(
            ? id, ? parent, ? transform, ? field, "Sent signal to server, {}::{}",
            "Interface", "create_zone"
        );
    }
    Ok(Zone::from_id(_client, id, true))
}
