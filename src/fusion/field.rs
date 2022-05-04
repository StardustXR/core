use super::values;
use crate::flex;

use super::{client::Client, node::NodeError, spatial::Spatial};

pub struct Field<'a> {
	pub spatial: Spatial<'a>,
}

impl<'a> Field<'a> {
	pub fn from_path(client: &Client<'a>, path: &str) -> Result<Self, NodeError> {
		Ok(Field {
			spatial: Spatial::from_path(client, path)?,
		})
	}

	pub fn distance(
		&self,
		space: &Spatial,
		point: values::Vec3,
		callback: impl Fn(f32) + 'a,
	) -> Result<(), NodeError> {
		self.spatial.node.execute_remote_method(
			"distance",
			flex::flexbuffer_from_vector_arguments(|vec| {
				vec.push(space.node.get_path());
				flex_from_vec3!(vec, point);
			})
			.as_slice(),
			Box::new(move |data| {
				let root = flexbuffers::Reader::get_root(data).unwrap();
				callback(root.get_f64().unwrap_or(0_f64) as f32);
			}),
		)
	}

	pub fn normal(
		&self,
		space: &Spatial,
		point: values::Vec3,
		callback: impl Fn(values::Vec3) + 'a,
	) -> Result<(), NodeError> {
		self.spatial.node.execute_remote_method(
			"normal",
			flex::flexbuffer_from_vector_arguments(|vec| {
				vec.push(space.node.get_path());
				flex_from_vec3!(vec, point);
			})
			.as_slice(),
			Box::new(move |data| {
				let root = flexbuffers::Reader::get_root(data).unwrap();
				callback(flex_to_vec3!(root));
			}),
		)
	}

	pub fn closestPoint(
		&self,
		space: &Spatial,
		point: values::Vec3,
		callback: impl Fn(values::Vec3) + 'a,
	) -> Result<(), NodeError> {
		self.spatial.node.execute_remote_method(
			"closestPoint",
			flex::flexbuffer_from_vector_arguments(|vec| {
				vec.push(space.node.get_path());
				flex_from_vec3!(vec, point);
			})
			.as_slice(),
			Box::new(move |data| {
				let root = flexbuffers::Reader::get_root(data).unwrap();
				callback(flex_to_vec3!(root));
			}),
		)
	}
}
