//! Your connection to the Stardust server and other essentials.

use crate::node::NodeError;
use crate::node::NodeInternals;
use crate::spatial::Spatial;

use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use stardust_xr::scenegraph;
use stardust_xr::scenegraph::ScenegraphError;
use stardust_xr::{
	client,
	messenger::{self, MessengerError},
	messenger::{MessageReceiver, MessageSender, MessageSenderHandle},
	schemas::flex::{deserialize, serialize},
};
use std::any::TypeId;
use std::future::Future;
use std::os::unix::io::RawFd;
use std::path::Path;
use std::sync::{Arc, Weak};
use thiserror::Error;
use tokio::net::UnixStream;
use tokio::sync::{Notify, OnceCell};
use tokio::task::JoinHandle;

/// Scenegraph full of aliases to nodes, needed so the `Messenger` can send messages to nodes.
#[derive(Default)]
pub struct Scenegraph {
	nodes: Mutex<FxHashMap<String, Weak<NodeInternals>>>,
}

impl Scenegraph {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn add_node(&self, node_internals: &Arc<NodeInternals>) {
		self.nodes
			.lock()
			.insert(node_internals.path(), Arc::downgrade(&node_internals));
	}

	pub fn remove_node(&self, node_path: &str) {
		self.nodes.lock().remove(node_path);
	}

	// pub fn get_node(&self, path: &str) -> Option<Node> {
	// 	self.nodes.lock().get(path).cloned().unwrap_or_default()
	// }
}

impl scenegraph::Scenegraph for Scenegraph {
	fn send_signal(&self, path: &str, method: &str, data: &[u8], fds: Vec<RawFd>) -> Result<(), ScenegraphError> {
		let node = self
			.nodes
			.lock()
			.get(path)
			.and_then(Weak::upgrade)
			.ok_or(ScenegraphError::NodeNotFound)?;
		let local_signals = node.local_signals.lock();
		let signal = local_signals
			.get(method)
			.ok_or(ScenegraphError::SignalNotFound)?
			.clone();
		signal(data).map_err(|e| ScenegraphError::SignalError {
			error: e.to_string(),
		})
	}
	fn execute_method(
		&self,
		path: &str,
		method: &str,
		data: &[u8],
		fds: Vec<RawFd>,
	) -> Result<Vec<u8>, ScenegraphError> {
		let node = self
			.nodes
			.lock()
			.get(path)
			.and_then(Weak::upgrade)
			.ok_or(ScenegraphError::NodeNotFound)?;
		let local_methods = node.local_methods.lock();
		let method = local_methods
			.get(method)
			.ok_or(ScenegraphError::MethodNotFound)?
			.clone();
		method(data).map_err(|e| ScenegraphError::MethodError {
			error: e.to_string(),
		})
	}
}

struct DummyHandler;
impl RootHandler for DummyHandler {
	fn frame(&mut self, _info: FrameInfo) {}
}

#[derive(Deserialize)]
struct LogicStepInfoInternal {
	delta: f64,
}
/// Information on the frame.
#[derive(Debug, Default, Clone, Copy)]
pub struct FrameInfo {
	/// The time between this frame and last frame's display time, in seconds.
	pub delta: f64,
	/// The total time in seconds the client has been connected to the server.
	pub elapsed: f64,
}

/// Handle the events that apply to the whole client.
pub trait RootHandler: Send + Sync + 'static {
	/// Runs every frame with information about the current frame, for animations and motion and a consistent update.
	fn frame(&mut self, _info: FrameInfo);
}

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

/// Your connection to the Stardust server.
pub struct Client {
	/// A handle to the messenger, allows you to send messages to nodes on the server.
	pub message_sender_handle: MessageSenderHandle,
	/// A reference to the scenegraph.
	pub scenegraph: Arc<Scenegraph>,

	stop_notifier: Notify,

	root: OnceCell<Arc<Spatial>>,
	hmd: OnceCell<Spatial>,
	pub(crate) registered_item_uis: Mutex<Vec<TypeId>>,

	elapsed_time: Mutex<f64>,
	life_cycle_handler: Mutex<Weak<Mutex<dyn RootHandler>>>,
}

impl Client {
	/// Try to connect to the server, return messenger halves for manually setting up the event loop.
	pub async fn connect() -> Result<(Arc<Self>, MessageSender, MessageReceiver), ClientError> {
		let connection = client::connect()
			.await
			.map_err(|_| ClientError::ConnectionFailure)?;
		Ok(Client::from_connection(connection))
	}

	/// Create a client and messenger halves from an established tokio async `UnixStream` for manually setting up the event loop.
	pub fn from_connection(connection: UnixStream) -> (Arc<Self>, MessageSender, MessageReceiver) {
		let (message_tx, message_rx) = messenger::create(connection);
		let client = Arc::new(Client {
			scenegraph: Arc::new(Scenegraph::new()),
			message_sender_handle: message_tx.handle(),

			stop_notifier: Default::default(),

			root: OnceCell::new(),
			hmd: OnceCell::new(),
			registered_item_uis: Mutex::new(Vec::new()),

			elapsed_time: Mutex::new(0.0),
			life_cycle_handler: Mutex::new(Weak::<Mutex<DummyHandler>>::new()),
		});

		(client, message_tx, message_rx)
	}

	/// Set up the client's root, HMD, and dummy handler when manually setting up the event loop.
	pub fn setup(client: &Arc<Client>) -> Result<(), ClientError> {
		let _ = client
			.root
			.set(Arc::new(Spatial::from_path(client, "", "", false)));
		let _ = client.hmd.set(Spatial::from_path(client, "", "hmd", false));

		client.get_root().node.add_local_signal("frame", {
			let client = client.clone();
			move |data| {
				if let Some(handler) = client.life_cycle_handler.lock().upgrade() {
					let info_internal: LogicStepInfoInternal = deserialize(data)?;
					let mut elapsed = client.elapsed_time.lock();
					(*elapsed) += info_internal.delta;
					let info = FrameInfo {
						delta: info_internal.delta,
						elapsed: *elapsed,
					};
					handler.lock().frame(info);
				}
				Ok(())
			}
		})?;
		Ok(())
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
		Client::setup(&client)?;

		Ok((client, event_loop))
	}

	/// Get a reference to the client's root node, a spatial that exists where the client was spawned.
	pub fn get_root(&self) -> &Spatial {
		self.root.get().as_ref().unwrap()
	}
	/// Get a reference to the head mounted display's spatial.
	pub fn get_hmd(&self) -> &Spatial {
		self.hmd.get().as_ref().unwrap()
	}

	/// Wrap the root in a handler and return an `Arc` to the handler.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler, and you want to check for errors too"]
	pub fn wrap_root<H: RootHandler>(&self, wrapped: H) -> Result<Arc<Mutex<H>>, NodeError> {
		self.get_root()
			.node
			.send_remote_signal_raw("subscribe_frame", &[])?;
		let wrapped = Arc::new(Mutex::new(wrapped));
		*self.life_cycle_handler.lock() =
			Arc::downgrade(&(wrapped.clone() as Arc<Mutex<dyn RootHandler>>));
		Ok(wrapped)
	}
	/// Wrap the root in an already wrapped handler
	pub fn wrap_root_raw<H: RootHandler>(&self, wrapped: &Arc<Mutex<H>>) -> Result<(), NodeError> {
		self.get_root()
			.node
			.send_remote_signal_raw("subscribe_frame", &[])?;
		*self.life_cycle_handler.lock() =
			Arc::downgrade(&(wrapped.clone() as Arc<Mutex<dyn RootHandler>>));
		Ok(())
	}

	/// Set the prefixes for any `NamespacedResource`s.
	pub fn set_base_prefixes<H: AsRef<Path>>(&self, prefixes: &[H]) {
		let env_prefixes = option_env!("STARDUST_RES_PREFIXES");

		let prefix_vec: Vec<&Path> = if let Some(env_prefixes) = env_prefixes {
			env_prefixes.split(":").map(|p| Path::new(p)).collect()
		} else {
			prefixes
				.iter()
				.map(|p| p.as_ref())
				.filter(|p| p.is_absolute() && p.exists())
				.collect()
		};

		self.message_sender_handle
			.signal("/", "set_base_prefixes", &serialize(prefix_vec).unwrap(), &[])
			.unwrap();
	}

	pub fn get_connection_environment(
		&self,
	) -> Result<impl Future<Output = Result<FxHashMap<String, String>, NodeError>>, NodeError> {
		let future = self
			.message_sender_handle
			.method("/startup", "get_connection_environment", &[], &[])
			.map_err(|e| NodeError::MessengerError { e })?;

		Ok(async move {
			let result = future.await.map_err(|e| NodeError::ReturnedError { e })?;
			deserialize(&result).map_err(|e| NodeError::Deserialization { e })
		})
	}

	/// Stop the event loop if created with async loop. Equivalent to a graceful disconnect.
	pub fn stop_loop(&self) {
		self.stop_notifier.notify_one();
	}
}
impl Drop for Client {
	fn drop(&mut self) {
		self.stop_loop();
		let _ = self
			.message_sender_handle
			.signal("/", "disconnect", &[0_u8; 0], &[]);
	}
}

#[tokio::test]
async fn fusion_client_connect() {
	color_eyre::install().unwrap();
	let (_client, event_loop) = Client::connect_with_async_loop().await.unwrap();

	tokio::select! {
		biased;
		_ = tokio::time::sleep(core::time::Duration::from_secs(1)) => (),
		e = event_loop => e.unwrap().unwrap(),
	}
}

#[tokio::test]
async fn fusion_client_life_cycle() {
	color_eyre::install().unwrap();
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();

	struct RootHandlerDummy(Arc<Client>);
	impl RootHandler for RootHandlerDummy {
		fn frame(&mut self, _info: FrameInfo) {
			self.0.stop_loop();
		}
	}

	let _wrapper = client.wrap_root(RootHandlerDummy(client.clone())).unwrap();

	tokio::select! {
		biased;
		_ = tokio::time::sleep(core::time::Duration::from_secs(5)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	};
}
