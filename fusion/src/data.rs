use stardust_xr::values::Transform;

use crate::node::Node;

use super::{fields::Field, node::NodeError, spatial::Spatial};

pub struct PulseReceiver {
	pub spatial: Spatial,
	// pub field: &'a Field,
}

#[buildstructor::buildstructor]
impl<'a> PulseReceiver {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		field: &'a Field,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(PulseReceiver {
			spatial: Spatial {
				node: Node::new(
					spatial_parent.node.client.clone(),
					"/data",
					"createPulseReceiver",
					"/data/receiver",
					&id.clone(),
					(
						id,
						spatial_parent,
						Transform {
							position,
							rotation,
							scale: None,
						},
						&field.spatial,
					),
				)?,
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
