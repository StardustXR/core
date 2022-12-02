pub mod action;
mod tip;

use crate::node::HandledNodeType;

use super::{
	fields::Field,
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
	HandlerWrapper,
};
pub use action as action_handler;
use anyhow::anyhow;
use parking_lot::Mutex;
pub use stardust_xr::schemas::flat::*;
use stardust_xr::{schemas::flex::serialize, values::Transform};
use std::sync::Arc;
pub use tip::TipInputMethod;

pub trait InputHandlerHandler: Send + Sync {
	fn input(&mut self, input: InputData) -> bool;
}

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

#[derive(Debug)]
pub struct InputHandler {
	pub spatial: Spatial,
}

impl<'a> InputHandler {
	pub fn create<Fi: Field>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
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
						Transform {
							position,
							rotation,
							scale: None,
						},
						field.node().get_path()?,
					),
				)?,
			},
		})
	}

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
impl std::ops::Deref for InputHandler {
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

	let field = super::fields::SphereField::builder()
		.spatial_parent(client.get_root())
		.radius(0.1)
		.build()
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
	let _input_handler = InputHandler::create(client.get_root(), None, None, &field)
		.unwrap()
		.wrap(InputHandlerTest)
		.unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
