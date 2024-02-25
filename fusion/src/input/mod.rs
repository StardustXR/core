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

// mod pointer;
// mod tip;

// pub use pointer::PointerInputMethod;
// pub use stardust_xr::schemas::flat::*;
// pub use tip::TipInputMethod;

use stardust_xr::values::Datamap;

use crate::{
	fields::FieldAspect,
	node::{NodeAspect, NodeResult, NodeType},
	spatial::{SpatialAspect, Transform},
};

stardust_xr_fusion_codegen::codegen_input_client_protocol!();
impl InputMethod {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		input_type: InputDataType,
		datamap: &Datamap,
	) -> NodeResult<Self> {
		create_input_method(
			&spatial_parent.client()?,
			&nanoid::nanoid!(),
			spatial_parent,
			transform,
			input_type,
			datamap,
		)
	}
}
impl InputHandler {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		field: &impl FieldAspect,
	) -> NodeResult<Self> {
		create_input_handler(
			&spatial_parent.client()?,
			&nanoid::nanoid!(),
			spatial_parent,
			transform,
			field,
		)
	}
}

#[tokio::test]
async fn fusion_input_handler() {
	use super::client::Client;
	color_eyre::install().unwrap();
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let field =
		super::fields::SphereField::create(client.get_root(), mint::Vector3::from([0.0; 3]), 0.1)
			.unwrap();

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

	// let input_handler = InputHandler::builder()
	// 	.spatial_parent(client.get_root())
	// 	.field(&field)
	// 	.wrapped_init(|_| InputHandlerTest)
	// 	.build()
	// 	.await
	// 	.unwrap();
	let _input_handler = InputHandler::create(client.get_root(), Transform::none(), &field)
		.unwrap()
		.wrap(InputHandlerTest)
		.unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
