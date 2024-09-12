//! Create audio!

use crate::{
	impl_aspects,
	node::{NodeResult, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
};
use stardust_xr::values::*;

stardust_xr_fusion_codegen::codegen_audio_protocol!();

impl_aspects!(Sound: OwnedAspect, SpatialRefAspect, SpatialAspect);
impl Sound {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		resource: &ResourceID,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_sound(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			resource,
		)
	}
}
#[tokio::test]
async fn fusion_sound() {
	let mut client = crate::client::Client::connect().await.unwrap();

	let lightspeed_resource = ResourceID::new_namespaced("fusion", "kittn_lightspeed");
	let sound = Sound::create(client.get_root(), Transform::none(), &lightspeed_resource).unwrap();
	sound.play().unwrap();
	client.try_flush().await.unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(10)).await;
	sound.stop().unwrap();
	client.try_flush().await.unwrap();
	tokio::time::sleep(core::time::Duration::from_secs(2)).await;
}
