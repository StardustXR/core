use super::values;
use crate::flex;

use super::node::Node;

pub struct Spatial<'a> {
	node: Node<'a>,
}

impl<'a> Spatial<'a> {
	pub fn create() {}

	pub fn get_transform(
		&mut self,
		space: &Spatial,
		callback: impl Fn(values::Vec3, values::Quat, values::Vec3) + 'a,
	) -> Result<(), std::io::Error> {
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
		&mut self,
		space: &Spatial,
		position: Option<values::Vec3>,
		rotation: Option<values::Quat>,
		scale: Option<values::Vec3>,
	) -> Result<(), std::io::Error> {
		self.node.send_remote_signal(
			"setTransform",
			flex::flexbuffer_from_arguments(|fbb| {
				let mut vec = fbb.start_vector();
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
				vec.end_vector();
			})
			.as_slice(),
		)
	}
}
