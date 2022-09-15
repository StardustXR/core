mod hand;
mod pointer;

pub use pointer::*;
// pub use hand::*;

use super::{
	fields::Field,
	node::{GenNodeInfo, Node, NodeError, NodeType},
	spatial::Spatial,
	HandlerWrapper, WeakNodeRef, WeakWrapped,
};
use crate::values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ZERO};
use anyhow::{anyhow, bail};
use ouroboros::self_referencing;
use schemas::input::{root_as_input_data, InputDataRawT, InputDataT};
use std::{
	convert::{TryFrom, TryInto},
	fmt::Debug,
};

#[derive(Debug, Clone)]
pub enum InputDataType {
	Pointer(Pointer),
	// Hand(Hand),
}

#[self_referencing]
pub struct InputData {
	pub uid: String,
	pub input: InputDataType,
	pub distance: f32,
	datamap_raw: Vec<u8>,

	#[borrows(datamap_raw)]
	#[not_covariant]
	pub datamap: flexbuffers::MapReader<&'this [u8]>,
}
impl TryFrom<InputDataT> for InputData {
	type Error = anyhow::Error;

	fn try_from(input: InputDataT) -> Result<Self, Self::Error> {
		InputData::try_new(
			input.uid,
			match input.input {
				InputDataRawT::Pointer(pointer) => InputDataType::Pointer(Pointer::new(
					pointer.origin.into(),
					pointer.orientation.into(),
					pointer.deepest_point.into(),
				)),
				InputDataRawT::Hand(_hand) => todo!("need hand struct format"),
				_ => bail!("Invalid input type"),
			},
			input.distance,
			input.datamap.ok_or_else(|| anyhow!("No datamap!"))?,
			|datamap_raw| flexbuffers::Reader::get_root(datamap_raw.as_slice())?.get_map(),
		)
		.map_err(anyhow::Error::from)
	}
}
impl Clone for InputData {
	fn clone(&self) -> Self {
		Self::new(
			self.borrow_uid().clone(),
			self.borrow_input().clone(),
			*self.borrow_distance(),
			self.borrow_datamap_raw().clone(),
			|datamap_raw| {
				flexbuffers::Reader::get_root(datamap_raw.as_slice())
					.unwrap()
					.get_map()
					.unwrap()
			},
		)
	}
}
impl Debug for InputData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("InputData")
			.field("uid", self.borrow_uid())
			.field("input", self.borrow_input())
			.field("distance", self.borrow_distance())
			.field(
				"datamap_raw",
				&String::from_utf8_lossy(self.borrow_datamap_raw()).into_owned(),
			)
			.finish_non_exhaustive()
	}
}

pub trait InputHandlerHandler: Send + Sync {
	fn input(&mut self, input: InputData) -> bool;
}

pub struct InputHandler {
	pub spatial: Spatial,
}

#[buildstructor::buildstructor]
impl<'a> InputHandler {
	#[builder(entry = "builder")]
	pub fn create<F, T>(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		field: &'a Field,
		wrapped_init: F,
	) -> Result<HandlerWrapper<Self, T>, NodeError>
	where
		F: FnOnce(WeakNodeRef<InputHandler>, &InputHandler) -> T,
		T: InputHandlerHandler + 'static,
	{
		let handler = InputHandler {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client: spatial_parent.node.client.clone(),
						parent_path: "/input/handler",
						interface_path: "/input",
						interface_method: "createInputHandler"
					},
					spatial_parent.node.get_path(),
					position.unwrap_or(VEC3_ZERO),
					rotation.unwrap_or(QUAT_IDENTITY),
					field.spatial.node.get_path()
				),
			},
		};

		let handler_wrapper =
			HandlerWrapper::new(handler, |weak_handler, weak_node_ref, input_handler| {
				let contents = wrapped_init(weak_node_ref, input_handler);
				input_handler.node.local_methods.insert(
					"input".to_string(),
					Box::new({
						let weak_handler: WeakWrapped<dyn InputHandlerHandler> = weak_handler;
						move |data| {
							let capture = if let Some(handler) = weak_handler.upgrade() {
								let input = root_as_input_data(data)?.unpack();

								handler.lock().input(input.try_into()?)
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
			dbg!(input);
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
		_ = event_loop => (),
	};
}
