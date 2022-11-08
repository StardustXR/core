use crate::{
	client::Client,
	node::{Node, NodeError},
	spatial::Spatial,
};
use std::{future::Future, sync::Arc};

#[derive(Debug)]
pub struct StartupSettings {
	pub(crate) node: Arc<Node>,
}
impl StartupSettings {
	pub fn create(client: &Arc<Client>) -> Result<Self, NodeError> {
		let (node, id) =
			Node::generate_with_parent(Arc::downgrade(client), "/startup/settings", true)?;
		client
			.message_sender_handle
			.signal(
				"/startup",
				"create_startup_settings",
				&flexbuffers::singleton(id.as_str()),
			)
			.map_err(|e| NodeError::MessengerError { e })?;
		Ok(StartupSettings { node })
	}

	pub fn set_root(&self, root: &Spatial) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("set_root", &root.node.get_path())
	}

	pub fn generate_desktop_startup_id(
		&self,
	) -> Result<impl Future<Output = anyhow::Result<String>>, NodeError> {
		self.node
			.execute_remote_method("generate_desktop_startup_id", &())
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
