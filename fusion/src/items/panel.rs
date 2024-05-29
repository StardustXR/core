use super::*;
use crate::{
	client::Client,
	drawable::ModelPartAspect,
	fields::{Field, FieldAspect},
	impl_aspects,
	node::{Node, NodeResult, NodeType, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
};
use stardust_xr::values::*;
use std::sync::Arc;

stardust_xr_fusion_codegen::codegen_item_panel_protocol!();

impl_aspects!(PanelItem: OwnedAspect, SpatialRefAspect, SpatialAspect, ItemAspect);

impl PanelItemUi {
	pub fn register(client: &Arc<Client>) -> NodeResult<Self> {
		register_panel_item_ui(client)?;
		Ok(PanelItemUi(Node::from_path(
			client,
			"/item/panel".to_string(),
			false,
		)))
	}
}

impl_aspects!(PanelItemAcceptor: OwnedAspect, SpatialRefAspect, SpatialAspect, ItemAcceptorAspect);
impl PanelItemAcceptor {
	pub fn create(
		client: &Arc<Client>,
		parent: &impl SpatialRefAspect,
		transform: Transform,
		field: &impl FieldAspect,
	) -> NodeResult<Self> {
		create_panel_item_acceptor(client, &nanoid::nanoid!(), parent, transform, field)
	}
}
