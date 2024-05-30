use super::*;
use crate::{
	client::Client,
	fields::{Field, FieldAspect},
	impl_aspects,
	node::{Node, NodeResult, NodeType, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
};
use stardust_xr::values::*;
use std::sync::Arc;

stardust_xr_fusion_codegen::codegen_item_camera_protocol!();

impl_aspects!(CameraItem: OwnedAspect, SpatialRefAspect, SpatialAspect, ItemAspect);
impl CameraItem {
	pub fn create(
		client: &Arc<Client>,
		parent: &impl SpatialRefAspect,
		transform: Transform,
		proj_matrix: Mat4,
		px_size: Vector2<u32>,
	) -> NodeResult<CameraItem> {
		create_camera_item(
			client,
			&nanoid::nanoid!(),
			parent,
			transform,
			proj_matrix,
			px_size,
		)
	}
}

impl_aspects!(CameraItemUi: ItemUiAspect);
impl CameraItemUi {
	pub fn register(client: &Arc<Client>) -> NodeResult<Self> {
		register_camera_item_ui(client)?;
		Ok(CameraItemUi(Node::from_path(
			client,
			"/item/camera".to_string(),
			false,
		)))
	}
}

impl_aspects!(CameraItemAcceptor: OwnedAspect, SpatialRefAspect, SpatialAspect, ItemAcceptorAspect);
impl CameraItemAcceptor {
	pub fn create(
		client: &Arc<Client>,
		parent: &impl SpatialRefAspect,
		transform: Transform,
		field: &impl FieldAspect,
	) -> NodeResult<Self> {
		create_camera_item_acceptor(client, &nanoid::nanoid!(), parent, transform, field)
	}
}
