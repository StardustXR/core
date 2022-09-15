use super::{
	fields::Field,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
};
use crate::values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ZERO};

pub struct PulseReceiver {
	pub spatial: Spatial,
	// pub field: &'a Field,
}

#[buildstructor::buildstructor]
impl<'a> PulseReceiver {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		field: &'a Field,
	) -> Result<Self, NodeError> {
		Ok(PulseReceiver {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client: spatial_parent.node.client.clone(),
						parent_path: "/data/receiver",
						interface_path: "/data",
						interface_method: "createPulseReceiver"
					},
					spatial_parent.node.get_path(),
					position.unwrap_or(VEC3_ZERO),
					rotation.unwrap_or(QUAT_IDENTITY),
					field.spatial.node.get_path()
				),
			},
		})
	}
}

#[tokio::test]
async fn fusion_pulse_receiver() {
	use super::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let field = super::fields::SphereField::builder()
		.spatial_parent(client.get_root())
		.radius(0.1)
		.build()
		.unwrap();
	let _pulse_receiver = PulseReceiver::builder()
		.spatial_parent(client.get_root())
		.field(&field)
		.build()
		.unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}
