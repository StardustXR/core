use super::client::Client;
use anyhow::Result;
use nanoid::nanoid;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize, Serializer};
use stardust_xr::{
	messenger::MessengerError,
	scenegraph::ScenegraphError,
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

pub trait NodeType: Sized {
	fn node(&self) -> &Node;
	fn client(&self) -> Option<Arc<Client>> {
		self.node().client.upgrade()
	}
}
pub trait ClientOwned: NodeType {}

type Signal = dyn Fn(&[u8]) -> Result<()> + Send + Sync + 'static;
type Method = dyn Fn(&[u8]) -> Result<Vec<u8>> + Send + Sync + 'static;

pub type BoxedFuture<O> = Pin<Box<dyn Future<Output = O>>>;

pub struct Node {
	path: String,
	trailing_slash_pos: usize,
	pub client: Weak<Client>,
	self_ref: Weak<Node>,
	pub(crate) local_signals: Mutex<FxHashMap<String, Arc<Signal>>>,
	pub(crate) local_methods: Mutex<FxHashMap<String, Arc<Method>>>,
	pub(crate) destroyable: bool,
}

impl Node {
	pub(crate) fn new<S: Serialize>(
		client: Weak<Client>,
		interface_path: &'static str,
		interface_method: &'static str,
		parent_path: &'static str,
		destroyable: bool,
		id: &str,
		data: S,
	) -> Result<Arc<Self>, NodeError> {
		let mut parent_path = parent_path.to_string();
		if !parent_path.ends_with('/') {
			parent_path += "/";
		}
		parent_path += id;

		let node = Node::from_path(client, parent_path, destroyable)?;

		node.client()?
			.message_sender_handle
			.signal(
				interface_path,
				interface_method,
				&serialize(data).map_err(|_| NodeError::Serialization)?,
			)
			.map_err(|e| NodeError::MessengerError { e })?;

		Ok(node)
	}

	pub fn client(&self) -> Result<Arc<Client>, NodeError> {
		self.client.upgrade().ok_or(NodeError::ClientDropped)
	}

	pub fn get_name(&self) -> &str {
		&self.path[self.trailing_slash_pos + 1..]
	}
	pub fn get_path(&self) -> &str {
		self.path.as_str()
	}

	pub fn from_path(
		client: Weak<Client>,
		path: String,
		destroyable: bool,
	) -> Result<Arc<Self>, NodeError> {
		if !path.starts_with('/') {
			return Err(NodeError::InvalidPath);
		}

		let trailing_slash_pos = path.rfind('/').ok_or(NodeError::InvalidPath)?;
		let node = Arc::new_cyclic(|self_ref| Node {
			trailing_slash_pos,
			path,
			client: client.clone(),
			self_ref: self_ref.clone(),
			local_signals: Mutex::new(FxHashMap::default()),
			local_methods: Mutex::new(FxHashMap::default()),
			destroyable,
		});
		client
			.upgrade()
			.ok_or(NodeError::ClientDropped)?
			.scenegraph
			.add_node(Arc::downgrade(&node));
		Ok(node)
	}
	pub fn generate_with_parent(
		client: Weak<Client>,
		parent: &str,
		destroyable: bool,
	) -> Result<(Arc<Self>, String), NodeError> {
		let id = nanoid!(10);
		let mut path = parent.to_string();
		if !path.starts_with('/') {
			return Err(NodeError::InvalidPath);
		}
		if !path.ends_with('/') {
			path.push('/');
		}
		path.push_str(&id);

		Node::from_path(client, path, destroyable).map(|node| (node, id))
	}

	pub fn send_local_signal(&self, signal_name: &str, data: &[u8]) -> Result<(), ScenegraphError> {
		let local_signals = self.local_signals.lock();
		let signal = local_signals
			.get(signal_name)
			.ok_or(ScenegraphError::SignalNotFound)?
			.clone();
		signal(data).map_err(|e| ScenegraphError::SignalError { error: e })
	}
	pub fn execute_local_method(
		&self,
		method_name: &str,
		data: &[u8],
	) -> Result<Vec<u8>, ScenegraphError> {
		let local_methods = self.local_methods.lock();
		let method = local_methods
			.get(method_name)
			.ok_or(ScenegraphError::MethodNotFound)?
			.clone();
		method(data).map_err(|e| ScenegraphError::MethodError { error: e })
	}
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
		self.client
			.upgrade()
			.ok_or(NodeError::ClientDropped)?
			.message_sender_handle
			.signal(self.path.as_str(), signal_name, data)
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
			.method(self.path.as_str(), method_name, data)
			.map_err(|e| NodeError::MessengerError { e })
	}
	fn set_enabled(&self, enabled: bool) -> Result<(), NodeError> {
		self.send_remote_signal("setEnabled", &enabled)
	}
}

impl Serialize for Node {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.get_path().serialize(serializer)
	}
}

impl Debug for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Node")
			.field("path", &self.path)
			.field(
				"local_signals",
				&self
					.local_signals
					.lock()
					.iter()
					.map(|(key, _)| key)
					.collect::<Vec<_>>(),
			)
			.field(
				"local_methods",
				&self
					.local_methods
					.lock()
					.iter()
					.map(|(key, _)| key)
					.collect::<Vec<_>>(),
			)
			.finish()
	}
}

impl Drop for Node {
	fn drop(&mut self) {
		if self.destroyable {
			let _ = self.send_remote_signal_raw("destroy", &[]);
		}
		if let Some(client) = self.client.upgrade() {
			client.scenegraph.remove_node(self.self_ref.clone());
		}
	}
}
