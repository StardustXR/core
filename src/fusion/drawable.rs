use super::{
	client::Client,
	node::{Node, NodeError},
	spatial::Spatial,
	values,
};
use crate::{flex, fusion::utilmacros::GenNodeInfo};
use std::path::{Path, PathBuf};
use color::{AlphaColor, Rgba, rgba};
use crate::fusion::values::{Color, TextFit};

pub struct Drawable<'a> {
	pub spatial: Spatial<'a>,
}
pub struct Model<'a> {
	pub drawable: Drawable<'a>,
}
pub struct Text<'a> {
	pub drawable: Drawable<'a>,
}
//TODO add tests and finish completeting this.
impl<'a> Model<'a> {
	pub fn create(
		client: &Client<'a>,
		spatial_parent: &Spatial<'a>,
		file_path: &Path,
		position: values::Vec3,
		rotation: values::Quat,
		scale: values::Vec3,
	) -> Result<Self, NodeError> {
		Ok(Model {
			drawable: Drawable {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client,
							spatial_parent,
							parent_path: "/model",
							interface_path: "/drawable",
							interface_method: "createModelFromFile"
						},
						PathBuf::from(file_path),
						position,
						rotation,
						scale
					),
				},
			},
		})
	}
}
impl <'a> Text<'a> {
	pub fn create(
		client: &Client<'a>,
		spatial_parent: &Spatial<'a>,
		text_string: &str,
		position: values::Vec3,
		rotation: values::Quat,
		character_height: Option<f32>,
		color: Option<values::Color>,
		font_path: Option<&str>,
		text_alignment: Option<values::Alignment>,
		bounds: Option<values::Vec2>,
		fit: Option<TextFit>,
		bounds_align: Option<values::Alignment>,
	) -> Result<Self, NodeError> {
		let mut bounds_align_2 = values::Alignment::TopLeft;
		let mut text_alignment_2 = values::Alignment::TopLeft;
		let mut bounds_2 = values::Vec2::from([0f32, 0f32]);
		let mut fit_2 = values::TextFit::Overflow;
		let mut font_path_2 = "";
		let mut character_height_2 = 1.0f32;
		let mut color_2 = rgba!(255, 255, 255, 255);
		if character_height.is_some() {character_height_2 = character_height.unwrap();}
		if bounds_align.is_some() {bounds_align_2 = bounds_align.unwrap();}
		if text_alignment.is_some() {text_alignment_2 = text_alignment.unwrap();}
		if bounds.is_some() {bounds_2 = bounds.unwrap();}
		if fit.is_some() {fit_2 = fit.unwrap();}
		if font_path.is_some() {font_path_2 = font_path.unwrap();}
		if color.is_some() {color_2 = color.unwrap();}
		Ok(Text {
			drawable: Drawable {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client,
							spatial_parent,
							parent_path: "/text",
							interface_path: "/drawable",
							interface_method: "createText"
						},
						position,
						rotation,
						text_string,
						font_path_2,
						character_height_2,
						text_alignment_2 as i32,
						bounds_2,
						fit_2 as i32,
						bounds_align_2 as i32,
						//color_2
						0//TODO switch out 0 with color_2 when color_2 works.
					),
				},
			},
		})
	}
}
