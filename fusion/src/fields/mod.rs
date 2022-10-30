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
use std::ops::Deref;

pub trait Field: NodeType {
	fn distance(
		&self,
		space: &Spatial,
		point: Vector3<f32>,
	) -> Result<BoxedFuture<Result<f32>>, NodeError> {
		self.node()
			.execute_remote_method_trait("distance", &(space.node.get_path().to_string(), point))
	}

	fn normal(
		&self,
		space: &Spatial,
		point: Vector3<f32>,
	) -> Result<BoxedFuture<Result<mint::Vector3<f32>>>, NodeError> {
		self.node()
			.execute_remote_method_trait("normal", &(space.node.get_path().to_string(), point))
	}

	fn closest_point(
		&self,
		space: &Spatial,
		point: Vector3<f32>,
	) -> Result<BoxedFuture<Result<mint::Vector3<f32>>>, NodeError> {
		self.node().execute_remote_method_trait(
			"closestPoint",
			&(space.node.get_path().to_string(), point),
		)
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
