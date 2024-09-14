//! Analog SDFs to define boundaries for input, interaction, and behavior.

use std::sync::Arc;

use crate::{
	client::ClientHandle,
	impl_aspects,
	node::{NodeResult, OwnedAspect},
	spatial::{Spatial, SpatialAspect, SpatialRef, SpatialRefAspect, Transform},
};

stardust_xr_fusion_codegen::codegen_field_protocol!();

impl_aspects!(FieldRef: SpatialRefAspect);
impl FieldRef {
	pub async fn import(client: &Arc<ClientHandle>, uid: u64) -> NodeResult<Self> {
		import_field_ref(client, uid).await
	}
}
impl_aspects!(Field: OwnedAspect, SpatialRefAspect, SpatialAspect, FieldRefAspect);
impl Field {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		shape: Shape,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_field(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			shape,
		)
	}
}

// TODO: write proper tests for each field shape and setting shape
