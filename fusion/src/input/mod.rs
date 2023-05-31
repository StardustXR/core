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
//! - The frame event is sent (`LifeCycle::frame`).
//!
//! To make this all easier, the `action` module exists, check it out.

pub mod action;
mod pointer;
mod tip;

pub use action as action_handler;
pub use pointer::PointerInputMethod;
use rustc_hash::FxHashMap;
pub use stardust_xr::schemas::flat::*;
pub use tip::TipInputMethod;

use crate::fields::UnknownField;

use super::{
	fields::Field,
	node::{HandledNodeType, Node, NodeError, NodeType},
	spatial::Spatial,
	HandlerWrapper,
};
use color_eyre::eyre::{anyhow, bail};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use stardust_xr::{
	schemas::flex::{deserialize, flexbuffers},
	values::Transform,
};
use std::{ops::Deref, sync::Arc};

/// Handle raw input events.
pub trait InputHandlerHandler: Send + Sync {
	/// An input method has sent an input event on this frame.
	fn input(&mut self, input: UnknownInputMethod, data: InputData);
}

/// Node representing a spatial input device.
#[derive(Debug)]
pub struct InputMethod {
	spatial: Spatial,
	input_handlers: Arc<RwLock<FxHashMap<String, InputHandler>>>,
}
impl InputMethod {
	pub(crate) fn handle_methods(&self) -> Result<(), NodeError> {
		let client = Arc::downgrade(&self.node().client()?);
		let path = self.node().get_path()?;
		let input_handlers = Arc::downgrade(&self.input_handlers);
		self.node().add_local_signal(
			"handler_created",
			move |data| -> color_eyre::eyre::Result<()> {
				let Some(client) = client.upgrade() else {bail!("no client??")};
				let Some(handlers) = input_handlers.upgrade() else {bail!("no handlers ohno")};
				let uid: String = deserialize(data)?;

				let node = Node::from_path(&client, &path, &uid, false);
				let spatial = Spatial { node };
				let field = UnknownField {
					spatial: Spatial {
						node: Node::from_path(
							&client,
							spatial.node().get_path().unwrap(),
							"field",
							false,
						),
					},
				};
				let input_handler = InputHandler { spatial, field };
				handlers.write().insert(uid, input_handler);
				Ok(())
			},
		)?;

		let input_handlers = Arc::downgrade(&self.input_handlers);
		self.node().add_local_signal(
			"handler_destroyed",
			move |data| -> color_eyre::eyre::Result<()> {
				let Some(handlers) = input_handlers.upgrade() else {bail!("no handlers ohno")};
				let uid: &str = deserialize(data)?;
				handlers.write().remove(uid);
				Ok(())
			},
		)?;

		Ok(())
	}

	pub fn set_datamap(&self, datamap: &[u8]) -> Result<(), NodeError> {
		flexbuffers::Reader::get_root(datamap)
			.and_then(|root| root.get_map())
			.map_err(|_| NodeError::MapInvalid)?;
		self.node().send_remote_signal_raw("set_datamap", datamap)
	}

	pub fn set_handler_order(&self, handlers: &[&InputHandler]) -> Result<(), NodeError> {
		let handlers: Vec<_> = handlers
			.into_iter()
			.filter_map(|h| h.node().get_path().ok())
			.collect();
		self.node().send_remote_signal("set_handlers", &handlers)
	}

	/// Get a read only guard to the handlers list.
	pub fn input_handlers(&self) -> RwLockReadGuard<FxHashMap<String, InputHandler>> {
		self.input_handlers.read()
	}
}
impl NodeType for InputMethod {
	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn alias(&self) -> Self {
		InputMethod {
			spatial: self.spatial.alias(),
			input_handlers: self.input_handlers.clone(),
		}
	}
}
impl Deref for InputMethod {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

/// An input method on the server, but the type is unknown.
pub struct UnknownInputMethod {
	spatial: Spatial,
	handler: Arc<InputHandler>,
}
impl UnknownInputMethod {
	fn from_path(handler: Arc<InputHandler>, uid: &str) -> Result<Self, NodeError> {
		Ok(UnknownInputMethod {
			spatial: Spatial {
				node: Node::from_path(&handler.client()?, handler.node().get_path()?, uid, false),
			},
			handler,
		})
	}
	/// Have the input handler that this method reference came from capture the method for the next frame.
	pub fn capture(&self) -> Result<(), NodeError> {
		self.node()
			.send_remote_signal("capture", &self.handler.node().get_path()?)
	}
}
impl NodeType for UnknownInputMethod {
	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn alias(&self) -> Self {
		UnknownInputMethod {
			spatial: self.spatial.alias(),
			handler: self.handler.clone(),
		}
	}
}
impl Deref for UnknownInputMethod {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

/// Node that can receive spatial input.
#[derive(Debug)]
pub struct InputHandler {
	spatial: Spatial,
	field: UnknownField,
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
			field: field.alias_unknown_field(),
		})
	}

	/// Wrap this node and an `InputHandlerHandler` in a `HandlerWrapper`. This is necessary to get any input events.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: InputHandlerHandler>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		self.wrap_raw(Arc::new(Mutex::new(handler)))
	}
	/// Wrap this node and an `InputHandlerHandler` in a `HandlerWrapper`. This is necessary to get any input events.
	#[must_use = "Dropping this handler wrapper would immediately drop the node"]
	pub fn wrap_raw<H: InputHandlerHandler>(
		self,
		handler: Arc<Mutex<H>>,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new_raw(self, handler);
		handler_wrapper.add_handled_signal("input", Self::handle_input)?;
		Ok(handler_wrapper)
	}

	fn handle_input<H: InputHandlerHandler>(
		input_handler: Arc<InputHandler>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> color_eyre::eyre::Result<()> {
		let data = InputData::deserialize(data).map_err(|e| anyhow!(e))?;
		handler.lock().input(
			UnknownInputMethod::from_path(input_handler, &data.uid)?,
			data,
		);
		Ok(())
	}

	pub fn field(&self) -> Option<UnknownField> {
		self.field.node().alive().then(|| self.field.alias())
	}
}
impl NodeType for InputHandler {
	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn alias(&self) -> Self {
		InputHandler {
			spatial: self.spatial.alias(),
			field: self.field.alias(),
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
	color_eyre::install().unwrap();
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let field =
		super::fields::SphereField::create(client.get_root(), mint::Vector3::from([0.0; 3]), 0.1)
			.unwrap();

	struct InputHandlerTest;
	impl InputHandlerHandler for InputHandlerTest {
		fn input(&mut self, _input: UnknownInputMethod, data: InputData) {
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
