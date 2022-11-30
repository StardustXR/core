use super::Field;
use crate::{
	node::{ClientOwned, Node, NodeError, NodeType},
	spatial::Spatial,
};
use anyhow::Result;
use std::ops::Deref;

#[derive(Debug)]
pub struct SphereField {
	pub spatial: Spatial,
}
#[buildstructor::buildstructor]
impl<'a> SphereField {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		radius: f32,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(SphereField {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/field",
					"create_sphere_field",
					"/field",
					true,
					&id.clone(),
					(id, spatial_parent.node().get_path()?, position, radius),
				)?,
			},
		})
	}

	pub fn set_radius(&self, radius: f32) -> Result<(), NodeError> {
		self.node.send_remote_signal("set_radius", &radius)
	}
}
impl NodeType for SphereField {
	fn node(&self) -> &Node {
		&self.spatial.node()
	}

	fn alias(&self) -> Self {
		SphereField {
			spatial: self.spatial.alias(),
		}
	}
}
impl ClientOwned for SphereField {}
impl Field for SphereField {}
impl Deref for SphereField {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_sphere_field() {
	use crate::client::Client;
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let sphere_field = SphereField::builder()
		.spatial_parent(client.get_root())
		.radius(0.5)
		.build()
		.expect("Unable to make sphere field");
	let distance = sphere_field
		.distance(client.get_root(), mint::Vector3::from([0.0, 1.0, 0.0]))
		.unwrap()
		.await
		.expect("Unable to get sphere field distance");
	assert_eq!(distance, 0.5);
	sphere_field.set_radius(1.0).unwrap();
	let distance = sphere_field
		.distance(client.get_root(), mint::Vector3::from([0.0, 2.0, 0.0]))
		.unwrap()
		.await
		.expect("Unable to get sphere field distance");
	assert_eq!(distance, 1.0);
}
