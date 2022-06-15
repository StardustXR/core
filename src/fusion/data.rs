use super::{
	client::Client,
	field::Field,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
	values,
};
use crate::flex;

pub struct PulseReceiver<'a> {
	pub spatial: Spatial<'a>,
	// pub field: &'a Field<'a>,
}

impl<'a> PulseReceiver<'a> {
	pub fn create(
		client: &Client<'a>,
		spatial_parent: &Spatial<'a>,
		position: values::Vec3,
		rotation: values::Quat,
		field: &Field<'a>,
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
			// field,
		})
	}
}

#[test]
fn fusion_pulse_receiver() {
	let mut client = Client::connect().expect("Couldn't connect");
	let stopper = client.get_cross_thread_stopper();
	let field = super::field::SphereField::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		0.1_f32,
	)
	.unwrap();
	let pulse_receiver = PulseReceiver::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		mint::Quaternion::from([0_f32, 0_f32, 0_f32, 1_f32]),
		&field.field,
	)
	.unwrap();
	let wait_thread = std::thread::spawn(move || {
		std::thread::sleep(core::time::Duration::from_secs(1));
		stopper.stop()
	});
	client.run_event_loop(None).expect("Event loop failed");
	let _ = wait_thread.join();
}
