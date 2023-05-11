//! A collection of utilities for launching new 2D or 3D clients with a sense of space and control over them.

use crate::{
	client::Client,
	items::{Item, ItemAcceptor},
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
};
use nanoid::nanoid;
use std::{future::Future, ops::Deref, sync::Arc};

/// A node to generate startup IDs for launching clients with.
///
/// Stardust clients by default will have their roots at the position your headset was at when the server was started, but this is often undesired behavior.
///
/// Also, app shells (3D interfaces that specialize in augmenting specific 2D apps) and certain panel shells (3D interfaces generalized around all 2D apps) will want to ensure they get access to their launched applications exclusively.
///
/// Startup settings let you set these before you launch the app, pass the ID with the app as an environment variable, then the app will launch with those settings applied.
#[derive(Debug)]
pub struct StartupSettings {
	pub(crate) node: Node,
}
impl StartupSettings {
	/// Create a new startup settings node with default values.
	pub fn create(client: &Arc<Client>) -> Result<Self, NodeError> {
		let id = nanoid!();
		Ok(StartupSettings {
			node: Node::new(
				client,
				"/startup",
				"create_startup_settings",
				"/startup/settings",
				true,
				&id.clone(),
				id,
			)?,
		})
	}

	/// Set where the root of the client will be. On Earth, with gravity, it's generally advised to not rotate this at all other than around the Y axis.
	pub fn set_root(&self, root: &Spatial) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("set_root", &root.node.get_path()?)
	}

	/// Make any items of the acceptor's type that the client spawns automatically get accepted into this item acceptor.
	/// Useful for having an interface launch a 2D app and getting full control over it.
	pub fn add_automatic_acceptor<I: Item>(
		&self,
		acceptor: &ItemAcceptor<I>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("add_automatic_acceptor", &acceptor.node.get_path()?)
	}

	/// Generate the token and return it back.
	///
	/// Doesn't destroy the node so feel free to edit the settings and generate a new token.
	///
	/// When launching a new client, set the environment variable `STARDUST_STARTUP_TOKEN` to the returned string.
	/// Make sure the environment variable shows in `/proc/{pid}/environ` as that's the only reliable way to pass the value to the server (suggestions welcome).
	///
	/// # Example
	/// ```
	/// use stardust_xr_fusion::startup_serttings::StartupSettings;
	///
	/// let startup_settings = StartupSettings::create(&client).expect("Unable to create startup settings");
	/// // set the startup settings settings
	/// let startup_token = startup_settings.generate_startup_token().unwrap().await.unwrap();
	///
	/// std::env::set_var("STARDUST_STARTUP_TOKEN", startup_token); // to make sure it ends up in `/proc/{pid}/environ` of the new process
	/// nix::unistd::execvp(ustr(&program).as_cstr(), &args).unwrap(); // make sure to use execv here to get the environment variable there properly.
	/// ```
	pub fn generate_startup_token(
		&self,
	) -> Result<impl Future<Output = Result<String, NodeError>>, NodeError> {
		self.node
			.execute_remote_method("generate_startup_token", &())
	}
}
impl NodeType for StartupSettings {
	fn node(&self) -> &Node {
		&self.node
	}

	fn alias(&self) -> Self {
		StartupSettings {
			node: self.node.alias(),
		}
	}
}
impl Deref for StartupSettings {
	type Target = Node;

	fn deref(&self) -> &Self::Target {
		self.node()
	}
}

#[tokio::test]
async fn fusion_startup_settings() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let startup_settings =
		StartupSettings::create(&client).expect("Unable to create startup settings");
	startup_settings.set_root(client.get_root()).unwrap();
	println!(
		"{}",
		startup_settings
			.generate_startup_token()
			.unwrap()
			.await
			.unwrap()
	);
}
