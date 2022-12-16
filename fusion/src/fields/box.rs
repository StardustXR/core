use super::Field;
use crate::{
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
};
use anyhow::Result;
use mint::Vector3;
use stardust_xr::values::Transform;
use std::ops::Deref;

#[derive(Debug)]
pub struct BoxField {
	spatial: Spatial,
}
impl<'a> BoxField {
	pub fn create(
		spatial_parent: &'a Spatial,
		transform: Transform,
		size: Vector3<f32>,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(BoxField {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/field",
					"create_box_field",
					"/field",
					true,
					&id.clone(),
					(id, spatial_parent.node().get_path()?, transform, size),
				)?,
			},
		})
	}

	pub fn set_size(&self, size: impl Into<Vector3<f32>>) -> Result<(), NodeError> {
		let size: Vector3<f32> = size.into();
		self.node.send_remote_signal("set_size", &size)
	}
}
impl NodeType for BoxField {
	fn node(&self) -> &Node {
		&self.spatial.node()
	}

	fn alias(&self) -> Self {
		BoxField {
			spatial: self.spatial.alias(),
		}
	}
}
impl Field for BoxField {}
impl Deref for BoxField {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_box_field() {
	use crate::client::Client;
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let box_field = BoxField::create(
		client.get_root(),
		Transform::default(),
		Vector3::from([1.0; 3]),
	)
	.expect("Unable to make box field");
	let distance = box_field
		.distance(client.get_root(), Vector3::from([0.0, 1.0, 0.0]))
		.unwrap()
		.await
		.expect("Unable to get box field distance");
	assert_eq!(distance, 0.5);
}
