use super::values;
use crate::flex;

use super::{
	client::Client,
	node::{Node, NodeError},
	spatial::Spatial,
};

pub struct Field<'a> {
	pub spatial: Spatial<'a>,
}

impl<'a> Field<'a> {
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

	pub fn closest_point(
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

pub struct BoxField<'a> {
	pub field: Field<'a>,
}
impl<'a> BoxField<'a> {
	pub fn create(
		client: &Client<'a>,
		spatial_parent: &Spatial<'a>,
		position: values::Vec3,
		rotation: values::Quat,
		size: values::Vec3,
	) -> Result<Self, NodeError> {
		let (node, id) = Node::generate_with_parent(client, "/field")?;

		node.messenger
			.upgrade()
			.ok_or(NodeError::InvalidMessenger)?
			.send_remote_signal(
				"/field",
				"createBoxField",
				flex::flexbuffer_from_vector_arguments(|vec| {
					vec.push(id.as_str());
					vec.push(spatial_parent.node.get_path());
					flex_from_vec3!(vec, position);
					flex_from_quat!(vec, rotation);
					flex_from_vec3!(vec, size);
				})
				.as_slice(),
			)
			.map_err(|_| NodeError::ServerCreationFailed)?;

		Ok(BoxField {
			field: Field {
				spatial: Spatial { node },
			},
		})
	}

	pub fn set_size(&self, size: values::Vec3) -> Result<(), NodeError> {
		self.field.spatial.node.send_remote_signal(
			"distance",
			flex::flexbuffer_from_arguments(|fbb| {
				flex_from_vec3!(fbb, size);
			})
			.as_slice(),
		)
	}
}

pub struct CylinderField<'a> {
	pub field: Field<'a>,
}
impl<'a> CylinderField<'a> {
	pub fn create(
		client: &Client<'a>,
		spatial_parent: &Spatial<'a>,
		position: values::Vec3,
		rotation: values::Quat,
		length: f32,
		radius: f32,
	) -> Result<Self, NodeError> {
		let (node, id) = Node::generate_with_parent(client, "/field")?;

		node.messenger
			.upgrade()
			.ok_or(NodeError::InvalidMessenger)?
			.send_remote_signal(
				"/field",
				"createCylinderField",
				flex::flexbuffer_from_vector_arguments(|vec| {
					vec.push(id.as_str());
					vec.push(spatial_parent.node.get_path());
					flex_from_vec3!(vec, position);
					flex_from_quat!(vec, rotation);
					vec.push(length);
					vec.push(radius);
				})
				.as_slice(),
			)
			.map_err(|_| NodeError::ServerCreationFailed)?;

		Ok(CylinderField {
			field: Field {
				spatial: Spatial { node },
			},
		})
	}
}

pub struct SphereField<'a> {
	pub field: Field<'a>,
}
impl<'a> SphereField<'a> {
	pub fn create(
		client: &Client<'a>,
		spatial_parent: &Spatial<'a>,
		position: values::Vec3,
		radius: f32,
	) -> Result<Self, NodeError> {
		let (node, id) = Node::generate_with_parent(client, "/field")?;

		node.messenger
			.upgrade()
			.ok_or(NodeError::InvalidMessenger)?
			.send_remote_signal(
				"/field",
				"createSphereField",
				flex::flexbuffer_from_vector_arguments(|vec| {
					vec.push(id.as_str());
					vec.push(spatial_parent.node.get_path());
					flex_from_vec3!(vec, position);
					vec.push(radius);
				})
				.as_slice(),
			)
			.map_err(|_| NodeError::ServerCreationFailed)?;

		Ok(SphereField {
			field: Field {
				spatial: Spatial { node },
			},
		})
	}
}
