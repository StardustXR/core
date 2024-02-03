use super::ItemAspect;
use crate::client::Client;
use crate::node::{Node, NodeError, NodeType};
use crate::spatial::{SpatialAspect, Transform};
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Item that contains the path to an equirectangular `.hdr` file.
pub struct EnvironmentItem(Node);

impl EnvironmentItem {
	/// Create a new environment item from a file path.
	pub fn create<P: AsRef<Path>>(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		file_path: P,
	) -> Result<Self, NodeError> {
		let path = file_path.as_ref();
		if path.is_relative() || !path.exists() {
			return Err(NodeError::InvalidPath);
		}

		let id = nanoid::nanoid!();
		Ok(EnvironmentItem(Node::new(
			&spatial_parent.node().client()?,
			"/item",
			"create_environment_item",
			"/item/environment/item",
			true,
			&id.clone(),
			(id, spatial_parent.node().get_path()?, transform, path),
		)?))
	}
}

impl NodeType for EnvironmentItem {
	fn node(&self) -> &Node {
		&self.0
	}

	fn alias(&self) -> Self {
		EnvironmentItem(self.0.alias())
	}

	fn from_path(client: &Arc<Client>, path: String, destroyable: bool) -> Self {
		EnvironmentItem(Node::from_path(client, path, destroyable))
	}
}
impl SpatialAspect for EnvironmentItem {}
impl ItemAspect for EnvironmentItem {
	type InitData = PathBuf;
	const TYPE_NAME: &'static str = "environment";
}

#[tokio::test]
async fn fusion_environment_ui() {
	color_eyre::install().unwrap();
	use manifest_dir_macros::file_relative_path;
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();

	let environment_item = EnvironmentItem::create(
		client.get_root(),
		Transform::none(),
		file_relative_path!("res/fusion/sky.hdr"),
	)
	.unwrap();

	struct EnvironmentUIManager(Arc<Client>);
	impl crate::items::ItemUIHandler<EnvironmentItem> for EnvironmentUIManager {
		fn item_created(&mut self, item_uid: String, _item: EnvironmentItem, path: PathBuf) {
			println!(
				"Environment item {item_uid} created with path {}",
				path.display()
			);
		}
		fn item_captured(&mut self, item_uid: String, acceptor_uid: String) {
			println!("Capturing environment item {item_uid} in acceptor {acceptor_uid}");
		}
		fn item_released(&mut self, item_uid: String, acceptor_uid: String) {
			println!("Released environment item {item_uid} from acceptor {acceptor_uid}");
		}
		fn item_destroyed(&mut self, _item_uid: String) {}
		fn acceptor_created(
			&mut self,
			_uid: String,
			_acceptor: crate::items::ItemAcceptor<EnvironmentItem>,
			_field: crate::fields::UnknownField,
		) {
		}
		fn acceptor_destroyed(&mut self, _uid: String) {}
	}
	impl crate::items::ItemAcceptorHandler<EnvironmentItem> for EnvironmentUIManager {
		fn captured(&mut self, uid: String, item: EnvironmentItem, path: PathBuf) {
			println!(
				"Item {uid} accepted sucessfully with path {}!",
				path.display()
			);
			item.release().unwrap();
		}
		fn released(&mut self, uid: String) {
			println!("Got {uid} released sucessfully!");
			self.0.stop_loop();
		}
	}

	let _item_ui = crate::items::ItemUI::register(&client)
		.unwrap()
		.wrap(EnvironmentUIManager(client.clone()))
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
	.wrap(EnvironmentUIManager(client.clone()))
	.unwrap();

	item_acceptor.node().capture(&environment_item).unwrap();

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(1)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	}
}
