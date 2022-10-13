use super::client::Client;
use anyhow::Result;
use futures::Future;
use nanoid::nanoid;
use serde::{de::DeserializeOwned, Serialize, Serializer};
use stardust_xr::schemas::flex::{deserialize, serialize};
use std::{
	sync::{Arc, Weak},
	vec::Vec,
};
use thiserror::Error;

use core::hash::BuildHasherDefault;
use dashmap::DashMap;
use rustc_hash::FxHasher;

// pub(crate) struct GenNodeInfo<'a> {
// 	pub(crate) client: Weak<Client>,
// 	pub(crate) parent_path: &'a str,
// 	pub(crate) interface_path: &'a str,
// 	pub(crate) interface_method: &'a str,
// }
// macro_rules! generate_node {
// 	($gen_node_info:expr, $($things_to_pass:expr),*) => {
// 		{
// 			let (node, id) = Node::generate_with_parent($gen_node_info.client, $gen_node_info.parent_path)?;
// 			node.client
// 				.upgrade()
// 				.ok_or(NodeError::ClientDropped)?
// 				.messenger
// 				.send_remote_signal(
// 					$gen_node_info.interface_path,
// 					$gen_node_info.interface_method,
// 					stardust_xr::flex::flexbuffer_from_vector_arguments(|vec| {
// 						stardust_xr::push_to_vec![vec, id.as_str(), $($things_to_pass),+]
// 					})
// 					.as_slice(),
// 				);
// 				node
// 		}

// 	}
// }

#[derive(Error, Debug)]
pub enum NodeError {
	#[error("server creation failed")]
	ServerCreationFailed,
	#[error("client has been dropped")]
	ClientDropped,
	#[error("messenger write failed")]
	MessengerWrite,
	#[error("invalid path")]
	InvalidPath,
	#[error("node doesn't exist")]
	NodeNotFound,
	#[error("method doesn't exist")]
	MethodNotFound,
	#[error("Signal failed")]
	SignalFailed,
	#[error("Method failed")]
	MethodFailed,
	#[error("Serialization failed")]
	Serialization,
	#[error("Desrialization failed")]
	Deserialization,
	#[error("Attempted to register to a singleton twice")]
	OverrideSingleton,
}

pub trait NodeType: Sized {
	fn node(&self) -> &Node;
	fn client(&self) -> Option<Arc<Client>> {
		self.node().client.upgrade()
	}
}

type Signal = dyn Fn(&[u8]) -> Result<()> + Send + Sync + 'static;
type Method = dyn Fn(&[u8]) -> Result<Vec<u8>> + Send + Sync + 'static;

pub struct Node {
	path: String,
	trailing_slash_pos: usize,
	pub client: Weak<Client>,
	pub(crate) local_signals: DashMap<String, Box<Signal>, BuildHasherDefault<FxHasher>>,
	pub(crate) local_methods: DashMap<String, Box<Method>, BuildHasherDefault<FxHasher>>,
}

impl Node {
	pub(crate) fn new<S: Serialize>(
		client: Weak<Client>,
		interface_path: &'static str,
		interface_method: &'static str,
		parent_path: &'static str,
		id: &str,
		data: S,
	) -> Result<Arc<Self>, NodeError> {
		let mut parent_path = parent_path.to_string();
		if !parent_path.ends_with('/') {
			parent_path += "/";
		}
		parent_path += id;

		let node = Node::from_path(client, parent_path)?;

		node.client
			.upgrade()
			.ok_or(NodeError::ClientDropped)?
			.messenger
			.send_remote_signal(
				interface_path,
				interface_method,
				&serialize(data).map_err(|_| NodeError::Serialization)?,
			);

		Ok(node)
	}

	pub fn get_name(&self) -> &str {
		&self.path[self.trailing_slash_pos + 1..]
	}
	pub fn get_path(&self) -> &str {
		self.path.as_str()
	}

	pub fn from_path(client: Weak<Client>, path: String) -> Result<Arc<Self>, NodeError> {
		if !path.starts_with('/') {
			return Err(NodeError::InvalidPath);
		}
		let node = Node {
			trailing_slash_pos: path.rfind('/').ok_or(NodeError::InvalidPath)?,
			path: path,
			client: client.clone(),
			local_signals: DashMap::default(),
			local_methods: DashMap::default(),
		};
		let node_ref = Arc::new(node);
		client
			.upgrade()
			.ok_or(NodeError::ClientDropped)?
			.scenegraph
			.add_node(Arc::downgrade(&node_ref));
		Ok(node_ref)
	}
	pub fn generate_with_parent(
		client: Weak<Client>,
		parent: &str,
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

		Node::from_path(client, path).map(|node| (node, id))
	}

	pub fn send_local_signal(&self, method: &str, data: &[u8]) -> Result<(), NodeError> {
		let signal = self
			.local_signals
			.get(method)
			.ok_or(NodeError::MethodNotFound)?;
		signal(data).map_err(|_| NodeError::SignalFailed)
	}
	pub fn execute_local_method(&self, method: &str, data: &[u8]) -> Result<Vec<u8>, NodeError> {
		let method = self
			.local_methods
			.get(method)
			.ok_or(NodeError::MethodNotFound)?;
		method(data).map_err(|_| NodeError::MethodFailed)
	}
	pub fn send_remote_signal<S: Serialize>(
		&self,
		method: &str,
		data: &S,
	) -> Result<(), NodeError> {
		self.send_remote_signal_raw(
			method,
			&serialize(data).map_err(|_| NodeError::Serialization)?,
		)
	}
	pub fn send_remote_signal_raw(&self, method: &str, data: &[u8]) -> Result<(), NodeError> {
		self.client
			.upgrade()
			.ok_or(NodeError::ClientDropped)?
			.messenger
			.send_remote_signal(self.path.as_str(), method, data);
		Ok(())
	}
	pub fn execute_remote_method<S: Serialize, D: DeserializeOwned>(
		&self,
		method: &str,
		send_data: &S,
	) -> Result<impl Future<Output = Result<D>>, NodeError> {
		let send_data = serialize(send_data).map_err(|_| NodeError::Serialization)?;
		let future = self.execute_remote_method_raw(method, &send_data)?;
		Ok(async move {
			future
				.await
				.and_then(|data| -> anyhow::Result<D> { deserialize(&data).map_err(|e| e.into()) })
		})
	}
	pub fn execute_remote_method_raw(
		&self,
		method: &str,
		data: &[u8],
	) -> Result<impl Future<Output = Result<Vec<u8>>>, NodeError> {
		match self.client.upgrade() {
			None => Err(NodeError::ClientDropped.into()),
			Some(client) => {
				Ok(client
					.messenger
					.execute_remote_method(self.path.as_str(), method, data))
			}
		}
	}
	fn set_enabled(&self, enabled: bool) -> Result<(), NodeError> {
		self.send_remote_signal("setEnabled", &flexbuffers::singleton(enabled))
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

impl Drop for Node {
	fn drop(&mut self) {
		let _ = self.send_remote_signal("destroy", &[0; 0]);
	}
}
