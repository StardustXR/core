use crate::node::NodeError;
use crate::DummyHandler;

use super::{scenegraph::Scenegraph, spatial::Spatial};
use anyhow::Result;
use parking_lot::Mutex;
use serde::Deserialize;
use stardust_xr::messenger::{self, MessengerError};
use stardust_xr::schemas::flex::{deserialize, serialize};
use stardust_xr::{
	client,
	messenger::{MessageReceiver, MessageSender, MessageSenderHandle},
};
use std::any::TypeId;
use std::path::Path;
use std::sync::{Arc, Weak};
use thiserror::Error;
use tokio::net::UnixStream;
use tokio::sync::{Notify, OnceCell};
use tokio::task::JoinHandle;

#[derive(Deserialize)]
struct LogicStepInfoInternal {
	delta: f64,
}
#[derive(Debug, Default, Clone, Copy)]
pub struct LogicStepInfo {
	pub delta: f64,
	pub elapsed: f64,
}

pub trait LifeCycleHandler: Send + Sync + 'static {
	fn logic_step(&mut self, _info: LogicStepInfo);
}

pub struct Root {
	pub spatial: Spatial,
}

#[derive(Error, Debug)]
pub enum ClientError {
	#[error("Could not connect to the stardust server")]
	ConnectionFailure,
	#[error("Node error: {e}")]
	NodeError { e: NodeError },
}

pub struct Client {
	pub message_sender_handle: MessageSenderHandle,
	pub scenegraph: Arc<Scenegraph>,

	stop_notifier: Notify,

	root: OnceCell<Arc<Spatial>>,
	hmd: OnceCell<Spatial>,
	pub(crate) registered_item_uis: Mutex<Vec<TypeId>>,

	elapsed_time: Mutex<f64>,
	life_cycle_handler: Mutex<Weak<Mutex<dyn LifeCycleHandler>>>,
}

impl Client {
	pub async fn connect() -> Result<(Arc<Self>, MessageSender, MessageReceiver), ClientError> {
		let connection = client::connect()
			.await
			.map_err(|_| ClientError::ConnectionFailure)?;
		Ok(Client::from_connection(connection))
	}

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

	pub fn setup(client: &Arc<Client>) -> Result<(), ClientError> {
		let _ = client
			.root
			.set(Arc::new(Spatial::from_path(client, "", "", false)));
		let _ = client.hmd.set(Spatial::from_path(client, "", "hmd", false));

		if let Ok(desktop_startup_id) = std::env::var("DESKTOP_STARTUP_ID") {
			client
				.get_root()
				.node
				.send_remote_signal("apply_desktop_startup_id", &desktop_startup_id)
				.unwrap();
		}

		client
			.get_root()
			.node
			.add_local_signal("logic_step", {
				let client = client.clone();
				move |data| {
					if let Some(handler) = client.life_cycle_handler.lock().upgrade() {
						let info_internal: LogicStepInfoInternal = deserialize(data)?;
						let mut elapsed = client.elapsed_time.lock();
						(*elapsed) += info_internal.delta;
						let info = LogicStepInfo {
							delta: info_internal.delta,
							elapsed: *elapsed,
						};
						handler.lock().logic_step(info);
					}
					Ok(())
				}
			})
			.unwrap();

		client
			.get_root()
			.node
			.send_remote_signal_raw("subscribe_logic_step", &[])
			.map_err(|e| ClientError::NodeError { e })?;
		Ok(())
	}

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

	pub fn get_root(&self) -> &Spatial {
		self.root.get().as_ref().unwrap()
	}
	pub fn get_hmd(&self) -> &Spatial {
		self.hmd.get().as_ref().unwrap()
	}

	pub fn wrap_root<H: LifeCycleHandler>(&self, wrapped: H) -> Arc<Mutex<H>> {
		let wrapped = Arc::new(Mutex::new(wrapped));
		*self.life_cycle_handler.lock() =
			Arc::downgrade(&(wrapped.clone() as Arc<Mutex<dyn LifeCycleHandler>>));
		wrapped
	}

	pub fn set_base_prefixes<H: AsRef<str>>(&self, prefixes: &[H]) {
		let prefixes: Vec<&Path> = prefixes
			.iter()
			.map(|p| Path::new(p.as_ref()))
			.filter(|p| p.is_absolute() && p.exists())
			.collect();

		self.message_sender_handle
			.signal("/", "set_base_prefixes", &serialize(prefixes).unwrap())
			.unwrap();
	}

	pub fn stop_loop(&self) {
		self.stop_notifier.notify_one();
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		self.stop_loop();
		let _ = self
			.message_sender_handle
			.signal("/", "disconnect", &[0_u8; 0]);
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
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();

	struct LifeCycleHandlerDummy(Arc<Client>);
	impl LifeCycleHandler for LifeCycleHandlerDummy {
		fn logic_step(&mut self, _info: LogicStepInfo) {
			self.0.stop_loop();
		}
	}

	let _wrapper = client.wrap_root(LifeCycleHandlerDummy(client.clone()));

	tokio::select! {
		biased;
		_ = tokio::time::sleep(core::time::Duration::from_secs(5)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	};
}
