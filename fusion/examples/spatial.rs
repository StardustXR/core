use glam::Quat;
use stardust_xr_fusion::{
	client::Client,
	drawable::ModelExt,
	project_local_resources,
	spatial::{Spatial, SpatialExt, Transform},
};
use stardust_xr_protocol::{
	model::{MaterialParameter, Model},
	types::{Color, Resource},
};
use tokio::sync::broadcast::error::RecvError;
use tracing::warn;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	tracing_subscriber::fmt::init();
	let (client, root) = Client::auto_connect(&[&project_local_resources!("res")])
		.await
		.unwrap();

	let gyro_spatial = Spatial::new(&client, &root, Transform::IDENTITY)
		.await
		.unwrap();
	let gyro = Model::new(
		&client,
		&gyro_spatial,
		Resource::Namespaced {
			namespace: "fusion".into(),
			path: "gyro".into(),
		},
	)
	.await
	.unwrap();

	let gem = gyro.get_part("Gem".into()).await.unwrap().unwrap();
	let gem_spatial = gem.get_spatial().await.unwrap();
	let ring_inner = gyro
		.get_part("OuterRing/MiddleRing/InnerRing".into())
		.await
		.unwrap()
		.unwrap();
	let ring_inner_spatial = ring_inner.get_spatial().await.unwrap();
	let ring_middle = gyro
		.get_part("OuterRing/MiddleRing".into())
		.await
		.unwrap()
		.unwrap();
	let ring_middle_spatial = ring_middle.get_spatial().await.unwrap();
	let ring_outer = gyro.get_part("OuterRing".into()).await.unwrap().unwrap();
	let ring_outer_spatial = ring_outer.get_spatial().await.unwrap();

	let mut elapsed = 0f32;
	let mut frame_recv = client.frame_receiver();
	loop {
		let info = match frame_recv.recv().await {
			Ok(v) => v,
			Err(RecvError::Lagged(n)) => {
				warn!("lost {n} frame events");
				continue;
			}
			Err(RecvError::Closed) => {
				break;
			}
		};
		elapsed += info.delta;

		gem.set_material_parameter(
			"color".into(),
			MaterialParameter::Color {
				value: Color::rgba(0.0, 0.25, 1.0, elapsed.sin().abs()),
			},
		)
		.await
		.unwrap();
		use stardust_xr_fusion::spatial::PartialTransform as PartTransform;
		gem_spatial
			.set_local_transform(PartTransform::from_rotation(Quat::from_rotation_y(elapsed)))
			.unwrap();
		ring_inner_spatial
			.set_local_transform(PartTransform::from_rotation(Quat::from_rotation_x(elapsed)))
			.unwrap();
		ring_middle_spatial
			.set_local_transform(PartTransform::from_rotation(Quat::from_rotation_z(elapsed)))
			.unwrap();
		ring_outer_spatial
			.set_local_transform(PartTransform::from_rotation(Quat::from_rotation_x(elapsed)))
			.unwrap();
	}
}
