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
use glam::{FloatExt, Quat, Vec3A, vec3a};
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
		let client = spatial_parent.client();
		create_input_method(
			client,
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
		let client = spatial_parent.client();
		create_input_handler(
			client,
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

// these heuristics made possible by https://github.com/ultraleap/UnityPlugin/blob/1c49cc1205ef3cae8b27b8e24e1fcf84fdad721c/Packages/Tracking/Core/Runtime/Scripts/Utils/HandUtils.cs
// thank you Leap Motion!! you will be missed :(
impl Finger {
	/// length of finger from knuckle to tip
	pub fn length(&self) -> f32 {
		let proximal_position: Vec3A = self.proximal.position.into();
		let intermediate_position: Vec3A = self.intermediate.position.into();
		let distal_position: Vec3A = self.distal.position.into();
		let tip_position: Vec3A = self.tip.position.into();

		proximal_position.distance(intermediate_position)
			+ intermediate_position.distance(distal_position)
			+ distal_position.distance(tip_position)
	}

	pub fn direction(&self) -> Vector3<f32> {
		let proximal_position: Vec3A = self.proximal.position.into();
		let tip_position: Vec3A = self.tip.position.into();

		(tip_position - proximal_position).normalize().into()
	}
}

impl Thumb {
	/// length of finger from knuckle to tip
	pub fn length(&self) -> f32 {
		let proximal_position: Vec3A = self.proximal.position.into();
		let distal_position: Vec3A = self.distal.position.into();
		let tip_position: Vec3A = self.tip.position.into();

		proximal_position.distance(distal_position) + distal_position.distance(tip_position)
	}

	pub fn direction(&self) -> Vector3<f32> {
		let proximal_position: Vec3A = self.proximal.position.into();
		let tip_position: Vec3A = self.tip.position.into();

		(tip_position - proximal_position).normalize().into()
	}
}

impl Hand {
	/// The direction vector pointing out of the palm
	pub fn palm_normal(&self) -> Vector3<f32> {
		(Quat::from(self.palm.rotation) * vec3a(0.0, -1.0, 0.0)).into()
	}
	/// The direction vector pointing from the palm to thumb
	pub fn radial_axis(&self) -> Vector3<f32> {
		(Quat::from(self.palm.rotation)
			* if self.right {
				vec3a(-1.0, 0.0, 0.0)
			} else {
				vec3a(1.0, 0.0, 0.0)
			})
		.into()
	}
	/// The direction vector pointing from the palm towards fingers.
	pub fn distal_axis(&self) -> Vector3<f32> {
		(Quat::from(self.palm.rotation) * vec3a(0.0, 0.0, -1.0)).into()
	}

	pub fn finger_curl(&self, finger: &Finger) -> f32 {
		let distal_axis: Vec3A = self.distal_axis().into();
		let direction: Vec3A = finger.direction().into();
		direction.dot(-distal_axis).remap(-1.0, 1.0, 0.0, 1.0)
	}

	pub fn thumb_curl(&self) -> f32 {
		let radial_axis: Vec3A = self.radial_axis().into();
		let thumb_direction: Vec3A = self.thumb.direction().into();
		thumb_direction.dot(-radial_axis).remap(-1.0, 1.0, 0.0, 1.0)
	}

	pub fn pinch_distance(&self, finger: &Finger) -> f32 {
		let thumb_tip: Vec3A = self.thumb.tip.position.into();
		let index_tip: Vec3A = finger.tip.position.into();
		thumb_tip.distance(index_tip)
	}

	/// Unstabilized pinch position
	pub fn pinch_position(&self) -> Vector3<f32> {
		let thumb_tip: Vec3A = self.thumb.tip.position.into();
		let index_tip: Vec3A = self.index.tip.position.into();

		((2.0 * thumb_tip + index_tip) * 0.3333333).into()
	}

	/// Predicted Pinch Position without influence from the thumb or index tip.
	/// Useful for calculating extremely stable pinch calculations.
	/// Not good for visualising the pinch point - recommended to use PredictedPinchPosition instead
	pub fn stable_pinch_position(&self) -> Vector3<f32> {
		let index_knuckle: Vec3A = self.index.proximal.position.into();

		let index_length = self.index.length();

		let radial_axis: Vec3A = self.radial_axis().into();
		let palm_normal: Vec3A = self.palm_normal().into();
		let distal_axis: Vec3A = self.distal_axis().into();

		let stable_pinch_position = index_knuckle
			+ (palm_normal * index_length * 0.85)
			+ (distal_axis * index_length * 0.20)
			+ (radial_axis * index_length * 0.20);

		stable_pinch_position.into()
	}

	/// A decent approximation of where the hand will pinch even if index and thumb are far apart.
	pub fn predicted_pinch_position(&self) -> Vector3<f32> {
		let thumb_tip: Vec3A = self.thumb.tip.position.into();
		let index_tip: Vec3A = self.index.tip.position.into();

		let index_knuckle: Vec3A = self.index.proximal.position.into();
		let index_length = self.index.length();

		let radial_axis: Vec3A = self.radial_axis().into();
		let palm_normal: Vec3A = self.palm_normal().into();
		let distal_axis: Vec3A = self.distal_axis().into();

		let thumb_influence = (thumb_tip - index_knuckle)
			.normalize()
			.dot(radial_axis)
			.remap(0.0, 1.0, 0.5, 0.0);

		let mut predicted_pinch_point = index_knuckle
			+ palm_normal * index_length * 0.85
			+ distal_axis * index_length * 0.20
			+ radial_axis * index_length * 0.20;

		predicted_pinch_point = predicted_pinch_point.lerp(thumb_tip, thumb_influence);
		predicted_pinch_point = predicted_pinch_point.lerp(index_tip, 0.15);

		predicted_pinch_point.into()
	}

	fn hand_scale(&self) -> f32 {
		let index_metacarpal: Vec3A = self.index.metacarpal.position.into();
		let index_proximal: Vec3A = self.index.proximal.position.into();

		let middle_metacarpal: Vec3A = self.middle.metacarpal.position.into();
		let middle_proximal: Vec3A = self.middle.proximal.position.into();

		let ring_metacarpal: Vec3A = self.ring.metacarpal.position.into();
		let ring_proximal: Vec3A = self.ring.proximal.position.into();

		let little_metacarpal: Vec3A = self.little.metacarpal.position.into();
		let little_proximal: Vec3A = self.little.proximal.position.into();

		let index_metacarpal_length = index_metacarpal.distance(index_proximal);
		let middle_metacarpal_length = middle_metacarpal.distance(middle_proximal);
		let ring_metacarpal_length = ring_metacarpal.distance(ring_proximal);
		let little_metacarpal_length = little_metacarpal.distance(little_proximal);

		let mut scale = 0.0;

		scale += index_metacarpal_length / 0.06812;
		scale += middle_metacarpal_length / 0.06460;
		scale += ring_metacarpal_length / 0.05800;
		scale += little_metacarpal_length / 0.05369;

		scale / 4.0
	}

	/// Confidence value from 0-1 of how strong this hand is pinching.
	pub fn pinch_strength(&self) -> f32 {
		let thumb_tip: Vec3A = self.thumb.tip.position.into();
		let index_tip: Vec3A = self.index.tip.position.into();
		let middle_tip: Vec3A = self.middle.tip.position.into();
		let ring_tip: Vec3A = self.ring.tip.position.into();
		let little_tip: Vec3A = self.little.tip.position.into();

		let min_distance = index_tip
			.distance_squared(thumb_tip)
			.min(middle_tip.distance_squared(thumb_tip))
			.min(ring_tip.distance_squared(thumb_tip))
			.min(little_tip.distance_squared(thumb_tip))
			.sqrt();

		let scale = self.hand_scale();
		let distance_zero = 0.0600 * scale;
		let distance_one = 0.0220 * scale;

		((min_distance - distance_zero) / (distance_one - distance_zero)).clamp(0.0, 1.0)
	}

	/// Confidence value from 0-1 of how strong this hand is making a fist.
	pub fn fist_strength(&self) -> f32 {
		let radial_axis: Vec3A = self.radial_axis().into();
		let distal_axis: Vec3A = self.distal_axis().into();

		let thumb_direction: Vec3A = self.thumb.direction().into();
		let index_direction: Vec3A = self.index.direction().into();
		let middle_direction: Vec3A = self.middle.direction().into();
		let ring_direction: Vec3A = self.ring.direction().into();
		let little_direction: Vec3A = self.little.direction().into();

		(thumb_direction.dot(-radial_axis)
			+ index_direction.dot(-distal_axis)
			+ middle_direction.dot(-distal_axis)
			+ ring_direction.dot(-distal_axis)
			+ little_direction.dot(-distal_axis))
		.remap(-5.0, 5.0, 0.0, 1.0)
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
