use super::{
	client::Client,
	field::Field,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
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
						parent_path: "/data/receiver",
						interface_path: "/data",
						interface_method: "createPulseReceiver"
					},
					spatial_parent.node.get_path(),
					position,
					rotation,
					field.spatial.node.get_path()
				),
			},
			field,
		})
	}
}
