use super::{
	client::Client,
	node::GenNodeInfo,
	node::{Node, NodeError},
	spatial::Spatial,
	values,
};
use crate::flex;

pub struct Field<'a> {
	pub spatial: Spatial<'a>,
}

impl<'a> Field<'a> {
	pub fn distance(
		&self,
		space: &Spatial,
		point: values::Vec3,
		callback: impl Fn(f32) + 'static,
	) -> Result<(), NodeError> {
		self.spatial.node.execute_remote_method(
			"distance",
			flex::flexbuffer_from_vector_arguments(|vec_builder| {
				push_to_vec!(vec_builder, space.node.get_path(), point);
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
		callback: impl Fn(values::Vec3) + 'static,
	) -> Result<(), NodeError> {
		self.spatial.node.execute_remote_method(
			"normal",
			flex::flexbuffer_from_vector_arguments(|vec_builder| {
				push_to_vec!(vec_builder, space.node.get_path(), point);
			})
			.as_slice(),
			Box::new(move |data| {
				let root = flexbuffers::Reader::get_root(data).unwrap();
				callback(flex_to_vec3!(root).unwrap());
			}),
		)
	}

	pub fn closest_point(
		&self,
		space: &Spatial,
		point: values::Vec3,
		callback: impl Fn(values::Vec3) + 'static,
	) -> Result<(), NodeError> {
		self.spatial.node.execute_remote_method(
			"closestPoint",
			flex::flexbuffer_from_vector_arguments(|vec_builder| {
				push_to_vec!(vec_builder, space.node.get_path(), point);
			})
			.as_slice(),
			Box::new(move |data| {
				let root = flexbuffers::Reader::get_root(data).unwrap();
				callback(flex_to_vec3!(root).unwrap());
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

#[test]
fn fusion_box_field() {
	let client = Client::connect().expect("Couldn't connect");

	println!("Creating box field");
	let box_field = BoxField::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		mint::Quaternion::from([0_f32, 0_f32, 0_f32, 1_f32]),
		mint::Vector3::from([1_f32, 1_f32, 1_f32]),
	)
	.expect("Unable to make box field");
	box_field
		.set_size(mint::Vector3::from([0.5_f32, 0.5_f32, 0.5_f32]))
		.expect("Unable to set box field size");
	box_field
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
			|distance| assert_eq!(distance, 1_f32),
		)
		.expect("Unable to get box field distance");

	let cylinder_field = CylinderField::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		mint::Quaternion::from([0_f32, 0_f32, 0_f32, 1_f32]),
		1_f32,
		0.5_f32,
	)
	.expect("Unable to make cylinder field");
	cylinder_field
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
			|distance| assert_eq!(distance, 1_f32),
		)
		.expect("Unable to cylinder box field distance");

	let sphere_field = SphereField::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		0.5_f32,
	)
	.expect("Unable to make sphere field");
	sphere_field
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
			|distance| assert_eq!(distance, 1_f32),
		)
		.expect("Unable to get sphere field distance");

	while client.messenger.dispatch(&client.scenegraph).is_ok() {}
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

#[test]
fn fusion_cylinder_field() {
	let client = Client::connect().expect("Couldn't connect");

	let cylinder_field = CylinderField::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		mint::Quaternion::from([0_f32, 0_f32, 0_f32, 1_f32]),
		1_f32,
		0.5_f32,
	)
	.expect("Unable to make cylinder field");
	cylinder_field
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
			|distance| assert_eq!(distance, 1_f32),
		)
		.expect("Unable to cylinder box field distance");

	while client.messenger.dispatch(&client.scenegraph).is_ok() {}
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

#[test]
fn fusion_sphere_field() {
	let client = Client::connect().expect("Couldn't connect");
	let sphere_field = SphereField::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		0.5_f32,
	)
	.expect("Unable to make sphere field");
	sphere_field
		.field
		.distance(
			client.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
			|distance| assert_eq!(distance, 1_f32),
		)
		.expect("Unable to get sphere field distance");
	// client.run_event_loop(None);
}
