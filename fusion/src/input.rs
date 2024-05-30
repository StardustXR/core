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
//!
//! Every frame, for each input method, the server will:
//! - Sort the input handlers by the distance from the input method to their fields (often absolute value for onion skinning)
//! - Send out input events (`InputHandlerHandler::input`) in order of distance until an input handler has captured the method.
//! - The frame event is sent (`LifeCycle::frame`).
//!
//! You may want to use the `InputAction`-based structs in molecules for an easy way to parse and react to the raw input.

use std::hash::Hash;

use crate::{
	fields::{Field, FieldAspect},
	impl_aspects,
	node::{NodeResult, NodeType, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
};
use glam::{vec3a, Quat};
use stardust_xr::values::*;

stardust_xr_fusion_codegen::codegen_input_protocol!();

impl_aspects!(InputMethodRef: SpatialRefAspect);
impl_aspects!(InputMethod: OwnedAspect, SpatialRefAspect, SpatialAspect, InputMethodRefAspect);
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
impl_aspects!(InputHandler: OwnedAspect, SpatialRefAspect, SpatialAspect);
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
		self.uid.hash(state)
	}
}
impl PartialEq for InputData {
	fn eq(&self, other: &Self) -> bool {
		self.uid.eq(&other.uid)
	}
}
impl Eq for InputData {}

#[tokio::test]
async fn fusion_input_handler() {
	use super::client::Client;
	color_eyre::install().unwrap();
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let field = super::fields::SphereField::create(client.get_root(), [0.0; 3], 0.1).unwrap();

	struct InputHandlerTest;
	impl InputHandlerHandler for InputHandlerTest {
		fn input(&mut self, _input: InputMethod, data: InputData) {
			dbg!(data.uid);
			dbg!(data.distance);
			match &data.input {
				InputDataType::Pointer(_) => {
					println!("Pointer input");
				}
				InputDataType::Hand(_) => {
					println!("Hand input");
					let _ = data.datamap.with_data(|datamap| {
						dbg!(datamap
							.iter_keys()
							.zip(datamap.iter_values())
							.collect::<Vec<_>>());
						let _ = dbg!(datamap.idx("right").get_bool());
					});
				}
				InputDataType::Tip(_) => {
					println!("Tip input");
				}
			}
		}
	}

	let _input_handler = InputHandler::create(client.get_root(), Transform::none(), &field)
		.unwrap()
		.wrap(InputHandlerTest)
		.unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	}
}

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
	let pointer = InputMethod::create(
		client.get_root(),
		Transform::none(),
		InputDataType::Pointer(Pointer::default()),
		&Datamap::from_typed(PointerData::default()).unwrap(),
	)
	.unwrap();

	#[derive(Default, serde::Serialize, serde::Deserialize)]
	struct PointerData {
		grab: f32,
		select: f32,
	}
	struct PointerDemo {
		root: crate::spatial::Spatial,
		pointer: InputMethod,
		model: Model,
		datamap: PointerData,
	}
	impl crate::client::RootHandler for PointerDemo {
		fn frame(&mut self, info: FrameInfo) {
			let (sin, cos) = (info.elapsed as f32).sin_cos();
			self.pointer
				.set_local_transform(Transform::from_translation([sin * 0.1, 0.0, cos * 0.1]))
				.unwrap();

			self.datamap.grab = sin;
			self.pointer
				.set_datamap(&Datamap::from_typed(&self.datamap).unwrap())
				.unwrap();
		}
		fn save_state(&mut self) -> crate::client::ClientState {
			crate::client::ClientState::from_root(&self.root)
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
		root: client.get_root().alias(),
		pointer,
		model,
		datamap: PointerData::default(),
	});

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	}
}

#[tokio::test]
async fn fusion_tip_input_method() {
	use crate::client::{Client, FrameInfo};
	use crate::drawable::Model;

	color_eyre::install().unwrap();
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

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
	struct TipDemo {
		root: crate::spatial::Spatial,
		tip: InputMethod,
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
	}
}
