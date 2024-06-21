//! Analog SDFs to define boundaries for input, interaction, and behavior.

use crate::{
	impl_aspects,
	node::{NodeResult, NodeType, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
};

stardust_xr_fusion_codegen::codegen_field_protocol!();

impl_aspects!(FieldRef: SpatialRefAspect);
impl FieldRef {
	pub fn alias_field<Fi: FieldRefAspect>(field: &Fi) -> Self {
		FieldRef(field.node().alias())
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
