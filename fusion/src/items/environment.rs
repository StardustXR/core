use super::Item;
use crate::client::Client;
use crate::node::{Node, NodeError, NodeType};
use crate::spatial::Spatial;
use stardust_xr::values::Transform;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

pub struct EnvironmentItem {
	spatial: Spatial,
	pub path: String,
}

#[buildstructor::buildstructor]
impl<'a> EnvironmentItem {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		file_path: &'a str,
	) -> Result<Self, NodeError> {
		let path = Path::new(file_path);
		if path.is_relative() || !path.exists() {
			return Err(NodeError::InvalidPath);
		}

		let id = nanoid::nanoid!();
		Ok(EnvironmentItem {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/item",
					"create_environment_item",
					"/item/environment/item",
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
						file_path,
					),
				)?,
			},
			path: file_path.to_string(),
		})
	}
}
impl NodeType for EnvironmentItem {
	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn alias(&self) -> Self {
		EnvironmentItem {
			spatial: self.spatial.alias(),
			path: self.path.clone(),
		}
	}
}
impl Item for EnvironmentItem {
	type ItemType = EnvironmentItem;
	type InitData = String;
	const TYPE_NAME: &'static str = "environment";

	fn from_path(
		client: &Arc<Client>,
		parent_path: impl ToString,
		name: impl ToString,
		init_data: &String,
	) -> Self {
		EnvironmentItem {
			spatial: Spatial {
				node: Node::from_path(client, parent_path, name, false),
			},
			path: init_data.clone(),
		}
	}
	// fn alias
}
impl Deref for EnvironmentItem {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_environment_ui() {
	use manifest_dir_macros::file_relative_path;
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();

	let environment_item = EnvironmentItem::builder()
		.spatial_parent(client.get_root())
		.file_path(file_relative_path!("res/fusion/sky.hdr"))
		.build()
		.unwrap();

	struct EnvironmentUIManager(Arc<Client>);
	impl crate::items::ItemUIHandler<EnvironmentItem> for EnvironmentUIManager {
		fn item_created(&mut self, uid: &str, _item: EnvironmentItem, path: String) {
			println!("Environment item {uid} created with path {path}");
		}
		fn item_captured(&mut self, uid: &str, acceptor_uid: &str, _item: EnvironmentItem) {
			println!("Capturing environment item {uid} in acceptor {acceptor_uid}");
		}
		fn item_released(&mut self, uid: &str, acceptor_uid: &str, _item: EnvironmentItem) {
			println!("Released environment item {uid} from acceptor {acceptor_uid}");
		}
		fn item_destroyed(&mut self, _uid: &str) {}
		fn acceptor_created(
			&mut self,
			_uid: &str,
			_acceptor: crate::items::ItemAcceptor<EnvironmentItem>,
		) {
		}
		fn acceptor_destroyed(&mut self, _uid: &str) {}
	}
	impl crate::items::ItemAcceptorHandler<EnvironmentItem> for EnvironmentUIManager {
		fn captured(&mut self, uid: &str, item: EnvironmentItem, path: String) {
			println!("Item {uid} accepted sucessfully with path {path}!");
			item.release().unwrap();
		}
		fn released(&mut self, uid: &str) {
			println!("Got {uid} released sucessfully!");
			self.0.stop_loop();
		}
	}

	let _item_ui = crate::items::ItemUI::register(&client)
		.unwrap()
		.wrap(EnvironmentUIManager(client.clone()))
		.unwrap();

	let item_acceptor_field =
		crate::fields::SphereField::create(client.get_root(), None, 0.5).unwrap();
	let item_acceptor =
		crate::items::ItemAcceptor::create(client.get_root(), None, None, &item_acceptor_field)
			.unwrap()
			.wrap(EnvironmentUIManager(client.clone()))
			.unwrap();

	item_acceptor.node().capture(&environment_item).unwrap();

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(1)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	}
}
