//! The base of all objects in Stardust.

use crate::client::Client;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize, Serializer};
use stardust_xr::{
	messenger::{Message, MessengerError},
	schemas::flex::{
		deserialize, flexbuffers::DeserializationError, serialize, FlexSerializeError,
	},
};
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

/// Common methods all nodes share, to make them easier to use.
// #[enum_dispatch(FieldType)]
pub trait NodeType: Sized + Send + Sync + 'static {
	/// Get a reference to the node.
	fn node(&self) -> &Node;
	fn from_id(client: &Arc<Client>, id: u64, destroyable: bool) -> Self;
	/// Try to get the client
	fn client(&self) -> NodeResult<Arc<Client>> {
		self.node().client()
	}
	/// Set whether the node is active or not. This has different effects depending on the node.
	fn set_enabled(&self, enabled: bool) -> Result<(), NodeError> {
		self.node()
			.send_remote_signal(OWNED_SET_ENABLED_SERVER_OPCODE, &enabled)
	}
	/// Create an alias of this node.
	/// Not the same as node scenegraph aliases,
	/// they are useful instead for getting a weak handle to a node.
	/// If the original node is destroyed, then any messages to the server will fail instantly with `NodeError::DoesNotExist`.
	fn alias(&self) -> Self
	where
		Self: Sized;
}

type Signal = dyn Fn(&[u8], Vec<OwnedFd>) -> color_eyre::eyre::Result<()> + Send + Sync + 'static;
type Method = dyn Fn(&[u8], Vec<OwnedFd>) -> color_eyre::eyre::Result<(Vec<u8>, Vec<OwnedFd>)>
	+ Send
	+ Sync
	+ 'static;

pub type NodeResult<O> = Result<O, NodeError>;

pub struct NodeInternals {
	client: Weak<Client>,
	self_ref: Weak<NodeInternals>,
	pub(crate) id: u64,
	pub(crate) local_signals: Mutex<FxHashMap<u64, Arc<Signal>>>,
	pub(crate) local_methods: Mutex<FxHashMap<u64, Arc<Method>>>,
	pub(crate) destroyable: bool,
}
impl Drop for NodeInternals {
	fn drop(&mut self) {
		if let Some(client) = self.client.upgrade() {
			if self.destroyable {
				let _ = client.message_sender_handle.signal(
					self.id,
					OWNED_DESTROY_SERVER_OPCODE,
					&[],
					Vec::new(),
				);
			}
			client.scenegraph.remove_node(self.id);
		}
	}
}

/// An object in the client's scenegraph on the server. Almost all calls to a node are IPC calls and so have several microseconds of delay, be aware.
pub enum Node {
	Owned(Arc<NodeInternals>),
	Aliased(Weak<NodeInternals>),
}
impl Node {
	/// Add a signal to the node so that the server can send a message to it. Not needed unless implementing functionality Fusion does not already have.
	pub fn add_local_signal<F>(&self, id: u64, signal: F) -> Result<(), NodeError>
	where
		F: Fn(&[u8], Vec<OwnedFd>) -> color_eyre::eyre::Result<()> + Send + Sync + 'static,
	{
		self.internals()?
			.local_signals
			.lock()
			.insert(id, Arc::new(signal));
		Ok(())
	}

	/// Add a signal to the node so that the server can send a message to it and get a response back. Not needed unless implementing functionality Fusion does not already have.
	pub fn add_local_method<F>(&self, id: u64, method: F) -> Result<(), NodeError>
	where
		F: Fn(&[u8], Vec<OwnedFd>) -> color_eyre::eyre::Result<(Vec<u8>, Vec<OwnedFd>)>
			+ Send
			+ Sync
			+ 'static,
	{
		self.internals()?
			.local_methods
			.lock()
			.insert(id, Arc::new(method));
		Ok(())
	}

	pub(crate) fn internals(&self) -> Result<Arc<NodeInternals>, NodeError> {
		match self {
			Node::Owned(node) => Ok(node.clone()),
			Node::Aliased(node) => node.upgrade().ok_or(NodeError::DoesNotExist),
		}
	}

	/// Try to get the client from the node, it's a result because that makes it work a lot better with `?` in internal functions.
	pub fn client(&self) -> Result<Arc<Client>, NodeError> {
		self.internals()?
			.client
			.upgrade()
			.ok_or(NodeError::ClientDropped)
	}

	/// Get the entire path of the node including the name.
	pub fn get_id(&self) -> Result<u64, NodeError> {
		Ok(self.internals()?.id)
	}

	/// Check if this node is still alive.
	pub fn alive(&self) -> bool {
		match self {
			Node::Owned(_) => true,
			Node::Aliased(a) => a.strong_count() > 0,
		}
	}

	/// Send a signal to the node on the server. Not needed unless implementing functionality Fusion does not already have.
	pub fn send_remote_signal<S: Serialize>(&self, signal: u64, data: &S) -> Result<(), NodeError> {
		self.send_remote_signal_raw(
			signal,
			&serialize(data).map_err(|_| NodeError::Serialization)?,
			Vec::new(),
		)
	}
	/// Send a signal to the node on the server with raw data (like when sending flatbuffers over). Not needed unless implementing functionality Fusion does not already have.
	pub fn send_remote_signal_raw(
		&self,
		signal: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), NodeError> {
		self.client()?
			.message_sender_handle
			.signal(self.get_id()?, signal, data, fds)
			.map_err(|e| NodeError::MessengerError { e })
	}
	/// Execute a method on the node on the server. Not needed unless implementing functionality Fusion does not already have.
	pub async fn execute_remote_method<S: Serialize, D: DeserializeOwned>(
		&self,
		method: u64,
		send_data: &S,
	) -> Result<D, NodeError> {
		let send_data = serialize(send_data).map_err(|_| NodeError::Serialization)?;
		let future = self.execute_remote_method_raw(method, &send_data, Vec::new())?;
		let data = future.await?;
		deserialize(&data.into_message()).map_err(|e| NodeError::Deserialization { e })
	}
	/// Execute a method on the node on the server with raw data (like when sending over flatbuffers). Not needed unless implementing functionality Fusion does not already have.
	pub fn execute_remote_method_raw(
		&self,
		method: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<impl Future<Output = Result<Message, NodeError>>, NodeError> {
		let future = self
			.client()?
			.message_sender_handle
			.method(self.get_id()?, method, data, fds)
			.map_err(|e| NodeError::MessengerError { e })?;

		Ok(async move { future.await.map_err(|e| NodeError::ReturnedError { e }) })
	}
}
impl NodeType for Node {
	fn node(&self) -> &Node {
		self
	}

	fn alias(&self) -> Self {
		match self {
			Node::Owned(internals) => Node::Aliased(Arc::downgrade(internals)),
			Node::Aliased(internals) => Node::Aliased(internals.clone()),
		}
	}

	fn from_id(client: &Arc<Client>, id: u64, destroyable: bool) -> Node {
		let node = Arc::new_cyclic(|self_ref| NodeInternals {
			client: Arc::downgrade(client),
			self_ref: self_ref.clone(),
			id,
			local_signals: Mutex::new(FxHashMap::default()),
			local_methods: Mutex::new(FxHashMap::default()),
			destroyable,
		});
		client.scenegraph.add_node(&node);
		Node::Owned(node)
	}
}
impl serde::Serialize for Node {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let node_id = self
			.node()
			.get_id()
			.map_err(|e| serde::ser::Error::custom(e))?;
		serializer.serialize_u64(node_id)
	}
}
impl Debug for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut dbg = f.debug_struct("Node");

		if let Ok(internals) = self.internals() {
			dbg.field("id", &internals.id)
				.field(
					"local_signals",
					&internals
						.local_signals
						.lock()
						.iter()
						.map(|(key, _)| key)
						.collect::<Vec<_>>(),
				)
				.field(
					"local_methods",
					&internals
						.local_methods
						.lock()
						.iter()
						.map(|(key, _)| key)
						.collect::<Vec<_>>(),
				);
		} else {
			dbg.field("node", &"broken");
		}

		dbg.finish()
	}
}
