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

mod pointer;
mod tip;

pub use pointer::PointerInputMethod;
pub use stardust_xr::schemas::flat::*;
pub use tip::TipInputMethod;

use super::{
	fields::FieldAspect,
	node::{Node, NodeError, NodeType},
	HandlerWrapper,
};
use crate::{
	client::Client,
	fields::UnknownField,
	node::NodeAspect,
	spatial::{SpatialAspect, Transform},
};
use color_eyre::eyre::{anyhow, bail};
use parking_lot::Mutex;
use stardust_xr::{
	schemas::flex::{deserialize, flexbuffers, serialize},
	values::Datamap,
};
use std::{os::fd::OwnedFd, sync::Arc};

pub(self) fn input_method_handler_wrapper<N: InputMethodAspect, H: InputMethodHandler>(
	handler_wrapper: &HandlerWrapper<N, H>,
) -> Result<(), NodeError> {
	handler_wrapper.add_handled_signal("handler_created", |node, handler, data, _fds| {
		let Ok(client) = node.client() else {
			bail!("no client??")
		};
		let Ok(path) = node.node().get_path() else {
			bail!("no path??")
		};
		let uid: String = deserialize(data)?;

		let node = Node::from_parent_name(&client, &path, &uid, false);
		let field =
			UnknownField::from_parent_name(&client, node.get_path().unwrap(), "field", false);
		let input_handler = InputHandler { node, field };
		handler.lock().create_handler(&uid, input_handler);
		Ok(())
	})?;
	handler_wrapper.add_handled_signal("handler_destroyed", |_node, handler, data, _fds| {
		let uid = deserialize(data)?;
		handler.lock().drop_handler(uid);
		Ok(())
	})?;

	Ok(())
}

/// Node representing a spatial input device.
pub trait InputMethodAspect: NodeType {
	fn set_datamap(&self, datamap: &Datamap) -> Result<(), NodeError> {
		flexbuffers::Reader::get_root(datamap.raw().as_slice())
			.and_then(|root| root.get_map())
			.map_err(|_| NodeError::MapInvalid)?;
		self.node()
			.send_remote_signal_raw("set_datamap", datamap.raw(), Vec::new())
	}

	fn set_handler_order(&self, handlers: &[&InputHandler]) -> Result<(), NodeError> {
		let handlers: Vec<_> = handlers
			.into_iter()
			.filter_map(|h| h.node().get_path().ok())
			.collect();
		self.node().send_remote_signal("set_handlers", &handlers)
	}
}

pub trait InputMethodHandler: Send + Sync {
	fn create_handler(&mut self, uid: &str, handler: InputHandler);
	fn drop_handler(&mut self, uid: &str);
}

/// Handle raw input events.
pub trait InputHandlerHandler: Send + Sync {
	/// An input method has sent an input event on this frame.
	fn input(&mut self, input: UnknownInputMethod, data: InputData);
}

/// An input method on the server, but the type is unknown.
pub struct UnknownInputMethod(Node);
impl UnknownInputMethod {
	/// Have the input handler that this method reference came from capture the method for the next frame.
	pub fn capture(&self, handler: &InputHandler) -> Result<(), NodeError> {
		self.node().send_remote_signal("capture", handler.node())
	}
}
impl NodeType for UnknownInputMethod {
	fn node(&self) -> &Node {
		&self.0
	}

	fn alias(&self) -> Self {
		UnknownInputMethod(self.0.alias())
	}

	fn from_path(client: &Arc<Client>, path: String, destroyable: bool) -> Self {
		UnknownInputMethod(Node::from_path(client, path, destroyable))
	}
}

/// Node that can receive spatial input.
#[derive(Debug)]
pub struct InputHandler {
	node: Node,
	field: UnknownField,
}
impl<'a> InputHandler {
	/// Create an input handler given a field, this will become inactive if the field is dropped.
	///
	/// Keep in mind the handler and its field are different spatials, they can move independently.
	pub fn create(
		spatial_parent: &'a impl SpatialAspect,
		transform: Transform,
		field: &'a impl FieldAspect,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		let client = spatial_parent.client()?;

		let node = Node::from_parent_name(&spatial_parent.client()?, "/input/handler", &id, true);
		client.message_sender_handle.signal(
			"/input",
			"create_input_handler",
			&serialize((
				id,
				spatial_parent.node().get_path()?,
				transform,
				field.node().get_path()?,
			))?,
			Vec::new(),
		)?;

		Ok(InputHandler {
			node,
			field: UnknownField::alias_field(field),
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
		_fds: Vec<OwnedFd>,
	) -> color_eyre::eyre::Result<()> {
		let data = InputData::deserialize(data).map_err(|e| anyhow!(e))?;
		handler.lock().input(
			UnknownInputMethod::from_parent_name(
				&input_handler.client()?,
				"/input/method",
				&data.uid,
				false,
			),
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
		&self.node
	}

	fn alias(&self) -> Self {
		InputHandler {
			node: self.node.alias(),
			field: self.field.alias(),
		}
	}

	fn from_path(client: &Arc<Client>, path: String, destroyable: bool) -> Self {
		let node = Node::from_path(client, path, destroyable);
		let field = UnknownField::from_parent_name(
			client,
			&node.node().get_path().unwrap(),
			"field",
			false,
		);
		InputHandler { node, field }
	}
}
impl NodeAspect for InputHandler {}
impl SpatialAspect for InputHandler {}

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
