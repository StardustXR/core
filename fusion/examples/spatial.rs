use glam::Quat;
use manifest_dir_macros::directory_relative_path;
use stardust_xr::values::Transform;
use stardust_xr_fusion::{
	client::{Client, LifeCycleHandler, LogicStepInfo},
	drawable::Model,
	resource::NamespacedResource,
	spatial::Spatial,
};
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	tracing_subscriber::fmt::init();
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();
	client.set_base_prefixes(&[directory_relative_path!("res")]);

	let _root = client.wrap_root(SpatialDemo::new(&client));

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}

struct SpatialDemo {
	_root: Spatial,
	gem: Model,
	ring_inner: Model,
	ring_middle: Model,
	ring_outer: Model,
}
impl SpatialDemo {
	fn new(client: &Arc<Client>) -> Self {
		let _root = Spatial::create(client.get_root(), Transform::default(), true).unwrap();

		let gem = Model::create(
			&_root,
			&NamespacedResource::new("fusion", "gyro_gem"),
			Transform::default(),
		)
		.unwrap();
		let ring_inner = Model::create(
			&_root,
			&NamespacedResource::new("fusion", "gyro_inside"),
			Transform::default(),
		)
		.unwrap();
		let ring_middle = Model::create(
			&ring_inner,
			&NamespacedResource::new("fusion", "gyro_middle"),
			Transform::default(),
		)
		.unwrap();
		let ring_outer = Model::create(
			&ring_middle,
			&NamespacedResource::new("fusion", "gyro_outside"),
			Transform::default(),
		)
		.unwrap();

		SpatialDemo {
			_root,
			gem,
			ring_inner,
			ring_middle,
			ring_outer,
		}
	}
}
impl LifeCycleHandler for SpatialDemo {
	fn logic_step(&mut self, info: LogicStepInfo) {
		let elapsed = info.elapsed as f32;

		self.gem
			.set_rotation(None, Quat::from_rotation_y(elapsed))
			.unwrap();
		self.ring_inner
			.set_rotation(None, Quat::from_rotation_z(elapsed))
			.unwrap();
		self.ring_middle
			.set_rotation(None, Quat::from_rotation_x(elapsed))
			.unwrap();
		self.ring_outer
			.set_rotation(None, Quat::from_rotation_z(elapsed))
			.unwrap();
	}
}
