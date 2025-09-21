//! Your connection to the Stardust server and other essentials.

use crate::node::NodeResult;
use crate::root::{Root, RootAspect};
use crate::{node::NodeError, scenegraph::Scenegraph};
use global_counter::primitive::exact::CounterU64;
use stardust_xr::schemas::flex::flexbuffers::DeserializationError;
use stardust_xr::{
	client,
	messenger::{self, MessengerError},
	messenger::{MessageReceiver, MessageSender, MessageSenderHandle},
};
use std::future::Future;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
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

#[macro_export]
macro_rules! project_local_resources {
	($relative_path:expr) => {
		std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join($relative_path)
	};
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

	pub fn setup_resources(&self, paths: &[&Path]) -> NodeResult<()> {
		let paths = paths.iter().map(|p| p.to_string_lossy().to_string());
		let runtime_prefixes = std::env::var("STARDUST_RES_PREFIXES").ok();
		let env_prefixes = runtime_prefixes
			.as_deref()
			.or(option_env!("STARDUST_RES_PREFIXES"))
			.into_iter()
			.flat_map(|f| f.split(':'))
			.map(|p| p.to_string());

		let prefixes = env_prefixes.chain(paths).collect::<Vec<String>>();
		self.get_root().set_base_prefixes(&prefixes)
	}

	pub async fn dispatch(&mut self) -> Result<(), MessengerError> {
		self.message_rx.dispatch(&*self.handle().scenegraph).await
	}
	/// this one will wait until there's some message to send
	pub async fn flush(&mut self) -> Result<(), MessengerError> {
		self.message_tx.flush().await
	}
	/// this one will try to send any messages if they're in the queue and if not return immediately
	pub async fn try_flush(&mut self) -> Result<(), MessengerError> {
		self.message_tx.try_flush().await
	}

	pub async fn await_method<O, F: Future<Output = O>>(
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

	pub async fn sync_event_loop<F: FnMut(&Arc<ClientHandle>, &mut ControlFlow)>(
		&mut self,
		mut f: F,
	) -> Result<(), MessengerError> {
		let mut flow = ControlFlow::Wait;
		let handle = self.handle();
		loop {
			self.try_flush().await?;
			match flow {
				ControlFlow::Poll => Ok(()),
				ControlFlow::Wait => self.dispatch().await,
				ControlFlow::WaitUntil(instant) => tokio::select! {
					_ = tokio::time::sleep_until(tokio::time::Instant::from_std(instant)) => Ok(()),
					r = self.dispatch() => r,
				},
				ControlFlow::Stop => break,
			}?;
			(f)(&handle, &mut flow);
		}
		Ok(())
	}
	pub fn async_event_loop(mut self) -> AsyncEventLoop {
		let client_handle = self.handle();
		let stop_notify = Arc::new(Notify::new());
		let wait_notify = Arc::new(Notify::new());
		let join_handle = tokio::spawn({
			let stop_notify = stop_notify.clone();
			let wait_notify = wait_notify.clone();
			async move {
				let scenegraph = self.internal.scenegraph.clone();
				loop {
					tokio::select! {
						r = self.message_tx.flush() => r?,
						r = self.message_rx.dispatch(&*scenegraph) => r?,
						_ = stop_notify.notified() => break,
					}
					wait_notify.notify_waiters();
				}
				Ok(Client {
					internal: self.internal,
					message_rx: self.message_rx,
					message_tx: self.message_tx,
				})
			}
		});
		AsyncEventLoop {
			client_handle,
			stop_notify,
			join_handle,
			wait_notify,
		}
	}
}

#[derive(Clone, Debug)]
pub struct AsyncEventHandle(Arc<Notify>);

impl AsyncEventHandle {
	pub async fn wait(&self) {
		self.0.notified().await
	}
}

pub struct AsyncEventLoop {
	pub client_handle: Arc<ClientHandle>,
	stop_notify: Arc<Notify>,
	join_handle: JoinHandle<Result<Client, MessengerError>>,
	wait_notify: Arc<Notify>,
}
impl AsyncEventLoop {
	pub async fn stop(self) -> Result<Client, MessengerError> {
		self.stop_notify.notify_waiters();
		self.join_handle
			.await
			.map_err(|e| MessengerError::IOError(e.into()))?
	}

	pub fn get_event_handle(&self) -> AsyncEventHandle {
		AsyncEventHandle(self.wait_notify.clone())
	}
}

#[derive(Debug, Clone, Copy)]
pub enum ControlFlow {
	Poll,
	Wait,
	WaitUntil(Instant),
	Stop,
}
impl ControlFlow {
	pub fn poll(&mut self) {
		*self = ControlFlow::Poll;
	}
	pub fn wait(&mut self) {
		*self = ControlFlow::Wait;
	}
	pub fn wait_until(&mut self, instant: Instant) {
		*self = ControlFlow::WaitUntil(instant);
	}
	pub fn stop(&mut self) {
		*self = ControlFlow::Stop;
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
		.sync_event_loop(|client, flow| {
			while let Some(event) = client.get_root().recv_root_event() {
				match event {
					RootEvent::Ping { response } => {
						response.send_ok(());
					}
					RootEvent::Frame { info: _ } => {
						println!("Got frame event");
						flow.stop();
					}
					RootEvent::SaveState { response } => response.send_ok(ClientState::default()),
				}
			}
		})
		.await
		.unwrap();
}
