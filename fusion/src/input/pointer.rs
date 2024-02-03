use super::{input_method_handler_wrapper, InputMethodAspect, InputMethodHandler};
use crate::{
	client::Client,
	node::{Node, NodeAspect, NodeError, NodeType},
	spatial::{SpatialAspect, Transform},
	HandlerWrapper,
};
use parking_lot::Mutex;
use stardust_xr::{schemas::flex::serialize, values::Datamap};
use std::sync::Arc;

/// Virtual spatial input device representing a ray/pointer (e.g. eye gaze, 3DoF controller)
#[derive(Debug)]
pub struct PointerInputMethod(Node);
impl<'a> PointerInputMethod {
	pub fn create(
		spatial_parent: &'a impl SpatialAspect,
		transform: Transform,
		datamap: Datamap,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		let client = spatial_parent.client()?;

		let pointer = PointerInputMethod::from_parent_name(
			&spatial_parent.client()?,
			"/input/method/pointer",
			&id,
			true,
		);
		client.message_sender_handle.signal(
			"/input",
			"create_input_method_pointer",
			&serialize((id, spatial_parent.node().get_path()?, transform, datamap))?,
			Vec::new(),
		)?;

		Ok(pointer)
	}

	/// Set the radius of influence for the pointer.
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
impl NodeType for PointerInputMethod {
	fn node(&self) -> &Node {
		&self.0
	}

	fn alias(&self) -> Self {
		PointerInputMethod(self.0.alias())
	}

	fn from_path(client: &Arc<Client>, path: String, destroyable: bool) -> Self {
		PointerInputMethod(Node::from_path(client, path, destroyable))
	}
}
impl NodeAspect for PointerInputMethod {}
impl SpatialAspect for PointerInputMethod {}
impl InputMethodAspect for PointerInputMethod {}

#[tokio::test]
async fn fusion_pointer_input_method() {
	use crate::client::{Client, FrameInfo};
	use crate::drawable::Model;

	color_eyre::install().unwrap();
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let mut fbb = stardust_xr::schemas::flex::flexbuffers::Builder::default();
	fbb.start_map();
	let pointer = PointerInputMethod::create(
		client.get_root(),
		Transform::none(),
		Datamap::from_typed(PointerData::default()).unwrap(),
	)
	.unwrap();

	#[derive(Default, serde::Serialize, serde::Deserialize)]
	struct PointerData {
		grab: f32,
		select: f32,
	}
	struct PointerDemo {
		pointer: PointerInputMethod,
		model: Model,
		datamap: PointerData,
	}
	impl crate::client::RootHandler for PointerDemo {
		fn frame(&mut self, info: FrameInfo) {
			let (sin, cos) = (info.elapsed as f32).sin_cos();
			self.pointer
				.set_local_transform(Transform::from_translation(mint::Vector3::from([
					sin * 0.1,
					0.0,
					cos * 0.1,
				])))
				.unwrap();

			self.datamap.grab = sin;
			self.pointer
				.set_datamap(&Datamap::from_typed(&self.datamap).unwrap())
				.unwrap();
		}
		fn save_state(&mut self) -> crate::client::ClientState {
			crate::client::ClientState::default()
		}
	}

	let model = Model::create(
		&pointer,
		Transform::from_rotation_scale(
			glam::Quat::from_rotation_x(std::f32::consts::PI * 0.5),
			[0.1; 3],
		),
		&stardust_xr::values::ResourceID::new_namespaced("fusion", "cursor_spike"),
	)
	.unwrap();
	let _wrapped_root = client.wrap_root(PointerDemo {
		pointer,
		model,
		datamap: PointerData::default(),
	});

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
