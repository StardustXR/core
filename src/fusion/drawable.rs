use super::values;
use crate::flex;
use std::path::{Path, PathBuf};

use super::{
	client::Client,
	node::{Node, NodeError},
	spatial::Spatial,
};

use crate::fusion::utilmacros::GenNodeInfo;
use std::rc::Rc;

pub struct Drawable<'a> {
	pub spatial: Spatial<'a>,
}
pub struct Model<'a> {
	pub drawable: Drawable<'a>,
}
//TODO add tests and finish completeting this.
impl<'a> Model<'a> {
	pub fn create(
		client: &Client<'a>,
		spatial_parent: &Spatial<'a>,
		file_path: &Path,
		position: values::Vec3,
		rotation: values::Quat,
		scale: values::Vec3,
	) -> Result<Self, NodeError> {
		let object_name =
			String::from(format!("/model/{}", client.messenger.generate_message_id()));
		Ok(Model {
			drawable: Drawable {
				spatial: Spatial {
					node: Rc::new(generate_node!(
						GenNodeInfo {
							client: client,
							spatial_parent: &spatial_parent,
							parent_name: "/drawable",
							object_name: object_name.as_str(),
							method_name: "createModelFromFile"
						},
						PathBuf::from(file_path),
						position,
						rotation,
						scale
					)),
				},
			},
		})
	}
}
