//! Analog SDFs to define boundaries for input, interaction, and behavior.

use std::sync::Arc;

use crate::{
	client::ClientHandle,
	node::NodeResult,
	spatial::{SpatialRefAspect, Transform},
};

pub use crate::protocol::field::*;

impl FieldRef {
	pub async fn import(client: &Arc<ClientHandle>, uid: u64) -> NodeResult<Self> {
		import_field_ref(client, uid).await
	}
}
impl Field {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		shape: Shape,
	) -> NodeResult<Self> {
		let client = spatial_parent.client();
		create_field(
			client,
			client.generate_id(),
			spatial_parent,
			transform,
			shape,
		)
	}
}

#[tokio::test]
async fn fusion_field_sphere() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let async_event_loop = client.async_event_loop();
	let root = async_event_loop.client_handle.get_root();
	let field = Field::create(root, Transform::none(), Shape::Sphere(0.5)).unwrap();
	let outside = field.distance(root, [1.0, 0.0, 0.0]).await.unwrap();
	assert!(
		outside > 0.0,
		"point outside sphere should have positive distance"
	);
	let inside = field.distance(root, [0.0, 0.0, 0.0]).await.unwrap();
	assert!(
		inside < 0.0,
		"center of sphere should have negative distance"
	);
}

#[tokio::test]
async fn fusion_field_box() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let async_event_loop = client.async_event_loop();
	let root = async_event_loop.client_handle.get_root();
	let field = Field::create(root, Transform::none(), Shape::Box([1.0, 1.0, 1.0].into())).unwrap();
	let outside = field.distance(root, [2.0, 0.0, 0.0]).await.unwrap();
	assert!(
		outside > 0.0,
		"point outside box should have positive distance"
	);
	let inside = field.distance(root, [0.0, 0.0, 0.0]).await.unwrap();
	assert!(inside < 0.0, "center of box should have negative distance");
}

#[tokio::test]
async fn fusion_field_cylinder() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let async_event_loop = client.async_event_loop();
	let root = async_event_loop.client_handle.get_root();
	let field = Field::create(
		root,
		Transform::none(),
		Shape::Cylinder(CylinderShape {
			length: 1.0,
			radius: 0.5,
		}),
	)
	.unwrap();
	let outside = field.distance(root, [2.0, 0.0, 0.0]).await.unwrap();
	assert!(
		outside > 0.0,
		"point outside cylinder should have positive distance"
	);
	let inside = field.distance(root, [0.0, 0.0, 0.0]).await.unwrap();
	assert!(
		inside < 0.0,
		"center of cylinder should have negative distance"
	);
}

#[tokio::test]
async fn fusion_field_torus() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let async_event_loop = client.async_event_loop();
	let root = async_event_loop.client_handle.get_root();
	let field = Field::create(
		root,
		Transform::none(),
		Shape::Torus(TorusShape {
			radius_a: 0.5,
			radius_b: 0.1,
		}),
	)
	.unwrap();
	// Point on the ring of the torus should be inside
	let inside = field.distance(root, [0.5, 0.0, 0.0]).await.unwrap();
	assert!(
		inside < 0.0,
		"point on torus ring should be inside (negative distance)"
	);
	// Point far away should be outside
	let outside = field.distance(root, [5.0, 0.0, 0.0]).await.unwrap();
	assert!(
		outside > 0.0,
		"point far from torus should have positive distance"
	);
}

#[tokio::test]
async fn fusion_field_spline() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let async_event_loop = client.async_event_loop();
	let root = async_event_loop.client_handle.get_root();
	let field = Field::create(
		root,
		Transform::none(),
		Shape::Spline(CubicSplineShape {
			control_points: vec![
				CubicControlPoint {
					handle_in: [-0.1, 0.0, 0.0].into(),
					anchor: [0.0, 0.0, 0.0].into(),
					handle_out: [0.1, 0.0, 0.0].into(),
					thickness: 0.01,
				},
				CubicControlPoint {
					handle_in: [0.0, 0.0, 0.0].into(),
					anchor: [0.1, 0.0, 0.0].into(),
					handle_out: [0.2, 0.0, 0.0].into(),
					thickness: 0.01,
				},
				CubicControlPoint {
					handle_in: [0.0, 0.0, 0.0].into(),
					anchor: [0.1, 0.1, 0.0].into(),
					handle_out: [0.1, 0.2, 0.0].into(),
					thickness: 0.005,
				},
			],
			cyclic: false,
		}),
	)
	.unwrap();

	// Point far away should be outside the spline tube
	let outside = field.distance(root, [0.5, 5.0, 0.0]).await.unwrap();
	assert!(
		outside > 0.0,
		"point far from spline should have positive distance"
	);
}

#[tokio::test]
async fn fusion_field_set_shape() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let async_event_loop = client.async_event_loop();
	let root = async_event_loop.client_handle.get_root();
	let field = Field::create(root, Transform::none(), Shape::Sphere(0.1)).unwrap();
	// Point is outside the small sphere
	let before = field.distance(root, [0.5, 0.0, 0.0]).await.unwrap();
	assert!(before > 0.0, "point should be outside the small sphere");

	// Grow the sphere to contain the point
	field.set_shape(Shape::Sphere(1.0)).unwrap();
	let after = field.distance(root, [0.5, 0.0, 0.0]).await.unwrap();
	assert!(
		after < 0.0,
		"point should be inside the larger sphere after set_shape"
	);
}

#[tokio::test]
async fn fusion_field_normal() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let async_event_loop = client.async_event_loop();
	let root = async_event_loop.client_handle.get_root();
	let field = Field::create(root, Transform::none(), Shape::Sphere(0.5)).unwrap();
	// Normal at [1, 0, 0] should point in the +X direction
	let normal = field.normal(root, [1.0, 0.0, 0.0]).await.unwrap();
	dbg!(normal);
	assert!(normal.x > 0.5, "normal at [1,0,0] should be roughly +X");
	assert!(normal.y.abs() < 0.5, "normal Y component should be near 0");
	assert!(normal.z.abs() < 0.5, "normal Z component should be near 0");
}

#[tokio::test]
async fn fusion_field_closest_point() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let async_event_loop = client.async_event_loop();
	let root = async_event_loop.client_handle.get_root();
	let field = Field::create(root, Transform::none(), Shape::Sphere(0.5)).unwrap();

	// Closest point on sphere from [2, 0, 0] should be near [0.5, 0, 0]
	let closest = field.closest_point(root, [2.0, 0.0, 0.0]).await.unwrap();
	assert!(
		(closest.x - 0.5).abs() < 0.01,
		"closest point X should be ~0.5, got {}",
		closest.x
	);
	assert!(closest.y.abs() < 0.01, "closest point Y should be ~0");
	assert!(closest.z.abs() < 0.01, "closest point Z should be ~0");
}

#[tokio::test]
async fn fusion_field_ray_march() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let async_event_loop = client.async_event_loop();
	let root = async_event_loop.client_handle.get_root();
	let field = Field::create(root, Transform::none(), Shape::Sphere(0.5)).unwrap();
	// Ray from [2, 0, 0] pointing toward -X should hit the sphere
	let result = field
		.ray_march(root, [2.0, 0.0, 0.0], [-1.0, 0.0, 0.0])
		.await
		.unwrap();
	assert!(
		result.min_distance < 0.0,
		"ray aimed at sphere should intersect (min_distance < 0), got {}",
		result.min_distance
	);
	assert!(
		result.ray_steps > 0,
		"ray march should take at least one step"
	);
}

#[tokio::test]
async fn fusion_field_export_import() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let async_event_loop = client.async_event_loop();
	let root = async_event_loop.client_handle.get_root();
	let field = Field::create(root, Transform::none(), Shape::Sphere(0.5)).unwrap();
	let uid = field.export_field().await.unwrap();
	let field_ref = FieldRef::import(&async_event_loop.client_handle, uid)
		.await
		.unwrap();
	// Both the original field and the imported ref should agree on distance
	let dist_field = field.distance(root, [1.0, 0.0, 0.0]).await.unwrap();
	let dist_ref = field_ref.distance(root, [1.0, 0.0, 0.0]).await.unwrap();
	assert!(
		(dist_field - dist_ref).abs() < 0.001,
		"Field and imported FieldRef should return the same distance"
	);
}
