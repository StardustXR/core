use crate::{
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
};
use anyhow::Result;
use mint::{Quaternion, Vector3};
use stardust_xr::values::Transform;
use std::ops::Deref;

use super::InputMethod;

#[derive(Debug)]
pub struct TipInputMethod {
	pub spatial: Spatial,
}
#[buildstructor::buildstructor]
impl<'a> TipInputMethod {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vector3<f32>>,
		rotation: Option<Quaternion<f32>>,
		radius: f32,
		datamap: Option<Vec<u8>>,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		if let Some(datamap) = &datamap {
			flexbuffers::Reader::get_root(datamap.as_slice())
				.and_then(|root| root.get_map())
				.map_err(|_| NodeError::MapInvalid)?;
		}
		Ok(TipInputMethod {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/input",
					"create_input_method_tip",
					"/input/method/tip",
					true,
					&id.clone(),
					(
						id,
						spatial_parent.node().get_path()?,
						Transform {
							position,
							rotation,
							scale: None,
						},
						radius,
						datamap,
					),
				)?,
			},
		})
	}

	pub fn set_radius(&self, radius: f32) -> Result<(), NodeError> {
		self.node.send_remote_signal("set_radius", &radius)
	}
}
impl InputMethod for TipInputMethod {
	fn node(&self) -> &Node {
		&self.node
	}
}
impl Deref for TipInputMethod {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_tip_input_method() {
	use crate::client::{Client, LogicStepInfo};
	use crate::drawable::Model;
	use crate::resource::NamespacedResource;

	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let mut fbb = flexbuffers::Builder::default();
	fbb.start_map();
	let tip = TipInputMethod::builder()
		.spatial_parent(client.get_root())
		.radius(0.05)
		.datamap(fbb.take_buffer())
		.build()
		.unwrap();

	fn summon_model(parent: &Spatial, rotation: glam::Quat) -> Model {
		Model::builder()
			.spatial_parent(parent)
			.resource(&NamespacedResource::new("fusion", "cursor_spike.glb"))
			.rotation(rotation)
			.scale(mint::Vector3::from([0.1; 3]))
			.build()
			.unwrap()
	}

	struct Cursor {
		top: Model,
		bottom: Model,
		left: Model,
		right: Model,
		forward: Model,
		backward: Model,
	}
	struct TipDemo {
		tip: TipInputMethod,
		cursor: Cursor,
	}
	impl crate::client::LifeCycleHandler for TipDemo {
		fn logic_step(&mut self, info: LogicStepInfo) {
			let (sin, cos) = (info.elapsed as f32).sin_cos();
			self.tip
				.set_position(None, mint::Vector3::from([sin * 0.1, 0.0, cos * 0.1]))
				.unwrap();
		}
	}

	let _wrapped_root = client.wrap_root(TipDemo {
		cursor: Cursor {
			top: summon_model(
				&tip,
				glam::Quat::from_axis_angle(glam::Vec3::X, 180.0f32.to_radians()),
			),
			bottom: summon_model(
				&tip,
				glam::Quat::from_axis_angle(glam::Vec3::X, 0.0f32.to_radians()),
			),
			left: summon_model(
				&tip,
				glam::Quat::from_axis_angle(glam::Vec3::Z, 90.0f32.to_radians()),
			),
			right: summon_model(
				&tip,
				glam::Quat::from_axis_angle(glam::Vec3::Z, -90.0f32.to_radians()),
			),
			forward: summon_model(
				&tip,
				glam::Quat::from_axis_angle(glam::Vec3::X, 90.0f32.to_radians()),
			),
			backward: summon_model(
				&tip,
				glam::Quat::from_axis_angle(glam::Vec3::X, -90.0f32.to_radians()),
			),
		},
		tip,
	});

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
