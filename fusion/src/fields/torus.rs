use super::Field;
use crate::{
	node::{ClientOwned, Node, NodeError, NodeType},
	spatial::Spatial,
};
use anyhow::Result;
use stardust_xr::values::Transform;
use std::ops::Deref;

#[derive(Debug)]
pub struct TorusField {
	pub spatial: Spatial,
}
#[buildstructor::buildstructor]
impl<'a> TorusField {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
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
						Transform {
							position,
							rotation,
							scale: None,
						},
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
impl ClientOwned for TorusField {}
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
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let cylinder_field = TorusField::builder()
		.spatial_parent(client.get_root())
		.radius_a(1.0)
		.radius_b(0.5)
		.build()
		.expect("Unable to make torus field");
	let distance = cylinder_field
		.distance(client.get_root(), mint::Vector3::from([1.0, 1.0, 0.0]))
		.unwrap()
		.await
		.expect("Unable to get torus field distance");
	assert_eq!(distance, 0.5);
}
