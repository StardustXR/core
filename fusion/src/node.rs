//! The base of all objects in Stardust.

use crate::{client::ClientHandle, scenegraph::EventParser};
use dashmap::DashMap;
use parking_lot::Mutex;
use serde::{de::DeserializeOwned, Serialize, Serializer};
use stardust_xr::{
	messenger::{Message, MessengerError},
	scenegraph::ScenegraphError,
	schemas::flex::{
		deserialize, flexbuffers::DeserializationError, serialize, FlexSerializeError,
	},
};
use std::any::Any;
use std::{
	fmt::Debug,
	future::Future,
	os::fd::OwnedFd,
	sync::{Arc, Weak},
	vec::Vec,
};
use thiserror::Error;

stardust_xr_fusion_codegen::codegen_node_protocol!();

pub type MethodResult<T> = color_eyre::eyre::Result<T>;

#[derive(Error, Debug)]
pub enum NodeError {
	#[error("client has been dropped")]
	ClientDropped,
	#[error("Messenger error: {e}")]
	MessengerError { e: MessengerError },
	#[error("Node does not exist anymore")]
	DoesNotExist,
	#[error("Node's signal/method isn't available because it is an alias node")]
	NotAliased,
	#[error("invalid path")]
	InvalidPath,
	#[error("Serialization failed")]
	Serialization,
	#[error("Deserialization failed with an error: {e}")]
	Deserialization { e: DeserializationError },
	/// The server returned an error on a method return.
	#[error("Server returned an error: {e}")]
	ReturnedError { e: String },
	#[error("Attempted to register to a singleton twice")]
	OverrideSingleton,
	/// The given data is not a valid flexbuffer map.
	#[error("Map is not a valid flexbuffer map at the root")]
	MapInvalid,
}
impl From<MessengerError> for NodeError {
	fn from(e: MessengerError) -> Self {
		NodeError::MessengerError { e }
	}
}
impl From<FlexSerializeError> for NodeError {
	fn from(_: FlexSerializeError) -> Self {
		NodeError::Serialization
	}
}
impl From<DeserializationError> for NodeError {
	fn from(e: DeserializationError) -> Self {
		NodeError::Deserialization { e }
	}
}
impl From<String> for NodeError {
	fn from(e: String) -> Self {
		NodeError::ReturnedError { e }
	}
}
impl From<NodeError> for ScenegraphError {
	fn from(e: NodeError) -> Self {
		ScenegraphError::MemberError {
			error: e.to_string(),
		}
	}
}

/// Common methods all nodes share, to make them easier to use.
// #[enum_dispatch(FieldType)]
pub trait NodeType: Sized + Send + Sync + 'static {
	/// Get a reference to the node.
	fn node(&self) -> &Node;
	fn from_id(client: &Arc<ClientHandle>, id: u64, owned: bool) -> Self;
	/// Try to get the client
	fn client(&self) -> NodeResult<Arc<ClientHandle>> {
		self.node().client()
	}
	/// Set whether the node is active or not. This has different effects depending on the node.
	fn set_enabled(&self, enabled: bool) -> Result<(), NodeError> {
		self.node()
			.send_remote_signal(OWNED_ASPECT_ID, OWNED_SET_ENABLED_SERVER_OPCODE, &enabled)
	}
}

pub type NodeResult<O> = Result<O, NodeError>;

/// An object in the client's scenegraph on the server. Almost all calls to a node are IPC calls and so have several microseconds of delay, be aware.
pub struct Node {
	client: Weak<ClientHandle>,
	pub(crate) id: u64,
	pub(crate) aspects: DashMap<u64, Mutex<Box<dyn Any + Send + Sync + 'static>>>,
	pub(crate) owned: bool,
}
impl Node {
	/// Try to get the client from the node, it's a result because that makes it work a lot better with `?` in internal functions.
	pub fn client(&self) -> Result<Arc<ClientHandle>, NodeError> {
		self.client.upgrade().ok_or(NodeError::ClientDropped)
	}

	/// Get the entire path of the node including the name.
	pub fn get_id(&self) -> Result<u64, NodeError> {
		Ok(self.id)
	}

	/// Check if this node is still alive.
	pub fn alive(&self) -> bool {
		self.client.strong_count() > 0
	}

	/// Send a signal to the node on the server. Not needed unless implementing functionality Fusion does not already have.
	pub fn send_remote_signal<S: Serialize>(
		&self,
		aspect: u64,
		signal: u64,
		data: &S,
	) -> Result<(), NodeError> {
		self.send_remote_signal_raw(
			aspect,
			signal,
			&serialize(data).map_err(|_| NodeError::Serialization)?,
			Vec::new(),
		)
	}
	/// Send a signal to the node on the server with raw data (like when sending flatbuffers over). Not needed unless implementing functionality Fusion does not already have.
	pub fn send_remote_signal_raw(
		&self,
		aspect: u64,
		signal: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), NodeError> {
		self.client()?
			.message_sender_handle
			.signal(self.get_id()?, aspect, signal, data, fds)
			.map_err(|e| NodeError::MessengerError { e })
	}
	/// Execute a method on the node on the server. Not needed unless implementing functionality Fusion does not already have.
	pub async fn execute_remote_method<S: Serialize, D: DeserializeOwned>(
		&self,
		aspect: u64,
		method: u64,
		send_data: &S,
	) -> Result<D, NodeError> {
		let send_data = serialize(send_data).map_err(|_| NodeError::Serialization)?;
		let future = self.execute_remote_method_raw(aspect, method, &send_data, Vec::new())?;
		let data = future.await?;
		deserialize(&data.into_message()).map_err(|e| NodeError::Deserialization { e })
	}
	/// Execute a method on the node on the server with raw data (like when sending over flatbuffers). Not needed unless implementing functionality Fusion does not already have.
	pub fn execute_remote_method_raw(
		&self,
		aspect: u64,
		method: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<impl Future<Output = Result<Message, NodeError>>, NodeError> {
		let future = self
			.client()?
			.message_sender_handle
			.method(self.get_id()?, aspect, method, data, fds)
			.map_err(|e| NodeError::MessengerError { e })?;

		Ok(async move { future.await.map_err(|e| NodeError::ReturnedError { e }) })
	}

	pub(crate) fn recv_event<E: EventParser>(&self, id: u64) -> Option<E> {
		let aspect = self.node().aspects.get(&id)?;
		let lock = &mut *aspect.lock();
		let receiver = lock.downcast_mut::<tokio::sync::mpsc::UnboundedReceiver<E>>()?;
		receiver.try_recv().ok()
	}
}
impl NodeType for Node {
	fn node(&self) -> &Node {
		self
	}

	fn from_id(client: &Arc<ClientHandle>, id: u64, owned: bool) -> Node {
		Node {
			client: Arc::downgrade(client),
			id,
			aspects: DashMap::default(),
			owned,
		}
	}
}
impl serde::Serialize for Node {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_u64(self.id)
	}
}
impl Debug for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut dbg = f.debug_struct("Node");
		dbg.field("id", &self.id).field("aspects", &self.aspects);
		dbg.finish()
	}
}
impl Drop for Node {
	fn drop(&mut self) {
		if let Some(client) = self.client.upgrade() {
			if self.owned {
				let _ = client.message_sender_handle.signal(
					self.id,
					OWNED_ASPECT_ID,
					OWNED_DESTROY_SERVER_OPCODE,
					&[],
					Vec::new(),
				);
			}
			client.scenegraph.remove_node(self.id);
		}
	}
}
