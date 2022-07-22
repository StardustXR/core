use super::{scenegraph::Scenegraph, spatial::Spatial};
use crate::{client, messenger::Messenger};
use once_cell::sync::OnceCell;
use std::sync::{Arc, Weak};
use tokio::net::UnixStream;
use tokio::sync::Notify;
use tokio::task::JoinHandle;

pub struct Client {
	pub messenger: Arc<Messenger>,
	pub scenegraph: Scenegraph,

	stop_notifier: Notify,

	root: OnceCell<Spatial>,
	hmd: OnceCell<Spatial>,
}

impl Client {
	pub async fn connect() -> Result<Arc<Self>, std::io::Error> {
		let connection = client::connect().await?;
		Client::from_connection(connection).await
	}

	pub async fn from_connection(connection: UnixStream) -> Result<Arc<Self>, std::io::Error> {
		let client = Client {
			scenegraph: Scenegraph::new(),
			messenger: Arc::new(Messenger::new(connection)),

			stop_notifier: Default::default(),

			root: OnceCell::new(),
			hmd: OnceCell::new(),
		};

		let _ = client.root.set(Spatial::from_path(&client, "/").unwrap());
		let _ = client
			.root
			.set(Spatial::from_path(&client, "/hmd").unwrap());

		Ok(Arc::new(client))
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

	pub async fn stop_loop(&self) {
		self.stop_notifier.notify_one();
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		let _ = self.stop_loop();
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
