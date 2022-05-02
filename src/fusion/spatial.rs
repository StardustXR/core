use crate::flex;
use super::values;

use super::node::Node;

pub struct Spatial<'a> {
	node: &'a mut Node<'a>,
}

impl<'a> Spatial<'a> {
	fn get_transform(
		&mut self,
		space: &Spatial,
		callback: &impl Fn(values::Vec3, values::Quat, values::Vec3),
	) -> Result<(), std::io::Error> {
		self.node.execute_remote_method(
			"getTransform",
			flex::flexbuffer_from_arguments(|fbb| fbb.build_singleton(space.node.get_path())).as_slice(),
			|data| {
				// TODO: get the callback working
			}
		)
	}
	fn set_transform(
		&mut self,
		space: &Spatial,
		position: Option<values::Vec3>,
		rotation: Option<values::Quat>,
		scale: Option<values::Vec3>,
	) -> Result<(), std::io::Error> {
		self.node.send_signal(
			"setTransform",
			flex::flexbuffer_from_arguments(|fbb| {
				if position.as_ref().is_some() { flex_vec3!(fbb, position.as_ref().unwrap()); }
				if rotation.as_ref().is_some() { flex_quat!(fbb, rotation.as_ref().unwrap()); }
				if scale   .as_ref().is_some() { flex_vec3!(fbb, scale   .as_ref().unwrap()); }
			}).as_slice()
		)
	}
}
