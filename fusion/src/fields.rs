//! Analog SDFs to define boundaries for input, interaction, and behavior.

use crate::{
	impl_aspects,
	node::{NodeResult, NodeType, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
};
use stardust_xr::values::*;

stardust_xr_fusion_codegen::codegen_field_protocol!();

impl_aspects!(Field: SpatialRefAspect);
impl Field {
	pub fn alias_field<Fi: FieldAspect>(field: &Fi) -> Self {
		Field(field.node().alias())
	}
}
impl_aspects!(BoxField: OwnedAspect, SpatialRefAspect, SpatialAspect, FieldAspect);
impl BoxField {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		size: impl Into<Vector3<f32>>,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_box_field(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			size.into(),
		)
	}
}

impl_aspects!(CylinderField: OwnedAspect, SpatialRefAspect, SpatialAspect, FieldAspect);
impl CylinderField {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		radius: f32,
		length: f32,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_cylinder_field(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			radius,
			length,
		)
	}
}
impl_aspects!(TorusField: OwnedAspect, SpatialRefAspect, SpatialAspect, FieldAspect);
impl TorusField {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		radius_a: f32,
		radius_b: f32,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_torus_field(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			radius_a,
			radius_b,
		)
	}
}
impl_aspects!(SphereField: OwnedAspect, SpatialRefAspect, SpatialAspect, FieldAspect);
impl SphereField {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		position: impl Into<Vector3<f32>>,
		radius: f32,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_sphere_field(
			&client,
			client.generate_id(),
			spatial_parent,
			position,
			radius,
		)
	}
}
