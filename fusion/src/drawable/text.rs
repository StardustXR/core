use color::rgba_linear;
use nanoid::nanoid;

use crate::{
	node::{NodeAspect, NodeResult},
	spatial::{SpatialAspect, Transform},
};

stardust_xr_fusion_codegen::codegen_drawable_text_client_protocol!();
impl Text {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		text: &str,
		style: TextStyle,
	) -> NodeResult<Self> {
		create_text(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			text,
			style,
		)
	}
}
impl Default for TextStyle {
	fn default() -> Self {
		Self {
			character_height: 0.01,
			color: rgba_linear!(1.0, 1.0, 1.0, 1.0),
			font: Default::default(),
			text_align_x: XAlign::Left,
			text_align_y: YAlign::Top,
			bounds: Default::default(),
		}
	}
}

#[tokio::test]
async fn fusion_text() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.unwrap();
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let style: TextStyle = TextStyle {
		font: Some(stardust_xr::values::ResourceID::new_namespaced(
			"fusion",
			"common_case",
		)),
		..Default::default()
	};
	let text = Text::create(client.get_root(), Transform::none(), "Test Text", style).unwrap();
	text.set_character_height(0.05).unwrap();
	text.set_text("Test Text: Changed").unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}
