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
#[allow(clippy::all)]
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Root(pub(crate) std::sync::Arc<crate::node::Node>);
#[allow(clippy::all)]
impl Root {
    pub(crate) fn from_id(
        client: &std::sync::Arc<crate::client::ClientHandle>,
        id: u64,
        owned: bool,
    ) -> Self {
        let node = crate::node::Node::from_id(client, id, owned);
        client.scenegraph.add_aspect::<RootEvent>(&node);
        Root(node)
    }
    pub fn as_spatial_ref(self) -> super::SpatialRef {
        super::SpatialRef(self.0)
    }
}
#[allow(clippy::all)]
impl crate::node::NodeType for Root {
    fn node(&self) -> &crate::node::Node {
        &self.0
    }
}
#[allow(clippy::all)]
impl serde::Serialize for Root {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0.id())
    }
}
#[allow(clippy::all)]
impl RootAspect for Root {}
pub(crate) const ROOT_ASPECT_ID: u64 = 7212020743076450030u64;
pub(crate) const ROOT_PING_CLIENT_OPCODE: u64 = 1374738518356883234u64;
pub(crate) const ROOT_FRAME_CLIENT_OPCODE: u64 = 2586777469268117179u64;
pub(crate) const ROOT_GET_STATE_SERVER_OPCODE: u64 = 14958324855167218950u64;
pub(crate) const ROOT_SAVE_STATE_CLIENT_OPCODE: u64 = 6559167809188075643u64;
pub(crate) const ROOT_GENERATE_STATE_TOKEN_SERVER_OPCODE: u64 = 530863980839400599u64;
pub(crate) const ROOT_GET_CONNECTION_ENVIRONMENT_SERVER_OPCODE: u64 = 3344613215577382567u64;
pub(crate) const ROOT_SET_BASE_PREFIXES_SERVER_OPCODE: u64 = 3714507829296596139u64;
pub(crate) const ROOT_DISCONNECT_SERVER_OPCODE: u64 = 662137628972844924u64;
#[derive(Debug)]
pub enum RootEvent {
    Ping { response: crate::TypedMethodResponse<()> },
    Frame { info: FrameInfo },
    SaveState { response: crate::TypedMethodResponse<ClientState> },
}
#[allow(clippy::all)]
impl crate::scenegraph::EventParser for RootEvent {
    const ASPECT_ID: u64 = 7212020743076450030u64;
    fn serialize_signal(
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
            1374738518356883234u64 => {
                let (): () = stardust_xr::schemas::flex::deserialize(_data)?;
                tracing::trace!("Method called from server, {}::{}", "Root", "ping");
                Ok(RootEvent::Ping {
                    response: crate::TypedMethodResponse(
                        response.borrow_mut().take().unwrap(),
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
                        response.borrow_mut().take().unwrap(),
                        std::marker::PhantomData,
                    ),
                })
            }
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
pub trait RootAspect: crate::node::NodeType + super::SpatialRefAspect + std::fmt::Debug {
    fn recv_root_event(&self) -> Option<RootEvent> {
        self.node().recv_event(7212020743076450030u64)
    }
    ///Get the current state. Useful to check the state before you initialize your application!
    async fn get_state(&self) -> crate::node::NodeResult<ClientState> {
        {
            let mut _fds = Vec::new();
            let data = ();
            {
                let () = &data;
                tracing::trace!("Called method on server, {}::{}", "Root", "get_state");
            }
            let result: ClientState = self
                .node()
                .execute_remote_method(
                    7212020743076450030u64,
                    14958324855167218950u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}", "Root",
                "get_state"
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
                .execute_remote_method(
                    7212020743076450030u64,
                    530863980839400599u64,
                    &data,
                    _fds,
                )
                .await?;
            let deserialized = result;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}", "Root",
                "generate_state_token"
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
                .execute_remote_method(
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
                    Result<stardust_xr::values::Map<String, _>, crate::node::NodeError>,
                >()?;
            tracing::trace!(
                "return" = ? deserialized, "Method return from server, {}::{}", "Root",
                "get_connection_environment"
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
            .send_remote_signal(
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
            .send_remote_signal(
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
