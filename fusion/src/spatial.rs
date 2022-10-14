use super::{
	client::Client,
	node::{Node, NodeError, NodeType},
};
use anyhow::Result;
use mint::{Quaternion, Vector3};
use nanoid::nanoid;
use serde::{Serialize, Serializer};
use stardust_xr::values::Transform;
use std::{
	future::Future,
	sync::{Arc, Weak},
};

#[derive(Debug)]
pub struct Spatial {
	pub(crate) node: Arc<Node>,
}
#[buildstructor::buildstructor]
impl Spatial {
	#[builder(entry = "builder")]
	#[allow(clippy::needless_lifetimes)] // Actually needed, buildstructor needs it
	pub fn create<'a>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		scale: Option<mint::Vector3<f32>>,
		zoneable: bool,
	) -> Result<Self, NodeError> {
		let id = nanoid!();
		Ok(Spatial {
			node: Node::new(
				spatial_parent.node.client.clone(),
				"/spatial",
				"createSpatial",
				"/spatial/spatial",
				&id.clone(),
				(
					id,
					spatial_parent,
					Transform {
						position,
						rotation,
						scale,
					},
					zoneable,
				),
			)?,
		})
	}

	pub(crate) fn from_path(client: Weak<Client>, path: impl ToString) -> Result<Self, NodeError> {
		Ok(Spatial {
			node: Node::from_path(client, path.to_string())?,
		})
	}

	pub fn get_translation_rotation_scale(
		&self,
		relative_space: &Spatial,
	) -> Result<
		impl Future<Output = Result<(Vector3<f32>, Quaternion<f32>, Vector3<f32>)>>,
		NodeError,
	> {
		self.node
			.execute_remote_method("getTransform", relative_space)
	}

	pub fn set_position(
		&self,
		relative_space: Option<&Spatial>,
		position: impl Into<mint::Vector3<f32>>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, Some(position.into()), None, None)
	}
	pub fn set_rotation(
		&self,
		relative_space: Option<&Spatial>,
		rotation: impl Into<mint::Quaternion<f32>>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, None, Some(rotation.into()), None)
	}
	pub fn set_scale(
		&self,
		relative_space: Option<&Spatial>,
		scale: impl Into<mint::Vector3<f32>>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, None, None, Some(scale.into()))
	}

	pub fn set_transform(
		&self,
		relative_space: Option<&Spatial>,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		scale: Option<mint::Vector3<f32>>,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"setTransform",
			&(
				relative_space,
				Transform {
					position,
					rotation,
					scale,
				},
			),
		)
	}

	pub fn set_spatial_parent(&self, parent: &Spatial) -> Result<(), NodeError> {
		self.node.send_remote_signal("setSpatialParent", parent)
	}

	pub fn set_spatial_parent_in_place(&self, parent: &Spatial) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("setSpatialParentInPlace", parent)
	}
}
impl NodeType for Spatial {
	fn node(&self) -> &Node {
		&self.node
	}
}
impl Serialize for Spatial {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		self.node.get_path().serialize(serializer)
	}
}

#[tokio::test]
async fn fusion_spatial() {
	use super::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let spatial = Spatial::builder()
		.spatial_parent(client.get_root())
		.zoneable(true)
		.build()
		.unwrap();
	drop(spatial);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}
