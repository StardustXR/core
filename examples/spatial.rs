use anyhow::Result;
use glam::{Quat, Vec3};
use libstardustxr::fusion::{
	async_trait,
	client::{Client, LifeCycleHandler, LogicStepInfo},
	drawable::Model,
	resource::Resource,
	spatial::Spatial,
};
use manifest_dir_macros::directory_relative_path;
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
	let (client, event_loop) = Client::connect_with_async_loop().await?;
	client
		.set_base_prefixes(&[directory_relative_path!("res")])
		.await?;

	let base = Arc::new(SpatialDemo::new(&client).await?);
	client.set_life_cycle_handler(&base);

	tokio::select! {
		_ = tokio::signal::ctrl_c() => Ok(()),
		_ = event_loop => Err(anyhow::anyhow!("Server crashed")),
	}
}

struct SpatialDemo {
	client: Arc<Client>,
	root: Spatial,
	gem: Model,
	ring_inner: Model,
	ring_middle: Model,
	ring_outer: Model,
}
impl SpatialDemo {
	async fn new(client: &Arc<Client>) -> Result<Self> {
		// let root = Spatial::create(client, client.get_root(), None, None, None, false).await?;
		let root = Spatial::builder()
			.client(Arc::downgrade(&client))
			.spatial_parent(client.get_root())
			.zoneable(true)
			.build()
			.await?;

		let gem = Model::resource_builder()
			.client(Arc::downgrade(&client))
			.spatial_parent(&root)
			.resource(&Resource::new("libstardustxr", "gyro_gem.glb"))
			.build()
			.await?;
		let ring_inner = Model::resource_builder()
			.client(Arc::downgrade(&client))
			.spatial_parent(&root)
			.resource(&Resource::new("libstardustxr", "gyro_inside.glb"))
			.build()
			.await?;
		let ring_middle = Model::resource_builder()
			.client(Arc::downgrade(&client))
			.spatial_parent(&ring_inner)
			.resource(&Resource::new("libstardustxr", "gyro_middle.glb"))
			.build()
			.await?;
		let ring_outer = Model::resource_builder()
			.client(Arc::downgrade(&client))
			.spatial_parent(&ring_middle)
			.resource(&Resource::new("libstardustxr", "gyro_outside.glb"))
			.build()
			.await?;

		Ok(SpatialDemo {
			client: client.clone(),
			root,
			gem,
			ring_inner,
			ring_middle,
			ring_outer,
		})
	}
}
#[async_trait]
impl LifeCycleHandler for SpatialDemo {
	async fn logic_step(&self, info: LogicStepInfo) {
		let elapsed = info.elapsed as f32;

		let _ = tokio::join!(
			self.gem
				.set_rotation(None, Quat::from_axis_angle(Vec3::Y, elapsed)),
			self.ring_inner
				.set_rotation(None, Quat::from_axis_angle(Vec3::Z, elapsed)),
			self.ring_middle
				.set_rotation(None, Quat::from_axis_angle(Vec3::X, elapsed)),
			self.ring_outer
				.set_rotation(None, Quat::from_axis_angle(Vec3::Z, elapsed)),
		);
	}
}
