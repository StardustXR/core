use super::*;
use crate::{
	client::ClientHandle,
	drawable::ModelPartAspect,
	fields::{Field, FieldAspect},
	node::{NodeResult, NodeType, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
};
use std::sync::Arc;

stardust_xr_fusion_codegen::codegen_item_panel_protocol!();
impl Copy for Geometry {}
impl Copy for SurfaceId {}

impl PanelItemUi {
	pub fn register(client: &Arc<ClientHandle>) -> NodeResult<Self> {
		register_panel_item_ui(client)?;
		// TODO: properly autogen this adding of aspect
		let panel_item_ui = PanelItemUi::from_id(client, INTERFACE_NODE_ID, true);
		client
			.scenegraph
			.add_aspect::<ItemUiEvent>(panel_item_ui.node());
		Ok(panel_item_ui)
	}
}

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

#[cfg(feature = "keymap")]
pub use xkbcommon::xkb;
#[cfg(feature = "keymap")]
use xkbcommon::xkb::{Context, FORMAT_TEXT_V1, KEYMAP_COMPILE_NO_FLAGS, Keymap};
#[cfg(feature = "keymap")]
impl crate::client::ClientHandle {
	pub fn register_xkb_keymap(
		&self,
		keymap_string: String,
	) -> NodeResult<impl std::future::Future<Output = NodeResult<u64>> + Send + Sync> {
		let client = self.get_root().client();
		Keymap::new_from_string(
			&Context::new(0),
			keymap_string.clone(),
			FORMAT_TEXT_V1,
			KEYMAP_COMPILE_NO_FLAGS,
		)
		.ok_or_else(|| crate::node::NodeError::ReturnedError {
			e: "Invalid keymap".to_string(),
		})?;
		Ok(async move { register_keymap(&client?, &keymap_string).await })
	}
	pub async fn get_xkb_keymap(&self, keymap_id: u64) -> NodeResult<Keymap> {
		let keymap_str = get_keymap(&self.get_root().client()?, keymap_id).await?;

		Keymap::new_from_string(
			&Context::new(0),
			keymap_str,
			FORMAT_TEXT_V1,
			KEYMAP_COMPILE_NO_FLAGS,
		)
		.ok_or_else(|| crate::node::NodeError::InvalidPath)
	}
}
