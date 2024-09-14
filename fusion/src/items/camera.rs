use super::*;
use crate::{
	client::ClientHandle,
	fields::{Field, FieldAspect},
	impl_aspects,
	node::{NodeResult, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
};
use stardust_xr::values::*;
use std::sync::Arc;

stardust_xr_fusion_codegen::codegen_item_camera_protocol!();

impl_aspects!(CameraItem: OwnedAspect, SpatialRefAspect, SpatialAspect, ItemAspect);
impl CameraItem {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		proj_matrix: Mat4,
		px_size: Vector2<u32>,
	) -> NodeResult<CameraItem> {
		let client = spatial_parent.client()?;
		create_camera_item(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			proj_matrix,
			px_size,
		)
	}
}

impl_aspects!(CameraItemUi: ItemUiAspect);
impl CameraItemUi {
	pub fn register(client: &Arc<ClientHandle>) -> NodeResult<Self> {
		register_camera_item_ui(client)?;
		Ok(CameraItemUi::from_id(client, INTERFACE_NODE_ID, true))
	}
}

impl_aspects!(CameraItemAcceptor: OwnedAspect, SpatialRefAspect, SpatialAspect, ItemAcceptorAspect);
impl CameraItemAcceptor {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		field: &impl FieldAspect,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_camera_item_acceptor(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			field,
		)
	}
}
