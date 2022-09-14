use super::{
	client::Client,
	node::{GenNodeInfo, Node, NodeError},
	resource::Resource,
	spatial::Spatial,
};
use crate::{
	push_to_vec,
	values::{Color, Quat, Vec2, Vec3, QUAT_IDENTITY, VEC3_ONE, VEC3_ZERO},
};
use anyhow::Result;
use color::{rgba, Rgba};
use flagset::{flags, FlagSet};
use flexbuffers::VectorBuilder;
use std::{
	ops::Deref,
	path::{Path, PathBuf},
	sync::Weak,
};

pub trait MaterialParameter {
	fn push_flex(&self, vec: &mut VectorBuilder);
}
impl MaterialParameter for f32 {
	fn push_flex(&self, vec: &mut VectorBuilder) {
		vec.push(*self);
	}
}
impl MaterialParameter for f64 {
	fn push_flex(&self, vec: &mut VectorBuilder) {
		vec.push(*self);
	}
}
impl MaterialParameter for Rgba {
	fn push_flex(&self, vec: &mut VectorBuilder) {
		let mut color_vec = vec.start_vector();
		color_vec.push(self.c.r);
		color_vec.push(self.c.g);
		color_vec.push(self.c.b);
		color_vec.push(self.a);
	}
}

pub struct Model {
	pub spatial: Spatial,
}
pub struct Text {
	pub spatial: Spatial,
}
#[buildstructor::buildstructor]
impl<'a> Model {
	#[builder(entry = "file_builder")]
	pub fn from_file(
		spatial_parent: &'a Spatial,
		file_path: &'a Path,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		scale: Option<Vec3>,
	) -> Result<Self, NodeError> {
		Ok(Model {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client: spatial_parent.node.client.clone(),
						parent_path: "/drawable/model",
						interface_path: "/drawable",
						interface_method: "createModelFromFile"
					},
					spatial_parent.node.get_path(),
					PathBuf::from(file_path),
					position.unwrap_or(VEC3_ZERO),
					rotation.unwrap_or(QUAT_IDENTITY),
					scale.unwrap_or(VEC3_ONE)
				),
			},
		})
	}
	#[builder(entry = "resource_builder")]
	pub fn from_resource(
		spatial_parent: &'a Spatial,
		resource: &'a Resource,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		scale: Option<Vec3>,
	) -> Result<Self, NodeError> {
		Ok(Model {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client: spatial_parent.node.client.clone(),
						parent_path: "/drawable/model",
						interface_path: "/drawable",
						interface_method: "createModelFromResource"
					},
					spatial_parent.node.get_path(),
					resource.namespace.as_str(),
					resource.path.as_str(),
					position.unwrap_or(VEC3_ZERO),
					rotation.unwrap_or(QUAT_IDENTITY),
					scale.unwrap_or(VEC3_ONE)
				),
			},
		})
	}
}
impl Deref for Model {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_model() -> Result<()> {
	use manifest_dir_macros::directory_relative_path;
	let client = super::client::Client::connect().await?;
	client.set_base_prefixes(&[directory_relative_path!("res")])?;

	let _model = Model::resource_builder()
		.spatial_parent(client.get_root())
		.resource(&Resource::new("libstardustxr", "gyro_gem.glb"))
		.build()?;

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
	Ok(())
}

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
