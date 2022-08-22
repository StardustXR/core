use super::{
	client::Client,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
	HandlerWrapper,
};
use crate::values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ZERO};
use async_trait::async_trait;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use std::{
	ops::Deref,
	path::Path,
	sync::{Arc, Weak},
};

pub trait Item: Send + Sync {
	type ItemType;
	const REGISTER_UI_FN: &'static str;
	const ROOT_PATH: &'static str;

	fn from_path(client: Weak<Client>, path: &str) -> Self;
	fn node(&self) -> &Node;
}

#[async_trait]
pub trait ItemUIHandler<I: Item>: Send + Sync {
	async fn create(&self, item_id: &str);
	async fn destroy(&self, item_id: &str);
}

pub struct ItemUI<I: Item> {
	handler: HandlerWrapper<dyn ItemUIHandler<I>>,
	node: Arc<Node>,
	pub items: Mutex<FxHashMap<String, I>>,
}
impl<I: Item<ItemType = I> + 'static> ItemUI<I> {
	pub async fn register(client: &Arc<Client>) -> Result<Arc<ItemUI<I>>, NodeError> {
		Ok(if client.item_uis.lock().contains::<Arc<ItemUI<I>>>() {
			client
				.item_uis
				.lock()
				.get::<Arc<ItemUI<I>>>()
				.unwrap()
				.clone()
		} else {
			let item_ui = Arc::new(ItemUI::<I> {
				handler: HandlerWrapper::new(),
				node: Node::from_path(Arc::downgrade(client), I::ROOT_PATH)?,
				items: Mutex::new(FxHashMap::default()),
			});

			item_ui
				.node
				.client
				.upgrade()
				.unwrap()
				.messenger
				.send_remote_signal("/item", I::REGISTER_UI_FN, &[])
				.await
				.map_err(|_| NodeError::ServerCreationFailed)?;

			item_ui.node.local_signals.insert(
				"create".to_string(),
				Box::new({
					let item_ui = item_ui.clone();
					move |data| {
						let name = flexbuffers::Reader::get_root(data)?.get_str()?;
						item_ui.handler.handle(|handler| {
							let item = I::from_path(
								item_ui.node.client.clone(),
								&format!("{}/{}", I::ROOT_PATH, name),
							);
							item_ui.items.lock().insert(name.to_string(), item);
							tokio::task::spawn({
								let name = name.to_string();
								async move { handler.create(&name).await }
							});
						});
						Ok(())
					}
				}),
			);
			item_ui.node.local_signals.insert(
				"destroy".to_string(),
				Box::new({
					let item_ui = item_ui.clone();
					move |data| {
						let name = flexbuffers::Reader::get_root(data)?.get_str()?;
						item_ui.handler.handle(|handler| {
							let items = item_ui.items.lock();
							if items.contains_key(name) {
								tokio::task::spawn({
									let name = name.to_string();
									async move { handler.destroy(&name).await }
								});
							}
						});
						Ok(())
					}
				}),
			);

			client.item_uis.lock().insert(item_ui.clone());
			item_ui
		})
	}

	pub fn set_handler<T: ItemUIHandler<I> + 'static>(&self, handler: &Arc<T>) {
		self.handler
			.set_handler(Arc::downgrade(handler) as Weak<dyn ItemUIHandler<I>>)
	}
}

pub struct EnvironmentItem {
	pub spatial: Spatial,
}

#[buildstructor::buildstructor]
impl<'a> EnvironmentItem {
	#[builder(entry = "builder")]
	pub async fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		file_path: &'a str,
	) -> Result<Self, NodeError> {
		let path = Path::new(file_path);
		if path.is_relative() || !path.exists() {
			return Err(NodeError::InvalidPath);
		}

		Ok(EnvironmentItem {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client: spatial_parent.node.client.clone(),
						parent_path: "/item/environment/item",
						interface_path: "/item",
						interface_method: "createEnvironmentItem"
					},
					spatial_parent.node.get_path(),
					position.unwrap_or(VEC3_ZERO),
					rotation.unwrap_or(QUAT_IDENTITY),
					file_path
				),
			},
		})
	}
}
impl Item for EnvironmentItem {
	type ItemType = EnvironmentItem;
	const REGISTER_UI_FN: &'static str = "registerEnvironmentItemUI";
	const ROOT_PATH: &'static str = "/item/environment";

	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn from_path(client: Weak<Client>, path: &str) -> Self {
		Self {
			spatial: Spatial {
				node: Node::from_path(client, path).unwrap(),
			},
		}
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
	use super::client::Client;
	use manifest_dir_macros::file_relative_path;
	let (client, _event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let _environment_item = EnvironmentItem::builder()
		.spatial_parent(client.get_root())
		.file_path(file_relative_path!("res/libstardustxr/grid_sky.hdr"))
		.build()
		.await
		.unwrap();
}

#[tokio::test]
async fn fusion_environment_ui() -> anyhow::Result<()> {
	let (client, event_loop) = Client::connect_with_async_loop().await?;

	struct EnvironmentUI {
		ui: Arc<ItemUI<EnvironmentItem>>,
	}
	#[async_trait]
	impl ItemUIHandler<EnvironmentItem> for EnvironmentUI {
		async fn create(&self, item_id: &str) {
			println!("Environment item {item_id} created");
		}
		async fn destroy(&self, item_id: &str) {
			println!("Environment item {item_id} destroyed");
		}
	}

	let test = Arc::new(EnvironmentUI {
		ui: ItemUI::register(&client).await?,
	});
	test.ui.set_handler(&test);

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}
