//! Node registry for efficient event handling with broadcast channels.
//! Replaces the old Scenegraph with a more efficient single-lookup system.

use crate::client::ClientHandle;
use dashmap::DashMap;
use stardust_xr::{
	messenger::MethodResponse,
	scenegraph::{self, ScenegraphError},
};
use std::{
	any::Any,
	os::fd::OwnedFd,
	sync::{Arc, Weak},
};
use tokio::sync::mpsc;

/// EventParser trait for compatibility with existing codegen
/// This allows the generated code to work with NodeRegistry
pub trait EventParser: Sized + Send + Sync + 'static {
	const ASPECT_ID: u64;

	fn parse_signal(
		_client: &Arc<ClientHandle>,
		signal_id: u64,
		_data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> Result<Self, ScenegraphError>;
	fn parse_method(
		client: &Arc<ClientHandle>,
		signal_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
		response: MethodResponse,
	) -> Result<Self, ScenegraphError>;
}

trait EventSender: Any + Send + Sync + 'static {
	fn send_signal(
		&self,
		client: &Arc<ClientHandle>,
		signal_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	);
	fn execute_method(
		&self,
		client: &Arc<ClientHandle>,
		method_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
		response: MethodResponse,
	);
}

struct Sender<Event: EventParser>(mpsc::UnboundedSender<Event>);
impl<Event: EventParser> EventSender for Sender<Event> {
	fn send_signal(
		&self,
		client: &Arc<ClientHandle>,
		signal_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) {
		if let Ok(parsed) = Event::parse_signal(client, signal_id, data, fds) {
			_ = self.0.send(parsed);
		}
	}

	fn execute_method(
		&self,
		client: &Arc<ClientHandle>,
		method_id: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
		response: MethodResponse,
	) {
		if let Ok(parsed) = Event::parse_method(client, method_id, data, fds, response) {
			_ = self.0.send(parsed);
		}
	}
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct MemberInfo {
	node_id: u64,
	aspect_id: u64,
}

pub struct NodeRegistry {
	client: Weak<ClientHandle>,
	aspects: DashMap<MemberInfo, Box<dyn EventSender>>,
}

impl NodeRegistry {
	pub fn new(client: Weak<ClientHandle>) -> Self {
		Self {
			client,
			aspects: DashMap::new(),
		}
	}

	pub fn add_aspect<E: EventParser>(
		&self,
		node_id: u64,
		aspect_id: u64,
	) -> mpsc::UnboundedReceiver<E> {
		let (tx, rx) = mpsc::unbounded_channel(); // Reasonable buffer size
		self.aspects
			.insert(MemberInfo { node_id, aspect_id }, Box::new(Sender(tx)));
		rx
	}

	pub fn remove_node(&self, node_id: u64) {
		self.aspects.retain(|info, _| info.node_id != node_id);
	}
}
impl scenegraph::Scenegraph for NodeRegistry {
	fn send_signal(
		&self,
		id: u64,
		aspect: u64,
		signal: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), ScenegraphError> {
		let Some(client) = self.client.upgrade() else {
			return Err(ScenegraphError::InternalError(
				"Client not found".to_string(),
			));
		};

		let aspect = self
			.aspects
			.get(&MemberInfo {
				node_id: id,
				aspect_id: aspect,
			})
			.ok_or(ScenegraphError::AspectNotFound)?;
		aspect.value().send_signal(&client, signal, data, fds);
		Ok(())
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
		let Some(client) = self.client.upgrade() else {
			response.send(Err(ScenegraphError::InternalError(
				"Client not found".to_string(),
			)));
			return;
		};

		let Some(aspect) = self.aspects.get(&MemberInfo {
			node_id: id,
			aspect_id: aspect,
		}) else {
			response.send(Err(ScenegraphError::AspectNotFound));
			return;
		};
		aspect
			.value()
			.execute_method(&client, method, data, fds, response);
	}
}
