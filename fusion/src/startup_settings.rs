use futures::Future;

use crate::{
	client::Client,
	node::{Node, NodeError},
	spatial::Spatial,
};
use std::sync::Arc;

pub struct StartupSettings {
	pub(crate) node: Arc<Node>,
}
impl StartupSettings {
	pub fn create(client: &Arc<Client>) -> Result<Self, NodeError> {
		let (node, id) = Node::generate_with_parent(Arc::downgrade(client), "/startup/settings")?;
		client.messenger.send_remote_signal(
			"/startup",
			"createStartupSettings",
			&flexbuffers::singleton(id.as_str()),
		);
		Ok(StartupSettings { node })
	}

	pub fn set_root(&self, root: &Spatial) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("setRoot", &flexbuffers::singleton(root.node.get_path()))
	}

	pub fn generate_desktop_startup_id(
		&self,
	) -> Result<impl Future<Output = anyhow::Result<String>>, NodeError> {
		self.node
			.execute_remote_method("generateDesktopStartupID", &())
	}
}

#[tokio::test]
async fn fusion_startup_settings() {
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let startup_settings =
		StartupSettings::create(&client).expect("Unable to create startup settings");
	startup_settings.set_root(client.get_root()).unwrap();
	println!(
		"{}",
		startup_settings
			.generate_desktop_startup_id()
			.unwrap()
			.await
			.unwrap()
	);
}
