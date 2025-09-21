//! Spatial input using the Spatial Universal Interaction System (SUIS).
//!
//! Input methods are nodes that represent a 3D pointer, hand, or tip (single point of interaction, like a controller).
//! They contain a datamap which is a flexbuffer map with non-spatial data like button/trackpad/grip.
//!
//! Input handlers are nodes that represent an object that can react to spatial input.
//! They have a field attached that is important data for the SUIS to determine what should get input.
//! On input (`InputHandlerHandler::input`) the input data's spatial components is relative to the input handler itself.
//! The return value for `InputHandlerHandler::input` is `true` if you want to capture the input method.
//! Capturing an input method is useful to indicate that only that handler should get its input.
//! For example when grabbing you don't want your hand to press buttons if you're grabbing the object through them.
//! Input handlers should account for the occasional case where their field is closer than an input handler that captured a method by filtering out interactions that are triggered the same frame the input method first becomes visible.
//! Capturing an input method may be delayed a frame or 2.

use crate::{
	fields::FieldAspect,
	node::NodeResult,
	spatial::{SpatialRefAspect, Transform},
};
use glam::{Quat, vec3a};
use stardust_xr::values::*;
use std::hash::Hash;

pub use crate::protocol::input::*;

impl InputMethod {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		input_type: InputDataType,
		datamap: &Datamap,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_input_method(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			input_type,
			datamap,
		)
	}
}
impl InputHandler {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		field: &impl FieldAspect,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_input_handler(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			field,
		)
	}
}
impl Default for Joint {
	fn default() -> Self {
		Joint {
			position: [0.0; 3].into(),
			rotation: Quat::IDENTITY.into(),
			radius: 0.0,
			distance: 0.0,
		}
	}
}
impl Default for Finger {
	fn default() -> Self {
		Finger {
			tip: Default::default(),
			distal: Default::default(),
			intermediate: Default::default(),
			proximal: Default::default(),
			metacarpal: Default::default(),
		}
	}
}
impl Default for Thumb {
	fn default() -> Self {
		Thumb {
			tip: Default::default(),
			distal: Default::default(),
			proximal: Default::default(),
			metacarpal: Default::default(),
		}
	}
}
impl Default for Hand {
	fn default() -> Self {
		Hand {
			right: Default::default(),
			thumb: Default::default(),
			index: Default::default(),
			middle: Default::default(),
			ring: Default::default(),
			little: Default::default(),
			palm: Default::default(),
			wrist: Default::default(),
			elbow: Default::default(),
		}
	}
}
impl Default for Pointer {
	fn default() -> Self {
		Pointer {
			origin: [0.0; 3].into(),
			orientation: Quat::IDENTITY.into(),
			deepest_point: [0.0; 3].into(),
		}
	}
}
impl Default for Tip {
	fn default() -> Self {
		Tip {
			origin: [0.0; 3].into(),
			orientation: Quat::IDENTITY.into(),
		}
	}
}

impl Pointer {
	pub fn direction(&self) -> Vector3<f32> {
		(Quat::from(self.orientation) * vec3a(0.0, 0.0, -1.0)).into()
	}
}

impl Hash for InputData {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.id.hash(state)
	}
}
impl PartialEq for InputData {
	fn eq(&self, other: &Self) -> bool {
		self.id.eq(&other.id)
	}
}
impl Eq for InputData {}

#[tokio::test]
async fn fusion_input_handler() {
	use crate::Client;

	let mut client = Client::connect().await.expect("Couldn't connect");

	let field = super::fields::Field::create(
		client.get_root(),
		Transform::identity(),
		crate::fields::Shape::Sphere(0.1),
	)
	.unwrap();
	let _input_handler =
		InputHandler::create(client.get_root(), Transform::none(), &field).unwrap();

	client
		.sync_event_loop(|_, _| {
			while let Some(input_event) = _input_handler.recv_input_handler_event() {
				match input_event {
					InputHandlerEvent::Input { methods: _, data } => on_input(data),
				}
			}
		})
		.await
		.unwrap();

	fn on_input(data: Vec<InputData>) {
		for data in data {
			dbg!(data.id);
			dbg!(data.distance);
			match &data.input {
				InputDataType::Pointer(_) => {
					println!("Pointer input");
				}
				InputDataType::Hand(_) => {
					println!("Hand input");
					data.datamap.with_data(|datamap| {
						dbg!(
							datamap
								.iter_keys()
								.zip(datamap.iter_values())
								.collect::<Vec<_>>()
						);
						let _ = dbg!(datamap.idx("right").get_bool());
					});
				}
				InputDataType::Tip(_) => {
					println!("Tip input");
				}
			}
		}
	}
}

#[tokio::test]
async fn fusion_pointer_input_method() {
	use crate::Client;
	use crate::drawable::Model;
	use crate::root::*;
	use crate::spatial::SpatialAspect;

	let mut client = Client::connect().await.expect("Couldn't connect");

	let mut fbb = stardust_xr::schemas::flex::flexbuffers::Builder::default();
	fbb.start_map();
	let pointer = InputMethod::create(
		client.get_root(),
		Transform::none(),
		InputDataType::Pointer(Pointer::default()),
		&Datamap::from_typed(PointerData::default()).unwrap(),
	)
	.unwrap();
	let _model = Model::create(
		&pointer,
		Transform::from_rotation_scale(
			glam::Quat::from_rotation_x(std::f32::consts::PI * 0.5),
			[0.1; 3],
		),
		&stardust_xr::values::ResourceID::new_namespaced("fusion", "cursor_spike"),
	)
	.unwrap();
	let mut datamap = PointerData::default();

	#[derive(Default, serde::Serialize, serde::Deserialize)]
	struct PointerData {
		grab: f32,
		select: f32,
	}

	client
		.sync_event_loop(|client, _| {
			while let Some(root_event) = client.get_root().recv_root_event() {
				match root_event {
					RootEvent::Ping { response } => {
						response.send_ok(());
					}
					RootEvent::Frame { info } => {
						let (sin, cos) = info.elapsed.sin_cos();
						pointer
							.set_local_transform(Transform::from_translation([
								sin * 0.1,
								0.0,
								cos * 0.1,
							]))
							.unwrap();

						datamap.grab = sin;
						pointer
							.set_datamap(&Datamap::from_typed(&datamap).unwrap())
							.unwrap();
					}
					RootEvent::SaveState { response } => response.send_ok(Default::default()),
				}
			}
		})
		.await
		.unwrap();
}

#[tokio::test]
async fn fusion_tip_input_method() {
	use crate::Client;
	use crate::drawable::Model;
	use crate::root::*;
	use crate::spatial::SpatialAspect;

	let mut client = Client::connect().await.expect("Couldn't connect");

	let tip = InputMethod::create(
		client.get_root(),
		Transform::none(),
		InputDataType::Tip(Tip::default()),
		&Datamap::from_typed(TipData::default()).unwrap(),
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
	let mut datamap = TipData::default();

	client
		.sync_event_loop(|client, _| {
			while let Some(root_event) = client.get_root().recv_root_event() {
				match root_event {
					RootEvent::Ping { response } => {
						response.send_ok(());
					}
					RootEvent::Frame { info } => {
						let (sin, cos) = info.elapsed.sin_cos();
						tip.set_local_transform(Transform::from_translation([
							sin * 0.1,
							0.0,
							cos * 0.1,
						]))
						.unwrap();

						datamap.grab = sin;
						tip.set_datamap(&Datamap::from_typed(&datamap).unwrap())
							.unwrap();
					}
					RootEvent::SaveState { response } => response.send_ok(Default::default()),
				}
			}
		})
		.await
		.unwrap();
}
