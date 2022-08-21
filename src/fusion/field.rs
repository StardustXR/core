use super::{
	node::GenNodeInfo,
	node::{Node, NodeError},
	spatial::Spatial,
	values::{Quat, Vec3},
};
use crate::{
	flex,
	fusion::values::{QUAT_IDENTITY, VEC3_ONE, VEC3_ZERO},
};
use anyhow::{anyhow, Result};

pub struct Field {
	pub spatial: Spatial,
}

impl Field {
	pub async fn distance(&self, space: &Spatial, point: Vec3) -> Result<f32> {
		self.spatial
			.node
			.execute_remote_method(
				"distance",
				&flex::flexbuffer_from_vector_arguments(|vec_builder| {
					push_to_vec!(vec_builder, space.node.get_path(), point);
				}),
			)
			.await
			.map(|data| {
				let root = flexbuffers::Reader::get_root(data.as_slice()).unwrap();
				root.get_f64().unwrap_or(0_f64) as f32
			})
	}

	pub async fn normal(&self, space: &Spatial, point: Vec3) -> Result<Vec3> {
		self.spatial
			.node
			.execute_remote_method(
				"normal",
				&flex::flexbuffer_from_vector_arguments(|vec_builder| {
					push_to_vec!(vec_builder, space.node.get_path(), point);
				}),
			)
			.await
			.and_then(|data| {
				let root = flexbuffers::Reader::get_root(data.as_slice()).unwrap();
				flex_to_vec3!(root).ok_or_else(|| anyhow!("Parsing error"))
			})
	}

	pub async fn closest_point(&self, space: &Spatial, point: Vec3) -> Result<Vec3> {
		self.spatial
			.node
			.execute_remote_method(
				"closestPoint",
				&flex::flexbuffer_from_vector_arguments(|vec_builder| {
					push_to_vec!(vec_builder, space.node.get_path(), point);
				}),
			)
			.await
			.and_then(|data| {
				let root = flexbuffers::Reader::get_root(data.as_slice()).unwrap();
				flex_to_vec3!(root).ok_or_else(|| anyhow!("Parsing error"))
			})
	}
}

pub struct BoxField {
	pub field: Field,
}
#[buildstructor::buildstructor]
impl<'a> BoxField {
	#[builder(entry = "builder")]
	pub async fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		size: Option<Vec3>,
	) -> Result<Self, NodeError> {
		Ok(BoxField {
			field: Field {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client: spatial_parent.node.client.clone(),
							parent_path: "/field",
							interface_path: "/field",
							interface_method: "createBoxField"
						},
						spatial_parent.node.get_path(),
						position.unwrap_or(VEC3_ZERO),
						rotation.unwrap_or(QUAT_IDENTITY),
						size.unwrap_or(VEC3_ONE)
					),
				},
			},
		})
	}

	pub async fn set_size(&self, size: Vec3) -> Result<(), NodeError> {
		self.field
			.spatial
			.node
			.send_remote_signal(
				"distance",
				flex::flexbuffer_from_arguments(|fbb| {
					flex_from_vec3!(fbb, size);
				})
				.as_slice(),
			)
			.await
	}
}

#[tokio::test]
async fn fusion_box_field() {
	use super::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let box_field = BoxField::builder()
		.spatial_parent(client.get_root())
		.build()
		.await
		.expect("Unable to make box field");

	let client_captured = client.clone();
	box_field
		.set_size(mint::Vector3::from([0.5_f32, 0.5_f32, 0.5_f32]))
		.await
		.expect("Unable to set box field size");
	let distance = box_field
		.field
		.distance(
			client_captured.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
		)
		.await
		.expect("Unable to get box field distance");
	assert_eq!(distance, 1_f32);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}

pub struct CylinderField {
	pub field: Field,
}
#[buildstructor::buildstructor]
impl<'a> CylinderField {
	#[builder(entry = "builder")]
	pub async fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		length: f32,
		radius: f32,
	) -> Result<Self, NodeError> {
		Ok(CylinderField {
			field: Field {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client: spatial_parent.node.client.clone(),
							parent_path: "/field",
							interface_path: "/field",
							interface_method: "createCylinderField"
						},
						spatial_parent.node.get_path(),
						position.unwrap_or(VEC3_ZERO),
						rotation.unwrap_or(QUAT_IDENTITY),
						length,
						radius
					),
				},
			},
		})
	}
}

#[tokio::test]
async fn fusion_cylinder_field() {
	use super::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let cylinder_field = CylinderField::builder()
		.spatial_parent(client.get_root())
		.length(1.0)
		.radius(0.5)
		.build()
		.await
		.expect("Unable to make cylinder field");
	let distance = cylinder_field
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
		)
		.await
		.expect("Unable to cylinder box field distance");
	assert_eq!(distance, 1_f32);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}

pub struct SphereField {
	pub field: Field,
}
#[buildstructor::buildstructor]
impl<'a> SphereField {
	#[builder(entry = "builder")]
	pub async fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		radius: f32,
	) -> Result<Self, NodeError> {
		Ok(SphereField {
			field: Field {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client: spatial_parent.node.client.clone(),
							parent_path: "/field",
							interface_path: "/field",
							interface_method: "createSphereField"
						},
						spatial_parent.node.get_path(),
						position.unwrap_or(VEC3_ZERO),
						radius
					),
				},
			},
		})
	}
}

#[tokio::test]
async fn fusion_sphere_field() {
	use super::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let sphere_field = SphereField::builder()
		.spatial_parent(client.get_root())
		.radius(0.5)
		.build()
		.await
		.expect("Unable to make sphere field");
	let distance = sphere_field
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
		)
		.await
		.expect("Unable to get sphere field distance");
	assert_eq!(distance, 1_f32);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}
