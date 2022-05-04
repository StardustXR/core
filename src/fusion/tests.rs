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
