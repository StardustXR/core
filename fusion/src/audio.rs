//! Create audio!

use crate::drawable::ResourceID;
use crate::{
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
};

use stardust_xr::values::Transform;
use std::ops::Deref;

#[derive(Debug)]
pub struct Sound {
	spatial: Spatial,
}
impl<'a> Sound {
	/// Create a sound node. WAV and MP3 are supported.
	pub fn create(
		spatial_parent: &'a Spatial,
		transform: Transform,
		resource: &ResourceID,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(Sound {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/audio",
					"create_sound",
					"/audio/sound",
					true,
					&id.clone(),
					(id, spatial_parent.node().get_path()?, transform, resource),
				)?,
			},
		})
	}

	/// Play sound effect.
	pub fn play(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("play", &())
	}

	/// Stop sound effect.
	pub fn stop(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("stop", &())
	}
}
impl NodeType for Sound {
	fn node(&self) -> &Node {
		&self.spatial.node()
	}

	fn alias(&self) -> Self {
		Sound {
			spatial: self.spatial.alias(),
		}
	}
}
impl Deref for Sound {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_sound() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.unwrap();
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let lightspeed_resource = ResourceID::new_namespaced("fusion", "kittn_lightspeed");
	let sound = Sound::create(
		client.get_root(),
		Transform::default(),
		&lightspeed_resource,
	)
	.unwrap();
	sound.play().unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(10)).await;
	sound.stop().unwrap();
	tokio::time::sleep(core::time::Duration::from_secs(2)).await;
}
