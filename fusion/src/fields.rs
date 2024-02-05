//! Analog SDFs to define boundaries for input, interaction, and behavior.

use crate::{
	node::{NodeAspect, NodeResult, NodeType},
	spatial::{SpatialAspect, Transform},
};
use mint::Vector3;
use nanoid::nanoid;

stardust_xr_fusion_codegen::codegen_field_client_protocol!();
impl UnknownField {
	pub fn alias_field<Fi: FieldAspect>(field: &Fi) -> Self {
		UnknownField(field.node().alias())
	}
}
impl BoxField {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		size: impl Into<Vector3<f32>>,
	) -> NodeResult<Self> {
		create_box_field(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			size.into(),
		)
	}
}
impl CylinderField {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		radius: f32,
		length: f32,
	) -> NodeResult<Self> {
		create_cylinder_field(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			radius,
			length,
		)
	}
}
impl TorusField {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		radius_a: f32,
		radius_b: f32,
	) -> NodeResult<Self> {
		create_torus_field(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			radius_a,
			radius_b,
		)
	}
}
impl SphereField {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		position: impl Into<Vector3<f32>>,
		radius: f32,
	) -> NodeResult<Self> {
		create_sphere_field(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			position,
			radius,
		)
	}
}
