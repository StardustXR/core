mod r#box;
mod cylinder;
mod sphere;

pub use cylinder::*;
use mint::Vector3;
pub use r#box::*;
pub use sphere::*;

use crate::{
	node::{BoxedFuture, Node, NodeError, NodeType},
	spatial::Spatial,
};
use anyhow::Result;
use serde::Deserialize;
use std::ops::Deref;

#[derive(Debug, Deserialize)]
pub struct RayMarchResult {
	pub ray_origin: Vector3<f32>,
	pub ray_direction: Vector3<f32>,
	pub min_distance: f32,
	pub deepest_point_distance: f32,
	pub ray_length: f32,
	pub ray_steps: u32,
}
impl RayMarchResult {
	pub fn deepest_point(&self) -> Vector3<f32> {
		Vector3 {
			x: self.ray_origin.x + (self.ray_direction.x * self.deepest_point_distance),
			y: self.ray_origin.y + (self.ray_direction.y * self.deepest_point_distance),
			z: self.ray_origin.z + (self.ray_direction.z * self.deepest_point_distance),
		}
	}
	pub fn hit(&self) -> bool {
		self.min_distance <= 0.0
	}
}

pub trait Field: NodeType {
	fn distance(
		&self,
		space: &Spatial,
		point: impl Into<Vector3<f32>>,
	) -> Result<BoxedFuture<Result<f32>>, NodeError> {
		self.node().execute_remote_method_trait(
			"distance",
			&(space.node.get_path().to_string(), point.into()),
		)
	}

	fn normal(
		&self,
		space: &Spatial,
		point: impl Into<Vector3<f32>>,
	) -> Result<BoxedFuture<Result<mint::Vector3<f32>>>, NodeError> {
		self.node().execute_remote_method_trait(
			"normal",
			&(space.node.get_path().to_string(), point.into()),
		)
	}

	fn closest_point(
		&self,
		space: &Spatial,
		point: impl Into<Vector3<f32>>,
	) -> Result<BoxedFuture<Result<mint::Vector3<f32>>>, NodeError> {
		self.node().execute_remote_method_trait(
			"closestPoint",
			&(space.node.get_path().to_string(), point.into()),
		)
	}

	fn ray_march(
		&self,
		space: &Spatial,
		ray_origin: impl Into<Vector3<f32>>,
		ray_direction: impl Into<Vector3<f32>>,
	) -> Result<BoxedFuture<Result<RayMarchResult>>, NodeError> {
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
			&(space.node.get_path().to_string(), ray_origin, ray_direction),
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

#[derive(Debug)]
pub struct UnknownField {
	pub(crate) spatial: Spatial,
}
impl NodeType for UnknownField {
	fn node(&self) -> &Node {
		self.spatial.node()
	}
}
impl Field for UnknownField {}
impl Deref for UnknownField {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}
