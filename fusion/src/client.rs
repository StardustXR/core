use super::HandlerWrapper;
use super::{scenegraph::Scenegraph, spatial::Spatial};
use anyhow::Result;
use parking_lot::Mutex;
use serde::Deserialize;
use stardust_xr::schemas::flex::{deserialize, serialize};
use stardust_xr::{client, messenger::Messenger};
use std::any::TypeId;
use std::path::Path;
use std::sync::{Arc, Weak};
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

struct LifeCycleHandlerDummy;
impl LifeCycleHandler for LifeCycleHandlerDummy {
	fn logic_step(&mut self, info: LogicStepInfo) {
		println!("Logic step delta is {}s", info.delta);
	}
}

pub struct Root {
	pub spatial: Spatial,
}

pub struct Client {
	pub messenger: Messenger,
	pub scenegraph: Scenegraph,

	stop_notifier: Notify,

	root: OnceCell<Arc<Spatial>>,
	hmd: OnceCell<Spatial>,
	pub(crate) registered_item_uis: Mutex<Vec<TypeId>>,

	elapsed_time: Mutex<f64>,
	life_cycle_handler: Mutex<Weak<Mutex<dyn LifeCycleHandler>>>,
}

impl Client {
	pub async fn connect() -> Result<Arc<Self>, std::io::Error> {
		let connection = client::connect().await?;
		Client::from_connection(connection).await
	}

	pub async fn from_connection(connection: UnixStream) -> Result<Arc<Self>, std::io::Error> {
		let client = Arc::new(Client {
			scenegraph: Scenegraph::new(),
			messenger: Messenger::new(tokio::runtime::Handle::current(), connection),

			stop_notifier: Default::default(),

			root: OnceCell::new(),
			hmd: OnceCell::new(),
			registered_item_uis: Mutex::new(Vec::new()),

			elapsed_time: Mutex::new(0.0),
			life_cycle_handler: Mutex::new(Weak::<Mutex<LifeCycleHandlerDummy>>::new()),
		});
		let weak_client = Arc::downgrade(&client);
		let _ = client.root.set(Arc::new(
			Spatial::from_path(weak_client.clone(), "/").unwrap(),
		));
		let _ = client
			.hmd
			.set(Spatial::from_path(weak_client, "/hmd").unwrap());

		if let Ok(desktop_startup_id) = std::env::var("DESKTOP_STARTUP_ID") {
			client
				.get_root()
				.node
				.send_remote_signal("applyDesktopStartupID", &desktop_startup_id)
				.unwrap();
		}

		client.get_root().node.local_signals.lock().insert(
			"logicStep".to_owned(),
			Arc::new({
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
			}),
		);

		client
			.get_root()
			.node
			.send_remote_signal("subscribeLogicStep", &[0, 0])
			.map_err(|_| std::io::Error::from(std::io::ErrorKind::NotConnected))?;

		Ok(client)
	}

	pub async fn connect_with_async_loop() -> Result<(Arc<Self>, JoinHandle<()>), std::io::Error> {
		let client = Client::connect().await?;

		let event_loop = tokio::task::spawn({
			let client = client.clone();
			async move {
				let dispatch_loop = async { while client.dispatch().await.is_ok() {} };
				let flush_loop = async { while client.flush().await.is_ok() {} };

				tokio::select! {
					_ = client.stop_notifier.notified() => (),
					_ = dispatch_loop => (),
					_ = flush_loop => (),
				}
				println!("Stopped the loop");
			}
		});

		Ok((client, event_loop))
	}

	pub async fn dispatch(&self) -> Result<(), std::io::Error> {
		self.messenger.dispatch(&self.scenegraph).await
	}

	pub async fn flush(&self) -> Result<(), std::io::Error> {
		self.messenger.flush().await
	}

	pub fn get_root(&self) -> &Spatial {
		self.root.get().as_ref().unwrap()
	}
	pub fn get_hmd(&self) -> &Spatial {
		self.hmd.get().as_ref().unwrap()
	}

	pub fn wrap_root<T: LifeCycleHandler>(&self, wrapped: T) -> HandlerWrapper<Spatial, T> {
		let wrapper = HandlerWrapper::new(
			Spatial {
				node: self.root.get().unwrap().node.clone(),
			},
			move |_, _, _| wrapped,
		);
		*self.life_cycle_handler.lock() = wrapper.weak_wrapped();
		wrapper
	}

	pub fn set_base_prefixes<T: AsRef<str>>(&self, prefixes: &[T]) {
		let prefixes: Vec<&Path> = prefixes
			.iter()
			.map(|p| Path::new(p.as_ref()))
			.filter(|p| p.is_absolute() && p.exists())
			.collect();

		self.messenger
			.send_remote_signal("/", "setBasePrefixes", &serialize(prefixes).unwrap());
	}

	pub fn stop_loop(&self) {
		self.stop_notifier.notify_one();
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		self.stop_loop();
		self.messenger
			.send_remote_signal("/", "disconnect", &[0_u8; 0]);
	}
}

#[tokio::test]
async fn fusion_client_connect() -> anyhow::Result<()> {
	let (client, event_loop) = Client::connect_with_async_loop().await?;

	tokio::select! {
		biased;
		_ = tokio::time::sleep(core::time::Duration::from_secs(1)) => (),
		_ = event_loop => (),
	};
	drop(client);
	Ok(())
}

#[tokio::test]
async fn fusion_client_life_cycle() -> anyhow::Result<()> {
	let (client, event_loop) = Client::connect_with_async_loop().await?;

	let _wrapper = client.wrap_root(LifeCycleHandlerDummy);

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}
