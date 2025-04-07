use glam::Quat;
use stardust_xr::values::{ResourceID, color::rgba_linear};
use stardust_xr_fusion::{
	Client,
	drawable::{MaterialParameter, Model, ModelPartAspect},
	project_local_resources,
	root::{ClientState, RootAspect, RootEvent},
	spatial::{SpatialAspect, Transform},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
	tracing_subscriber::fmt::init();
	let mut client = Client::connect().await.unwrap();
	client
		.setup_resources(&[&project_local_resources!("res")])
		.unwrap();

	let gyro = Model::create(
		client.get_root(),
		Transform::none(),
		&ResourceID::new_namespaced("fusion", "gyro"),
	)
	.unwrap();
	gyro.set_zoneable(true).unwrap();

	let client_handle = client.handle();
	let mut elapsed: f32 = client
		.await_method(client_handle.get_root().get_state())
		.await
		.unwrap()
		.unwrap()
		.data()
		.unwrap_or_default();
	let gem = gyro.part("Gem").unwrap();
	let ring_inner = gyro.part("OuterRing/MiddleRing/InnerRing").unwrap();
	let ring_middle = gyro.part("OuterRing/MiddleRing").unwrap();
	let ring_outer = gyro.part("OuterRing").unwrap();
	let _gyro = gyro;

	client
		.sync_event_loop(|client, _stop| {
			while let Some(root_event) = client.get_root().recv_root_event() {
				match root_event {
					RootEvent::Ping { response } => {
						response.send(Ok(()));
					}
					RootEvent::Frame { info } => {
						elapsed += info.delta;

						gem.set_material_parameter(
							"color",
							MaterialParameter::Color(rgba_linear!(
								0.0,
								0.25,
								1.0,
								elapsed.sin().abs()
							)),
						)
						.unwrap();
						gem.set_local_transform(Transform::from_rotation(Quat::from_rotation_y(
							elapsed,
						)))
						.unwrap();
						ring_inner
							.set_local_transform(Transform::from_rotation(Quat::from_rotation_x(
								elapsed,
							)))
							.unwrap();
						ring_middle
							.set_local_transform(Transform::from_rotation(Quat::from_rotation_z(
								elapsed,
							)))
							.unwrap();
						ring_outer
							.set_local_transform(Transform::from_rotation(Quat::from_rotation_x(
								elapsed,
							)))
							.unwrap();
					}
					RootEvent::SaveState { response } => response.send(
						ClientState::from_data_root(Some(elapsed), client.get_root()),
					),
				}
			}
		})
		.await
		.unwrap();
}
