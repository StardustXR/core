use super::{
	client::Client,
	field::Field,
	node::{Node, NodeError},
	spatial::Spatial,
	utilmacros::GenNodeInfo,
	values,
};
use crate::flex;

pub struct PulseReceiver<'a> {
	pub spatial: Spatial<'a>,
	pub field: &'a Field<'a>,
}

impl<'a> PulseReceiver<'a> {
	pub fn create(
		client: &Client<'a>,
		spatial_parent: &Spatial<'a>,
		position: values::Vec3,
		rotation: values::Quat,
		field: &'a Field<'a>,
	) -> Result<Self, NodeError> {
		Ok(PulseReceiver {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client,
						spatial_parent,
						parent_path: "/data/receiver",
						interface_path: "/data",
						interface_method: "createPulseReceiver"
					},
					position,
					rotation,
					field.spatial.node.get_path()
				),
			},
			field,
		})
	}
}
