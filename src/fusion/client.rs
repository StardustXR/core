use super::HandlerWrapper;
use super::{scenegraph::Scenegraph, spatial::Spatial};
use crate::flex::flexbuffer_from_vector_arguments;
use crate::{client, messenger::Messenger};
use anyhow::Result;
use erased_set::ErasedSyncSet;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::path::Path;
use std::sync::{Arc, Weak};
use tokio::net::UnixStream;
use tokio::sync::Notify;
use tokio::task::JoinHandle;

#[derive(Default)]
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

	root: OnceCell<Spatial>,
	hmd: OnceCell<Spatial>,
	pub(crate) item_uis: Mutex<ErasedSyncSet>,

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
			item_uis: Mutex::new(ErasedSyncSet::new()),

			elapsed_time: Mutex::new(0.0),
			life_cycle_handler: Mutex::new(Weak::<Mutex<LifeCycleHandlerDummy>>::new()),
		});
		let weak_client = Arc::downgrade(&client);
		let _ = client
			.root
			.set(Spatial::from_path(weak_client.clone(), "/").unwrap());
		let _ = client
			.hmd
			.set(Spatial::from_path(weak_client, "/hmd").unwrap());

		client
			.get_root()
			.node
			.send_remote_signal("subscribeLogicStep", &[0, 0])
			.map_err(|_| std::io::Error::from(std::io::ErrorKind::NotConnected))?;

		client.get_root().node.local_signals.insert(
			"logicStep".to_owned(),
			Box::new({
				let client = client.clone();
				move |data| {
					if let Some(handler) = client.life_cycle_handler.lock().upgrade() {
						let flex_vec = flexbuffers::Reader::get_root(data)?.get_vector()?;
						let delta = flex_vec.index(0)?.get_f64()?;
						let mut elapsed = client.elapsed_time.lock();
						(*elapsed) += delta;
						let info = LogicStepInfo {
							delta,
							elapsed: *elapsed,
						};
						handler.lock().logic_step(info);
					}
					Ok(())
				}
			}),
		);

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

	pub fn wrap_root<T: LifeCycleHandler>(&self, wrapped: T) -> HandlerWrapper<(), T> {
		let wrapper = HandlerWrapper::new((), move |_, _| wrapped);
		*self.life_cycle_handler.lock() = wrapper.weak_wrapped();
		wrapper
	}

	pub fn set_base_prefixes<T: AsRef<str>>(&self, prefixes: &[T]) -> Result<(), std::io::Error> {
		let flexbuffer = flexbuffer_from_vector_arguments(|fbb| {
			for prefix in prefixes {
				let prefix = prefix.as_ref();
				let path = Path::new(prefix);
				if path.is_absolute() && path.exists() {
					fbb.push(prefix);
				}
			}
		});

		self.messenger
			.send_remote_signal("/", "setBasePrefixes", &flexbuffer)
	}

	pub fn stop_loop(&self) {
		self.stop_notifier.notify_one();
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		self.stop_loop();
		let _ = self
			.messenger
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
