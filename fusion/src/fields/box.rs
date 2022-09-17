use crate::{
	node::GenNodeInfo,
	node::{Node, NodeError},
	spatial::Spatial,
};
use anyhow::Result;
use stardust_xr::{
	flex::FlexBuffable,
	values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ZERO},
};
use std::ops::Deref;

use super::Field;

pub struct BoxField {
	pub field: Field,
}
#[buildstructor::buildstructor]
impl<'a> BoxField {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		size: Vec3,
	) -> Result<Self, NodeError> {
		Ok(BoxField {
			field: Field {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client: spatial_parent.node.client.clone(),
							parent_path: "/field",
							interface_path: "/field",
							interface_method: "createBoxField"
						},
						spatial_parent.node.get_path(),
						position.unwrap_or(VEC3_ZERO),
						rotation.unwrap_or(QUAT_IDENTITY),
						size
					),
				},
			},
		})
	}

	pub fn set_size(&self, size: impl Into<Vec3>) -> Result<(), NodeError> {
		let size: Vec3 = size.into();
		self.node
			.send_remote_signal("setSize", &FlexBuffable::build_singleton(&size.into()))
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
		.size(Vec3::from([1.0, 1.0, 1.0]))
		.build()
		.expect("Unable to make box field");
	let distance = box_field
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 1_f32, 0_f32]),
		)
		.await
		.expect("Unable to get box field distance");
	assert_eq!(distance, 0.5_f32);
}
