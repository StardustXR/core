#![allow(async_fn_in_trait, unused_parens, clippy::all)]
use crate::node::NodeType;
pub(crate) trait AddAspect<A> {
    fn add_aspect(
        registry: &crate::scenegraph::NodeRegistry,
        node_id: u64,
        aspect_id: u64,
    );
}
#[allow(unused_imports)]
use root::*;
pub mod root {
    #[allow(unused_imports)]
    use super::*;
    pub(crate) const INTERFACE_VERSION: u32 = 1u32;
    ///
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct FrameInfo {
        pub delta: f32,
        pub elapsed: f32,
    }
    ///The persistent state of a Stardust client.
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct ClientState {
        pub data: Option<Vec<u8>>,
        pub root: u64,
        pub spatial_anchors: stardust_xr::values::Map<String, u64>,
    }
    ///The hub of the client. Spatially this is positioned where the client is started so is a stable base to position things relative to.
    #[derive(Debug, Clone)]
    pub struct Root {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
        pub(crate) root_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<RootEvent>>,
        >,
    }
    impl Root {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            let root_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 7212020743076450030u64).into(),
            );
            Root { core, root_event }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for Root {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for Root {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for Root {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for Root {}
    impl serde::Serialize for Root {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialRefAspect for Root {}
    impl RootAspect for Root {
        fn recv_root_event(&self) -> Option<RootEvent> {
            self.root_event.lock().unwrap().try_recv().ok()
        }
    }
    #[derive(Debug)]
    pub enum RootEvent {
        Ping { response: crate::TypedMethodResponse<()> },
        Frame { info: FrameInfo },
        SaveState { response: crate::TypedMethodResponse<ClientState> },
    }
    impl crate::scenegraph::EventParser for RootEvent {
        const ASPECT_ID: u64 = 7212020743076450030u64;
        fn parse_signal(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            signal_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match signal_id {
                2586777469268117179u64 => {
                    let (info): (FrameInfo) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? info, "Got signal from server, {}::{}", "Root", "frame"
                    );
                    Ok(RootEvent::Frame { info: info })
                }
                _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
            }
        }
        fn parse_method(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            method_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
            response: stardust_xr::messenger::MethodResponse,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match method_id {
                1374738518356883234u64 => {
                    let (): () = stardust_xr::schemas::flex::deserialize(_data)?;
                    tracing::trace!("Method called from server, {}::{}", "Root", "ping");
                    Ok(RootEvent::Ping {
                        response: crate::TypedMethodResponse(
                            response,
                            std::marker::PhantomData,
                        ),
                    })
                }
                6559167809188075643u64 => {
                    let (): () = stardust_xr::schemas::flex::deserialize(_data)?;
                    tracing::trace!(
                        "Method called from server, {}::{}", "Root", "save_state"
                    );
                    Ok(RootEvent::SaveState {
                        response: crate::TypedMethodResponse(
                            response,
                            std::marker::PhantomData,
                        ),
                    })
                }
                _ => {
                    let _ = response
                        .send(
                            Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
                        );
                    Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
                }
            }
        }
    }
    ///The hub of the client. Spatially this is positioned where the client is started so is a stable base to position things relative to.
    pub trait RootAspect: crate::node::NodeType + super::SpatialRefAspect + std::fmt::Debug {
        fn recv_root_event(&self) -> Option<RootEvent>;
        ///Get the current state. Useful to check the state before you initialize your application!
        async fn get_state(&self) -> crate::node::NodeResult<ClientState> {
            {
                let mut _fds = Vec::new();
                let data = ();
                {
                    let () = &data;
                    tracing::trace!(
                        "Called method on server, {}::{}", "Root", "get_state"
                    );
                }
                let result: ClientState = self
                    .node()
                    .call_method(
                        7212020743076450030u64,
                        14958324855167218950u64,
                        &data,
                        _fds,
                    )
                    .await?;
                let deserialized = result;
                tracing::trace!(
                    "return" = ? deserialized, "Method return from server, {}::{}",
                    "Root", "get_state"
                );
                Ok(deserialized)
            }
        }
        /**
			Generate a client state token and return it back.

			When launching a new client, set the environment variable `STARDUST_STARTUP_TOKEN` to the returned string.
			Make sure the environment variable shows in `/proc/{pid}/environ` as that's the only reliable way to pass the value to the server (suggestions welcome).
		*/
        async fn generate_state_token(
            &self,
            state: ClientState,
        ) -> crate::node::NodeResult<String> {
            {
                let mut _fds = Vec::new();
                let data = (state);
                {
                    let (state) = &data;
                    tracing::trace!(
                        ? state, "Called method on server, {}::{}", "Root",
                        "generate_state_token"
                    );
                }
                let result: String = self
                    .node()
                    .call_method(
                        7212020743076450030u64,
                        530863980839400599u64,
                        &data,
                        _fds,
                    )
                    .await?;
                let deserialized = result;
                tracing::trace!(
                    "return" = ? deserialized, "Method return from server, {}::{}",
                    "Root", "generate_state_token"
                );
                Ok(deserialized)
            }
        }
        ///Get a hashmap of all the environment variables to connect a given app to the stardust server
        async fn get_connection_environment(
            &self,
        ) -> crate::node::NodeResult<stardust_xr::values::Map<String, String>> {
            {
                let mut _fds = Vec::new();
                let data = ();
                {
                    let () = &data;
                    tracing::trace!(
                        "Called method on server, {}::{}", "Root",
                        "get_connection_environment"
                    );
                }
                let result: stardust_xr::values::Map<String, String> = self
                    .node()
                    .call_method(
                        7212020743076450030u64,
                        3344613215577382567u64,
                        &data,
                        _fds,
                    )
                    .await?;
                let deserialized = result
                    .into_iter()
                    .map(|(k, a)| Ok((k, a)))
                    .collect::<
                        Result<
                            stardust_xr::values::Map<String, _>,
                            crate::node::NodeError,
                        >,
                    >()?;
                tracing::trace!(
                    "return" = ? deserialized, "Method return from server, {}::{}",
                    "Root", "get_connection_environment"
                );
                Ok(deserialized)
            }
        }
        ///Set initial list of folders to look for namespaced resources in
        fn set_base_prefixes(&self, prefixes: &[String]) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (prefixes
                .iter()
                .map(|a| Ok(a))
                .collect::<crate::node::NodeResult<Vec<_>>>()?);
            self.node()
                .send_signal(
                    7212020743076450030u64,
                    3714507829296596139u64,
                    &data,
                    _fds,
                )?;
            let (prefixes) = data;
            tracing::trace!(
                ? prefixes, "Sent signal to server, {}::{}", "Root", "set_base_prefixes"
            );
            Ok(())
        }
        ///Cleanly disconnect from the server
        fn disconnect(&self) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = ();
            self.node()
                .send_signal(
                    7212020743076450030u64,
                    662137628972844924u64,
                    &data,
                    _fds,
                )?;
            let () = data;
            tracing::trace!("Sent signal to server, {}::{}", "Root", "disconnect");
            Ok(())
        }
    }
}
#[allow(unused_imports)]
use node::*;
pub mod node {
    #[allow(unused_imports)]
    use super::*;
    pub(crate) const INTERFACE_VERSION: u32 = 1u32;
    #[derive(Debug)]
    pub enum OwnedEvent {}
    ///This node was created by the current client and can be disabled/destroyed
    pub trait OwnedAspect: crate::node::NodeType + std::fmt::Debug {
        ///Set if this node is enabled or not. Disabled drawables won't render, input handlers won't receive input, etc.
        fn set_enabled(&self, enabled: bool) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (enabled);
            self.node()
                .send_signal(
                    15801764205032075891u64,
                    13365497663235993822u64,
                    &data,
                    _fds,
                )?;
            let (enabled) = data;
            tracing::trace!(
                ? enabled, "Sent signal to server, {}::{}", "Owned", "set_enabled"
            );
            Ok(())
        }
        ///Destroy this node immediately. Not all nodes will have this method, those that don't can be dropped client-side without issue.
        fn destroy(&self) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = ();
            self.node()
                .send_signal(
                    15801764205032075891u64,
                    8637450960623370830u64,
                    &data,
                    _fds,
                )?;
            let () = data;
            tracing::trace!("Sent signal to server, {}::{}", "Owned", "destroy");
            Ok(())
        }
    }
}
#[allow(unused_imports)]
use spatial::*;
pub mod spatial {
    #[allow(unused_imports)]
    use super::*;
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
    /**
		A reference to a node with spatial attributes (position, rotation, scale).

		Equivalent to a Transform in Unity, Spatial in Godot, etc.
	*/
    #[derive(Debug, Clone)]
    pub struct SpatialRef {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl SpatialRef {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            SpatialRef { core }
        }
    }
    impl crate::node::NodeType for SpatialRef {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for SpatialRef {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for SpatialRef {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for SpatialRef {}
    impl serde::Serialize for SpatialRef {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialRefAspect for SpatialRef {}
    #[derive(Debug)]
    pub enum SpatialRefEvent {}
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
                    .call_method(
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
                let data = (relative_to.node().id);
                {
                    let (relative_to) = &data;
                    tracing::trace!(
                        ? relative_to, "Called method on server, {}::{}", "SpatialRef",
                        "get_relative_bounding_box"
                    );
                }
                let result: BoundingBox = self
                    .node()
                    .call_method(
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
                let data = (relative_to.node().id);
                {
                    let (relative_to) = &data;
                    tracing::trace!(
                        ? relative_to, "Called method on server, {}::{}", "SpatialRef",
                        "get_transform"
                    );
                }
                let result: Transform = self
                    .node()
                    .call_method(
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
    /**
		A node with spatial attributes (position, rotation, scale).

		Equivalent to a Transform in Unity, Spatial in Godot, etc.
	*/
    #[derive(Debug, Clone)]
    pub struct Spatial {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl Spatial {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            Spatial { core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for Spatial {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for Spatial {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for Spatial {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for Spatial {}
    impl serde::Serialize for Spatial {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialRefAspect for Spatial {}
    impl OwnedAspect for Spatial {}
    impl SpatialAspect for Spatial {}
    #[derive(Debug)]
    pub enum SpatialEvent {}
    /**
		A node with spatial attributes (position, rotation, scale).

		Equivalent to a Transform in Unity, Spatial in Godot, etc.
	*/
    pub trait SpatialAspect: crate::node::NodeType + super::SpatialRefAspect + super::OwnedAspect + std::fmt::Debug {
        ///Set the transform of this spatial relative to its spatial parent.
        fn set_local_transform(
            &self,
            transform: Transform,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (transform);
            self.node()
                .send_signal(
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
            let data = (relative_to.node().id, transform);
            self.node()
                .send_signal(
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
            let data = (parent.node().id);
            self.node()
                .send_signal(
                    17785849468685298036u64,
                    12472379656662040034u64,
                    &data,
                    _fds,
                )?;
            let (parent) = data;
            tracing::trace!(
                ? parent, "Sent signal to server, {}::{}", "Spatial",
                "set_spatial_parent"
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
            let data = (parent.node().id);
            self.node()
                .send_signal(
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
                    .call_method(
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
                ? uid, "Called method on server, {}::{}", "Interface",
                "import_spatial_ref"
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
        let deserialized = SpatialRef::from_id(_client, result, false);
        tracing::trace!(
            "return" = ? deserialized, "Method return from server, {}::{}", "Interface",
            "import_spatial_ref"
        );
        Ok(deserialized)
    }
    ///Create a spatial relative to another spatial
    pub fn create_spatial(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        parent: &impl SpatialRefAspect,
        transform: Transform,
    ) -> crate::node::NodeResult<Spatial> {
        {
            let mut _fds = Vec::new();
            let data = (id, parent.node().id, transform);
            let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
            _client
                .message_sender_handle
                .signal(1u64, 0u64, 3949276749019911643u64, &serialized_data, _fds)?;
            let (id, parent, transform) = data;
            tracing::trace!(
                ? id, ? parent, ? transform, "Sent signal to server, {}::{}",
                "Interface", "create_spatial"
            );
        }
        Ok(Spatial::from_id(_client, id, true))
    }
}
#[allow(unused_imports)]
use field::*;
pub mod field {
    #[allow(unused_imports)]
    use super::*;
    pub(crate) const INTERFACE_VERSION: u32 = 1u32;
    pub(crate) const INTERFACE_NODE_ID: u64 = 2u64;
    ///The shape of a given field.
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(tag = "t", content = "c")]
    pub enum Shape {
        ///Box with a given size in meters
        Box(stardust_xr::values::Vector3<f32>),
        ///Cylinder aligned to the XZ plane
        Cylinder(CylinderShape),
        ///Sphere with a given radius in meters
        Sphere(f32),
        ///Torus aligned to the XZ plane
        Torus(TorusShape),
    }
    ///Information about raymarching a field. All vectors are relative to the spatial reference used.
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct RayMarchResult {
        pub ray_origin: stardust_xr::values::Vector3<f32>,
        pub ray_direction: stardust_xr::values::Vector3<f32>,
        pub min_distance: f32,
        pub deepest_point_distance: f32,
        pub ray_length: f32,
        pub ray_steps: u32,
    }
    ///Cylinder shape info
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct CylinderShape {
        pub length: f32,
        pub radius: f32,
    }
    ///Torus shape info
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct TorusShape {
        pub radius_a: f32,
        pub radius_b: f32,
    }
    ///A reference to a signed distance field that you can sample
    #[derive(Debug, Clone)]
    pub struct FieldRef {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl FieldRef {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            FieldRef { core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for FieldRef {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for FieldRef {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for FieldRef {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for FieldRef {}
    impl serde::Serialize for FieldRef {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialRefAspect for FieldRef {}
    impl FieldRefAspect for FieldRef {}
    #[derive(Debug)]
    pub enum FieldRefEvent {}
    ///A reference to a signed distance field that you can sample
    pub trait FieldRefAspect: crate::node::NodeType + super::SpatialRefAspect + std::fmt::Debug {
        ///Get the distance to the surface of this field relative to the `point` in `space`
        async fn distance(
            &self,
            space: &impl SpatialRefAspect,
            point: impl Into<stardust_xr::values::Vector3<f32>>,
        ) -> crate::node::NodeResult<f32> {
            {
                let mut _fds = Vec::new();
                let data = (space.node().id, point.into());
                {
                    let (space, point) = &data;
                    tracing::trace!(
                        ? space, ? point, "Called method on server, {}::{}", "FieldRef",
                        "distance"
                    );
                }
                let result: f32 = self
                    .node()
                    .call_method(
                        10662923473076663509u64,
                        12706699825100237095u64,
                        &data,
                        _fds,
                    )
                    .await?;
                let deserialized = result;
                tracing::trace!(
                    "return" = ? deserialized, "Method return from server, {}::{}",
                    "FieldRef", "distance"
                );
                Ok(deserialized)
            }
        }
        ///Get a vector pointing away from surface of this field relative to the `point` in `space`
        async fn normal(
            &self,
            space: &impl SpatialRefAspect,
            point: impl Into<stardust_xr::values::Vector3<f32>>,
        ) -> crate::node::NodeResult<stardust_xr::values::Vector3<f32>> {
            {
                let mut _fds = Vec::new();
                let data = (space.node().id, point.into());
                {
                    let (space, point) = &data;
                    tracing::trace!(
                        ? space, ? point, "Called method on server, {}::{}", "FieldRef",
                        "normal"
                    );
                }
                let result: stardust_xr::values::Vector3<f32> = self
                    .node()
                    .call_method(
                        10662923473076663509u64,
                        10933809934326220183u64,
                        &data,
                        _fds,
                    )
                    .await?;
                let deserialized = result;
                tracing::trace!(
                    "return" = ? deserialized, "Method return from server, {}::{}",
                    "FieldRef", "normal"
                );
                Ok(deserialized)
            }
        }
        ///Get the closest point on the surface of this field relative to the `point` in `space`
        async fn closest_point(
            &self,
            space: &impl SpatialRefAspect,
            point: impl Into<stardust_xr::values::Vector3<f32>>,
        ) -> crate::node::NodeResult<stardust_xr::values::Vector3<f32>> {
            {
                let mut _fds = Vec::new();
                let data = (space.node().id, point.into());
                {
                    let (space, point) = &data;
                    tracing::trace!(
                        ? space, ? point, "Called method on server, {}::{}", "FieldRef",
                        "closest_point"
                    );
                }
                let result: stardust_xr::values::Vector3<f32> = self
                    .node()
                    .call_method(
                        10662923473076663509u64,
                        13473947755141124846u64,
                        &data,
                        _fds,
                    )
                    .await?;
                let deserialized = result;
                tracing::trace!(
                    "return" = ? deserialized, "Method return from server, {}::{}",
                    "FieldRef", "closest_point"
                );
                Ok(deserialized)
            }
        }
        ///Get information from the server raymarching the given ray in `space` through this field such as steps, closest/deepest distance, etc.
        async fn ray_march(
            &self,
            space: &impl SpatialRefAspect,
            ray_origin: impl Into<stardust_xr::values::Vector3<f32>>,
            ray_direction: impl Into<stardust_xr::values::Vector3<f32>>,
        ) -> crate::node::NodeResult<RayMarchResult> {
            {
                let mut _fds = Vec::new();
                let data = (space.node().id, ray_origin.into(), ray_direction.into());
                {
                    let (space, ray_origin, ray_direction) = &data;
                    tracing::trace!(
                        ? space, ? ray_origin, ? ray_direction,
                        "Called method on server, {}::{}", "FieldRef", "ray_march"
                    );
                }
                let result: RayMarchResult = self
                    .node()
                    .call_method(
                        10662923473076663509u64,
                        7352457860499612292u64,
                        &data,
                        _fds,
                    )
                    .await?;
                let deserialized = result;
                tracing::trace!(
                    "return" = ? deserialized, "Method return from server, {}::{}",
                    "FieldRef", "ray_march"
                );
                Ok(deserialized)
            }
        }
    }
    ///A signed distance field with adjustable shape. Replaces colliders.
    #[derive(Debug, Clone)]
    pub struct Field {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl Field {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            Field { core }
        }
        pub fn as_field_ref(self) -> super::FieldRef {
            super::FieldRef { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
    }
    impl crate::node::NodeType for Field {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for Field {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for Field {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for Field {}
    impl serde::Serialize for Field {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl FieldRefAspect for Field {}
    impl SpatialRefAspect for Field {}
    impl SpatialAspect for Field {}
    impl OwnedAspect for Field {}
    impl FieldAspect for Field {}
    #[derive(Debug)]
    pub enum FieldEvent {}
    ///A signed distance field with adjustable shape. Replaces colliders.
    pub trait FieldAspect: crate::node::NodeType + super::FieldRefAspect + super::SpatialRefAspect + super::SpatialAspect + super::OwnedAspect + std::fmt::Debug {
        ///Set the shape of this field (and its parameters)
        fn set_shape(&self, shape: Shape) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (shape);
            self.node()
                .send_signal(
                    3948434400034960392u64,
                    10076774457453995458u64,
                    &data,
                    _fds,
                )?;
            let (shape) = data;
            tracing::trace!(
                ? shape, "Sent signal to server, {}::{}", "Field", "set_shape"
            );
            Ok(())
        }
        ///Return a UUID representing this node's FieldRef that you can send to other clients
        async fn export_field(&self) -> crate::node::NodeResult<u64> {
            {
                let mut _fds = Vec::new();
                let data = ();
                {
                    let () = &data;
                    tracing::trace!(
                        "Called method on server, {}::{}", "Field", "export_field"
                    );
                }
                let result: u64 = self
                    .node()
                    .call_method(
                        3948434400034960392u64,
                        939650650519133349u64,
                        &data,
                        _fds,
                    )
                    .await?;
                let deserialized = result;
                tracing::trace!(
                    "return" = ? deserialized, "Method return from server, {}::{}",
                    "Field", "export_field"
                );
                Ok(deserialized)
            }
        }
    }
    ///Import a FieldRef from a UUID generated by Field::export_field
    pub async fn import_field_ref(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        uid: u64,
    ) -> crate::node::NodeResult<FieldRef> {
        let mut _fds = Vec::new();
        let data = (uid);
        {
            let (uid) = &data;
            tracing::trace!(
                ? uid, "Called method on server, {}::{}", "Interface", "import_field_ref"
            );
        }
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        let message = _client
            .message_sender_handle
            .method(2u64, 0u64, 5844955584634021418u64, &serialized_data, _fds)
            .await?
            .map_err(|e| crate::node::NodeError::ReturnedError {
                e,
            })?
            .into_message();
        let result: u64 = stardust_xr::schemas::flex::deserialize(&message)?;
        let deserialized = FieldRef::from_id(_client, result, false);
        tracing::trace!(
            "return" = ? deserialized, "Method return from server, {}::{}", "Interface",
            "import_field_ref"
        );
        Ok(deserialized)
    }
    ///Create a field with the shape of a box
    pub fn create_field(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        parent: &impl SpatialRefAspect,
        transform: Transform,
        shape: Shape,
    ) -> crate::node::NodeResult<Field> {
        {
            let mut _fds = Vec::new();
            let data = (id, parent.node().id, transform, shape);
            let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
            _client
                .message_sender_handle
                .signal(2u64, 0u64, 3216373392735127623u64, &serialized_data, _fds)?;
            let (id, parent, transform, shape) = data;
            tracing::trace!(
                ? id, ? parent, ? transform, ? shape, "Sent signal to server, {}::{}",
                "Interface", "create_field"
            );
        }
        Ok(Field::from_id(_client, id, true))
    }
}
#[allow(unused_imports)]
use audio::*;
pub mod audio {
    #[allow(unused_imports)]
    use super::*;
    pub(crate) const INTERFACE_VERSION: u32 = 1u32;
    pub(crate) const INTERFACE_NODE_ID: u64 = 10u64;
    ///Simple spatial point audio source
    #[derive(Debug, Clone)]
    pub struct Sound {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl Sound {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            Sound { core }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for Sound {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for Sound {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for Sound {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for Sound {}
    impl serde::Serialize for Sound {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialAspect for Sound {}
    impl OwnedAspect for Sound {}
    impl SpatialRefAspect for Sound {}
    impl SoundAspect for Sound {}
    #[derive(Debug)]
    pub enum SoundEvent {}
    ///Simple spatial point audio source
    pub trait SoundAspect: crate::node::NodeType + super::SpatialAspect + super::OwnedAspect + super::SpatialRefAspect + std::fmt::Debug {
        ///Play sound effect
        fn play(&self) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = ();
            self.node()
                .send_signal(
                    17761155925539609649u64,
                    18267594382511242772u64,
                    &data,
                    _fds,
                )?;
            let () = data;
            tracing::trace!("Sent signal to server, {}::{}", "Sound", "play");
            Ok(())
        }
        ///Stop sound effect
        fn stop(&self) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = ();
            self.node()
                .send_signal(
                    17761155925539609649u64,
                    4968801543080236686u64,
                    &data,
                    _fds,
                )?;
            let () = data;
            tracing::trace!("Sent signal to server, {}::{}", "Sound", "stop");
            Ok(())
        }
    }
    ///Create a sound node. WAV and MP3 are supported.
    pub fn create_sound(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        parent: &impl SpatialRefAspect,
        transform: Transform,
        resource: &stardust_xr::values::ResourceID,
    ) -> crate::node::NodeResult<Sound> {
        {
            let mut _fds = Vec::new();
            let data = (id, parent.node().id, transform, resource);
            let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
            _client
                .message_sender_handle
                .signal(10u64, 0u64, 3197851813257440734u64, &serialized_data, _fds)?;
            let (id, parent, transform, resource) = data;
            tracing::trace!(
                ? id, ? parent, ? transform, ? resource, "Sent signal to server, {}::{}",
                "Interface", "create_sound"
            );
        }
        Ok(Sound::from_id(_client, id, true))
    }
}
#[allow(unused_imports)]
use drawable::*;
pub mod drawable {
    #[allow(unused_imports)]
    use super::*;
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
    ///A collection of polylines drawn by the server. Meant for debug, so the spatial transform affects point positions but the thickness is in world space. Any mesh made to represent these lines cannot be distorted.
    #[derive(Debug, Clone)]
    pub struct Lines {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl Lines {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            Lines { core }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for Lines {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for Lines {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for Lines {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for Lines {}
    impl serde::Serialize for Lines {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialAspect for Lines {}
    impl OwnedAspect for Lines {}
    impl SpatialRefAspect for Lines {}
    impl LinesAspect for Lines {}
    #[derive(Debug)]
    pub enum LinesEvent {}
    ///A collection of polylines drawn by the server. Meant for debug, so the spatial transform affects point positions but the thickness is in world space. Any mesh made to represent these lines cannot be distorted.
    pub trait LinesAspect: crate::node::NodeType + super::SpatialAspect + super::OwnedAspect + super::SpatialRefAspect + std::fmt::Debug {
        ///Replace all polylines with the given lines
        fn set_lines(&self, lines: &[Line]) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (lines
                .iter()
                .map(|a| Ok(a))
                .collect::<crate::node::NodeResult<Vec<_>>>()?);
            self.node()
                .send_signal(
                    16705186951373789081u64,
                    17689001183742889136u64,
                    &data,
                    _fds,
                )?;
            let (lines) = data;
            tracing::trace!(
                ? lines, "Sent signal to server, {}::{}", "Lines", "set_lines"
            );
            Ok(())
        }
    }
    ///A GLTF model loaded by the server.
    #[derive(Debug, Clone)]
    pub struct Model {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl Model {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            Model { core }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for Model {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for Model {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for Model {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for Model {}
    impl serde::Serialize for Model {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialAspect for Model {}
    impl OwnedAspect for Model {}
    impl SpatialRefAspect for Model {}
    impl ModelAspect for Model {}
    #[derive(Debug)]
    pub enum ModelEvent {}
    ///A GLTF model loaded by the server.
    pub trait ModelAspect: crate::node::NodeType + super::SpatialAspect + super::OwnedAspect + super::SpatialRefAspect + std::fmt::Debug {
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
                    .send_signal(
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
            Ok(ModelPart::from_id(&self.node().client, id, true))
        }
    }
    ///A graphical node in the GLTF hierarchy for the given model. Can be reparented and have material parameters set on.
    #[derive(Debug, Clone)]
    pub struct ModelPart {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl ModelPart {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            ModelPart { core }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for ModelPart {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for ModelPart {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for ModelPart {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for ModelPart {}
    impl serde::Serialize for ModelPart {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialAspect for ModelPart {}
    impl OwnedAspect for ModelPart {}
    impl SpatialRefAspect for ModelPart {}
    impl ModelPartAspect for ModelPart {}
    #[derive(Debug)]
    pub enum ModelPartEvent {}
    ///A graphical node in the GLTF hierarchy for the given model. Can be reparented and have material parameters set on.
    pub trait ModelPartAspect: crate::node::NodeType + super::SpatialAspect + super::OwnedAspect + super::SpatialRefAspect + std::fmt::Debug {
        ///Set this model part's material to one that cuts a hole in the world. Often used for overlays/passthrough where you want to show the background through an object.
        fn apply_holdout_material(&self) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = ();
            self.node()
                .send_signal(
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
                .send_signal(
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
    ///Text rendered to work best in XR
    #[derive(Debug, Clone)]
    pub struct Text {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl Text {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            Text { core }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for Text {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for Text {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for Text {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for Text {}
    impl serde::Serialize for Text {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialAspect for Text {}
    impl OwnedAspect for Text {}
    impl SpatialRefAspect for Text {}
    impl TextAspect for Text {}
    #[derive(Debug)]
    pub enum TextEvent {}
    ///Text rendered to work best in XR
    pub trait TextAspect: crate::node::NodeType + super::SpatialAspect + super::OwnedAspect + super::SpatialRefAspect + std::fmt::Debug {
        ///Set the character height in meters
        fn set_character_height(&self, height: f32) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (height);
            self.node()
                .send_signal(
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
                .send_signal(
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
        tracing::trace!(
            ? tex, "Sent signal to server, {}::{}", "Interface", "set_sky_tex"
        );
        Ok(())
    }
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
    ///Create a lines node
    pub fn create_lines(
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
                parent.node().id,
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
    ///Load a GLTF model into a Model node
    pub fn load_model(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        parent: &impl SpatialRefAspect,
        transform: Transform,
        model: &stardust_xr::values::ResourceID,
    ) -> crate::node::NodeResult<Model> {
        {
            let mut _fds = Vec::new();
            let data = (id, parent.node().id, transform, model);
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
    ///Create a text node
    pub fn create_text(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        parent: &impl SpatialRefAspect,
        transform: Transform,
        text: &str,
        style: TextStyle,
    ) -> crate::node::NodeResult<Text> {
        {
            let mut _fds = Vec::new();
            let data = (id, parent.node().id, transform, text, style);
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
}
#[allow(unused_imports)]
use input::*;
pub mod input {
    #[allow(unused_imports)]
    use super::*;
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
    ///A fully articulated and tracked hand according to OpenXR spec for its coordinate system and joints.
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
    ///A 3D pointer, such as a gaze pointer for eye tracking or a mouse or a ray from a controller.
    #[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
    pub struct Pointer {
        pub origin: stardust_xr::values::Vector3<f32>,
        pub orientation: stardust_xr::values::Quaternion,
        pub deepest_point: stardust_xr::values::Vector3<f32>,
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
    ///Node representing a spatial input device
    #[derive(Debug, Clone)]
    pub struct InputMethodRef {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl InputMethodRef {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            InputMethodRef { core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for InputMethodRef {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for InputMethodRef {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for InputMethodRef {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for InputMethodRef {}
    impl serde::Serialize for InputMethodRef {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialRefAspect for InputMethodRef {}
    impl InputMethodRefAspect for InputMethodRef {}
    #[derive(Debug)]
    pub enum InputMethodRefEvent {}
    ///Node representing a spatial input device
    pub trait InputMethodRefAspect: crate::node::NodeType + super::SpatialRefAspect + std::fmt::Debug {
        ///Try to capture the input method with the given handler. When the handler does not get input from the method, it will be released.
        fn try_capture(
            &self,
            handler: &impl InputHandlerAspect,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (handler.node().id);
            self.node()
                .send_signal(
                    2611007814387963428u64,
                    12158986667525139020u64,
                    &data,
                    _fds,
                )?;
            let (handler) = data;
            tracing::trace!(
                ? handler, "Sent signal to server, {}::{}", "InputMethodRef",
                "try_capture"
            );
            Ok(())
        }
        ///If captured by this handler, release it (e.g. the object is let go of after grabbing).
        fn release(
            &self,
            handler: &impl InputHandlerAspect,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (handler.node().id);
            self.node()
                .send_signal(
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
    ///Node representing a spatial input device
    #[derive(Debug, Clone)]
    pub struct InputMethod {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
        pub(crate) input_method_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<InputMethodEvent>>,
        >,
    }
    impl InputMethod {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            let input_method_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 14883688361483968991u64).into(),
            );
            InputMethod {
                core,
                input_method_event,
            }
        }
        pub fn as_input_method_ref(self) -> super::InputMethodRef {
            super::InputMethodRef {
                core: self.core,
            }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
    }
    impl crate::node::NodeType for InputMethod {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for InputMethod {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for InputMethod {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for InputMethod {}
    impl serde::Serialize for InputMethod {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl InputMethodRefAspect for InputMethod {}
    impl SpatialRefAspect for InputMethod {}
    impl SpatialAspect for InputMethod {}
    impl OwnedAspect for InputMethod {}
    impl InputMethodAspect for InputMethod {
        fn recv_input_method_event(&self) -> Option<InputMethodEvent> {
            self.input_method_event.lock().unwrap().try_recv().ok()
        }
    }
    #[derive(Debug)]
    pub enum InputMethodEvent {
        CreateHandler { handler: InputHandler, field: Field },
        RequestCaptureHandler { id: u64 },
        ReleaseHandler { id: u64 },
        DestroyHandler { id: u64 },
    }
    impl crate::scenegraph::EventParser for InputMethodEvent {
        const ASPECT_ID: u64 = 14883688361483968991u64;
        fn parse_signal(
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
                        ? handler, ? field, "Got signal from server, {}::{}",
                        "InputMethod", "create_handler"
                    );
                    Ok(InputMethodEvent::CreateHandler {
                        handler: InputHandler::from_id(_client, handler, false),
                        field: Field::from_id(_client, field, false),
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
        fn parse_method(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            method_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
            response: stardust_xr::messenger::MethodResponse,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match method_id {
                _ => {
                    let _ = response
                        .send(
                            Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
                        );
                    Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
                }
            }
        }
    }
    ///Node representing a spatial input device
    pub trait InputMethodAspect: crate::node::NodeType + super::InputMethodRefAspect + super::SpatialRefAspect + super::SpatialAspect + super::OwnedAspect + std::fmt::Debug {
        fn recv_input_method_event(&self) -> Option<InputMethodEvent>;
        ///Set the spatial input component of this input method. You must keep the same input data type throughout the entire thing.
        fn set_input(&self, input: InputDataType) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (input);
            self.node()
                .send_signal(
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
                .send_signal(
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
                .map(|a| Ok(a.node().id))
                .collect::<crate::node::NodeResult<Vec<_>>>()?);
            self.node()
                .send_signal(
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
        fn set_captures(
            &self,
            handlers: &[InputHandler],
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (handlers
                .iter()
                .map(|a| Ok(a.node().id))
                .collect::<crate::node::NodeResult<Vec<_>>>()?);
            self.node()
                .send_signal(
                    14883688361483968991u64,
                    4141712352465076448u64,
                    &data,
                    _fds,
                )?;
            let (handlers) = data;
            tracing::trace!(
                ? handlers, "Sent signal to server, {}::{}", "InputMethod",
                "set_captures"
            );
            Ok(())
        }
    }
    ///Receives input from input methods.
    #[derive(Debug, Clone)]
    pub struct InputHandler {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
        pub(crate) input_handler_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<InputHandlerEvent>>,
        >,
    }
    impl InputHandler {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            let input_handler_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 537028132086008694u64).into(),
            );
            InputHandler {
                core,
                input_handler_event,
            }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for InputHandler {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for InputHandler {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for InputHandler {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for InputHandler {}
    impl serde::Serialize for InputHandler {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialAspect for InputHandler {}
    impl OwnedAspect for InputHandler {}
    impl SpatialRefAspect for InputHandler {}
    impl InputHandlerAspect for InputHandler {
        fn recv_input_handler_event(&self) -> Option<InputHandlerEvent> {
            self.input_handler_event.lock().unwrap().try_recv().ok()
        }
    }
    #[derive(Debug)]
    pub enum InputHandlerEvent {
        Input { methods: Vec<InputMethodRef>, data: Vec<InputData> },
    }
    impl crate::scenegraph::EventParser for InputHandlerEvent {
        const ASPECT_ID: u64 = 537028132086008694u64;
        fn parse_signal(
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
                        ? methods, ? data, "Got signal from server, {}::{}",
                        "InputHandler", "input"
                    );
                    Ok(InputHandlerEvent::Input {
                        methods: methods
                            .into_iter()
                            .map(|a| Ok(InputMethodRef::from_id(_client, a, false)))
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
        fn parse_method(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            method_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
            response: stardust_xr::messenger::MethodResponse,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match method_id {
                _ => {
                    let _ = response
                        .send(
                            Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
                        );
                    Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
                }
            }
        }
    }
    ///Receives input from input methods.
    pub trait InputHandlerAspect: crate::node::NodeType + super::SpatialAspect + super::OwnedAspect + super::SpatialRefAspect + std::fmt::Debug {
        fn recv_input_handler_event(&self) -> Option<InputHandlerEvent>;
    }
    ///Create an input method node
    pub fn create_input_method(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        parent: &impl SpatialRefAspect,
        transform: Transform,
        initial_data: InputDataType,
        datamap: &stardust_xr::values::Datamap,
    ) -> crate::node::NodeResult<InputMethod> {
        {
            let mut _fds = Vec::new();
            let data = (id, parent.node().id, transform, initial_data, datamap);
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
    ///Create an input handler node
    pub fn create_input_handler(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        parent: &impl SpatialRefAspect,
        transform: Transform,
        field: &impl FieldAspect,
    ) -> crate::node::NodeResult<InputHandler> {
        {
            let mut _fds = Vec::new();
            let data = (id, parent.node().id, transform, field.node().id);
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
}
#[allow(unused_imports)]
use item::*;
pub mod item {
    #[allow(unused_imports)]
    use super::*;
    pub(crate) const INTERFACE_VERSION: u32 = 1u32;
    pub(crate) const INTERFACE_NODE_ID: u64 = 10u64;
    ///
    #[derive(Debug, Clone)]
    pub struct Item {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl Item {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            Item { core }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for Item {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for Item {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for Item {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for Item {}
    impl serde::Serialize for Item {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialAspect for Item {}
    impl OwnedAspect for Item {}
    impl SpatialRefAspect for Item {}
    impl ItemAspect for Item {}
    #[derive(Debug)]
    pub enum ItemEvent {}
    ///
    pub trait ItemAspect: crate::node::NodeType + super::SpatialAspect + super::OwnedAspect + super::SpatialRefAspect + std::fmt::Debug {
        ///
        fn release(&self) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = ();
            self.node()
                .send_signal(
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
    ///
    #[derive(Debug, Clone)]
    pub struct ItemAcceptor {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
        pub(crate) item_acceptor_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<ItemAcceptorEvent>>,
        >,
    }
    impl ItemAcceptor {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            let item_acceptor_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 10274055739447304636u64).into(),
            );
            ItemAcceptor {
                core,
                item_acceptor_event,
            }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
    }
    impl crate::node::NodeType for ItemAcceptor {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for ItemAcceptor {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for ItemAcceptor {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for ItemAcceptor {}
    impl serde::Serialize for ItemAcceptor {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl SpatialAspect for ItemAcceptor {}
    impl OwnedAspect for ItemAcceptor {}
    impl SpatialRefAspect for ItemAcceptor {}
    impl ItemAcceptorAspect for ItemAcceptor {
        fn recv_item_acceptor_event(&self) -> Option<ItemAcceptorEvent> {
            self.item_acceptor_event.lock().unwrap().try_recv().ok()
        }
    }
    #[derive(Debug)]
    pub enum ItemAcceptorEvent {
        ReleaseItem { item_id: u64 },
    }
    impl crate::scenegraph::EventParser for ItemAcceptorEvent {
        const ASPECT_ID: u64 = 10274055739447304636u64;
        fn parse_signal(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            signal_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match signal_id {
                14821884892980204849u64 => {
                    let (item_id): (u64) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
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
        fn parse_method(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            method_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
            response: stardust_xr::messenger::MethodResponse,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match method_id {
                _ => {
                    let _ = response
                        .send(
                            Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
                        );
                    Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
                }
            }
        }
    }
    ///
    pub trait ItemAcceptorAspect: crate::node::NodeType + super::SpatialAspect + super::OwnedAspect + super::SpatialRefAspect + std::fmt::Debug {
        fn recv_item_acceptor_event(&self) -> Option<ItemAcceptorEvent>;
    }
    ///
    #[derive(Debug, Clone)]
    pub struct ItemUi {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
        pub(crate) item_ui_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<ItemUiEvent>>,
        >,
    }
    impl ItemUi {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            let item_ui_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 7265392688253796589u64).into(),
            );
            ItemUi { core, item_ui_event }
        }
    }
    impl crate::node::NodeType for ItemUi {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for ItemUi {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for ItemUi {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for ItemUi {}
    impl serde::Serialize for ItemUi {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl ItemUiAspect for ItemUi {
        fn recv_item_ui_event(&self) -> Option<ItemUiEvent> {
            self.item_ui_event.lock().unwrap().try_recv().ok()
        }
    }
    #[derive(Debug)]
    pub enum ItemUiEvent {
        CaptureItem { item_id: u64, acceptor_id: u64 },
        ReleaseItem { item_id: u64, acceptor_id: u64 },
        DestroyItem { id: u64 },
        DestroyAcceptor { id: u64 },
    }
    impl crate::scenegraph::EventParser for ItemUiEvent {
        const ASPECT_ID: u64 = 7265392688253796589u64;
        fn parse_signal(
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
                        ? item_id, ? acceptor_id, "Got signal from server, {}::{}",
                        "ItemUi", "capture_item"
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
                        ? item_id, ? acceptor_id, "Got signal from server, {}::{}",
                        "ItemUi", "release_item"
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
                        ? id, "Got signal from server, {}::{}", "ItemUi",
                        "destroy_acceptor"
                    );
                    Ok(ItemUiEvent::DestroyAcceptor {
                        id: id,
                    })
                }
                _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
            }
        }
        fn parse_method(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            method_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
            response: stardust_xr::messenger::MethodResponse,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match method_id {
                _ => {
                    let _ = response
                        .send(
                            Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
                        );
                    Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
                }
            }
        }
    }
    ///
    pub trait ItemUiAspect: crate::node::NodeType + std::fmt::Debug {
        fn recv_item_ui_event(&self) -> Option<ItemUiEvent>;
    }
}
#[allow(unused_imports)]
use item_camera::*;
pub mod item_camera {
    #[allow(unused_imports)]
    use super::*;
    pub(crate) const INTERFACE_VERSION: u32 = 1u32;
    pub(crate) const INTERFACE_NODE_ID: u64 = 11u64;
    ///
    #[derive(Debug, Clone)]
    pub struct CameraItem {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
    }
    impl CameraItem {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            CameraItem { core }
        }
        pub fn as_item(self) -> super::Item {
            super::Item { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
    }
    impl crate::node::NodeType for CameraItem {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for CameraItem {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for CameraItem {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for CameraItem {}
    impl serde::Serialize for CameraItem {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl ItemAspect for CameraItem {}
    impl SpatialRefAspect for CameraItem {}
    impl OwnedAspect for CameraItem {}
    impl SpatialAspect for CameraItem {}
    impl CameraItemAspect for CameraItem {}
    #[derive(Debug)]
    pub enum CameraItemEvent {}
    ///
    pub trait CameraItemAspect: crate::node::NodeType + super::ItemAspect + super::SpatialRefAspect + super::OwnedAspect + super::SpatialAspect + std::fmt::Debug {}
    ///
    #[derive(Debug, Clone)]
    pub struct CameraItemUi {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
        pub(crate) camera_item_ui_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<CameraItemUiEvent>>,
        >,
    }
    impl CameraItemUi {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            let camera_item_ui_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 708021061010127172u64).into(),
            );
            CameraItemUi {
                core,
                camera_item_ui_event,
            }
        }
    }
    impl crate::node::NodeType for CameraItemUi {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for CameraItemUi {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for CameraItemUi {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for CameraItemUi {}
    impl serde::Serialize for CameraItemUi {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl CameraItemUiAspect for CameraItemUi {
        fn recv_camera_item_ui_event(&self) -> Option<CameraItemUiEvent> {
            self.camera_item_ui_event.lock().unwrap().try_recv().ok()
        }
    }
    #[derive(Debug)]
    pub enum CameraItemUiEvent {
        CreateItem { item: CameraItem },
        CreateAcceptor { acceptor: CameraItemAcceptor, acceptor_field: Field },
    }
    impl crate::scenegraph::EventParser for CameraItemUiEvent {
        const ASPECT_ID: u64 = 708021061010127172u64;
        fn parse_signal(
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
                        item: CameraItem::from_id(_client, item, false),
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
                        acceptor: CameraItemAcceptor::from_id(_client, acceptor, false),
                        acceptor_field: Field::from_id(_client, acceptor_field, false),
                    })
                }
                _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
            }
        }
        fn parse_method(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            method_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
            response: stardust_xr::messenger::MethodResponse,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match method_id {
                _ => {
                    let _ = response
                        .send(
                            Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
                        );
                    Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
                }
            }
        }
    }
    ///
    pub trait CameraItemUiAspect: crate::node::NodeType + std::fmt::Debug {
        fn recv_camera_item_ui_event(&self) -> Option<CameraItemUiEvent>;
    }
    ///
    #[derive(Debug, Clone)]
    pub struct CameraItemAcceptor {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
        pub(crate) item_acceptor_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<ItemAcceptorEvent>>,
        >,
        pub(crate) camera_item_acceptor_event: std::sync::Arc<
            std::sync::Mutex<
                tokio::sync::mpsc::UnboundedReceiver<CameraItemAcceptorEvent>,
            >,
        >,
    }
    impl CameraItemAcceptor {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            let item_acceptor_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 10274055739447304636u64).into(),
            );
            let camera_item_acceptor_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 5036088114779304421u64).into(),
            );
            CameraItemAcceptor {
                core,
                item_acceptor_event,
                camera_item_acceptor_event,
            }
        }
        pub fn as_item_acceptor(self) -> super::ItemAcceptor {
            super::ItemAcceptor {
                core: self.core,
                item_acceptor_event: self.item_acceptor_event,
            }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
    }
    impl crate::node::NodeType for CameraItemAcceptor {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for CameraItemAcceptor {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for CameraItemAcceptor {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for CameraItemAcceptor {}
    impl serde::Serialize for CameraItemAcceptor {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl ItemAcceptorAspect for CameraItemAcceptor {
        fn recv_item_acceptor_event(&self) -> Option<ItemAcceptorEvent> {
            self.item_acceptor_event.lock().unwrap().try_recv().ok()
        }
    }
    impl SpatialRefAspect for CameraItemAcceptor {}
    impl OwnedAspect for CameraItemAcceptor {}
    impl SpatialAspect for CameraItemAcceptor {}
    impl CameraItemAcceptorAspect for CameraItemAcceptor {
        fn recv_camera_item_acceptor_event(&self) -> Option<CameraItemAcceptorEvent> {
            self.camera_item_acceptor_event.lock().unwrap().try_recv().ok()
        }
    }
    #[derive(Debug)]
    pub enum CameraItemAcceptorEvent {
        CaptureItem { item: CameraItem },
    }
    impl crate::scenegraph::EventParser for CameraItemAcceptorEvent {
        const ASPECT_ID: u64 = 5036088114779304421u64;
        fn parse_signal(
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
                        item: CameraItem::from_id(_client, item, false),
                    })
                }
                _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
            }
        }
        fn parse_method(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            method_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
            response: stardust_xr::messenger::MethodResponse,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match method_id {
                _ => {
                    let _ = response
                        .send(
                            Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
                        );
                    Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
                }
            }
        }
    }
    ///
    pub trait CameraItemAcceptorAspect: crate::node::NodeType + super::ItemAcceptorAspect + super::SpatialRefAspect + super::OwnedAspect + super::SpatialAspect + std::fmt::Debug {
        fn recv_camera_item_acceptor_event(&self) -> Option<CameraItemAcceptorEvent>;
        ///
        fn capture_item(
            &self,
            item: &impl CameraItemAspect,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (item.node().id);
            self.node()
                .send_signal(
                    5036088114779304421u64,
                    1751367302976798762u64,
                    &data,
                    _fds,
                )?;
            let (item) = data;
            tracing::trace!(
                ? item, "Sent signal to server, {}::{}", "CameraItemAcceptor",
                "capture_item"
            );
            Ok(())
        }
    }
    ///Create a camera item at a specific location
    pub fn create_camera_item(
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
                parent.node().id,
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
    ///Create an item acceptor to allow temporary ownership of a given type of item. Creates a node at `/item/camera/acceptor/<name>`.
    pub fn create_camera_item_acceptor(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        parent: &impl SpatialRefAspect,
        transform: Transform,
        field: &impl FieldAspect,
    ) -> crate::node::NodeResult<CameraItemAcceptor> {
        {
            let mut _fds = Vec::new();
            let data = (id, parent.node().id, transform, field.node().id);
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
}
#[allow(unused_imports)]
use item_panel::*;
pub mod item_panel {
    #[allow(unused_imports)]
    use super::*;
    pub(crate) const INTERFACE_VERSION: u32 = 1u32;
    pub(crate) const INTERFACE_NODE_ID: u64 = 12u64;
    ///
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    #[serde(tag = "t", content = "c")]
    pub enum SurfaceId {
        Toplevel(()),
        Child(u64),
    }
    ///The origin and size of the surface's "solid" part.
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Geometry {
        pub origin: stardust_xr::values::Vector2<i32>,
        pub size: stardust_xr::values::Vector2<u32>,
    }
    ///The state of the panel item's toplevel.
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct ToplevelInfo {
        pub parent: Option<u64>,
        pub title: Option<String>,
        pub app_id: Option<String>,
        pub size: stardust_xr::values::Vector2<u32>,
        pub min_size: Option<stardust_xr::values::Vector2<f32>>,
        pub max_size: Option<stardust_xr::values::Vector2<f32>>,
        pub logical_rectangle: Geometry,
    }
    ///Data on positioning a child.
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct ChildInfo {
        pub id: u64,
        pub parent: SurfaceId,
        pub geometry: Geometry,
        pub z_order: i32,
        pub receives_input: bool,
    }
    ///The init data for the panel item.
    #[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct PanelItemInitData {
        pub cursor: Option<Geometry>,
        pub toplevel: ToplevelInfo,
        pub children: Vec<ChildInfo>,
        pub pointer_grab: Option<SurfaceId>,
        pub keyboard_grab: Option<SurfaceId>,
    }
    ///An item that represents a toplevel 2D window's surface (base window) and all its children (context menus, modals, etc.).
    #[derive(Debug, Clone)]
    pub struct PanelItem {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
        pub(crate) panel_item_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<PanelItemEvent>>,
        >,
    }
    impl PanelItem {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            let panel_item_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 16007573185838633179u64).into(),
            );
            PanelItem {
                core,
                panel_item_event,
            }
        }
        pub fn as_item(self) -> super::Item {
            super::Item { core: self.core }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
    }
    impl crate::node::NodeType for PanelItem {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for PanelItem {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for PanelItem {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for PanelItem {}
    impl serde::Serialize for PanelItem {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl ItemAspect for PanelItem {}
    impl SpatialRefAspect for PanelItem {}
    impl OwnedAspect for PanelItem {}
    impl SpatialAspect for PanelItem {}
    impl PanelItemAspect for PanelItem {
        fn recv_panel_item_event(&self) -> Option<PanelItemEvent> {
            self.panel_item_event.lock().unwrap().try_recv().ok()
        }
    }
    #[derive(Debug)]
    pub enum PanelItemEvent {
        ToplevelParentChanged { parent_id: u64 },
        ToplevelTitleChanged { title: String },
        ToplevelAppIdChanged { app_id: String },
        ToplevelFullscreenActive { active: bool },
        ToplevelMoveRequest {},
        ToplevelResizeRequest { up: bool, down: bool, left: bool, right: bool },
        ToplevelSizeChanged { size: stardust_xr::values::Vector2<u32> },
        SetCursor { geometry: Geometry },
        HideCursor {},
        CreateChild { uid: u64, info: ChildInfo },
        RepositionChild { uid: u64, geometry: Geometry },
        DestroyChild { uid: u64 },
    }
    impl crate::scenegraph::EventParser for PanelItemEvent {
        const ASPECT_ID: u64 = 16007573185838633179u64;
        fn parse_signal(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            signal_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match signal_id {
                1408884359956576105u64 => {
                    let (parent_id): (u64) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? parent_id, "Got signal from server, {}::{}", "PanelItem",
                        "toplevel_parent_changed"
                    );
                    Ok(PanelItemEvent::ToplevelParentChanged {
                        parent_id: parent_id,
                    })
                }
                566483566315648641u64 => {
                    let (title): (String) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? title, "Got signal from server, {}::{}", "PanelItem",
                        "toplevel_title_changed"
                    );
                    Ok(PanelItemEvent::ToplevelTitleChanged {
                        title: title,
                    })
                }
                8706869778156655494u64 => {
                    let (app_id): (String) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? app_id, "Got signal from server, {}::{}", "PanelItem",
                        "toplevel_app_id_changed"
                    );
                    Ok(PanelItemEvent::ToplevelAppIdChanged {
                        app_id: app_id,
                    })
                }
                11059551561818960198u64 => {
                    let (active): (bool) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? active, "Got signal from server, {}::{}", "PanelItem",
                        "toplevel_fullscreen_active"
                    );
                    Ok(PanelItemEvent::ToplevelFullscreenActive {
                        active: active,
                    })
                }
                3715781852227007625u64 => {
                    let (): () = stardust_xr::schemas::flex::deserialize(_data)?;
                    tracing::trace!(
                        "Got signal from server, {}::{}", "PanelItem",
                        "toplevel_move_request"
                    );
                    Ok(PanelItemEvent::ToplevelMoveRequest {
                    })
                }
                4540754955116125050u64 => {
                    let (up, down, left, right): (bool, bool, bool, bool) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? up, ? down, ? left, ? right, "Got signal from server, {}::{}",
                        "PanelItem", "toplevel_resize_request"
                    );
                    Ok(PanelItemEvent::ToplevelResizeRequest {
                        up: up,
                        down: down,
                        left: left,
                        right: right,
                    })
                }
                3665525014775618530u64 => {
                    let (size): (stardust_xr::values::Vector2<u32>) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? size, "Got signal from server, {}::{}", "PanelItem",
                        "toplevel_size_changed"
                    );
                    Ok(PanelItemEvent::ToplevelSizeChanged {
                        size: size,
                    })
                }
                6092877811616586203u64 => {
                    let (geometry): (Geometry) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? geometry, "Got signal from server, {}::{}", "PanelItem",
                        "set_cursor"
                    );
                    Ok(PanelItemEvent::SetCursor {
                        geometry: geometry,
                    })
                }
                12365625385177885025u64 => {
                    let (): () = stardust_xr::schemas::flex::deserialize(_data)?;
                    tracing::trace!(
                        "Got signal from server, {}::{}", "PanelItem", "hide_cursor"
                    );
                    Ok(PanelItemEvent::HideCursor {})
                }
                13878060402106144481u64 => {
                    let (uid, info): (u64, ChildInfo) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? uid, ? info, "Got signal from server, {}::{}", "PanelItem",
                        "create_child"
                    );
                    Ok(PanelItemEvent::CreateChild {
                        uid: uid,
                        info: info,
                    })
                }
                4614990113965355127u64 => {
                    let (uid, geometry): (u64, Geometry) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? uid, ? geometry, "Got signal from server, {}::{}", "PanelItem",
                        "reposition_child"
                    );
                    Ok(PanelItemEvent::RepositionChild {
                        uid: uid,
                        geometry: geometry,
                    })
                }
                7048616010698587017u64 => {
                    let (uid): (u64) = stardust_xr::schemas::flex::deserialize(_data)?;
                    tracing::trace!(
                        ? uid, "Got signal from server, {}::{}", "PanelItem",
                        "destroy_child"
                    );
                    Ok(PanelItemEvent::DestroyChild {
                        uid: uid,
                    })
                }
                _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
            }
        }
        fn parse_method(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            method_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
            response: stardust_xr::messenger::MethodResponse,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match method_id {
                _ => {
                    let _ = response
                        .send(
                            Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
                        );
                    Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
                }
            }
        }
    }
    ///An item that represents a toplevel 2D window's surface (base window) and all its children (context menus, modals, etc.).
    pub trait PanelItemAspect: crate::node::NodeType + super::ItemAspect + super::SpatialRefAspect + super::OwnedAspect + super::SpatialAspect + std::fmt::Debug {
        fn recv_panel_item_event(&self) -> Option<PanelItemEvent>;
        ///Apply the cursor as a material to a model.
        fn apply_cursor_material(
            &self,
            model_part: &impl ModelPartAspect,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (model_part.node().id);
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    12984352657777750687u64,
                    &data,
                    _fds,
                )?;
            let (model_part) = data;
            tracing::trace!(
                ? model_part, "Sent signal to server, {}::{}", "PanelItem",
                "apply_cursor_material"
            );
            Ok(())
        }
        ///Apply a surface's visuals as a material to a model.
        fn apply_surface_material(
            &self,
            surface: SurfaceId,
            model_part: &impl ModelPartAspect,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (surface, model_part.node().id);
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    5538717944649978650u64,
                    &data,
                    _fds,
                )?;
            let (surface, model_part) = data;
            tracing::trace!(
                ? surface, ? model_part, "Sent signal to server, {}::{}", "PanelItem",
                "apply_surface_material"
            );
            Ok(())
        }
        /**Try to close the toplevel.

        The panel item UI handler or panel item acceptor will drop the panel item if this succeeds.*/
        fn close_toplevel(&self) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = ();
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    11149391162473273576u64,
                    &data,
                    _fds,
                )?;
            let () = data;
            tracing::trace!(
                "Sent signal to server, {}::{}", "PanelItem", "close_toplevel"
            );
            Ok(())
        }
        ///Request a resize of the surface to whatever size the 2D app wants.
        fn auto_size_toplevel(&self) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = ();
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    7177229187692151305u64,
                    &data,
                    _fds,
                )?;
            let () = data;
            tracing::trace!(
                "Sent signal to server, {}::{}", "PanelItem", "auto_size_toplevel"
            );
            Ok(())
        }
        ///Request a resize of the surface (in pixels).
        fn set_toplevel_size(
            &self,
            size: impl Into<stardust_xr::values::Vector2<u32>>,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (size.into());
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    8102855835344875634u64,
                    &data,
                    _fds,
                )?;
            let (size) = data;
            tracing::trace!(
                ? size, "Sent signal to server, {}::{}", "PanelItem", "set_toplevel_size"
            );
            Ok(())
        }
        ///Tell the toplevel to appear focused visually if true, or unfocused if false.
        fn set_toplevel_focused_visuals(
            &self,
            focused: bool,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (focused);
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    3934600665134956080u64,
                    &data,
                    _fds,
                )?;
            let (focused) = data;
            tracing::trace!(
                ? focused, "Sent signal to server, {}::{}", "PanelItem",
                "set_toplevel_focused_visuals"
            );
            Ok(())
        }
        ///Send an event to set the pointer's position (in pixels, relative to top-left of surface). This will activate the pointer.
        fn pointer_motion(
            &self,
            surface: SurfaceId,
            position: impl Into<stardust_xr::values::Vector2<f32>>,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (surface, position.into());
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    651662101921814334u64,
                    &data,
                    _fds,
                )?;
            let (surface, position) = data;
            tracing::trace!(
                ? surface, ? position, "Sent signal to server, {}::{}", "PanelItem",
                "pointer_motion"
            );
            Ok(())
        }
        ///Send an event to set a pointer button's state if the pointer's active. The `button` is from the `input_event_codes` crate (e.g. BTN_LEFT for left click).
        fn pointer_button(
            &self,
            surface: SurfaceId,
            button: u32,
            pressed: bool,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (surface, button, pressed);
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    1617963334017359776u64,
                    &data,
                    _fds,
                )?;
            let (surface, button, pressed) = data;
            tracing::trace!(
                ? surface, ? button, ? pressed, "Sent signal to server, {}::{}",
                "PanelItem", "pointer_button"
            );
            Ok(())
        }
        /**Send an event to scroll the pointer if it's active.
Scroll distance is a value in pixels corresponding to the `distance` the surface should be scrolled.
Scroll steps is a value in columns/rows corresponding to the wheel clicks of a mouse or such. This also supports fractions of a wheel click.*/
        fn pointer_scroll(
            &self,
            surface: SurfaceId,
            scroll_distance: impl Into<stardust_xr::values::Vector2<f32>>,
            scroll_steps: impl Into<stardust_xr::values::Vector2<f32>>,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (surface, scroll_distance.into(), scroll_steps.into());
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    18077910517219850499u64,
                    &data,
                    _fds,
                )?;
            let (surface, scroll_distance, scroll_steps) = data;
            tracing::trace!(
                ? surface, ? scroll_distance, ? scroll_steps,
                "Sent signal to server, {}::{}", "PanelItem", "pointer_scroll"
            );
            Ok(())
        }
        ///Send an event to stop scrolling the pointer.
        fn pointer_stop_scroll(
            &self,
            surface: SurfaceId,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (surface);
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    13177724628894942354u64,
                    &data,
                    _fds,
                )?;
            let (surface) = data;
            tracing::trace!(
                ? surface, "Sent signal to server, {}::{}", "PanelItem",
                "pointer_stop_scroll"
            );
            Ok(())
        }
        ///Send a key press or release event with the given keymap ID.
        fn keyboard_key(
            &self,
            surface: SurfaceId,
            keymap_id: u64,
            key: u32,
            pressed: bool,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (surface, keymap_id, key, pressed);
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    18230480350930328965u64,
                    &data,
                    _fds,
                )?;
            let (surface, keymap_id, key, pressed) = data;
            tracing::trace!(
                ? surface, ? keymap_id, ? key, ? pressed,
                "Sent signal to server, {}::{}", "PanelItem", "keyboard_key"
            );
            Ok(())
        }
        ///Put a touch down on this surface with the unique ID `uid` at `position` (in pixels) from top left corner of the surface.
        fn touch_down(
            &self,
            surface: SurfaceId,
            uid: u32,
            position: impl Into<stardust_xr::values::Vector2<f32>>,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (surface, uid, position.into());
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    10543081656468919422u64,
                    &data,
                    _fds,
                )?;
            let (surface, uid, position) = data;
            tracing::trace!(
                ? surface, ? uid, ? position, "Sent signal to server, {}::{}",
                "PanelItem", "touch_down"
            );
            Ok(())
        }
        ///Move an existing touch point.
        fn touch_move(
            &self,
            uid: u32,
            position: impl Into<stardust_xr::values::Vector2<f32>>,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (uid, position.into());
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    15126475688563381777u64,
                    &data,
                    _fds,
                )?;
            let (uid, position) = data;
            tracing::trace!(
                ? uid, ? position, "Sent signal to server, {}::{}", "PanelItem",
                "touch_move"
            );
            Ok(())
        }
        ///Release a touch from its surface.
        fn touch_up(&self, uid: u32) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (uid);
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    6589027081119653997u64,
                    &data,
                    _fds,
                )?;
            let (uid) = data;
            tracing::trace!(
                ? uid, "Sent signal to server, {}::{}", "PanelItem", "touch_up"
            );
            Ok(())
        }
        ///Reset all input, such as pressed keys and pointer clicks and touches. Useful for when it's newly captured into an item acceptor to make sure no input gets stuck.
        fn reset_input(&self) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = ();
            self.node()
                .send_signal(
                    16007573185838633179u64,
                    14629122800709746500u64,
                    &data,
                    _fds,
                )?;
            let () = data;
            tracing::trace!("Sent signal to server, {}::{}", "PanelItem", "reset_input");
            Ok(())
        }
    }
    ///
    #[derive(Debug, Clone)]
    pub struct PanelItemUi {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
        pub(crate) item_ui_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<ItemUiEvent>>,
        >,
        pub(crate) panel_item_ui_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<PanelItemUiEvent>>,
        >,
    }
    impl PanelItemUi {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            let item_ui_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 7265392688253796589u64).into(),
            );
            let panel_item_ui_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 11713374794499719853u64).into(),
            );
            PanelItemUi {
                core,
                item_ui_event,
                panel_item_ui_event,
            }
        }
        pub fn as_item_ui(self) -> super::ItemUi {
            super::ItemUi {
                core: self.core,
                item_ui_event: self.item_ui_event,
            }
        }
    }
    impl crate::node::NodeType for PanelItemUi {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for PanelItemUi {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for PanelItemUi {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for PanelItemUi {}
    impl serde::Serialize for PanelItemUi {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl ItemUiAspect for PanelItemUi {
        fn recv_item_ui_event(&self) -> Option<ItemUiEvent> {
            self.item_ui_event.lock().unwrap().try_recv().ok()
        }
    }
    impl PanelItemUiAspect for PanelItemUi {
        fn recv_panel_item_ui_event(&self) -> Option<PanelItemUiEvent> {
            self.panel_item_ui_event.lock().unwrap().try_recv().ok()
        }
    }
    #[derive(Debug)]
    pub enum PanelItemUiEvent {
        CreateItem { item: PanelItem, initial_data: PanelItemInitData },
        CreateAcceptor { acceptor: PanelItemAcceptor, acceptor_field: Field },
    }
    impl crate::scenegraph::EventParser for PanelItemUiEvent {
        const ASPECT_ID: u64 = 11713374794499719853u64;
        fn parse_signal(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            signal_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match signal_id {
                15524466827491111758u64 => {
                    let (item, initial_data): (u64, PanelItemInitData) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? item, ? initial_data, "Got signal from server, {}::{}",
                        "PanelItemUi", "create_item"
                    );
                    Ok(PanelItemUiEvent::CreateItem {
                        item: PanelItem::from_id(_client, item, false),
                        initial_data: initial_data,
                    })
                }
                16628549773568263004u64 => {
                    let (acceptor, acceptor_field): (u64, u64) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? acceptor, ? acceptor_field, "Got signal from server, {}::{}",
                        "PanelItemUi", "create_acceptor"
                    );
                    Ok(PanelItemUiEvent::CreateAcceptor {
                        acceptor: PanelItemAcceptor::from_id(_client, acceptor, false),
                        acceptor_field: Field::from_id(_client, acceptor_field, false),
                    })
                }
                _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
            }
        }
        fn parse_method(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            method_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
            response: stardust_xr::messenger::MethodResponse,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match method_id {
                _ => {
                    let _ = response
                        .send(
                            Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
                        );
                    Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
                }
            }
        }
    }
    ///
    pub trait PanelItemUiAspect: crate::node::NodeType + super::ItemUiAspect + std::fmt::Debug {
        fn recv_panel_item_ui_event(&self) -> Option<PanelItemUiEvent>;
    }
    ///
    #[derive(Debug, Clone)]
    pub struct PanelItemAcceptor {
        pub(crate) core: std::sync::Arc<crate::node::NodeCore>,
        pub(crate) item_acceptor_event: std::sync::Arc<
            std::sync::Mutex<tokio::sync::mpsc::UnboundedReceiver<ItemAcceptorEvent>>,
        >,
        pub(crate) panel_item_acceptor_event: std::sync::Arc<
            std::sync::Mutex<
                tokio::sync::mpsc::UnboundedReceiver<PanelItemAcceptorEvent>,
            >,
        >,
    }
    impl PanelItemAcceptor {
        pub(crate) fn from_id(
            client: &std::sync::Arc<crate::client::ClientHandle>,
            id: u64,
            owned: bool,
        ) -> Self {
            let core = std::sync::Arc::new(
                crate::node::NodeCore::new(client.clone(), id, owned),
            );
            let item_acceptor_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 10274055739447304636u64).into(),
            );
            let panel_item_acceptor_event = std::sync::Arc::new(
                client.registry.add_aspect(id, 6398932320740499836u64).into(),
            );
            PanelItemAcceptor {
                core,
                item_acceptor_event,
                panel_item_acceptor_event,
            }
        }
        pub fn as_item_acceptor(self) -> super::ItemAcceptor {
            super::ItemAcceptor {
                core: self.core,
                item_acceptor_event: self.item_acceptor_event,
            }
        }
        pub fn as_spatial_ref(self) -> super::SpatialRef {
            super::SpatialRef {
                core: self.core,
            }
        }
        pub fn as_spatial(self) -> super::Spatial {
            super::Spatial { core: self.core }
        }
    }
    impl crate::node::NodeType for PanelItemAcceptor {
        fn node(&self) -> &crate::node::NodeCore {
            &self.core
        }
    }
    impl std::hash::Hash for PanelItemAcceptor {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.core.id.hash(state);
        }
    }
    impl std::cmp::PartialEq for PanelItemAcceptor {
        fn eq(&self, other: &Self) -> bool {
            self.core.id == other.core.id
        }
    }
    impl std::cmp::Eq for PanelItemAcceptor {}
    impl serde::Serialize for PanelItemAcceptor {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> Result<S::Ok, S::Error> {
            serializer.serialize_u64(self.core.id)
        }
    }
    impl ItemAcceptorAspect for PanelItemAcceptor {
        fn recv_item_acceptor_event(&self) -> Option<ItemAcceptorEvent> {
            self.item_acceptor_event.lock().unwrap().try_recv().ok()
        }
    }
    impl SpatialRefAspect for PanelItemAcceptor {}
    impl OwnedAspect for PanelItemAcceptor {}
    impl SpatialAspect for PanelItemAcceptor {}
    impl PanelItemAcceptorAspect for PanelItemAcceptor {
        fn recv_panel_item_acceptor_event(&self) -> Option<PanelItemAcceptorEvent> {
            self.panel_item_acceptor_event.lock().unwrap().try_recv().ok()
        }
    }
    #[derive(Debug)]
    pub enum PanelItemAcceptorEvent {
        CaptureItem { item: PanelItem, initial_data: PanelItemInitData },
    }
    impl crate::scenegraph::EventParser for PanelItemAcceptorEvent {
        const ASPECT_ID: u64 = 6398932320740499836u64;
        fn parse_signal(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            signal_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match signal_id {
                1751367302976798762u64 => {
                    let (item, initial_data): (u64, PanelItemInitData) = stardust_xr::schemas::flex::deserialize(
                        _data,
                    )?;
                    tracing::trace!(
                        ? item, ? initial_data, "Got signal from server, {}::{}",
                        "PanelItemAcceptor", "capture_item"
                    );
                    Ok(PanelItemAcceptorEvent::CaptureItem {
                        item: PanelItem::from_id(_client, item, false),
                        initial_data: initial_data,
                    })
                }
                _ => Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
            }
        }
        fn parse_method(
            _client: &std::sync::Arc<crate::client::ClientHandle>,
            method_id: u64,
            _data: &[u8],
            _fds: Vec<std::os::fd::OwnedFd>,
            response: stardust_xr::messenger::MethodResponse,
        ) -> Result<Self, stardust_xr::scenegraph::ScenegraphError> {
            match method_id {
                _ => {
                    let _ = response
                        .send(
                            Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound),
                        );
                    Err(stardust_xr::scenegraph::ScenegraphError::MemberNotFound)
                }
            }
        }
    }
    ///
    pub trait PanelItemAcceptorAspect: crate::node::NodeType + super::ItemAcceptorAspect + super::SpatialRefAspect + super::OwnedAspect + super::SpatialAspect + std::fmt::Debug {
        fn recv_panel_item_acceptor_event(&self) -> Option<PanelItemAcceptorEvent>;
        ///
        fn capture_item(
            &self,
            item: &impl PanelItemAspect,
        ) -> crate::node::NodeResult<()> {
            let mut _fds = Vec::new();
            let data = (item.node().id);
            self.node()
                .send_signal(
                    6398932320740499836u64,
                    1751367302976798762u64,
                    &data,
                    _fds,
                )?;
            let (item) = data;
            tracing::trace!(
                ? item, "Sent signal to server, {}::{}", "PanelItemAcceptor",
                "capture_item"
            );
            Ok(())
        }
    }
    ///Register a keymap with the server to easily identify it later
    pub async fn register_keymap(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        keymap: &str,
    ) -> crate::node::NodeResult<u64> {
        let mut _fds = Vec::new();
        let data = (keymap);
        {
            let (keymap) = &data;
            tracing::trace!(
                ? keymap, "Called method on server, {}::{}", "Interface",
                "register_keymap"
            );
        }
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        let message = _client
            .message_sender_handle
            .method(12u64, 0u64, 13267771052011565359u64, &serialized_data, _fds)
            .await?
            .map_err(|e| crate::node::NodeError::ReturnedError {
                e,
            })?
            .into_message();
        let result: u64 = stardust_xr::schemas::flex::deserialize(&message)?;
        let deserialized = result;
        tracing::trace!(
            "return" = ? deserialized, "Method return from server, {}::{}", "Interface",
            "register_keymap"
        );
        Ok(deserialized)
    }
    ///Get the keymap string representation from an ID
    pub async fn get_keymap(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        keymap_id: u64,
    ) -> crate::node::NodeResult<String> {
        let mut _fds = Vec::new();
        let data = (keymap_id);
        {
            let (keymap_id) = &data;
            tracing::trace!(
                ? keymap_id, "Called method on server, {}::{}", "Interface", "get_keymap"
            );
        }
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        let message = _client
            .message_sender_handle
            .method(12u64, 0u64, 18393315648981916968u64, &serialized_data, _fds)
            .await?
            .map_err(|e| crate::node::NodeError::ReturnedError {
                e,
            })?
            .into_message();
        let result: String = stardust_xr::schemas::flex::deserialize(&message)?;
        let deserialized = result;
        tracing::trace!(
            "return" = ? deserialized, "Method return from server, {}::{}", "Interface",
            "get_keymap"
        );
        Ok(deserialized)
    }
    ///Register this client to manage the items of a certain type and create default 3D UI for them.
    pub fn register_panel_item_ui(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
    ) -> crate::node::NodeResult<()> {
        let mut _fds = Vec::new();
        let data = ();
        let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
        _client
            .message_sender_handle
            .signal(12u64, 0u64, 13016197282381545765u64, &serialized_data, _fds)?;
        let () = data;
        tracing::trace!(
            "Sent signal to server, {}::{}", "Interface", "register_panel_item_ui"
        );
        Ok(())
    }
    ///Create an item acceptor to allow temporary ownership of a given type of item. Creates a node at `/item/panel/acceptor/<name>`.
    pub fn create_panel_item_acceptor(
        _client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        parent: &impl SpatialRefAspect,
        transform: Transform,
        field: &impl FieldAspect,
    ) -> crate::node::NodeResult<PanelItemAcceptor> {
        {
            let mut _fds = Vec::new();
            let data = (id, parent.node().id, transform, field.node().id);
            let serialized_data = stardust_xr::schemas::flex::serialize(&data)?;
            _client
                .message_sender_handle
                .signal(12u64, 0u64, 793626320493717815u64, &serialized_data, _fds)?;
            let (id, parent, transform, field) = data;
            tracing::trace!(
                ? id, ? parent, ? transform, ? field, "Sent signal to server, {}::{}",
                "Interface", "create_panel_item_acceptor"
            );
        }
        Ok(PanelItemAcceptor::from_id(_client, id, true))
    }
}
