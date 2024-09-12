use super::*;
use crate::{
	client::ClientHandle,
	drawable::ModelPartAspect,
	fields::{Field, FieldAspect},
	impl_aspects,
	node::{Node, NodeResult, NodeType, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
};
use std::sync::Arc;

stardust_xr_fusion_codegen::codegen_item_panel_protocol!();

impl_aspects!(PanelItem: OwnedAspect, SpatialRefAspect, SpatialAspect, ItemAspect);

impl PanelItemUi {
	pub fn register(client: &Arc<ClientHandle>) -> NodeResult<Self> {
		register_panel_item_ui(client)?;
		Ok(PanelItemUi(Node::from_id(client, INTERFACE_NODE_ID, true)))
	}
}

impl_aspects!(PanelItemAcceptor: OwnedAspect, SpatialRefAspect, SpatialAspect, ItemAcceptorAspect);
impl PanelItemAcceptor {
	pub fn create(
		client: &Arc<ClientHandle>,
		parent: &impl SpatialRefAspect,
		transform: Transform,
		field: &impl FieldAspect,
	) -> NodeResult<Self> {
		create_panel_item_acceptor(client, client.generate_id(), parent, transform, field)
	}
}
