use super::Item;
use crate::client::Client;
use crate::drawable::ModelPart;
use crate::node::{Node, NodeError, NodeType};
use crate::spatial::Spatial;
use mint::{RowMatrix4, Vector2};
use stardust_xr::schemas::flex::serialize;
use stardust_xr::values::{BufferInfo, Transform};
use std::future::Future;
use std::ops::Deref;
use std::os::fd::OwnedFd;
use std::sync::Arc;

/// Item that can render using a virtual camera to a preview material
pub struct CameraItem {
	spatial: Spatial,
}
impl CameraItem {
	/// Create a new camera item.
	pub fn create(
		spatial_parent: &Spatial,
		transform: Transform,
		proj_matrix: impl Into<RowMatrix4<f32>>,
		preview_size: impl Into<Vector2<u32>>,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(CameraItem {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/item",
					"create_camera_item",
					"/item/camera/item",
					true,
					&id.clone(),
					(
						id,
						spatial_parent.node().get_path()?,
						transform,
						proj_matrix.into(),
						preview_size.into(),
					),
				)?,
			},
		})
	}

	/// Apply the camera's preview as a material to a model.
	///
	/// The material index is global across the whole model for now, just play around with it a bit.
	pub fn apply_preview_material(&self, model_part: &ModelPart) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("apply_preview_material", &(model_part.node().get_path()?))
	}

	/// Set the camera's projection matrix.
	pub fn set_proj_matrix(
		&self,
		proj_matrix: impl Into<RowMatrix4<f32>>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("set_proj_matrix", &(proj_matrix.into()))
	}

	/// Request the camera's view to be render to a Dmabuf.
	///
	/// To avoid tearing, the buffer should not be read until receiving the response that it has been rendered.
	pub fn render(
		&self,
		buffer_info: BufferInfo,
		fds: Vec<OwnedFd>,
	) -> Result<impl Future<Output = Result<(), NodeError>>, NodeError> {
		let send_data = serialize(buffer_info).map_err(|_| NodeError::Serialization)?;
		let future = self
			.node
			.execute_remote_method_raw("render", &send_data, fds)?;
		Ok(async move { future.await.map(|_| ()) })
	}
}
impl NodeType for CameraItem {
	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn alias(&self) -> Self {
		CameraItem {
			spatial: self.spatial.alias(),
		}
	}
}
impl Item for CameraItem {
	type InitData = ();
	const TYPE_NAME: &'static str = "camera";

	fn from_path(
		client: &Arc<Client>,
		parent_path: impl ToString,
		name: impl ToString,
		_init_data: &(),
	) -> Self {
		CameraItem {
			spatial: Spatial {
				node: Node::from_path(client, parent_path, name, false),
			},
		}
	}
	// fn alias
}
impl Deref for CameraItem {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_camera_ui() {
	color_eyre::install().unwrap();
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();

	let camera_item = CameraItem::create(
		client.get_root(),
		Transform::none(),
		glam::Mat4::perspective_infinite_rh(
			std::f32::consts::PI * 0.5,
			std::f32::consts::PI * 0.5,
			0.01,
		),
		[512, 512],
	)
	.unwrap();

	struct CameraUIManager(Arc<Client>);
	impl crate::items::ItemUIHandler<CameraItem> for CameraUIManager {
		fn item_created(&mut self, uid: &str, _item: CameraItem, _data: ()) {
			println!("Camera item {uid} created");
		}
		fn item_captured(&mut self, uid: &str, acceptor_uid: &str, _item: CameraItem) {
			println!("Capturing camera item {uid} in acceptor {acceptor_uid}");
		}
		fn item_released(&mut self, uid: &str, acceptor_uid: &str, _item: CameraItem) {
			println!("Released camera item {uid} from acceptor {acceptor_uid}");
		}
		fn item_destroyed(&mut self, _uid: &str) {}
		fn acceptor_created(
			&mut self,
			_uid: &str,
			_acceptor: crate::items::ItemAcceptor<CameraItem>,
			_field: crate::fields::UnknownField,
		) {
		}
		fn acceptor_destroyed(&mut self, _uid: &str) {}
	}
	impl crate::items::ItemAcceptorHandler<CameraItem> for CameraUIManager {
		fn captured(&mut self, uid: &str, item: CameraItem, _data: ()) {
			println!("Item {uid} accepted sucessfully!");
			item.release().unwrap();
		}
		fn released(&mut self, uid: &str) {
			println!("Got {uid} released sucessfully!");
			self.0.stop_loop();
		}
	}

	let _item_ui = crate::items::ItemUI::register(&client)
		.unwrap()
		.wrap(CameraUIManager(client.clone()))
		.unwrap();

	let item_acceptor_field =
		crate::fields::SphereField::create(client.get_root(), mint::Vector3::from([0.0; 3]), 0.5)
			.unwrap();
	let item_acceptor = crate::items::ItemAcceptor::create(
		client.get_root(),
		Transform::none(),
		&item_acceptor_field,
	)
	.unwrap()
	.wrap(CameraUIManager(client.clone()))
	.unwrap();

	item_acceptor.node().capture(&camera_item).unwrap();

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(1)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	}
}
