use super::Field;
use crate::{
	fusion::{
		node::GenNodeInfo,
		node::{Node, NodeError},
		spatial::Spatial,
	},
	push_to_vec,
	values::{Vec3, VEC3_ZERO},
};
use anyhow::Result;
use std::ops::Deref;

pub struct SphereField {
	pub field: Field,
}
#[buildstructor::buildstructor]
impl<'a> SphereField {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		radius: f32,
	) -> Result<Self, NodeError> {
		Ok(SphereField {
			field: Field {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client: spatial_parent.node.client.clone(),
							parent_path: "/field",
							interface_path: "/field",
							interface_method: "createSphereField"
						},
						spatial_parent.node.get_path(),
						position.unwrap_or(VEC3_ZERO),
						radius
					),
				},
			},
		})
	}
}
impl Deref for SphereField {
	type Target = Field;

	fn deref(&self) -> &Self::Target {
		&self.field
	}
}

#[tokio::test]
async fn fusion_sphere_field() {
	use crate::fusion::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let sphere_field = SphereField::builder()
		.spatial_parent(client.get_root())
		.radius(0.5)
		.build()
		.expect("Unable to make sphere field");
	let distance = sphere_field
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
		)
		.await
		.expect("Unable to get sphere field distance");
	assert_eq!(distance, 1_f32);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}
