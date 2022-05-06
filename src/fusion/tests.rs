use super::*;

#[test]
fn connect() {
	client::Client::connect().expect("Couldn't connect");
}

#[test]
fn create_spatial() {
	let client = client::Client::connect().expect("Couldn't connect");
	spatial::Spatial::create(
		&client,
		client.get_root(),
		mint::Vector3::from([0_f32, 0_f32, 0_f32]),
		mint::Quaternion::from([0_f32, 0_f32, 0_f32, 1_f32]),
		mint::Vector3::from([1_f32, 1_f32, 1_f32]),
		true,
		true,
		true,
		false,
	)
	.ok();
}

#[test]
fn test_fields() {
	let client = client::Client::connect().expect("Couldn't connect");

	println!("Creating box field");
	let box_field = field::BoxField::create(
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

	println!("Creating cylinder field");
	let cylinder_field = field::CylinderField::create(
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

	println!("Creating sphere field");
	let sphere_field = field::SphereField::create(
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
