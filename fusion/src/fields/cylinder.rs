use super::Field;
use crate::{
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
};

use stardust_xr::values::Transform;
use std::ops::Deref;

#[derive(Debug)]
pub struct CylinderField {
	spatial: Spatial,
}
impl<'a> CylinderField {
	pub fn create(
		spatial_parent: &'a Spatial,
		transform: Transform,
		length: f32,
		radius: f32,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(CylinderField {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/field",
					"create_cylinder_field",
					"/field",
					true,
					&id,
					(
						&id.clone(),
						spatial_parent.node().get_path()?,
						transform,
						length,
						radius,
					),
				)?,
			},
		})
	}

	pub fn set_size(&self, length: f32, radius: f32) -> Result<(), NodeError> {
		self.node.send_remote_signal("set_size", &(length, radius))
	}
}
impl NodeType for CylinderField {
	fn node(&self) -> &Node {
		&self.spatial.node()
	}

	fn alias(&self) -> Self {
		CylinderField {
			spatial: self.spatial.alias(),
		}
	}
}
impl Field for CylinderField {}
impl Deref for CylinderField {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_cylinder_field() {
	use crate::client::Client;
	color_eyre::install().unwrap();
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let cylinder_field = CylinderField::create(client.get_root(), Transform::none(), 1.0, 0.5)
		.expect("Unable to make cylinder field");
	let distance = cylinder_field
		.distance(client.get_root(), mint::Vector3::from([0.0, 1.0, 0.0]))
		.unwrap()
		.await
		.expect("Unable to get cylinder field distance");
	assert_eq!(distance, 0.5);
}
