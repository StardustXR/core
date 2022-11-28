pub mod action;
mod tip;

use super::{
	fields::Field,
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
	HandlerWrapper, WeakNodeRef, WeakWrapped,
};
pub use action as action_handler;
pub use stardust_xr::schemas::flat::*;
use stardust_xr::values::Transform;
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
	pub fn create<F, Fi: Field, T>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		field: &'a Fi,
		wrapped_init: F,
	) -> Result<HandlerWrapper<Self, T>, NodeError>
	where
		F: FnOnce(WeakNodeRef<InputHandler>, &InputHandler) -> T,
		T: InputHandlerHandler + 'static,
	{
		let id = nanoid::nanoid!();
		let handler = InputHandler {
			spatial: Spatial {
				node: Node::new(
					spatial_parent.node.client.clone(),
					"/input",
					"create_input_handler",
					"/input/handler",
					true,
					&id.clone(),
					(
						id,
						spatial_parent,
						Transform {
							position,
							rotation,
							scale: None,
						},
						&field.node(),
					),
				)?,
			},
		};

		let handler_wrapper =
			HandlerWrapper::new(handler, |weak_handler, weak_node_ref, input_handler| {
				let contents = wrapped_init(weak_node_ref, input_handler);
				input_handler.node.local_methods.lock().insert(
					"input".to_string(),
					Arc::new({
						let weak_handler: WeakWrapped<dyn InputHandlerHandler> = weak_handler;
						move |data| {
							let capture = if let Some(handler) = weak_handler.upgrade() {
								handler.lock().input(InputData::deserialize(data)?)
							} else {
								false
							};
							Ok(flexbuffers::singleton(capture))
						}
					}),
				);
				contents
			});

		// handler_wrapper.
		Ok(handler_wrapper)
	}
}
impl NodeType for InputHandler {
	fn node(&self) -> &Node {
		&self.spatial.node
	}
}
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
	let _input_handler = InputHandler::create(client.get_root(), None, None, &field, |_, _| {
		InputHandlerTest
	})
	.unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
