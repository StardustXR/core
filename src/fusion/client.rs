use super::HandlerWrapper;
use super::{scenegraph::Scenegraph, spatial::Spatial};
use crate::flex::flexbuffer_from_vector_arguments;
use crate::{client, messenger::Messenger};
use anyhow::Result;
use async_trait::async_trait;
use once_cell::sync::OnceCell;
use std::path::Path;
use std::sync::{Arc, Weak};
use tokio::net::UnixStream;
use tokio::sync::Notify;
use tokio::task::JoinHandle;

#[derive(Default)]
pub struct LogicStepInfo {
	pub delta: f64,
}

#[async_trait]
pub trait LifeCycleHandler: Send + Sync + 'static {
	async fn logic_step(&self, _info: LogicStepInfo);
}

pub struct Root {
	pub spatial: Spatial,
}

pub struct Client {
	pub messenger: Arc<Messenger>,
	pub scenegraph: Scenegraph,

	stop_notifier: Notify,

	root: OnceCell<Spatial>,
	hmd: OnceCell<Spatial>,

	life_cycle_handler: HandlerWrapper<dyn LifeCycleHandler>,
}

impl Client {
	pub async fn connect() -> Result<Arc<Self>, std::io::Error> {
		let connection = client::connect().await?;
		Client::from_connection(connection).await
	}

	pub async fn from_connection(connection: UnixStream) -> Result<Arc<Self>, std::io::Error> {
		let client = Arc::new(Client {
			scenegraph: Scenegraph::new(),
			messenger: Arc::new(Messenger::new(connection)),

			stop_notifier: Default::default(),

			root: OnceCell::new(),
			hmd: OnceCell::new(),

			life_cycle_handler: HandlerWrapper::new(),
		});
		let _ = client.root.set(Spatial::from_path(&client, "/").unwrap());
		let _ = client.hmd.set(Spatial::from_path(&client, "/hmd").unwrap());

		client
			.get_root()
			.node
			.send_remote_signal("subscribeLogicStep", &[0, 0])
			.await
			.map_err(|_| std::io::Error::from(std::io::ErrorKind::NotConnected))?;

		client.get_root().node.local_signals.insert(
			"logicStep".to_owned(),
			Box::new({
				let handler = client.life_cycle_handler.clone();
				move |data| {
					handler
						.handle(|handler| -> Result<()> {
							let flex_vec = flexbuffers::Reader::get_root(data)?.get_vector()?;
							let info = LogicStepInfo {
								delta: flex_vec.index(0)?.get_f64()?,
							};
							tokio::task::spawn(async move { handler.logic_step(info).await });
							Ok(())
						})
						.transpose()?;
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
				let event_loop = async { while client.dispatch().await.is_ok() {} };

				tokio::select! {
					_ = client.stop_notifier.notified() => (),
					_ = event_loop => (),
				}
				println!("Stopped the loop");
			}
		});

		Ok((client, event_loop))
	}

	pub async fn dispatch(&self) -> Result<(), std::io::Error> {
		self.messenger.dispatch(&self.scenegraph).await
	}

	pub fn get_weak_messenger(&self) -> Weak<Messenger> {
		Arc::downgrade(&self.messenger)
	}

	pub fn get_root(&self) -> &Spatial {
		self.root.get().as_ref().unwrap()
	}
	pub fn get_hmd(&self) -> &Spatial {
		self.hmd.get().as_ref().unwrap()
	}

	pub fn set_life_cycle_handler<T: LifeCycleHandler>(&self, handler: &Arc<T>) {
		self.life_cycle_handler
			.set_handler(Arc::downgrade(handler) as Weak<dyn LifeCycleHandler>)
	}

	pub async fn set_base_prefixes<T: AsRef<str>>(
		&self,
		prefixes: &[T],
	) -> Result<(), std::io::Error> {
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
			.await
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

	struct LifeCycle;
	#[async_trait]
	impl LifeCycleHandler for LifeCycle {
		async fn logic_step(&self, info: LogicStepInfo) {
			println!("Logic step delta is {}s", info.delta);
		}
	}

	let life_cycle = Arc::new(LifeCycle);
	client.set_life_cycle_handler(&life_cycle);

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}
