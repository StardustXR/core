use super::{
	client::Client,
	node::{GenNodeInfo, Node, NodeError},
	values,
};
use crate::flex;
use std::rc::Rc;

pub struct Spatial<'a> {
	pub node: Rc<Node<'a>>,
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
				callback(pos, rot, scl);
			}),
		)
	}
	pub fn set_transform(
		&self,
		space: &Spatial,
		position: Option<values::Vec3>,
		rotation: Option<values::Quat>,
		scale: Option<values::Vec3>,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"setTransform",
			flex::flexbuffer_from_vector_arguments(|vec| {
				vec.push(space.node.get_path());
				if position.as_ref().is_some() {
					flex_from_vec3!(vec, position.as_ref().unwrap());
				}
				if rotation.as_ref().is_some() {
					flex_from_quat!(vec, rotation.as_ref().unwrap());
				}
				if scale.as_ref().is_some() {
					flex_from_vec3!(vec, scale.as_ref().unwrap());
				}
			})
			.as_slice(),
		)
	}
}

#[test]
fn create_spatial() {
	let client = Client::connect().expect("Couldn't connect");
	Spatial::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		mint::Quaternion::from([0_f32, 0_f32, 0_f32, 1_f32]),
		mint::Vector3::from([1_f32, 1_f32, 1_f32]),
		false,
	)
	.ok();
}
