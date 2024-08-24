//! Your connection to the Stardust server and other essentials.

use crate::node::{NodeResult, NodeType};
use crate::root::{ClientState, Root, RootAspect};
use crate::{node::NodeError, scenegraph::Scenegraph};
use color_eyre::eyre::Result;
use global_counter::primitive::exact::CounterU64;
use stardust_xr::schemas::flex::flexbuffers::DeserializationError;
use stardust_xr::{
	client,
	messenger::{self, MessengerError},
	messenger::{MessageReceiver, MessageSender, MessageSenderHandle},
};
use std::sync::Arc;
use thiserror::Error;
use tokio::net::UnixStream;
use tokio::sync::{Notify, OnceCell};
use tokio::task::JoinHandle;

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

/// Your connection to the Stardust server.
pub struct Client {
	pub message_sender_handle: MessageSenderHandle,
	pub scenegraph: Arc<Scenegraph>,

	id_counter: CounterU64,
	stop_notifier: Notify,

	root: OnceCell<Root>,
	state: OnceCell<ClientState>,
}

impl Client {
	/// Try to connect to the server, return messenger halves for manually setting up the event loop.
	pub async fn connect() -> Result<(Arc<Self>, MessageSender, MessageReceiver), ClientError> {
		let connection = client::connect()
			.await
			.map_err(|_| ClientError::ConnectionFailure)?;
		Client::from_connection(connection).await
	}

	/// Create a client and messenger halves from an established tokio async `UnixStream` for manually setting up the event loop.
	pub async fn from_connection(
		connection: UnixStream,
	) -> Result<(Arc<Self>, MessageSender, MessageReceiver), ClientError> {
		let (message_tx, message_rx) = messenger::create(connection);
		let client = Arc::new(Client {
			scenegraph: Arc::new(Scenegraph::new()),
			message_sender_handle: message_tx.handle(),

			id_counter: CounterU64::new(u64::MAX / 2),
			stop_notifier: Default::default(),

			root: OnceCell::new(),
			state: OnceCell::new(),
		});
		let _ = client.root.set(Root::from_id(&client, 0, true));

		Ok((client, message_tx, message_rx))
	}

	/// Automatically set up the client with an async loop. This option is generally what you'll want to use.
	pub async fn connect_with_async_loop(
	) -> Result<(Arc<Self>, JoinHandle<Result<(), MessengerError>>), ClientError> {
		let (client, mut message_tx, mut message_rx) = Client::connect().await?;

		let event_loop = tokio::task::spawn({
			let client = client.clone();
			let scenegraph = client.scenegraph.clone();
			async move {
				let dispatch_loop = async move {
					loop {
						match message_rx.dispatch(&*scenegraph).await {
							Ok(_) => continue,
							Err(e) => break e,
						}
					}
				};
				let flush_loop = async move {
					loop {
						match message_tx.flush().await {
							Ok(_) => continue,
							Err(e) => break e,
						}
					}
				};

				tokio::select! {
					_ = client.stop_notifier.notified() => Ok(()),
					e = dispatch_loop => Err(e),
					e = flush_loop => Err(e),
				}
			}
		});
		let _ = client
			.state
			.set(client.get_root().get_state().await.unwrap_or_default());

		Ok((client, event_loop))
	}

	/// Get a reference to the client's root node, a spatial that exists where the client was spawned.
	pub fn get_root(&self) -> &Root {
		self.root.get().as_ref().unwrap()
	}
	pub fn get_state(&self) -> &ClientState {
		self.state.get().unwrap()
	}

	/// Set the prefixes for any `NamespacedResource`s.
	pub fn set_base_prefixes(&self, prefixes: &[&str]) -> NodeResult<()> {
		let mut prefixes = prefixes
			.iter()
			.map(ToString::to_string)
			.collect::<Vec<String>>();

		let env_prefixes = option_env!("STARDUST_RES_PREFIXES").map(ToString::to_string);
		if let Some(env_prefixes) = env_prefixes {
			for prefix in env_prefixes.split(':') {
				prefixes.push(prefix.to_string());
			}
		}

		self.get_root().set_base_prefixes(&prefixes)
	}

	pub fn generate_id(&self) -> u64 {
		self.id_counter.inc()
	}

	/// Stop the event loop if created with async loop. Equivalent to a graceful disconnect.
	pub fn stop_loop(&self) {
		self.stop_notifier.notify_one();
	}
}
impl Drop for Client {
	fn drop(&mut self) {
		let _ = self.get_root().disconnect();
		self.stop_loop();
	}
}

#[tokio::test]
async fn fusion_client_connect() {
	let (_client, event_loop) = Client::connect_with_async_loop().await.unwrap();

	tokio::select! {
		biased;
		_ = tokio::time::sleep(core::time::Duration::from_secs(1)) => (),
		e = event_loop => e.unwrap().unwrap(),
	}
}

#[tokio::test]
async fn fusion_client_life_cycle() {
	use crate::root::*;
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();

	struct RootHandlerDummy(Arc<Client>);
	impl RootHandler for RootHandlerDummy {
		fn frame(&mut self, _info: FrameInfo) {
			self.0.stop_loop();
		}
		fn save_state(&mut self) -> color_eyre::eyre::Result<ClientState> {
			Ok(ClientState::default())
		}
	}

	let _wrapper = client
		.get_root()
		.alias()
		.wrap(RootHandlerDummy(client.clone()))
		.unwrap();

	tokio::select! {
		biased;
		_ = tokio::time::sleep(core::time::Duration::from_secs(5)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	};
}
