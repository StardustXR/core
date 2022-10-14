use super::Field;
use crate::{
	node::{Node, NodeError},
	spatial::Spatial,
};
use anyhow::Result;
use stardust_xr::values::Transform;
use std::ops::Deref;

#[derive(Debug)]
pub struct CylinderField {
	pub field: Field,
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
			field: Field {
				spatial: Spatial {
					node: Node::new(
						spatial_parent.node.client.clone(),
						"/field",
						"createCylinderField",
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
			},
		})
	}

	pub fn set_size(&self, length: f32, radius: f32) -> Result<(), NodeError> {
		self.node.send_remote_signal("setSize", &(length, radius))
	}
}
impl Deref for CylinderField {
	type Target = Field;

	fn deref(&self) -> &Self::Target {
		&self.field
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
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 1_f32, 0_f32]),
		)
		.unwrap()
		.await
		.expect("Unable to cylinder box field distance");
	assert_eq!(distance, 0.5_f32);
}
