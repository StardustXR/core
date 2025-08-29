use dashmap::DashMap;
use parking_lot::Mutex;
use stardust_xr::{
	messenger::MethodResponse,
	scenegraph::{self, ScenegraphError},
};
use std::{
	os::fd::OwnedFd,
	sync::{Arc, Weak},
};
use tokio::sync::mpsc;

use crate::{client::ClientHandle, node::Node};

pub(crate) trait EventParser: Sized + Send + Sync + 'static {
	const ASPECT_ID: u64;
	fn serialize_signal(
		_client: &Arc<crate::client::ClientHandle>,
		signal_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<Self, ScenegraphError>;
	fn serialize_method(
		_client: &Arc<crate::client::ClientHandle>,
		method_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
		response: MethodResponse,
	) -> Option<Self>;
}

trait EventSender: Send + Sync + 'static {
	fn send_signal(
		&self,
		client: &Arc<crate::client::ClientHandle>,
		signal_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), ScenegraphError>;
	fn send_method(
		&self,
		client: &Arc<crate::client::ClientHandle>,
		method_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
		response: MethodResponse,
	);
}
struct EventSenderWrapper<E: EventParser>(mpsc::UnboundedSender<E>);
impl<E: EventParser> EventSender for EventSenderWrapper<E> {
	fn send_signal(
		&self,
		client: &Arc<crate::client::ClientHandle>,
		signal_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), ScenegraphError> {
		let _ = self
			.0
			.send(E::serialize_signal(client, signal_id, data, fds)?);
		Ok(())
	}
	fn send_method(
		&self,
		client: &Arc<crate::client::ClientHandle>,
		method_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
		response: MethodResponse,
	) {
		if let Some(event) = E::serialize_method(client, method_id, data, fds, response) {
			let _ = self.0.send(event);
		}
	}
}

/// Scenegraph full of aliases to nodes, needed so the `Messenger` can send messages to nodes.
pub struct Scenegraph {
	client_ref: Weak<ClientHandle>,
	nodes: DashMap<u64, DashMap<u64, Box<dyn EventSender>>>,
}
impl Scenegraph {
	pub(crate) fn new(client_ref: Weak<ClientHandle>) -> Self {
		Scenegraph {
			client_ref,
			nodes: Default::default(),
		}
	}
	pub(crate) fn add_aspect<E: EventParser>(&self, node: &Node) {
		let (sender, receiver) = mpsc::unbounded_channel::<E>();
		let scenegraph_node = self.nodes.entry(node.id).or_default();
		scenegraph_node
			.entry(E::ASPECT_ID)
			.or_insert_with(|| Box::new(EventSenderWrapper(sender)));
		node.aspects
			.entry(E::ASPECT_ID)
			.insert(Mutex::new(Box::new(receiver)));
	}
	pub(crate) fn remove_node(&self, id: u64) {
		self.nodes.remove(&id);
	}
}

impl scenegraph::Scenegraph for Scenegraph {
	fn send_signal(
		&self,
		id: u64,
		aspect: u64,
		signal: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), ScenegraphError> {
		let node = self.nodes.get(&id).ok_or(ScenegraphError::NodeNotFound)?;
		let aspect = node.get(&aspect).ok_or(ScenegraphError::AspectNotFound)?;
		aspect.send_signal(&self.client_ref.upgrade().unwrap(), signal, data, fds)
	}
	fn execute_method(
		&self,
		id: u64,
		aspect: u64,
		method: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
		response: MethodResponse,
	) {
		let Some(node) = self.nodes.get(&id) else {
			response.send(Err(ScenegraphError::NodeNotFound));
			return;
		};
		let Some(aspect) = node.get(&aspect) else {
			response.send(Err(ScenegraphError::AspectNotFound));
			return;
		};
		aspect.send_method(
			&self.client_ref.upgrade().unwrap(),
			method,
			data,
			fds,
			response,
		);
	}
}
