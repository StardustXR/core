use crate::{
	node::{Node, NodeError, NodeType},
	resource::Resource,
	spatial::Spatial,
};
use anyhow::Result;
use stardust_xr::values::Transform;
use std::ops::Deref;

// pub trait MaterialParameter {
// 	fn push_flex(&self, vec: &mut VectorBuilder);
// }
// impl MaterialParameter for f32 {
// 	fn push_flex(&self, vec: &mut VectorBuilder) {
// 		vec.push(*self);
// 	}
// }
// impl MaterialParameter for f64 {
// 	fn push_flex(&self, vec: &mut VectorBuilder) {
// 		vec.push(*self);
// 	}
// }
// impl MaterialParameter for Rgba {
// 	fn push_flex(&self, vec: &mut VectorBuilder) {
// 		let mut color_vec = vec.start_vector();
// 		color_vec.push(self.c.r);
// 		color_vec.push(self.c.g);
// 		color_vec.push(self.c.b);
// 		color_vec.push(self.a);
// 	}
// }

/// A 3D model in the GLTF format.
///
/// # Example
/// ```
/// let gyro_gem_resource = crate::resource::NamespacedResource::new("fusion", "gyro_gem");
/// let _model = Model::builder()
/// 	.spatial_parent(client.get_root())
/// 	.resource(&gyro_gem_resource)
/// 	.build().unwrap();
/// ```
#[derive(Debug)]
pub struct Model {
	spatial: Spatial,
}
// #[buildstructor::buildstructor]
impl<'a> Model {
	/// Create a model node. GLTF and GLB are supported.
	// #[builder(entry = "builder")]
	pub fn create<R: Resource + 'a>(
		spatial_parent: &'a Spatial,
		transform: Transform,
		resource: &'a R,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(Model {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/drawable",
					"create_model",
					"/drawable/model",
					true,
					&id.clone(),
					(
						id,
						spatial_parent.node().get_path()?,
						transform,
						resource.parse().as_str(),
					),
				)?,
			},
		})
	}
}
impl NodeType for Model {
	fn node(&self) -> &Node {
		&self.spatial.node()
	}

	fn alias(&self) -> Self {
		Model {
			spatial: self.spatial.alias(),
		}
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

	let gyro_gem_resource = crate::resource::NamespacedResource::new("fusion", "gyro_gem");
	let _model = Model::create(client.get_root(), Transform::default(), &gyro_gem_resource)?;

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
	Ok(())
}
