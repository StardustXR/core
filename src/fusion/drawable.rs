use std::convert::TryFrom;
use std::path::Path;
use super::values;
use crate::flex;

use super::{
	client::Client,
	node::{Node, NodeError},
	spatial::Spatial,
};

use std::rc::Rc;
use crate::fusion::utilmacros::{FlexBuffable, GenNodeInfo};

pub struct Drawable<'a> {
	pub spatial: Spatial<'a>,
}
pub struct Model<'a> {
	pub drawable: Drawable<'a>
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
		let mut object_name = String::from("/model/");
		object_name.push_str(String::from(client.messenger.generate_message_id()).as_str());
		Ok(Model {
			drawable: Drawable {
				spatial: Spatial {
					node: Rc::new(
						generate_node!(
								GenNodeInfo{
									client: &client,
									spatial_parent: &spatial_parent,
									parent_name: "/drawable",
									object_name: object_name,
									method_name: "createModelFromFile"
								},
								file_path,
								position,
								rotation,
								scale))
				},
			}
		})
	}
}
