use super::{
	client::Client,
	node::{GenNodeInfo, Node, NodeError, NodeType},
};
use anyhow::{anyhow, Result};
use futures::Future;
use stardust_xr::{
	flex::{self, flexbuffer_from_arguments},
	flex_from_quat, flex_from_vec3,
	values::{parse_quat, parse_vec3, Quat, Transform, Vec3},
};
use std::sync::{Arc, Weak};

pub struct Spatial {
	pub(crate) node: Arc<Node>,
}
#[buildstructor::buildstructor]
impl<'a> Spatial {
	#[builder(entry = "builder")]
	pub fn create(
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
				Transform {
					position,
					rotation,
					scale,
				},
				zoneable
			),
		})
	}

	pub(crate) fn from_path(client: Weak<Client>, path: &str) -> Result<Self, NodeError> {
		Ok(Spatial {
			node: Node::from_path(client, path)?,
		})
	}

	pub fn get_translation_rotation_scale<'d>(
		&self,
		relative_space: &'d Spatial,
	) -> Result<impl Future<Output = Result<(Vec3, Quat, Vec3)>>, NodeError> {
		let future = self.node.execute_remote_method(
			"getTransform",
			&flex::flexbuffer_from_arguments(|fbb| {
				fbb.build_singleton(relative_space.node.get_path())
			}),
		)?;
		Ok(async move {
			future.await.and_then(|data| {
				let root = flexbuffers::Reader::get_root(data.as_slice()).unwrap();
				let flex_vec = root.get_vector().unwrap();
				let pos = parse_vec3(flex_vec.idx(0)).ok_or_else(|| anyhow!("Parsing error"))?;
				let rot = parse_quat(flex_vec.idx(1)).ok_or_else(|| anyhow!("Parsing error"))?;
				let scl = parse_vec3(flex_vec.idx(2)).ok_or_else(|| anyhow!("Parsing error"))?;
				Ok((pos, rot, scl))
			})
		})
	}

	pub fn set_position(
		&self,
		relative_space: Option<&Spatial>,
		position: impl Into<Vec3>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, Some(position.into()), None, None)
	}
	pub fn set_rotation(
		&self,
		relative_space: Option<&Spatial>,
		rotation: impl Into<Quat>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, None, Some(rotation.into()), None)
	}
	pub fn set_scale(
		&self,
		relative_space: Option<&Spatial>,
		scale: impl Into<Vec3>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, None, None, Some(scale.into()))
	}

	pub fn set_transform(
		&self,
		relative_space: Option<&Spatial>,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		scale: Option<Vec3>,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
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
	}

	pub fn set_spatial_parent(&self, parent: &Spatial) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"setSpatialParent",
			&flexbuffer_from_arguments(|flex| flex.build_singleton(parent.node.get_path())),
		)
	}

	pub fn set_spatial_parent_in_place(&self, parent: &Spatial) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"setSpatialParentInPlace",
			&flexbuffer_from_arguments(|flex| flex.build_singleton(parent.node.get_path())),
		)
	}
}
impl NodeType for Spatial {
	fn node(&self) -> &Node {
		&self.node
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
