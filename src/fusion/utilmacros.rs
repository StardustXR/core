use std::convert::TryInto;
use crate::fusion::values::{Quat, Vec3};
//TODO this isn't finished.
macro_rules! generate_node {
	($scope_tuple:expr,$parent:expr,$object:expr,$method_name:expr,$code:block) => {
		{
				let (node, id) = Node::generate_with_parent($scope_tuple.0, $parent)?;
				node.messenger
					.upgrade()
					.ok_or(NodeError::InvalidMessenger)?
					.send_remote_signal(
						$object,
						$method_name,
						flex::flexbuffer_from_vector_arguments(|vec| {
							vec.push(id.as_str());
							vec.push($scope_tuple.1.node.get_path());
							$code
						})
						.as_slice(),
					)
					.map_err(|_| NodeError::ServerCreationFailed)?;
		}
	}
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

