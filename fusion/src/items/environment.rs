use std::ops::Deref;
use std::path::Path;
use std::sync::Weak;

use stardust_xr::values::Transform;

use crate::client::Client;
use crate::node::{Node, NodeError, NodeType};
use crate::spatial::Spatial;
use crate::{HandlerWrapper, WeakNodeRef, WeakWrapped};

use super::{Item, ItemUI, ItemUIType};

pub struct EnvironmentItem {
	pub spatial: Spatial,
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
					spatial_parent.node.client.clone(),
					"/item",
					"createEnvironmentItem",
					"/item/environment/item",
					&id.clone(),
					(
						id,
						spatial_parent,
						Transform {
							position,
							rotation,
							scale: None,
						},
						file_path,
					),
				)?,
			},
		})
	}
}
impl NodeType for EnvironmentItem {
	fn node(&self) -> &Node {
		&self.spatial.node
	}
}
impl Item for EnvironmentItem {
	type ItemType = EnvironmentItem;
	type InitData = String;
	const REGISTER_UI_FN: &'static str = "registerEnvironmentItemUI";
	const ROOT_PATH: &'static str = "/item/environment";

	fn node(&self) -> &Node {
		&self.spatial.node
	}
}
impl<T: Send + Sync + 'static> ItemUIType<T> for ItemUI<EnvironmentItem, T> {
	type Item = EnvironmentItem;

	fn from_path<F>(
		client: Weak<Client>,
		path: &str,
		init_data: String,
		mut ui_init_fn: F,
	) -> HandlerWrapper<EnvironmentItem, T>
	where
		F: FnMut(String, WeakWrapped<T>, WeakNodeRef<EnvironmentItem>, &EnvironmentItem) -> T
			+ Clone
			+ Send
			+ Sync
			+ 'static,
		T: Send + Sync + 'static,
	{
		let item = EnvironmentItem {
			spatial: Spatial {
				node: Node::from_path(client, path.to_string()).unwrap(),
			},
		};
		HandlerWrapper::new(item, |weak_wrapped, weak_node_ref, f| {
			ui_init_fn(init_data, weak_wrapped, weak_node_ref, f)
		})
	}
}
impl Deref for EnvironmentItem {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_environment_item() {
	use manifest_dir_macros::file_relative_path;
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let _environment_item = EnvironmentItem::builder()
		.spatial_parent(client.get_root())
		.file_path(file_relative_path!("res/fusion/sky.hdr"))
		.build()
		.unwrap();
}

#[tokio::test]
async fn fusion_environment_ui() -> anyhow::Result<()> {
	let (client, event_loop) = Client::connect_with_async_loop().await?;

	struct EnvironmentUI {
		path: String,
		_item: WeakNodeRef<EnvironmentItem>,
	}
	impl EnvironmentUI {
		pub fn new(path: String, _item: WeakNodeRef<EnvironmentItem>) -> Self {
			println!("Environment item with path {path} created");
			EnvironmentUI { path, _item }
		}
	}
	impl Drop for EnvironmentUI {
		fn drop(&mut self) {
			println!("Environment item with path {} destroyed", self.path)
		}
	}

	let _item_ui = ItemUI::register(
		&client,
		|init_data, _weak_wrapped, weak_node_ref, _item: &EnvironmentItem| {
			EnvironmentUI::new(init_data, weak_node_ref)
		},
	)?;

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}
