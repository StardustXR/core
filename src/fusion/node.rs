use super::{client::Client, HandlerWrapper};
use crate::flex;
use anyhow::Result;
use nanoid::nanoid;
use parking_lot::Mutex;
use std::{
	sync::{Arc, Weak},
	vec::Vec,
};
use thiserror::Error;

use core::hash::BuildHasherDefault;
use dashmap::DashMap;
use rustc_hash::FxHasher;

pub(crate) struct GenNodeInfo<'a> {
	pub(crate) client: Weak<Client>,
	pub(crate) parent_path: &'a str,
	pub(crate) interface_path: &'a str,
	pub(crate) interface_method: &'a str,
}
macro_rules! generate_node {
	($gen_node_info:expr, $($things_to_pass:expr),*) => {
		{
			let (node, id) = Node::generate_with_parent($gen_node_info.client, $gen_node_info.parent_path)?;
			node.client
				.upgrade()
				.ok_or(NodeError::ClientDropped)?
				.messenger
				.send_remote_signal(
					$gen_node_info.interface_path,
					$gen_node_info.interface_method,
					$crate::flex::flexbuffer_from_vector_arguments(|vec| {
						push_to_vec![vec, id.as_str(), $($things_to_pass),+]
					})
					.as_slice(),
				)
				.map_err(|_| NodeError::ServerCreationFailed)?;
				node
		}

	}
}

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
	pub fn get_name(&self) -> &str {
		&self.path[self.trailing_slash_pos + 1..]
	}
	pub fn get_path(&self) -> &str {
		self.path.as_str()
	}

	pub fn from_path(client: Weak<Client>, path: &str) -> Result<Arc<Self>, NodeError> {
		if !path.starts_with('/') {
			return Err(NodeError::InvalidPath);
		}
		let node = Node {
			path: path.to_string(),
			trailing_slash_pos: path.rfind('/').ok_or(NodeError::InvalidPath)?,
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

		Node::from_path(client, path.as_str()).map(|node| (node, id))
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
	pub fn send_remote_signal(&self, method: &str, data: &[u8]) -> Result<(), NodeError> {
		self.client
			.upgrade()
			.ok_or(NodeError::ClientDropped)?
			.messenger
			.send_remote_signal(self.path.as_str(), method, data)
			.map_err(|_| NodeError::MessengerWrite)
	}
	pub async fn execute_remote_method(&self, method: &str, data: &[u8]) -> Result<Vec<u8>> {
		match self.client.upgrade() {
			None => Err(NodeError::ClientDropped.into()),
			Some(client) => {
				client
					.messenger
					.execute_remote_method(self.path.as_str(), method, data)
					.await
			}
		}
	}
	fn set_enabled(&self, enabled: bool) -> Result<(), NodeError> {
		self.send_remote_signal(
			"setEnabled",
			flex::flexbuffer_from_arguments(|fbb| fbb.build_singleton(enabled)).as_slice(),
		)
	}
}

impl Drop for Node {
	fn drop(&mut self) {
		let _ = self.send_remote_signal("destroy", &[0; 0]);
	}
}
