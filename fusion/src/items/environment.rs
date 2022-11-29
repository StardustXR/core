use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use stardust_xr::values::Transform;

use super::{HandledItem, Item};
use crate::client::Client;
use crate::node::{Node, NodeError, NodeType};
use crate::spatial::Spatial;
use crate::{HandlerWrapper, WeakWrapped};

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
	const TYPE_NAME: &'static str = "environment";
}

impl<T: Send + Sync + 'static> HandledItem<T> for EnvironmentItem {
	fn from_path<F>(
		client: &Arc<Client>,
		parent_path: impl ToString,
		name: impl ToString,
		init_data: Self::InitData,
		mut ui_init_fn: F,
	) -> HandlerWrapper<Self, T>
	where
		F: FnMut(Self::InitData, WeakWrapped<T>, &Arc<Self>) -> T + Clone + Send + Sync + 'static,
		T: Send + Sync + 'static,
	{
		let item = EnvironmentItem {
			spatial: Spatial {
				node: Node::from_path(client, parent_path, name, false),
			},
		};
		HandlerWrapper::new(item, |weak_wrapped, node_ref| {
			ui_init_fn(init_data, weak_wrapped, node_ref)
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
async fn fusion_environment_ui() -> anyhow::Result<()> {
	use manifest_dir_macros::file_relative_path;
	let (client, event_loop) = Client::connect_with_async_loop().await?;

	let environment_item = EnvironmentItem::builder()
		.spatial_parent(client.get_root())
		.file_path(file_relative_path!("res/fusion/sky.hdr"))
		.build()
		.unwrap();
	dbg!(environment_item.node());

	struct EnvironmentUI {
		path: String,
		_item: Arc<EnvironmentItem>,
		acceptor: bool,
	}
	impl EnvironmentUI {
		pub fn new(path: String, _item: Arc<EnvironmentItem>, acceptor: bool) -> Self {
			println!("Environment item with path {path} created");
			EnvironmentUI {
				path,
				_item,
				acceptor,
			}
		}
	}
	impl crate::items::ItemHandler<EnvironmentItem> for EnvironmentUI {
		fn captured(&mut self, item: &EnvironmentItem, acceptor_uid: &str) {
			println!(
				"Acceptor {} captured environment item {}",
				acceptor_uid,
				item.uid().unwrap()
			);
			if self.acceptor {
				println!("Got accepted sucessfully!");
				item.release().unwrap();
			}
		}
		fn released(&mut self, item: &EnvironmentItem, acceptor_uid: &str) {
			println!(
				"Acceptor {} released environment item {}",
				acceptor_uid,
				item.uid().unwrap()
			);
			if self.acceptor {
				println!("Got released sucessfully!");
				if let Ok(client) = item.client() {
					client.stop_loop();
				}
			}
		}
	}
	impl Drop for EnvironmentUI {
		fn drop(&mut self) {
			println!("Environment item with path {} destroyed", self.path)
		}
	}

	let _item_ui = crate::items::ItemUI::register(
		&client,
		|init_data, _weak_wrapped, item: &Arc<EnvironmentItem>| {
			EnvironmentUI::new(init_data, item.clone(), false)
		},
	)?;

	let item_acceptor_field =
		crate::fields::SphereField::create(client.get_root(), None, 0.5).unwrap();
	let item_acceptor = crate::items::ItemAcceptor::create(
		client.get_root(),
		None,
		None,
		&item_acceptor_field,
		|init_data, _weak_wrapped, item: &Arc<EnvironmentItem>| {
			EnvironmentUI::new(init_data, item.clone(), true)
		},
	)
	.unwrap();
	dbg!(item_acceptor.node());

	item_acceptor.capture(&environment_item).unwrap();

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out"))?,
		e = event_loop => e??,
	}
	Ok(())
}
