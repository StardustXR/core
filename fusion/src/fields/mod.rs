mod r#box;
mod cylinder;
mod sphere;

pub use cylinder::*;
use futures::Future;
use mint::Vector3;
pub use r#box::*;
pub use sphere::*;

use anyhow::Result;
use std::ops::Deref;

use crate::{node::NodeError, spatial::Spatial};

#[derive(Debug)]
pub struct Field {
	pub spatial: Spatial,
}
impl Field {
	pub fn distance(
		&self,
		space: &Spatial,
		point: Vector3<f32>,
	) -> Result<impl Future<Output = Result<f32>>, NodeError> {
		self.spatial
			.node
			.execute_remote_method("distance", &(space.node.get_path().to_string(), point))
	}

	pub fn normal(
		&self,
		space: &Spatial,
		point: Vector3<f32>,
	) -> Result<impl Future<Output = Result<mint::Vector3<f32>>>, NodeError> {
		self.spatial
			.node
			.execute_remote_method("normal", &(space.node.get_path().to_string(), point))
	}

	pub fn closest_point(
		&self,
		space: &Spatial,
		point: Vector3<f32>,
	) -> Result<impl Future<Output = Result<mint::Vector3<f32>>>, NodeError> {
		self.spatial
			.node
			.execute_remote_method("closestPoint", &(space.node.get_path().to_string(), point))
	}
}
impl Deref for Field {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}
