use super::ResourceID;
use crate::{
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
};

use color::{color_space::LinearRgb, rgba_linear, Rgba};
use mint::{ColumnMatrix4, Vector2, Vector3, Vector4};
use serde::Serialize;
use serde_with::serde_as;
use stardust_xr::values::Transform;
use std::ops::Deref;

serde_with::serde_conv!(
	RgbaAsArray,
	Rgba<f32, LinearRgb>,
	|rgba: &Rgba<f32, LinearRgb>| [rgba.c.r, rgba.c.g, rgba.c.b, rgba.a],
	|value: [f32; 4]| -> Result<_, std::convert::Infallible> {
		Ok(rgba_linear!(value[0], value[1], value[2], value[3]))
	}
);

#[serde_as]
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "t", content = "c")]
pub enum MaterialParameter {
	Float(f32),
	Vector2(Vector2<f32>),
	Vector3(Vector3<f32>),
	Vector4(Vector4<f32>),
	Color(#[serde_as(as = "RgbaAsArray")] Rgba<f32, LinearRgb>),
	Int(i32),
	Int2(Vector2<i32>),
	Int3(Vector3<i32>),
	Int4(Vector4<i32>),
	Bool(bool),
	UInt(u32),
	UInt2(Vector2<u32>),
	UInt3(Vector3<u32>),
	UInt4(Vector4<u32>),
	Matrix(ColumnMatrix4<f32>),
	Texture(ResourceID),
}

#[derive(Debug)]
pub struct ModelPart {
	spatial: Spatial,
}
impl ModelPart {
	/// Set a property of a material on this model node.
	pub fn set_material_parameter(
		&self,
		name: &str,
		value: MaterialParameter,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("set_material_parameter", &(name, value))
	}
}
impl NodeType for ModelPart {
	fn node(&self) -> &Node {
		&self.spatial.node()
	}

	fn alias(&self) -> Self {
		ModelPart {
			spatial: self.spatial.alias(),
		}
	}
}
impl Deref for ModelPart {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

/// A 3D model in the GLTF format.
///
/// # Example
/// ```
/// let gyro_gem_resource = crate::resource::NamespacedResource::new("fusion", "gyro_gem");
/// let model = Model::create(client.get_root(), Transform::none(), &gyro_gem_resource).unwrap();
/// ```
#[derive(Debug)]
pub struct Model {
	spatial: Spatial,
}
impl<'a> Model {
	/// Create a model node. GLTF and GLB are supported.
	pub fn create(
		spatial_parent: &'a Spatial,
		transform: Transform,
		resource: &ResourceID,
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
					(id, spatial_parent.node().get_path()?, transform, resource),
				)?,
			},
		})
	}

	/// Set a property of a material on this model.
	pub fn model_part(&self, relative_path: &str) -> Result<ModelPart, NodeError> {
		if relative_path.starts_with('/') {
			return Err(NodeError::InvalidPath);
		}
		Ok(ModelPart {
			spatial: Spatial {
				node: Node::from_path(
					&self.node().client()?,
					self.node().get_path()?,
					relative_path,
					false,
				),
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
async fn fusion_model() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.unwrap();
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let gyro_gem_resource = ResourceID::new_namespaced("fusion", "gyro_gem");
	let model = Model::create(client.get_root(), Transform::default(), &gyro_gem_resource).unwrap();
	model
		.model_part("Gem")
		.unwrap()
		.set_material_parameter(
			"color",
			MaterialParameter::Color(color::rgba_linear!(0.0, 1.0, 0.5, 0.75)),
		)
		.unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}
