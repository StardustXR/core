//! Analog SDFs to define boundaries for input, interaction, and behavior.

mod r#box;
mod cylinder;
mod sphere;
mod torus;

pub use cylinder::*;
pub use r#box::*;
pub use sphere::*;
pub use torus::*;

use crate::{
	node::{BoxedFuture, Node, NodeError, NodeType},
	spatial::Spatial,
};
use anyhow::Result;
use mint::Vector3;
use serde::Deserialize;
use std::ops::Deref;

// Information about raymarching a field, all values are in the space entered in.
#[derive(Debug, Deserialize)]
pub struct RayMarchResult {
	pub ray_origin: Vector3<f32>,
	pub ray_direction: Vector3<f32>,
	/// The minimum distance this ray ever got to the field, including inside the field.
	/// This can let you see how close to the center something is pointing.
	pub min_distance: f32,
	/// The length along the ray that the point closest/deepest inside the field lies.
	pub deepest_point_distance: f32,
	pub ray_length: f32,
	pub ray_steps: u32,
}
impl RayMarchResult {
	/// Get the point on the ray that is closest/deepest in the field.
	pub fn deepest_point(&self) -> Vector3<f32> {
		Vector3 {
			x: self.ray_origin.x + (self.ray_direction.x * self.deepest_point_distance),
			y: self.ray_origin.y + (self.ray_direction.y * self.deepest_point_distance),
			z: self.ray_origin.z + (self.ray_direction.z * self.deepest_point_distance),
		}
	}
	/// Did this ray hit the field at all?
	pub fn hit(&self) -> bool {
		self.min_distance <= 0.0
	}
}

// #[enum_dispatch]
// pub enum FieldType {
// 	BoxField,
// 	CylinderField,
// 	SphereField,
// 	TorusField,
// 	UnknownField,
// }

/// A node that is spatial and contains an SDF.
///
/// This is used in place of colliders as it provides a much more analog and emergent set of behaviors, leading to more intuitive design.
// #[enum_dispatch(FieldType)]
pub trait Field: NodeType {
	/// Get the distance from a point in the given space to the field's surface. Outside is a positive distance, inside is negative.
	fn distance(
		&self,
		space: &Spatial,
		point: impl Into<Vector3<f32>>,
	) -> Result<BoxedFuture<Result<f32, NodeError>>, NodeError>
	where
		Self: Sized,
	{
		self.node()
			.execute_remote_method_trait("distance", &(space.node().get_path()?, point.into()))
	}

	/// Get the normal vector pointing outside the field at the point in the given space. As this is a field, normals extend beyond the surface.
	fn normal(
		&self,
		space: &Spatial,
		point: impl Into<Vector3<f32>>,
	) -> Result<BoxedFuture<Result<Vector3<f32>, NodeError>>, NodeError>
	where
		Self: Sized,
	{
		self.node()
			.execute_remote_method_trait("normal", &(space.node().get_path()?, point.into()))
	}

	/// Get the closest point on the surface from the point in the given space.
	fn closest_point(
		&self,
		space: &Spatial,
		point: impl Into<Vector3<f32>>,
	) -> Result<BoxedFuture<Result<Vector3<f32>, NodeError>>, NodeError>
	where
		Self: Sized,
	{
		self.node()
			.execute_remote_method_trait("closestPoint", &(space.node().get_path()?, point.into()))
	}

	/// Ray march through the given field at a server-defined ray march step length.
	fn ray_march(
		&self,
		space: &Spatial,
		ray_origin: impl Into<Vector3<f32>>,
		ray_direction: impl Into<Vector3<f32>>,
	) -> Result<BoxedFuture<Result<RayMarchResult, NodeError>>, NodeError>
	where
		Self: Sized,
	{
		let ray_origin = ray_origin.into();
		let ray_direction = ray_direction.into();
		#[derive(Debug, Deserialize)]
		struct RayMarchResultRaw {
			min_distance: f32,
			deepest_point_distance: f32,
			ray_length: f32,
			ray_steps: u32,
		}

		let future = self.node().execute_remote_method_trait(
			"closestPoint",
			&(space.node().get_path()?, ray_origin, ray_direction),
		)?;

		Ok(Box::pin(async move {
			let raw_result: RayMarchResultRaw = future.await?;
			Ok(RayMarchResult {
				ray_origin,
				ray_direction,
				min_distance: raw_result.min_distance,
				deepest_point_distance: raw_result.deepest_point_distance,
				ray_length: raw_result.ray_length,
				ray_steps: raw_result.ray_steps,
			})
		}))
	}
}

/// A field that isn't owned by the client, so no data is known about it.
#[derive(Debug)]
pub struct UnknownField {
	pub(crate) spatial: Spatial,
}
impl NodeType for UnknownField {
	fn node(&self) -> &Node {
		&self.spatial.node()
	}

	fn alias(&self) -> Self {
		UnknownField {
			spatial: self.spatial.alias(),
		}
	}
}
impl Field for UnknownField {}
impl Deref for UnknownField {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}
