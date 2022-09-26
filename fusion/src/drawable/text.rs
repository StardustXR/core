use crate::{
	client::Client,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
};
use anyhow::Result;
use color::rgba;
use flagset::{flags, FlagSet};
use stardust_xr::values::{Color, Quat, Vec2, Vec3};
use std::sync::Weak;

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

pub struct TextStyle {
	character_height: f32,
	color: Color,
	font_path: String,
	text_align: FlagSet<Alignment>,
	bounds: Vec2,
	fit: TextFit,
	bounds_align: FlagSet<Alignment>,
}

impl Default for TextStyle {
	fn default() -> Self {
		TextStyle {
			character_height: 1_f32,
			color: rgba!(255, 255, 255, 255),
			font_path: "".to_owned(),
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
impl Text {
	#[allow(clippy::redundant_clone)]
	pub async fn create(
		client: Weak<Client>,
		spatial_parent: &Spatial,
		text_string: &str,
		position: Vec3,
		rotation: Quat,
		style: TextStyle,
	) -> Result<Self, NodeError> {
		Ok(Text {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client: client.clone(),
						parent_path: "/text",
						interface_path: "/drawable",
						interface_method: "createText"
					},
					spatial_parent.node.get_path(),
					position,
					rotation,
					text_string,
					style.font_path.as_str(),
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
}
