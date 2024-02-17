//! Anything the user can see such as lines, models and text.

use crate::{
	node::{NodeAspect, NodeError, NodeResult, NodeType},
	spatial::{SpatialAspect, Transform},
};
use color::rgba_linear;
use nanoid::nanoid;
use stardust_xr::values::ResourceID;

stardust_xr_fusion_codegen::codegen_drawable_client_protocol!();

impl Lines {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		lines: &[Line],
	) -> NodeResult<Self> {
		create_lines(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			lines,
		)
	}
}
impl Default for LinePoint {
	fn default() -> Self {
		Self {
			point: [0.0; 3].into(),
			thickness: 0.01,
			color: rgba_linear!(1.0, 1.0, 1.0, 1.0),
		}
	}
}
impl Copy for LinePoint {}
impl Default for Line {
	fn default() -> Self {
		Self {
			points: Default::default(),
			cyclic: Default::default(),
		}
	}
}

impl Model {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		model: &ResourceID,
	) -> NodeResult<Self> {
		load_model(
			&spatial_parent.client()?,
			&nanoid::nanoid!(),
			spatial_parent,
			transform,
			model,
		)
	}

	/// Set a property of a material on this model.
	pub fn model_part(&self, relative_path: &str) -> NodeResult<ModelPart> {
		if relative_path.starts_with('/') {
			return Err(NodeError::InvalidPath);
		}
		Ok(ModelPart::from_parent_name(
			&self.client()?,
			&self.node().get_path()?,
			relative_path,
			false,
		))
	}
}
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
async fn fusion_lines() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.unwrap();

	let points = vec![
		LinePoint {
			point: mint::Vector3 {
				x: 1.0,
				y: 0.0,
				z: 0.0,
			},
			thickness: 0.0025,
			..Default::default()
		},
		LinePoint {
			thickness: 0.0025,
			..Default::default()
		},
		LinePoint {
			point: mint::Vector3 {
				x: 0.0,
				y: 1.0,
				z: 0.0,
			},
			thickness: 0.0025,
			..Default::default()
		},
	];
	let line = Line {
		points,
		cyclic: true,
	};
	let _lines = Lines::create(client.get_root(), Transform::none(), &[line]).unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}

#[tokio::test]
async fn fusion_model() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.unwrap();
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let gyro_resource = ResourceID::new_namespaced("fusion", "gyro");
	let gyro_model = Model::create(client.get_root(), Transform::none(), &gyro_resource).unwrap();
	gyro_model
		.model_part("Gem")
		.unwrap()
		.set_material_parameter(
			"color",
			MaterialParameter::Color(color::rgba_linear!(0.0, 1.0, 0.5, 0.75)),
		)
		.unwrap();

	let spike_resource = ResourceID::new_namespaced("fusion", "cursor_spike");
	let spike_model = Model::create(
		client.get_root(),
		Transform::from_translation_scale([0.0, 0.1, 0.0], [0.1; 3]),
		&spike_resource,
	)
	.unwrap();
	spike_model
		.model_part("Cone")
		.unwrap()
		.apply_holdout_material()
		.unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
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

#[tokio::test]
async fn fusion_sky() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);
	let sky_resource = stardust_xr::values::ResourceID::new_namespaced("fusion", "sky");

	set_sky_light(&client, &sky_resource).unwrap();
	set_sky_tex(&client, &sky_resource).unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(5)).await;
}
