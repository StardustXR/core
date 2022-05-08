use std::convert::TryInto;
use crate::fusion::client::Client;
use crate::fusion::spatial::Spatial;
use crate::fusion::values::{Quat, Vec3};
//TODO this isn't finished.
macro_rules! generate_node {
	($gen_node_info:expr, $($things_to_pass:expr),*) => {
		{
				let (node, id) = Node::generate_with_parent($gen_node_info.client, $gen_node_info.parent_name)?;
				node.messenger
					.upgrade()
					.ok_or(NodeError::InvalidMessenger)?
					.send_remote_signal(
						$gen_node_info.object_name,
						$gen_node_info.method_name,
						flex::flexbuffer_from_vector_arguments(|vec| {
							push_to_vec![vec, id.as_str(), $gen_node_info.spatial_parent.node.get_path(), $($things_to_pass),+]
						})
						.as_slice(),
					)
					.map_err(|_| NodeError::ServerCreationFailed)?;
					node
		}

	}
}
pub struct GenNodeInfo<'a, 'b> {
	pub(crate) client: &'b Client<'a>,
	pub(crate) spatial_parent: &'b Spatial<'a>,
	pub(crate) parent_name: &'b str,
	pub(crate) object_name: &'b str,
	pub(crate) method_name: &'b str
}

macro_rules! push_to_vec {
	($vec:expr, $thing_to_pass:expr) => {{
		{
			match FlexBuffable::from($thing_to_pass) {
				FlexBuffable::Float(f) => {$vec.push(f)},
				FlexBuffable::Vec3(vec3) => {flex_from_vec3!($vec, vec3)},
				FlexBuffable::Quat(quat) => {flex_from_quat!($vec, quat)},
				FlexBuffable::Boolean(my_bool) => {$vec.push(my_bool)},
				FlexBuffable::String(my_string) => {$vec.push(my_string.as_str())},
			}
		}
	}};
	($vec:expr, $first_thing:expr, $($thing_to_pass:expr),+) => {{
		{
			push_to_vec! {$vec, $first_thing}
			push_to_vec! {$vec, $($thing_to_pass),+}
		}
	}};
}
pub enum FlexBuffable {
	Float(f32),
	Boolean(bool),
	Vec3(Vec3),
	Quat(Quat),
	String(String),
}
impl From<f32> for FlexBuffable {
	fn from(var: f32) -> Self {
		FlexBuffable::Float(var)
	}
}
impl From<bool> for FlexBuffable {
	fn from(var: bool) -> Self {
		FlexBuffable::Boolean(var)
	}
}
impl From<Vec3> for FlexBuffable {
	fn from(var: Vec3) -> Self {
		FlexBuffable::Vec3(var)
	}
}
impl From<Quat> for FlexBuffable {
	fn from(var: Quat) -> Self {
		FlexBuffable::Quat(var)
	}
}
impl From<String> for FlexBuffable {
	fn from(var: String) -> Self {
		FlexBuffable::String(var)
	}
}
impl From<&str> for FlexBuffable {
	fn from(var: &str) -> Self {
		FlexBuffable::String(String::from(var))
	}
}

