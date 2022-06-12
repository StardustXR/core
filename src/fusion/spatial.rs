use super::{
	client::Client,
	node::{GenNodeInfo, Node, NodeError},
	values,
};
use crate::flex;
use std::sync::Arc;

pub struct Spatial<'a> {
	pub node: Arc<Node<'a>>,
}

impl<'a> Spatial<'a> {
	pub fn create(
		client: &Client<'a>,
		spatial_parent: &Spatial<'a>,
		position: values::Vec3,
		rotation: values::Quat,
		scale: values::Vec3,
		zoneable: bool,
	) -> Result<Self, NodeError> {
		Ok(Spatial {
			node: generate_node!(
				GenNodeInfo {
					client,
					parent_path: "/spatial/spatial",
					interface_path: "/spatial",
					interface_method: "createSpatial"
				},
				spatial_parent.node.get_path(),
				position,
				rotation,
				scale,
				zoneable
			),
		})
	}

	pub fn from_path(client: &Client<'a>, path: &str) -> Result<Self, NodeError> {
		Ok(Spatial {
			node: Node::from_path(client, path)?,
		})
	}

	pub fn get_transform(
		&self,
		space: &Spatial,
		callback: impl Fn(values::Vec3, values::Quat, values::Vec3) + 'a,
	) -> Result<(), NodeError> {
		self.node.execute_remote_method(
			"getTransform",
			flex::flexbuffer_from_arguments(|fbb| fbb.build_singleton(space.node.get_path()))
				.as_slice(),
			Box::new(move |data| {
				let root = flexbuffers::Reader::get_root(data).unwrap();
				let flex_vec = root.get_vector().unwrap();
				let pos = flex_to_vec3!(flex_vec.idx(0));
				let rot = flex_to_quat!(flex_vec.idx(1));
				let scl = flex_to_vec3!(flex_vec.idx(2));
				callback(pos.unwrap(), rot.unwrap(), scl.unwrap());
			}),
		)
	}
	pub fn set_transform(
		&self,
		relative_space: Option<&Spatial>,
		position: Option<values::Vec3>,
		rotation: Option<values::Quat>,
		scale: Option<values::Vec3>,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"setTransform",
			flex::flexbuffer_from_vector_arguments(|vec| {
				if let Some(space) = relative_space {
					vec.push(space.node.get_path());
				} else {
					vec.push(())
				}
				if position.is_some() {
					flex_from_vec3!(vec, position.unwrap());
				} else {
					vec.push(());
				}
				if rotation.is_some() {
					flex_from_quat!(vec, rotation.unwrap());
				} else {
					vec.push(());
				}
				if scale.is_some() {
					flex_from_vec3!(vec, scale.unwrap());
				} else {
					vec.push(());
				}
			})
			.as_slice(),
		)
	}
}

#[test]
fn spatial() {
	let client = Client::connect().expect("Couldn't connect");
	let spatial = Spatial::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		mint::Quaternion::from([0_f32, 0_f32, 0_f32, 1_f32]),
		mint::Vector3::from([1_f32, 1_f32, 1_f32]),
		false,
	)
	.unwrap();
	drop(spatial);
	client.dispatch().expect("Dispatch error");
}
