use glam::Quat;
use manifest_dir_macros::directory_relative_path;
use stardust_xr::values::{color::rgba_linear, ResourceID};
use stardust_xr_fusion::{
	client::{Client, ClientState, FrameInfo, RootHandler},
	core::schemas::flex::flexbuffers,
	drawable::{MaterialParameter, Model, ModelPart, ModelPartAspect},
	node::NodeType,
	spatial::{Spatial, SpatialAspect, Transform},
};
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	tracing_subscriber::fmt::init();
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();
	client.set_base_prefixes(&[directory_relative_path!("res")]);

	let _root = client.wrap_root(SpatialDemo::new(&client)).unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}

struct SpatialDemo {
	t: f32,
	root: Spatial,
	_gyro: Model,
	gem: ModelPart,
	ring_inner: ModelPart,
	ring_middle: ModelPart,
	ring_outer: ModelPart,
}
impl SpatialDemo {
	fn new(client: &Arc<Client>) -> Self {
		let gyro = Model::create(
			client.get_root(),
			Transform::none(),
			&ResourceID::new_namespaced("fusion", "gyro"),
		)
		.unwrap();
		gyro.set_zoneable(true).unwrap();

		SpatialDemo {
			t: flexbuffers::from_slice(&client.state().data).unwrap_or_default(),
			root: client.get_root().alias(),
			gem: gyro.part("Gem").unwrap(),
			ring_inner: gyro.part("OuterRing/MiddleRing/InnerRing").unwrap(),
			ring_middle: gyro.part("OuterRing/MiddleRing").unwrap(),
			ring_outer: gyro.part("OuterRing").unwrap(),
			_gyro: gyro,
		}
	}
}
impl RootHandler for SpatialDemo {
	fn frame(&mut self, info: FrameInfo) {
		self.t += info.delta as f32;

		self.gem
			.set_material_parameter(
				"color",
				MaterialParameter::Color(rgba_linear!(0.0, 0.25, 1.0, self.t.sin().abs())),
			)
			.unwrap();
		self.gem
			.set_local_transform(Transform::from_rotation(Quat::from_rotation_y(self.t)))
			.unwrap();
		self.ring_inner
			.set_local_transform(Transform::from_rotation(Quat::from_rotation_x(self.t)))
			.unwrap();
		self.ring_middle
			.set_local_transform(Transform::from_rotation(Quat::from_rotation_z(self.t)))
			.unwrap();
		self.ring_outer
			.set_local_transform(Transform::from_rotation(Quat::from_rotation_x(self.t)))
			.unwrap();
	}
	fn save_state(&mut self) -> ClientState {
		ClientState {
			data: flexbuffers::to_vec(self.t).unwrap(),
			root: self.root.alias(),
			spatial_anchors: Default::default(),
		}
	}
}
