use super::{
	client::Client,
	node::GenNodeInfo,
	node::{Node, NodeError},
	spatial::Spatial,
	values,
};
use crate::flex;
use anyhow::{anyhow, Result};

pub struct Field {
	pub spatial: Spatial,
}

impl Field {
	pub async fn distance(&self, space: &Spatial, point: values::Vec3) -> Result<f32> {
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

	pub async fn normal(&self, space: &Spatial, point: values::Vec3) -> Result<values::Vec3> {
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

	pub async fn closest_point(
		&self,
		space: &Spatial,
		point: values::Vec3,
	) -> Result<values::Vec3> {
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
impl BoxField {
	pub async fn create(
		client: &Client,
		spatial_parent: &Spatial,
		position: values::Vec3,
		rotation: values::Quat,
		size: values::Vec3,
	) -> Result<Self, NodeError> {
		Ok(BoxField {
			field: Field {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client,
							parent_path: "/field",
							interface_path: "/field",
							interface_method: "createBoxField"
						},
						spatial_parent.node.get_path(),
						position,
						rotation,
						size
					),
				},
			},
		})
	}

	pub async fn set_size(&self, size: values::Vec3) -> Result<(), NodeError> {
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
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let box_field = BoxField::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		mint::Quaternion::from([0_f32, 0_f32, 0_f32, 1_f32]),
		mint::Vector3::from([1_f32, 1_f32, 1_f32]),
	)
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
impl CylinderField {
	pub async fn create(
		client: &Client,
		spatial_parent: &Spatial,
		position: values::Vec3,
		rotation: values::Quat,
		length: f32,
		radius: f32,
	) -> Result<Self, NodeError> {
		Ok(CylinderField {
			field: Field {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client,
							parent_path: "/field",
							interface_path: "/field",
							interface_method: "createCylinderField"
						},
						spatial_parent.node.get_path(),
						position,
						rotation,
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
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let cylinder_field = CylinderField::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		mint::Quaternion::from([0_f32, 0_f32, 0_f32, 1_f32]),
		1_f32,
		0.5_f32,
	)
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
impl SphereField {
	pub async fn create(
		client: &Client,
		spatial_parent: &Spatial,
		position: values::Vec3,
		radius: f32,
	) -> Result<Self, NodeError> {
		Ok(SphereField {
			field: Field {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client,
							parent_path: "/field",
							interface_path: "/field",
							interface_method: "createSphereField"
						},
						spatial_parent.node.get_path(),
						position,
						radius
					),
				},
			},
		})
	}
}

#[tokio::test]
async fn fusion_sphere_field() {
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let sphere_field = SphereField::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		0.5_f32,
	)
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
