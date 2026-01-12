pub use crate::protocol::camera::*;
use crate::{
	node::NodeResult,
	spatial::{SpatialRefAspect, Transform},
};
impl Camera {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
	) -> NodeResult<Camera> {
		let client = spatial_parent.client();
		create_camera(client, client.generate_id(), spatial_parent, transform)
	}
}
