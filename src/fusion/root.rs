use super::{client::Client, spatial::Spatial};
use anyhow::{anyhow, Result};
use std::{ops::Deref, sync::Arc};
use tokio::sync::watch;

#[derive(Default)]
pub struct LogicStepInfo {
	delta: f64,
}

pub struct Root {
	pub spatial: Spatial,
	logic_step: watch::Sender<LogicStepInfo>,
}

impl Root {
	pub(crate) async fn new(client: &Client) -> Result<Arc<Self>> {
		let (logic_step, _) = watch::channel(LogicStepInfo::default());
		let root = Arc::new(Root {
			spatial: Spatial::from_path(&client, "/")?,
			logic_step,
		});
		root.spatial
			.node
			.send_remote_signal("subscribeLogicStep", &[0, 0])
			.await
			.map_err(|_| std::io::Error::from(std::io::ErrorKind::NotConnected))?;

		root.spatial.node.local_signals.insert(
			"logicStep".to_owned(),
			Box::new({
				let root = root.clone();
				move |data| {
					let flex_vec = flexbuffers::Reader::get_root(data)?.get_vector()?;
					let info = LogicStepInfo {
						delta: flex_vec.index(0)?.get_f64()?,
					};
					root.logic_step.send(info).map_err(|_| anyhow!("error"))
				}
			}),
		);

		Ok(root)
	}
}

impl Deref for Root {
	type Target = Spatial;
	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_client_life_cycle() -> anyhow::Result<()> {
	let (client, event_loop) = Client::connect_with_async_loop().await?;

	tokio::spawn({
		let mut logic_step = client.get_root().logic_step.subscribe();
		async move {
			while logic_step.changed().await.is_ok() {
				println!("Logic step delta is {}s", logic_step.borrow().delta);
			}
		}
	});

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}
