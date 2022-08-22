use super::{
	client::Client,
	drawable::Model,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
	HandlerWrapper,
};
use crate::{
	flex,
	values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ZERO},
};
use async_trait::async_trait;
use rustc_hash::FxHashMap;
use std::{
	ops::Deref,
	path::Path,
	sync::{Arc, Weak},
};
use tokio::sync::Mutex;

pub trait Item: Send + Sync {
	type ItemType;
	const REGISTER_UI_FN: &'static str;
	const ROOT_PATH: &'static str;

	fn from_path(client: Weak<Client>, path: &str) -> Self;
	fn node(&self) -> &Node;
}

#[async_trait]
pub trait ItemUIHandler<I: Item>: Send + Sync {
	async fn create(&self, item_id: &str, item: &I);
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
								&format!("{}/item/{}", I::ROOT_PATH, name),
							);
							let item_ui = item_ui.clone();
							tokio::task::spawn({
								let name = name.to_string();
								async move {
									let mut items = item_ui.items.lock().await;
									items.insert(name.clone(), item);
									handler.create(&name, items.get(&name).unwrap()).await
								}
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
						tokio::task::spawn({
							let name = name.to_string();
							let item_ui = item_ui.clone();
							async move {
								if item_ui.items.lock().await.contains_key(&name) {
									if let Some(handler) = item_ui.handler.handle(|handler| handler)
									{
										handler.destroy(&name).await;
									}
								}
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

pub struct PanelItem {
	pub spatial: Spatial,
}
impl PanelItem {
	pub async fn apply_surface_material(
		&self,
		model: &Model,
		material_index: u32,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal(
				"applySurfaceMaterial",
				flex::flexbuffer_from_vector_arguments(|vec| {
					vec.push(model.node.get_path());
					vec.push(material_index);
				})
				.as_slice(),
			)
			.await
	}
}
impl Item for PanelItem {
	type ItemType = PanelItem;
	const REGISTER_UI_FN: &'static str = "registerPanelItemUI";
	const ROOT_PATH: &'static str = "/item/panel";

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
impl Deref for PanelItem {
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
		async fn create(&self, item_id: &str, _item: &EnvironmentItem) {
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

#[tokio::test]
async fn fusion_panel_ui() -> anyhow::Result<()> {
	use manifest_dir_macros::directory_relative_path;
	let (client, event_loop) = Client::connect_with_async_loop().await?;
	client
		.set_base_prefixes(&[directory_relative_path!("res")])
		.await?;

	struct PanelUI {
		tex_cube: Model,
		ui: Arc<ItemUI<PanelItem>>,
	}
	#[async_trait]
	impl ItemUIHandler<PanelItem> for PanelUI {
		async fn create(&self, item_id: &str, item: &PanelItem) {
			println!("Panel item {item_id} created");
			let _ = item.apply_surface_material(&self.tex_cube, 0).await;
		}
		async fn destroy(&self, item_id: &str) {
			println!("Panel item {item_id} destroyed");
		}
	}

	let test = Arc::new(PanelUI {
		tex_cube: Model::resource_builder()
			.spatial_parent(client.get_root())
			.resource(&crate::fusion::resource::Resource::new(
				"libstardustxr",
				"tex_cube.glb",
			))
			.scale(glam::vec3(0.1, 0.1, 0.1))
			.build()
			.await?,
		ui: ItemUI::register(&client).await?,
	});
	test.ui.set_handler(&test);

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}
