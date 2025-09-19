//! Anything the user can see such as lines, models and text.

use std::hash::Hash;

use crate::{
	node::{NodeResult, NodeType},
	spatial::{SpatialRefAspect, Transform},
};
use stardust_xr::values::*;

pub use crate::protocol::drawable::*;

impl Lines {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		lines: &[Line],
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_lines(
			&client,
			client.generate_id(),
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
			color: color::rgba_linear!(1.0, 1.0, 1.0, 1.0),
		}
	}
}
impl Copy for LinePoint {}
impl Hash for LinePoint {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.color.c.r.to_bits().hash(state);
		self.color.c.g.to_bits().hash(state);
		self.color.c.b.to_bits().hash(state);
		self.color.a.to_bits().hash(state);

		self.point.x.to_bits().hash(state);
		self.point.y.to_bits().hash(state);
		self.point.z.to_bits().hash(state);

		self.thickness.to_bits().hash(state);
	}
}
impl Default for Line {
	fn default() -> Self {
		Self {
			points: Default::default(),
			cyclic: Default::default(),
		}
	}
}
impl Hash for Line {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.cyclic.hash(state);
		self.points.hash(state);
	}
}

impl Model {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		model: &ResourceID,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		load_model(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			model,
		)
	}
	pub fn part(&self, relative_path: &str) -> NodeResult<ModelPart> {
		let client = self.client()?;
		self.bind_model_part(client.generate_id(), relative_path)
	}
}
impl Text {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		text: &str,
		style: TextStyle,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_text(
			&client,
			client.generate_id(),
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
			color: color::rgba_linear!(1.0, 1.0, 1.0, 1.0),
			font: Default::default(),
			text_align_x: XAlign::Left,
			text_align_y: YAlign::Top,
			bounds: Default::default(),
		}
	}
}
impl Hash for TextBounds {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.bounds.x.to_bits().hash(state);
		self.bounds.y.to_bits().hash(state);

		self.fit.hash(state);
		self.anchor_align_x.hash(state);
		self.anchor_align_y.hash(state);
	}
}
impl Hash for TextStyle {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.character_height.to_bits().hash(state);

		self.color.c.r.to_bits().hash(state);
		self.color.c.g.to_bits().hash(state);
		self.color.c.b.to_bits().hash(state);
		self.color.a.to_bits().hash(state);

		self.font.hash(state);
		self.text_align_x.hash(state);
		self.text_align_y.hash(state);
		self.bounds.hash(state);
	}
}

#[tokio::test]
async fn fusion_lines() {
	let mut client = crate::Client::connect().await.unwrap();

	let points = vec![
		LinePoint {
			point: Vector3 {
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
			point: Vector3 {
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

	client.flush().await.unwrap();
	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}

#[tokio::test]
async fn fusion_model() {
	let mut client = crate::Client::connect().await.unwrap();

	let gyro_resource = ResourceID::new_namespaced("fusion", "gyro");
	let gyro_model = Model::create(client.get_root(), Transform::none(), &gyro_resource).unwrap();
	gyro_model
		.part("Gem")
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
		.part("Cone")
		.unwrap()
		.apply_holdout_material()
		.unwrap();

	client.flush().await.unwrap();
	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}
#[tokio::test]
async fn fusion_text() {
	let mut client = crate::Client::connect().await.unwrap();

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

	client.flush().await.unwrap();
	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}

#[tokio::test]
async fn fusion_sky() {
	let client = crate::Client::connect().await.expect("Couldn't connect");
	client
		.setup_resources(&[&crate::project_local_resources!("res")])
		.unwrap();
	let client_handle = client.handle();
	let _event_loop = client.async_event_loop();
	let sky_resource = stardust_xr::values::ResourceID::new_namespaced("fusion", "sky");

	set_sky_light(&client_handle, Some(&sky_resource)).unwrap();
	set_sky_tex(&client_handle, Some(&sky_resource)).unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(5)).await;

	set_sky_light(&client_handle, None).unwrap();
	set_sky_tex(&client_handle, None).unwrap();
	tokio::time::sleep(core::time::Duration::from_secs(1)).await;
}
