use crate::{
	node::{GenNodeInfo, Node, NodeError},
	resource::Resource,
	spatial::Spatial,
};
use anyhow::Result;
use color::Rgba;
use flexbuffers::VectorBuilder;
use stardust_xr::values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ONE, VEC3_ZERO};
use std::ops::Deref;

pub trait MaterialParameter {
	fn push_flex(&self, vec: &mut VectorBuilder);
}
impl MaterialParameter for f32 {
	fn push_flex(&self, vec: &mut VectorBuilder) {
		vec.push(*self);
	}
}
impl MaterialParameter for f64 {
	fn push_flex(&self, vec: &mut VectorBuilder) {
		vec.push(*self);
	}
}
impl MaterialParameter for Rgba {
	fn push_flex(&self, vec: &mut VectorBuilder) {
		let mut color_vec = vec.start_vector();
		color_vec.push(self.c.r);
		color_vec.push(self.c.g);
		color_vec.push(self.c.b);
		color_vec.push(self.a);
	}
}

pub struct Model {
	pub spatial: Spatial,
}
#[buildstructor::buildstructor]
impl<'a> Model {
	#[builder(entry = "builder")]
	pub fn create<R: Resource + 'a>(
		spatial_parent: &'a Spatial,
		resource: &'a R,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		scale: Option<Vec3>,
	) -> Result<Self, NodeError> {
		Ok(Model {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client: spatial_parent.node.client.clone(),
						parent_path: "/drawable/model",
						interface_path: "/drawable",
						interface_method: "createModel"
					},
					spatial_parent.node.get_path(),
					resource.parse().as_str(),
					position.unwrap_or(VEC3_ZERO),
					rotation.unwrap_or(QUAT_IDENTITY),
					scale.unwrap_or(VEC3_ONE)
				),
			},
		})
	}
}
impl Deref for Model {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_model() -> Result<()> {
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop().await?;
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let _model = Model::builder()
		.spatial_parent(client.get_root())
		.resource(&crate::resource::NamespacedResource::new(
			"fusion",
			"gyro_gem.glb",
		))
		.build()?;

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
	Ok(())
}
