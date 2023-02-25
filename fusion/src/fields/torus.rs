use super::Field;
use crate::{
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
};

use stardust_xr::values::Transform;
use std::ops::Deref;

#[derive(Debug)]
pub struct TorusField {
	spatial: Spatial,
}
impl<'a> TorusField {
	pub fn create(
		spatial_parent: &'a Spatial,
		transform: Transform,
		radius_a: f32,
		radius_b: f32,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(TorusField {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/field",
					"create_torus_field",
					"/field",
					true,
					&id,
					(
						&id.clone(),
						spatial_parent.node().get_path()?,
						transform,
						radius_a,
						radius_b,
					),
				)?,
			},
		})
	}

	pub fn set_size(&self, radius_a: f32, radius_b: f32) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("set_size", &(radius_a, radius_b))
	}
}
impl NodeType for TorusField {
	fn node(&self) -> &Node {
		&self.spatial.node()
	}

	fn alias(&self) -> Self {
		TorusField {
			spatial: self.spatial.alias(),
		}
	}
}
impl Field for TorusField {}
impl Deref for TorusField {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_torus_field() {
	use crate::client::Client;
	color_eyre::install().unwrap();
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let cylinder_field = TorusField::create(client.get_root(), Transform::default(), 1.0, 0.5)
		.expect("Unable to make torus field");
	let distance = cylinder_field
		.distance(client.get_root(), mint::Vector3::from([1.0, 1.0, 0.0]))
		.unwrap()
		.await
		.expect("Unable to get torus field distance");
	assert_eq!(distance, 0.5);
}
