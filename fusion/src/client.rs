//! Your connection to the Stardust server and other essentials.

use crate::node::{NodeResult, NodeType};
use crate::root::{Root, RootAspect};
use crate::{node::NodeError, scenegraph::Scenegraph};
use color_eyre::eyre::Result;
use global_counter::primitive::exact::CounterU64;
use stardust_xr::schemas::flex::flexbuffers::DeserializationError;
use stardust_xr::{
	client,
	messenger::{self, MessengerError},
	messenger::{MessageReceiver, MessageSender, MessageSenderHandle},
};
use std::future::Future;
use std::path::PathBuf;
use std::sync::{mpsc, Arc};
use thiserror::Error;
use tokio::net::UnixStream;
use tokio::sync::OnceCell;

#[derive(Error, Debug)]
pub enum ClientError {
	#[error("Could not connect to the stardust server")]
	ConnectionFailure,
	#[error("Node error: {0}")]
	NodeError(NodeError),
}
impl From<NodeError> for ClientError {
	fn from(e: NodeError) -> Self {
		ClientError::NodeError(e)
	}
}
impl From<MessengerError> for ClientError {
	fn from(e: MessengerError) -> Self {
		ClientError::NodeError(NodeError::MessengerError { e })
	}
}
impl From<String> for ClientError {
	fn from(e: String) -> Self {
		ClientError::NodeError(NodeError::ReturnedError { e })
	}
}
impl From<DeserializationError> for ClientError {
	fn from(e: DeserializationError) -> Self {
		ClientError::NodeError(NodeError::Deserialization { e })
	}
}

pub struct Client {
	internal: Arc<ClientHandle>,
	message_rx: MessageReceiver,
	message_tx: MessageSender,
}
impl Client {
	/// Try to connect to the server, return messenger halves for manually setting up the event loop.
	pub async fn connect() -> Result<Self, ClientError> {
		let connection = client::connect()
			.await
			.map_err(|_| ClientError::ConnectionFailure)?;
		let client = Client::from_connection(connection);
		client.setup_resources()?;
		Ok(client)
	}

	/// Create a client and messenger halves from an established tokio async `UnixStream` for manually setting up the event loop.
	pub fn from_connection(connection: UnixStream) -> Self {
		let (message_tx, message_rx) = messenger::create(connection);
		Client {
			internal: ClientHandle::new(&message_tx),
			message_rx,
			message_tx,
		}
	}

	pub fn handle(&self) -> Arc<ClientHandle> {
		self.internal.clone()
	}
	pub fn get_root(&self) -> &Root {
		self.internal.get_root()
	}

	fn setup_resources(&self) -> NodeResult<()> {
		let manifest_dir = option_env!("CARGO_MANIFEST_DIR")
			.map(PathBuf::from)
			.map(|p| p.join("res"))
			.filter(|p| p.exists())
			.map(|p| p.to_string_lossy().to_string())
			.into_iter();
		let env_prefixes = option_env!("STARDUST_RES_PREFIXES")
			.into_iter()
			.flat_map(|f| f.split(':'))
			.map(|p| p.to_string());

		let prefixes = manifest_dir.chain(env_prefixes).collect::<Vec<String>>();

		self.get_root().set_base_prefixes(&prefixes)
	}

	pub async fn dispatch(&mut self) -> Result<(), MessengerError> {
		self.message_rx.dispatch(&*self.handle().scenegraph).await
	}
	pub async fn flush(&mut self) -> Result<(), MessengerError> {
		self.message_tx.flush().await
	}
	pub async fn try_flush(&mut self) -> Result<(), MessengerError> {
		self.message_tx.try_flush().await
	}

	pub async fn with_event_loop<O, F: Future<Output = O>>(
		&mut self,
		f: F,
	) -> Result<O, MessengerError> {
		let dispatch_loop = async {
			loop {
				self.try_flush().await?;
				self.dispatch().await?;
			}
		};
		tokio::select! {
			e = dispatch_loop => e,
			v = f => Ok(v),
		}
	}
	pub async fn event_loop<F: FnMut(Arc<ClientHandle>, StopSender)>(
		&mut self,
		mut f: F,
	) -> Result<(), MessengerError> {
		let (stop_tx, stop_rx) = std::sync::mpsc::channel();
		let stop_sender = StopSender(stop_tx);
		while stop_rx.try_recv().is_err() {
			self.try_flush().await?;
			self.dispatch().await?;
			(f)(self.handle(), stop_sender.clone());
		}
		Ok(())
	}
}

#[derive(Clone)]
pub struct StopSender(mpsc::Sender<()>);
impl StopSender {
	pub fn stop(&self) {
		let _ = self.0.send(());
	}
}

/// Your connection to the Stardust server.
pub struct ClientHandle {
	pub message_sender_handle: MessageSenderHandle,
	pub scenegraph: Arc<Scenegraph>,
	id_counter: CounterU64,
	root: OnceCell<Root>,
}

impl ClientHandle {
	fn new(message_tx: &MessageSender) -> Arc<Self> {
		let client = Arc::new_cyclic(|client_ref| ClientHandle {
			scenegraph: Arc::new(Scenegraph::new(client_ref.clone())),
			message_sender_handle: message_tx.handle(),

			id_counter: CounterU64::new(u64::MAX / 2),

			root: OnceCell::new(),
		});
		let _ = client.root.set(Root::from_id(&client, 0, true));
		client
	}

	/// Get a reference to the client's root node, a spatial that exists where the client was spawned.
	pub fn get_root(&self) -> &Root {
		self.root.get().as_ref().unwrap()
	}

	pub fn generate_id(&self) -> u64 {
		self.id_counter.inc()
	}
}
impl Drop for ClientHandle {
	fn drop(&mut self) {
		let _ = self.get_root().disconnect();
	}
}

#[tokio::test]
async fn fusion_client_connect() {
	Client::connect().await.unwrap();
}

#[tokio::test]
async fn fusion_client_life_cycle() {
	use crate::root::*;
	let mut client = Client::connect().await.unwrap();
	tokio::task::spawn(async {
		tokio::time::sleep(core::time::Duration::from_secs(5)).await;
		panic!("Timed Out");
	});
	client
		.event_loop(|client, stop| {
			while let Some(event) = client.get_root().recv_event() {
				match event {
					RootEvent::Frame { info: _ } => {
						println!("Got frame event");
						stop.stop();
					}
					RootEvent::SaveState { response } => response.send(Ok(ClientState::default())),
				}
			}
		})
		.await
		.unwrap();
}
