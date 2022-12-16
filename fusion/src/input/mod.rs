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
//! Every frame, the server will do this for each input method:
//! - Sort the input handlers by the distance from the input method to their fields (often absolute value for onion skinning)
//! - Send out input events (`InputHandlerHandler::input`) in order of distance until an input handler has captured the method.
//! - The logic_step event is sent (`LifeCycle::logic_step`).
//!
//! To make this all easier, the `action` module exists, check it out.

pub mod action;
mod tip;

pub use action as action_handler;
pub use stardust_xr::schemas::flat::*;
pub use tip::TipInputMethod;

use super::{
	fields::Field,
	node::{HandledNodeType, Node, NodeError, NodeType},
	spatial::Spatial,
	HandlerWrapper,
};
use anyhow::anyhow;
use parking_lot::Mutex;
use stardust_xr::{
	schemas::flex::{flexbuffers, serialize},
	values::Transform,
};
use std::{ops::Deref, sync::Arc};

/// Handle raw input events.
pub trait InputHandlerHandler: Send + Sync {
	/// An input method has sent an input event on this frame.
	///
	/// Return "true" to capture the input method or "false" to not.
	fn input(&mut self, input: InputData) -> bool;
}

/// Node representing a spatial input device.
pub trait InputMethod {
	fn node(&self) -> &Node;
	fn set_enabled(&self, enabled: bool) -> Result<(), NodeError> {
		self.node().send_remote_signal("set_enabled", &enabled)
	}
	fn set_datamap(&self, datamap: &[u8]) -> Result<(), NodeError> {
		flexbuffers::Reader::get_root(datamap)
			.and_then(|root| root.get_map())
			.map_err(|_| NodeError::MapInvalid)?;
		self.node().send_remote_signal_raw("set_datamap", datamap)
	}
}

/// Node that can receive spatial input.
#[derive(Debug)]
pub struct InputHandler {
	spatial: Spatial,
}
impl<'a> InputHandler {
	/// Create an input handler given a field, this will become inactive if the field is dropped.
	///
	/// Keep in mind the handler and its field are different spatials, they can move independently.
	pub fn create<Fi: Field>(
		spatial_parent: &'a Spatial,
		transform: Transform,
		field: &'a Fi,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(InputHandler {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/input",
					"create_input_handler",
					"/input/handler",
					true,
					&id.clone(),
					(
						id,
						spatial_parent.node().get_path()?,
						transform,
						field.node().get_path()?,
					),
				)?,
			},
		})
	}

	/// Wrap this node and an `InputHandlerHandler` in a `HandlerWrapper`. This is necessary to get any input events.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: InputHandlerHandler>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new(self, handler);
		handler_wrapper.add_handled_method("input", Self::handle_input)?;
		Ok(handler_wrapper)
	}

	fn handle_input<H: InputHandlerHandler>(
		_zone: Arc<InputHandler>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> anyhow::Result<Vec<u8>> {
		let capture = handler
			.lock()
			.input(InputData::deserialize(data).map_err(|e| anyhow!(e))?);
		Ok(serialize(capture)?)
	}
}
impl NodeType for InputHandler {
	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn alias(&self) -> Self {
		InputHandler {
			spatial: self.spatial.alias(),
		}
	}
}
impl HandledNodeType for InputHandler {}
impl Deref for InputHandler {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_input_handler() {
	use super::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let field =
		super::fields::SphereField::create(client.get_root(), mint::Vector3::from([0.0; 3]), 0.1)
			.unwrap();

	struct InputHandlerTest;
	impl InputHandlerHandler for InputHandlerTest {
		fn input(&mut self, input: InputData) -> bool {
			dbg!(input.uid);
			dbg!(input.distance);
			match &input.input {
				InputDataType::Pointer(_) => {
					println!("Pointer input");
				}
				InputDataType::Hand(_) => {
					println!("Hand input");
					let _ = input.datamap.with_data(|datamap| {
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
			false
		}
	}

	// let input_handler = InputHandler::builder()
	// 	.spatial_parent(client.get_root())
	// 	.field(&field)
	// 	.wrapped_init(|_| InputHandlerTest)
	// 	.build()
	// 	.await
	// 	.unwrap();
	let _input_handler = InputHandler::create(client.get_root(), Transform::default(), &field)
		.unwrap()
		.wrap(InputHandlerTest)
		.unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
