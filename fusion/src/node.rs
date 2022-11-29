use super::client::Client;
use anyhow::Result;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize, Serializer};
use stardust_xr::{
	messenger::MessengerError,
	schemas::flex::{deserialize, serialize},
};
use std::{
	fmt::Debug,
	future::Future,
	pin::Pin,
	sync::{Arc, Weak},
	vec::Vec,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeError {
	#[error("client has been dropped")]
	ClientDropped,
	#[error("Messenger error: {e}")]
	MessengerError { e: MessengerError },
	#[error("Node does not exist anymore")]
	DoesNotExist,
	#[error("invalid path")]
	InvalidPath,
	#[error("Serialization failed")]
	Serialization,
	#[error("Desrialization failed")]
	Deserialization,
	#[error("Attempted to register to a singleton twice")]
	OverrideSingleton,
	#[error("Map is not a valid flexbuffer map at the root")]
	MapInvalid,
}

pub trait NodeType: Send + Sync + Sized + 'static {
	fn node(&self) -> &Node;
	fn client(&self) -> Result<Arc<Client>, NodeError> {
		self.node().client()
	}
}
pub trait ClientOwned: NodeType {}

type Signal = dyn Fn(&[u8]) -> Result<()> + Send + Sync + 'static;
type Method = dyn Fn(&[u8]) -> Result<Vec<u8>> + Send + Sync + 'static;

pub type BoxedFuture<O> = Pin<Box<dyn Future<Output = O>>>;

pub struct NodeInternals {
	pub client: Weak<Client>,
	self_ref: Weak<NodeInternals>,
	parent: String,
	name: String,
	pub(crate) local_signals: Mutex<FxHashMap<String, Arc<Signal>>>,
	pub(crate) local_methods: Mutex<FxHashMap<String, Arc<Method>>>,
	pub(crate) destroyable: bool,
}
impl NodeInternals {
	pub(crate) fn path(&self) -> String {
		self.parent.clone() + "/" + &self.name
	}
}
impl Serialize for NodeInternals {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.path().serialize(serializer)
	}
}

impl Drop for NodeInternals {
	fn drop(&mut self) {
		if let Some(client) = self.client.upgrade() {
			let path = self.path();
			if self.destroyable {
				let _ = client.message_sender_handle.signal(&path, "destroy", &[]);
			}
			client.scenegraph.remove_node(&path);
		}
	}
}

pub enum Node {
	Owned(Arc<NodeInternals>),
	Aliased(Weak<NodeInternals>),
}
impl Node {
	pub(crate) fn new<'a, S: Serialize>(
		client: &Arc<Client>,
		interface_path: &'a str,
		interface_method: &'a str,
		parent_path: &'a str,
		destroyable: bool,
		id: &str,
		data: S,
	) -> Result<Node, NodeError> {
		let node = Node::from_path(client, parent_path, id, destroyable);

		client
			.message_sender_handle
			.signal(
				interface_path,
				interface_method,
				&serialize(data).map_err(|_| NodeError::Serialization)?,
			)
			.map_err(|e| NodeError::MessengerError { e })?;

		Ok(node)
	}
	pub fn from_path(
		client: &Arc<Client>,
		parent: impl ToString,
		name: impl ToString,
		destroyable: bool,
	) -> Node {
		let node = Arc::new_cyclic(|self_ref| NodeInternals {
			client: Arc::downgrade(client),
			self_ref: self_ref.clone(),
			parent: parent.to_string(),
			name: name.to_string(),
			local_signals: Mutex::new(FxHashMap::default()),
			local_methods: Mutex::new(FxHashMap::default()),
			destroyable,
		});
		client.scenegraph.add_node(&node);
		Node::Owned(node)
	}

	pub(crate) fn aliased(&self) -> Node {
		match self {
			Node::Owned(internals) => Node::Aliased(Arc::downgrade(internals)),
			Node::Aliased(internals) => Node::Aliased(internals.clone()),
		}
	}

	pub fn add_local_signal<F>(&self, name: impl ToString, signal: F) -> Result<(), NodeError>
	where
		F: Fn(&[u8]) -> Result<()> + Send + Sync + 'static,
	{
		self.internals()?
			.local_signals
			.lock()
			.insert(name.to_string(), Arc::new(signal));
		Ok(())
	}

	pub fn add_local_method<F>(&self, name: impl ToString, method: F) -> Result<(), NodeError>
	where
		F: Fn(&[u8]) -> Result<Vec<u8>> + Send + Sync + 'static,
	{
		self.internals()?
			.local_methods
			.lock()
			.insert(name.to_string(), Arc::new(method));
		Ok(())
	}

	pub(crate) fn internals(&self) -> Result<Arc<NodeInternals>, NodeError> {
		match self {
			Node::Owned(node) => Ok(node.clone()),
			Node::Aliased(node) => node.upgrade().ok_or(NodeError::DoesNotExist),
		}
	}

	pub fn client(&self) -> Result<Arc<Client>, NodeError> {
		self.internals()?
			.client
			.upgrade()
			.ok_or(NodeError::ClientDropped)
	}

	pub fn get_name(&self) -> Result<String, NodeError> {
		Ok(self.internals()?.parent.to_string())
	}
	pub fn get_path(&self) -> Result<String, NodeError> {
		Ok(self.internals()?.path())
	}
	// pub fn generate_with_parent(
	// 	client: Weak<Client>,
	// 	parent: &str,
	// 	destroyable: bool,
	// ) -> Result<(Arc<Self>, String), NodeError> {
	// 	let id = nanoid!(10);
	// 	let mut path = parent.to_string();
	// 	if !path.starts_with('/') {
	// 		return Err(NodeError::InvalidPath);
	// 	}
	// 	if !path.ends_with('/') {
	// 		path.push('/');
	// 	}
	// 	path.push_str(&id);

	// 	Node::from_path(client, path, destroyable).map(|node| (node, id))
	// }

	pub fn send_remote_signal<S: Serialize>(
		&self,
		signal_name: &str,
		data: &S,
	) -> Result<(), NodeError> {
		self.send_remote_signal_raw(
			signal_name,
			&serialize(data).map_err(|_| NodeError::Serialization)?,
		)
	}
	pub fn send_remote_signal_raw(&self, signal_name: &str, data: &[u8]) -> Result<(), NodeError> {
		self.client()?
			.message_sender_handle
			.signal(&self.get_path()?, signal_name, data)
			.map_err(|e| NodeError::MessengerError { e })
	}
	pub fn execute_remote_method<S: Serialize, D: DeserializeOwned>(
		&self,
		method_name: &str,
		send_data: &S,
	) -> Result<impl Future<Output = Result<D>>, NodeError> {
		let send_data = serialize(send_data).map_err(|_| NodeError::Serialization)?;
		let future = self.execute_remote_method_raw(method_name, &send_data)?;
		Ok(async move {
			future
				.await
				.and_then(|data| -> anyhow::Result<D> { deserialize(&data).map_err(|e| e.into()) })
		})
	}
	pub fn execute_remote_method_trait<S: Serialize, D: DeserializeOwned>(
		&self,
		method_name: &str,
		send_data: &S,
	) -> Result<Pin<Box<dyn Future<Output = Result<D>>>>, NodeError> {
		let send_data = serialize(send_data).map_err(|_| NodeError::Serialization)?;
		let future = self.execute_remote_method_raw(method_name, &send_data)?;
		Ok(Box::pin(async move {
			future
				.await
				.and_then(|data| -> anyhow::Result<D> { deserialize(&data).map_err(|e| e.into()) })
		}))
	}
	pub fn execute_remote_method_raw(
		&self,
		method_name: &str,
		data: &[u8],
	) -> Result<impl Future<Output = Result<Vec<u8>>>, NodeError> {
		self.client()?
			.message_sender_handle
			.method(&self.get_path()?, method_name, data)
			.map_err(|e| NodeError::MessengerError { e })
	}
	fn set_enabled(&self, enabled: bool) -> Result<(), NodeError> {
		self.send_remote_signal("set_enabled", &enabled)
	}
}

impl Debug for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut dbg = f.debug_struct("Node");

		if let Ok(internals) = self.internals() {
			dbg.field("path", &internals.path())
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
