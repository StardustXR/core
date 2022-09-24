mod r#box;
mod cylinder;
mod sphere;

pub use cylinder::*;
use futures::Future;
pub use r#box::*;
pub use sphere::*;

use anyhow::{anyhow, Result};
use stardust_xr::{flex, flex_to_vec3, push_to_vec, values::Vec3};
use std::ops::Deref;

use crate::{node::NodeError, spatial::Spatial};

pub struct Field {
	pub spatial: Spatial,
}
impl Field {
	pub fn distance(
		&self,
		space: &Spatial,
		point: Vec3,
	) -> Result<impl Future<Output = Result<f32>>, NodeError> {
		let future = self.spatial.node.execute_remote_method(
			"distance",
			&flex::flexbuffer_from_vector_arguments(|vec_builder| {
				push_to_vec!(vec_builder, space.node.get_path(), point);
			}),
		)?;
		Ok(async move {
			future.await.map(|data| {
				let root = flexbuffers::Reader::get_root(data.as_slice()).unwrap();
				root.get_f64().unwrap_or(0_f64) as f32
			})
		})
	}

	pub fn normal(
		&self,
		space: &Spatial,
		point: Vec3,
	) -> Result<impl Future<Output = Result<Vec3>>, NodeError> {
		let future = self.spatial.node.execute_remote_method(
			"normal",
			&flex::flexbuffer_from_vector_arguments(|vec_builder| {
				push_to_vec!(vec_builder, space.node.get_path(), point);
			}),
		)?;
		Ok(async move {
			future.await.and_then(|data| {
				let root = flexbuffers::Reader::get_root(data.as_slice()).unwrap();
				flex_to_vec3!(root).ok_or_else(|| anyhow!("Parsing error"))
			})
		})
	}

	pub fn closest_point(
		&self,
		space: &Spatial,
		point: Vec3,
	) -> Result<impl Future<Output = Result<Vec3>>, NodeError> {
		let future = self.spatial.node.execute_remote_method(
			"closestPoint",
			&flex::flexbuffer_from_vector_arguments(|vec_builder| {
				push_to_vec!(vec_builder, space.node.get_path(), point);
			}),
		)?;
		Ok(async move {
			future.await.and_then(|data| {
				let root = flexbuffers::Reader::get_root(data.as_slice()).unwrap();
				flex_to_vec3!(root).ok_or_else(|| anyhow!("Parsing error"))
			})
		})
	}
}
impl Deref for Field {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}
