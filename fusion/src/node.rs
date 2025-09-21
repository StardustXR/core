//! The base of all objects in Stardust.

use crate::client::ClientHandle;
use serde::{Serialize, de::DeserializeOwned};
use stardust_xr::{
	messenger::MessengerError,
	scenegraph::ScenegraphError,
	schemas::flex::{
		FlexSerializeError, deserialize, flexbuffers::DeserializationError, serialize,
	},
};
use std::{fmt::Debug, os::fd::OwnedFd, sync::Arc, vec::Vec};
use thiserror::Error;

pub use crate::protocol::node::*;

pub type NodeResult<O> = Result<O, NodeError>;

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
	#[error("Serialization failed with an error: {e}")]
	Serialization { e: FlexSerializeError },
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
	fn from(e: FlexSerializeError) -> Self {
		NodeError::Serialization { e }
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
	fn node(&self) -> &NodeCore;
	// What's the node's ID? (used for comparison)
	fn id(&self) -> u64 {
		self.node().id
	}
	/// Try to get the client
	fn client(&self) -> &Arc<ClientHandle> {
		&self.node().client
	}
	/// Set whether the node is active or not. This has different effects depending on the node.
	fn set_enabled(&self, enabled: bool) -> Result<(), NodeError> {
		if self.node().owned {
			OwnedAspect::set_enabled(self.node(), enabled)
		} else {
			Err(NodeError::DoesNotExist)
		}
	}
}

pub(crate) struct NodeCore {
	pub client: Arc<ClientHandle>,
	pub id: u64,
	pub(crate) owned: bool,
}
impl NodeCore {
	pub(crate) fn new(client: Arc<ClientHandle>, id: u64, owned: bool) -> Self {
		Self { client, id, owned }
	}

	/// Send a signal directly - no weak reference upgrade!
	pub(crate) fn send_signal<S: Serialize>(
		&self,
		aspect: u64,
		signal: u64,
		data: &S,
		fds: Vec<OwnedFd>,
	) -> Result<(), NodeError> {
		let serialized = serialize(data).map_err(|e| NodeError::Serialization { e })?;
		self.client
			.message_sender_handle
			.signal(self.id, aspect, signal, &serialized, fds)
			.map_err(|e| match e {
				MessengerError::ReceiverDropped => NodeError::ClientDropped,
				other => NodeError::MessengerError { e: other },
			})
	}

	/// Execute a method directly - no weak reference upgrade!
	pub(crate) async fn call_method<S: Serialize, D: DeserializeOwned>(
		&self,
		aspect: u64,
		method: u64,
		data: &S,
		fds: Vec<OwnedFd>,
	) -> Result<D, NodeError> {
		let serialized = serialize(data).map_err(|e| NodeError::Serialization { e })?;

		let response = self
			.client
			.message_sender_handle
			.method(self.id, aspect, method, &serialized, fds)
			.await
			.map_err(|e| match e {
				MessengerError::ReceiverDropped => NodeError::ClientDropped,
				other => NodeError::MessengerError { e: other },
			})?
			.map_err(|e| NodeError::ReturnedError { e })?;

		deserialize(&response.into_message()).map_err(|e| NodeError::Deserialization { e })
	}
}
impl NodeType for NodeCore {
	fn node(&self) -> &NodeCore {
		self
	}
}
impl OwnedAspect for NodeCore {}
impl std::fmt::Debug for NodeCore {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("NodeCore")
			.field("id", &self.id)
			.field("owned", &self.owned)
			.finish()
	}
}
impl Drop for NodeCore {
	fn drop(&mut self) {
		if self.owned {
			let _ = self.destroy();
		}
	}
}

// impl NodeType for Node {
// 	fn node(&self) -> &Node {
// 		self
// 	}
// }
// impl serde::Serialize for Node {
// 	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
// 		serializer.serialize_u64(self.core.id)
// 	}
// }
// impl Debug for Node {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		let mut dbg = f.debug_struct("Node");
// 		dbg.field("id", &self.core.id)
// 			.field("aspects", &self.aspects);
// 		dbg.finish()
// 	}
// }
// impl Hash for Node {
// 	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
// 		self.core.id.hash(state)
// 	}
// }
// impl PartialEq for Node {
// 	fn eq(&self, other: &Self) -> bool {
// 		self.core.id.eq(&other.core.id)
// 	}
// }
// impl Eq for Node {}
