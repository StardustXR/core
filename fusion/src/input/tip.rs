use crate::{
	client::Client,
	node::{Node, NodeAspect, NodeError, NodeType},
	spatial::{Spatial, SpatialAspect, Transform},
	HandlerWrapper,
};
use parking_lot::Mutex;
use stardust_xr::{
	schemas::flex::{
		flexbuffers::{self},
		serialize,
	},
	values::Datamap,
};
use std::sync::Arc;

use super::{input_method_handler_wrapper, InputMethodAspect, InputMethodHandler};

/// Virtual spatial input device representing a tool device with a single point of interaction (pen tip, controller tip, etc.)
#[derive(Debug)]
pub struct TipInputMethod(Node);
impl<'a> TipInputMethod {
	pub fn create(
		spatial_parent: &'a Spatial,
		transform: Transform,
		radius: f32,
		datamap: Option<Datamap>,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		let client = spatial_parent.client()?;
		if let Some(datamap) = &datamap {
			flexbuffers::Reader::get_root(datamap.raw().as_slice())
				.and_then(|root| root.get_map())
				.map_err(|_| NodeError::MapInvalid)?;
		}

		let tip = TipInputMethod::from_parent_name(
			&spatial_parent.client()?,
			"/input/method/tip",
			&id,
			true,
		);
		client.message_sender_handle.signal(
			"/input",
			"create_input_method_tip",
			&serialize((
				id,
				spatial_parent.node().get_path()?,
				transform,
				radius,
				datamap,
			))?,
			Vec::new(),
		)?;

		Ok(tip)
	}

	/// Set the radius of influence for the tip.
	pub fn set_radius(&self, radius: f32) -> Result<(), NodeError> {
		self.node().send_remote_signal("set_radius", &radius)
	}
	/// Wrap this node and an `InputMethodHandler` in a `HandlerWrapper` to run code ASAP.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: InputMethodHandler>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		self.wrap_raw(Arc::new(Mutex::new(handler)))
	}
	/// Wrap this node and an `InputMethodHandler` in a `HandlerWrapper` to run code ASAP.
	#[must_use = "Dropping this handler wrapper would immediately drop the node"]
	pub fn wrap_raw<H: InputMethodHandler>(
		self,
		handler: Arc<Mutex<H>>,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new_raw(self, handler);
		input_method_handler_wrapper(&handler_wrapper)?;
		Ok(handler_wrapper)
	}
}
impl NodeType for TipInputMethod {
	fn node(&self) -> &Node {
		&self.0
	}

	fn alias(&self) -> Self {
		TipInputMethod(self.0.alias())
	}

	fn from_path(client: &Arc<Client>, path: String, destroyable: bool) -> Self {
		TipInputMethod(Node::from_path(client, path, destroyable))
	}
}
impl NodeAspect for TipInputMethod {}
impl SpatialAspect for TipInputMethod {}
impl InputMethodAspect for TipInputMethod {}

#[tokio::test]
async fn fusion_tip_input_method() {
	use crate::client::{Client, FrameInfo};
	use crate::drawable::Model;

	color_eyre::install().unwrap();
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let mut fbb = flexbuffers::Builder::default();
	fbb.start_map();
	let tip = TipInputMethod::create(
		client.get_root(),
		Transform::none(),
		0.5,
		Some(Datamap::from_typed(TipData::default()).unwrap()),
	)
	.unwrap();

	fn summon_model(parent: &impl SpatialAspect, rotation: glam::Quat) -> Model {
		Model::create(
			parent,
			Transform::from_rotation_scale(rotation, [0.1; 3]),
			&stardust_xr::values::ResourceID::new_namespaced("fusion", "cursor_spike"),
		)
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
	#[derive(Default, serde::Serialize, serde::Deserialize)]
	struct TipData {
		grab: f32,
		select: f32,
	}
	struct TipDemo {
		root: crate::spatial::Spatial,
		tip: TipInputMethod,
		cursor: Cursor,
		datamap: TipData,
	}
	impl crate::client::RootHandler for TipDemo {
		fn frame(&mut self, info: FrameInfo) {
			let (sin, cos) = (info.elapsed as f32).sin_cos();
			self.tip
				.set_local_transform(Transform::from_translation([sin * 0.1, 0.0, cos * 0.1]))
				.unwrap();

			self.datamap.grab = sin;
			self.tip
				.set_datamap(&Datamap::from_typed(&self.datamap).unwrap())
				.unwrap();
		}
		fn save_state(&mut self) -> crate::client::ClientState {
			crate::client::ClientState::from_root(&self.root)
		}
	}

	let _wrapped_root = client.wrap_root(TipDemo {
		root: client.get_root().alias(),
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
		datamap: TipData::default(),
	});

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
