use super::{
	client::Client,
	spatial::Spatial,
	values::{Quat, Vec3},
};
use std::path::PathBuf;
use crate::fusion::values::Vec2;

macro_rules! generate_node {
	($gen_node_info:expr, $($things_to_pass:expr),*) => {
		{
			let (node, id) = Node::generate_with_parent($gen_node_info.client, $gen_node_info.parent_path)?;
			node.messenger
				.upgrade()
				.ok_or(NodeError::InvalidMessenger)?
				.send_remote_signal(
					$gen_node_info.interface_path,
					$gen_node_info.interface_method,
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
	pub(crate) parent_path: &'b str,
	pub(crate) interface_path: &'b str,
	pub(crate) interface_method: &'b str,
}
macro_rules! push_to_vec {
	($vec:expr, $thing_to_pass:expr) => {{
		{
			match crate::fusion::utilmacros::FlexBuffable::from($thing_to_pass) {
				crate::fusion::utilmacros::FlexBuffable::Float(f) => {$vec.push(f)},
				crate::fusion::utilmacros::FlexBuffable::Vec3(vec3) => {flex_from_vec3!($vec, vec3)},
				crate::fusion::utilmacros::FlexBuffable::Quat(quat) => {flex_from_quat!($vec, quat)},
				crate::fusion::utilmacros::FlexBuffable::Boolean(my_bool) => {$vec.push(my_bool)},
				crate::fusion::utilmacros::FlexBuffable::String(my_string) => {$vec.push(my_string.as_str())},
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
	Integer(i32),
	Boolean(bool),
	Vec3(Vec3),
	Vec2(Vec2),
	Quat(Quat),
	String(String),
}
impl From<f32> for FlexBuffable {
	fn from(var: f32) -> Self {
		FlexBuffable::Float(var)
	}
}
impl From<i32> for FlexBuffable {
	fn from(var: i32) -> Self {
		FlexBuffable::Integer(var)
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
impl From<Vec2> for FlexBuffable {
	fn from(var: Vec2) -> Self {
		FlexBuffable::Vec2(var)
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
impl From<PathBuf> for FlexBuffable {
	fn from(var: PathBuf) -> Self {
		FlexBuffable::String(String::from(var.to_str().unwrap()))
	}
}
