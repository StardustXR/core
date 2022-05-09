use super::{
	client::Client,
	node::{Node, NodeError},
	spatial::Spatial,
	values,
};
use crate::{flex, fusion::utilmacros::GenNodeInfo};
use std::{
	path::{Path, PathBuf},
	rc::Rc,
};

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
		Ok(Model {
			drawable: Drawable {
				spatial: Spatial {
					node: Rc::new(generate_node!(
						GenNodeInfo {
							client: client,
							spatial_parent: &spatial_parent,
							parent_path: "/model",
							interface_path: "/drawable",
							interface_method: "createModelFromFile"
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
