//! Create audio!

use crate::{impl_aspects, node::{NodeAspect, NodeResult}, spatial::{SpatialAspect, Transform}};
use nanoid::nanoid;
use stardust_xr::values::ResourceID;

stardust_xr_fusion_codegen::codegen_audio_protocol!();

impl_aspects!(Sound: NodeAspect, SpatialAspect);
impl Sound {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		resource: &ResourceID,
	) -> NodeResult<Self> {
		create_sound(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			resource,
		)
	}
}
#[tokio::test]
async fn fusion_sound() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.unwrap();
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let lightspeed_resource = ResourceID::new_namespaced("fusion", "kittn_lightspeed");
	let sound = Sound::create(client.get_root(), Transform::none(), &lightspeed_resource).unwrap();
	sound.play().unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(10)).await;
	sound.stop().unwrap();
	tokio::time::sleep(core::time::Duration::from_secs(2)).await;
}
