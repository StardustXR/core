use crate::{
	node::{Node, NodeError},
	spatial::Spatial,
};
use anyhow::Result;
use mint::Vector3;
use stardust_xr::values::Transform;
use std::ops::Deref;

use super::Field;

#[derive(Debug)]
pub struct BoxField {
	pub field: Field,
}
#[buildstructor::buildstructor]
impl<'a> BoxField {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		size: Vector3<f32>,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(BoxField {
			field: Field {
				spatial: Spatial {
					node: Node::new(
						spatial_parent.node.client.clone(),
						"/field",
						"createBoxField",
						"/field",
						true,
						&id.clone(),
						(
							id,
							spatial_parent,
							Transform {
								position,
								rotation,
								scale: None,
							},
							size,
						),
					)?,
				},
			},
		})
	}

	pub fn set_size(&self, size: impl Into<Vector3<f32>>) -> Result<(), NodeError> {
		let size: Vector3<f32> = size.into();
		self.node.send_remote_signal("setSize", &size)
	}
}
impl Deref for BoxField {
	type Target = Field;

	fn deref(&self) -> &Self::Target {
		&self.field
	}
}

#[tokio::test]
async fn fusion_box_field() {
	use crate::client::Client;
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let box_field = BoxField::builder()
		.spatial_parent(client.get_root())
		.size(Vector3::from([1.0, 1.0, 1.0]))
		.build()
		.expect("Unable to make box field");
	let distance = box_field
		.field
		.distance(client.get_root(), Vector3::from([0_f32, 1_f32, 0_f32]))
		.unwrap()
		.await
		.expect("Unable to get box field distance");
	assert_eq!(distance, 0.5_f32);
}
