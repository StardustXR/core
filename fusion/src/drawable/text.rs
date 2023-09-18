use super::ResourceID;
use crate::{
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
};
use color::{rgba, Rgba};

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

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
	pub bounds: Vector2<f32>,
	pub fit: TextFit,
	pub bounds_align: FlagSet<Alignment>,
}

#[derive(Debug, Clone)]
pub struct TextStyle {
	pub character_height: f32,
	pub color: Rgba<f32>,
	pub font_resource: Option<ResourceID>,
	pub text_align: FlagSet<Alignment>,
	pub bounds: Option<Bounds>,
}

impl Default for TextStyle {
	fn default() -> Self {
		TextStyle {
			character_height: 1.0,
			color: rgba!(1.0, 1.0, 1.0, 1.0),
			font_resource: None,
			text_align: Alignment::TopLeft.into(),
			bounds: None,
		}
	}
}

/// 2D text in 3D space.
#[derive(Debug)]
pub struct Text {
	spatial: Spatial,
}
impl Text {
	pub fn create(
		spatial_parent: &Spatial,
		transform: Transform,
		text_string: &str,
		style: TextStyle,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(Text {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/drawable",
					"create_text",
					"/drawable/text",
					true,
					&id.clone(),
					(
						id,
						spatial_parent.node().get_path()?,
						transform,
						text_string,
						style.font_resource.map(|res| res.parse()),
						style.character_height,
						style.text_align.bits(),
						style.bounds.as_ref().map(|b| b.bounds),
						style
							.bounds
							.as_ref()
							.map(|b| b.fit)
							.unwrap_or(TextFit::Overflow) as u32,
						style
							.bounds
							.as_ref()
							.map(|b| b.bounds_align)
							.unwrap_or(Alignment::TopLeft.into())
							.bits(),
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

	/// "global" height in meters, regardless of scale.
	pub fn set_character_height(&self, height: f32) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("set_character_height", &height)
	}
	pub fn set_text(&self, text: impl AsRef<str>) -> Result<(), NodeError> {
		self.node.send_remote_signal("set_text", &text.as_ref())
	}
}
impl NodeType for Text {
	fn node(&self) -> &Node {
		&self.spatial.node()
	}

	fn alias(&self) -> Self {
		Text {
			spatial: self.spatial.alias(),
		}
	}
}
impl Deref for Text {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
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
		font_resource: Some(ResourceID::new_namespaced("fusion", "common_case")),
		..Default::default()
	};
	let text = Text::create(client.get_root(), Transform::none(), "Test Text", style).unwrap();
	text.set_character_height(0.05).unwrap();
	text.set_text("Test Text: Changed").unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}
