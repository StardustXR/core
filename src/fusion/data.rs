use super::{
	client::Client,
	field::Field,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
	values,
};
use crate::flex;

pub struct PulseReceiver {
	pub spatial: Spatial,
	// pub field: &'a Field,
}

impl PulseReceiver {
	pub async fn create(
		client: &Client,
		spatial_parent: &Spatial,
		position: values::Vec3,
		rotation: values::Quat,
		field: &Field,
	) -> Result<Self, NodeError> {
		Ok(PulseReceiver {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client,
						parent_path: "/data/receiver",
						interface_path: "/data",
						interface_method: "createPulseReceiver"
					},
					spatial_parent.node.get_path(),
					position,
					rotation,
					field.spatial.node.get_path()
				),
			},
		})
	}
}

#[tokio::test]
async fn fusion_pulse_receiver() {
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let field = super::field::SphereField::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		0.1_f32,
	)
	.await
	.unwrap();
	let _pulse_receiver = PulseReceiver::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		mint::Quaternion::from([0_f32, 0_f32, 0_f32, 1_f32]),
		&field.field,
	)
	.await
	.unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}
