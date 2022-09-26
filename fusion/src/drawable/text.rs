use crate::{
	node::{GenNodeInfo, Node, NodeError},
	resource::Resource,
	spatial::Spatial,
};
use anyhow::Result;
use color::rgba;
use flagset::{flags, FlagSet};
use stardust_xr::values::{Color, Quat, Transform, Vec2, Vec3};
use std::ops::Deref;

flags! {
pub enum Alignment: u8 {
	XLeft = 1 << 0,
	YTop = 1 << 1,
	XCenter = 1 << 2,
	YCenter = 1 << 3,
	XRight = 1 << 4,
	YBottom = 1 << 5,
	Center = (Alignment::XCenter | Alignment::YCenter).bits(),
	CenterLeft = (Alignment::XLeft | Alignment::YCenter).bits(),
	CenterRight = (Alignment::XRight | Alignment::YCenter).bits(),
	TopCenter = (Alignment::XCenter | Alignment::YTop).bits(),
	TopLeft = (Alignment::XLeft | Alignment::YTop).bits(),
	TopRight = (Alignment::XRight | Alignment::YTop).bits(),
	BottomCenter = (Alignment::XCenter | Alignment::YBottom).bits(),
	BottomLeft = (Alignment::XLeft | Alignment::YBottom).bits(),
	BottomRight = (Alignment::XRight | Alignment::YBottom).bits(),
}
}
#[derive(Clone, Copy)]
pub enum TextFit {
	Wrap = 1 << 0,
	Clip = 1 << 1,
	Squeeze = 1 << 2,
	Exact = 1 << 3,
	Overflow = 1 << 4,
}

pub struct TextStyle<R: Resource> {
	character_height: f32,
	color: Color,
	font_resource: Option<R>,
	text_align: FlagSet<Alignment>,
	bounds: Vec2,
	fit: TextFit,
	bounds_align: FlagSet<Alignment>,
}

impl<R: Resource> Default for TextStyle<R> {
	fn default() -> Self {
		TextStyle {
			character_height: 1_f32,
			color: rgba!(255, 255, 255, 255),
			font_resource: None,
			text_align: Alignment::TopLeft.into(),
			bounds: Vec2::from([0f32, 0f32]),
			fit: TextFit::Overflow,
			bounds_align: Alignment::TopLeft.into(),
		}
	}
}

pub struct Text {
	pub spatial: Spatial,
}
#[buildstructor::buildstructor]
impl Text {
	#[builder(entry = "builder")]
	fn create<'a, R: Resource>(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		scale: Option<Vec3>,
		text_string: &'a str,
		style: TextStyle<R>,
	) -> Result<Self, NodeError> {
		Ok(Text {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client: spatial_parent.node.client.clone(),
						parent_path: "/drawable/text",
						interface_path: "/drawable",
						interface_method: "createText"
					},
					spatial_parent.node.get_path(),
					Transform {
						position,
						rotation,
						scale
					},
					text_string,
					style.font_resource.map(|res| res.parse()),
					style.character_height,
					style.text_align.bits(),
					style.bounds,
					style.fit as u8,
					style.bounds_align.bits(),
					style.color
				),
			},
		})
	}

	pub fn set_character_height(&self, height: f32) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("setCharacterHeight", &flexbuffers::singleton(height))
	}
	pub fn set_text(&self, text: impl AsRef<str>) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("setText", &flexbuffers::singleton(text.as_ref()))
	}
}
impl Deref for Text {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_text() -> Result<()> {
	use crate::resource::NamespacedResource;
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop().await?;
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let mut style: TextStyle<NamespacedResource> = TextStyle::default();
	style.font_resource = Some(NamespacedResource::new("fusion", "common_case.ttf"));

	let text = Text::builder()
		.spatial_parent(client.get_root())
		.text_string("Test Text")
		.style(style)
		.build()?;

	text.set_character_height(0.05)?;
	text.set_text("Test Text: Changed")?;

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
	Ok(())
}
