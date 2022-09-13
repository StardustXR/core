use super::{
	field::Field,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
	HandlerWrapper,
};
use crate::values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ZERO};
use anyhow::{anyhow, bail};
use glam::Mat4;
use mint::RowMatrix4;
use once_cell::sync::OnceCell;
use ouroboros::self_referencing;
use schemas::input::{root_as_input_data, InputDataRawT};
use std::{
	fmt::Debug,
	sync::{Arc, Weak},
};

pub struct Pointer {
	origin: Vec3,
	orientation: Quat,
	deepest_point: Vec3,
	transform: OnceCell<RowMatrix4<f32>>,
}
impl Pointer {
	fn new(origin: Vec3, orientation: Quat, deepest_point: Vec3) -> Self {
		Self {
			origin,
			orientation,
			deepest_point,
			transform: OnceCell::new(),
		}
	}

	pub fn transform(&self) -> RowMatrix4<f32> {
		*self.transform.get_or_init(|| {
			glam::Mat4::from_rotation_translation(self.orientation.into(), self.origin.into())
				.into()
		})
	}
	pub fn origin(&self) -> Vec3 {
		self.origin
	}
	pub fn orientation(&self) -> Quat {
		self.orientation
	}
	pub fn direction(&self) -> Vec3 {
		let transform: Mat4 = self.transform().into();

		transform
			.transform_vector3(glam::vec3(0.0, 0.0, 1.0))
			.into()
	}
	pub fn deepest_point(&self) -> Vec3 {
		self.deepest_point
	}
}
impl Debug for Pointer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Pointer")
			.field("origin", &self.origin)
			.field("orientation", &self.orientation)
			.field("deepest_point", &self.deepest_point)
			.finish()
	}
}

#[derive(Debug)]
pub struct Hand {}

#[derive(Debug)]
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
	fn input(&self, input: InputData) -> bool;
}

pub struct InputHandler {
	pub spatial: Spatial,
	handler: HandlerWrapper<dyn InputHandlerHandler>,
}

#[buildstructor::buildstructor]
impl<'a> InputHandler {
	#[builder(entry = "builder")]
	pub async fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		field: &'a Field,
	) -> Result<Self, NodeError> {
		let node = generate_node!(
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
		);
		let handler = HandlerWrapper::new();

		node.add_handled_method(
			"input",
			handler.clone(),
			|handler: Arc<dyn InputHandlerHandler>, data| {
				let input = root_as_input_data(data)?.unpack();

				let input = InputData::try_new(
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
				)?;

				let capture = handler.input(input);

				Ok(flexbuffers::singleton(capture))
			},
		);

		Ok(InputHandler {
			spatial: Spatial { node },
			handler,
		})
	}
	pub fn set_handler<T: InputHandlerHandler + 'static>(&self, handler: &Arc<T>) {
		self.handler
			.set_handler(Arc::downgrade(handler) as Weak<dyn InputHandlerHandler>)
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

	let field = super::field::SphereField::builder()
		.spatial_parent(client.get_root())
		.radius(0.1)
		.build()
		.await
		.unwrap();

	let input_handler = InputHandler::builder()
		.spatial_parent(client.get_root())
		.field(&field)
		.build()
		.await
		.unwrap();

	struct InputHandlerTest;
	impl InputHandlerHandler for InputHandlerTest {
		fn input(&self, input: InputData) -> bool {
			dbg!(input);
			false
		}
	}

	let handler = Arc::new(InputHandlerTest);
	input_handler.set_handler(&handler);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}
