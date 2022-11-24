use super::Field;
use crate::{
	node::{ClientOwned, Node, NodeError, NodeType},
	spatial::Spatial,
};
use anyhow::Result;
use stardust_xr::values::Transform;
use std::ops::Deref;

#[derive(Debug)]
pub struct CylinderField {
	pub spatial: Spatial,
}
#[buildstructor::buildstructor]
impl<'a> CylinderField {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		length: f32,
		radius: f32,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(CylinderField {
			spatial: Spatial {
				node: Node::new(
					spatial_parent.node.client.clone(),
					"/field",
					"create_cylinder_field",
					"/field",
					true,
					&id,
					(
						&id.clone(),
						spatial_parent,
						Transform {
							position,
							rotation,
							scale: None,
						},
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
		self.spatial.node()
	}
}
impl ClientOwned for CylinderField {}
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
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let cylinder_field = CylinderField::builder()
		.spatial_parent(client.get_root())
		.length(1.0)
		.radius(0.5)
		.build()
		.expect("Unable to make cylinder field");
	let distance = cylinder_field
		.distance(client.get_root(), mint::Vector3::from([0.0, 1.0, 0.0]))
		.unwrap()
		.await
		.expect("Unable to get cylinder field distance");
	assert_eq!(distance, 0.5);
}
