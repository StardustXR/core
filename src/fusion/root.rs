use super::{client::Client, spatial::Spatial, HandlerWrapper};
use anyhow::Result;
use std::{
	ops::Deref,
	sync::{Arc, Weak},
};

#[derive(Default)]
pub struct LogicStepInfo {
	delta: f64,
}

pub trait RootHandler: Send + Sync + 'static {
	fn logic_step(&self, _info: LogicStepInfo);
}

pub struct Root {
	pub spatial: Spatial,
	handler: HandlerWrapper<dyn RootHandler>,
}

impl Root {
	pub(crate) async fn new(client: &Client) -> Result<Arc<Self>> {
		let root = Arc::new(Root {
			spatial: Spatial::from_path(&client, "/")?,
			handler: HandlerWrapper::new(),
		});
		root.spatial
			.node
			.send_remote_signal("subscribeLogicStep", &[0, 0])
			.await
			.map_err(|_| std::io::Error::from(std::io::ErrorKind::NotConnected))?;

		root.spatial.node.local_signals.insert(
			"logicStep".to_owned(),
			Box::new({
				let handler = root.handler.clone();
				move |data| {
					handler
						.handle(|handler| -> Result<()> {
							let flex_vec = flexbuffers::Reader::get_root(data)?.get_vector()?;
							let info = LogicStepInfo {
								delta: flex_vec.index(0)?.get_f64()?,
							};
							handler.logic_step(info);
							Ok(())
						})
						.transpose()?;
					Ok(())
				}
			}),
		);

		Ok(root)
	}

	pub fn set_handler<T: RootHandler>(&self, handler: &Arc<T>) {
		self.handler
			.set_handler(Arc::downgrade(handler) as Weak<dyn RootHandler>)
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

	struct LifeCycle {
		root: Arc<Root>,
	}
	// impl Handler for LifeCycle {}
	impl RootHandler for LifeCycle {
		fn logic_step(&self, info: LogicStepInfo) {
			println!("Logic step delta is {}s", info.delta);
		}
	}

	let life_cycle = Arc::new(LifeCycle {
		root: client.get_root().clone(),
	});
	life_cycle.root.set_handler(&life_cycle);

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}
