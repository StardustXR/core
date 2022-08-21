use super::{
	client::Client,
	node::{GenNodeInfo, Node, NodeError},
};
use crate::{
	flex, flex_from_quat, flex_from_vec3, flex_to_quat, flex_to_vec3,
	values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ONE, VEC3_ZERO},
};
use anyhow::Result;
use std::sync::{Arc, Weak};

pub struct Spatial {
	pub node: Arc<Node>,
}
#[buildstructor::buildstructor]
impl<'a> Spatial {
	#[builder(entry = "builder")]
	pub async fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		scale: Option<Vec3>,
		zoneable: bool,
	) -> Result<Self, NodeError> {
		Ok(Spatial {
			node: generate_node!(
				GenNodeInfo {
					client: spatial_parent.node.client.clone(),
					parent_path: "/spatial/spatial",
					interface_path: "/spatial",
					interface_method: "createSpatial"
				},
				spatial_parent.node.get_path(),
				position.unwrap_or(VEC3_ZERO),
				rotation.unwrap_or(QUAT_IDENTITY),
				scale.unwrap_or(VEC3_ONE),
				zoneable
			),
		})
	}

	pub(crate) fn from_path(client: Weak<Client>, path: &str) -> Result<Self, NodeError> {
		Ok(Spatial {
			node: Node::from_path(client, path)?,
		})
	}

	pub async fn get_translation_rotation_scale(
		&self,
		relative_space: &Spatial,
	) -> Result<(Vec3, Quat, Vec3)> {
		self.node
			.execute_remote_method(
				"getTransform",
				&flex::flexbuffer_from_arguments(|fbb| {
					fbb.build_singleton(relative_space.node.get_path())
				}),
			)
			.await
			.map(|data| {
				let root = flexbuffers::Reader::get_root(data.as_slice()).unwrap();
				let flex_vec = root.get_vector().unwrap();
				let pos = flex_to_vec3!(flex_vec.idx(0));
				let rot = flex_to_quat!(flex_vec.idx(1));
				let scl = flex_to_vec3!(flex_vec.idx(2));
				(pos.unwrap(), rot.unwrap(), scl.unwrap())
			})
	}

	pub async fn set_position(
		&self,
		relative_space: Option<&Spatial>,
		position: impl Into<Vec3>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, Some(position.into()), None, None)
			.await
	}
	pub async fn set_rotation(
		&self,
		relative_space: Option<&Spatial>,
		rotation: impl Into<Quat>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, None, Some(rotation.into()), None)
			.await
	}
	pub async fn set_scale(
		&self,
		relative_space: Option<&Spatial>,
		scale: impl Into<Vec3>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, None, None, Some(scale.into()))
			.await
	}

	pub async fn set_transform(
		&self,
		relative_space: Option<&Spatial>,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		scale: Option<Vec3>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal(
				"setTransform",
				flex::flexbuffer_from_vector_arguments(|vec| {
					if let Some(space) = relative_space {
						vec.push(space.node.get_path());
					} else {
						vec.push(())
					}
					if let Some(position) = position {
						flex_from_vec3!(vec, position);
					} else {
						vec.push(());
					}
					if let Some(rotation) = rotation {
						flex_from_quat!(vec, rotation);
					} else {
						vec.push(());
					}
					if let Some(scale) = scale {
						flex_from_vec3!(vec, scale);
					} else {
						vec.push(());
					}
				})
				.as_slice(),
			)
			.await
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
		.await
		.unwrap();
	drop(spatial);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}
