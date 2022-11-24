use crate::{
	node::{Node, NodeError},
	resource::Resource,
	spatial::Spatial,
};
use anyhow::Result;
use color::rgba;
use flagset::{flags, FlagSet};
use mint::Vector2;
use stardust_xr::values::Transform;
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
#[derive(Debug, Clone, Copy)]
pub enum TextFit {
	Wrap = 1 << 0,
	Clip = 1 << 1,
	Squeeze = 1 << 2,
	Exact = 1 << 3,
	Overflow = 1 << 4,
}

#[derive(Debug)]
pub struct TextStyle<R: Resource> {
	pub character_height: f32,
	pub color: color::Rgba,
	pub font_resource: Option<R>,
	pub text_align: FlagSet<Alignment>,
	pub bounds: Option<Vector2<f32>>,
	pub fit: TextFit,
	pub bounds_align: FlagSet<Alignment>,
}

impl<R: Resource> Default for TextStyle<R> {
	fn default() -> Self {
		TextStyle {
			character_height: 1.0,
			color: rgba!(255, 255, 255, 255),
			font_resource: None,
			text_align: Alignment::TopLeft.into(),
			bounds: None,
			fit: TextFit::Overflow,
			bounds_align: Alignment::TopLeft.into(),
		}
	}
}

#[derive(Debug)]
pub struct Text {
	pub spatial: Spatial,
}
#[buildstructor::buildstructor]
impl Text {
	#[builder(entry = "builder")]
	pub fn create<'a, R: Resource>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		scale: Option<mint::Vector3<f32>>,
		text_string: &'a str,
		style: TextStyle<R>,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(Text {
			spatial: Spatial {
				node: Node::new(
					spatial_parent.node.client.clone(),
					"/drawable",
					"create_text",
					"/drawable/text",
					true,
					&id.clone(),
					(
						id,
						spatial_parent,
						Transform {
							position,
							rotation,
							scale,
						},
						text_string,
						style.font_resource.map(|res| res.parse()),
						style.character_height,
						style.text_align.bits(),
						style.bounds,
						style.fit as u32,
						style.bounds_align.bits(),
						[
							style.color.c.r,
							style.color.c.g,
							style.color.c.b,
							style.color.a,
						],
					),
				)?,
			},
		})
	}

	pub fn set_character_height(&self, height: f32) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("set_character_height", &height)
	}
	pub fn set_text(&self, text: impl AsRef<str>) -> Result<(), NodeError> {
		self.node.send_remote_signal("set_text", &text.as_ref())
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
