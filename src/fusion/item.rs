use super::{
	client::Client,
	drawable::Model,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
	HandlerWrapper,
};
use crate::{
	flex::{self, flexbuffer_from_vector_arguments},
	values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ZERO},
};
use async_trait::async_trait;
use mint::Vector2;
use rustc_hash::FxHashMap;
use std::{
	ops::Deref,
	path::Path,
	sync::{Arc, Weak},
};
use tokio::sync::Mutex;

pub trait Item: Send + Sync {
	type ItemType;
	type InitData: Send;
	const REGISTER_UI_FN: &'static str;
	const ROOT_PATH: &'static str;

	fn parse_init_data(
		flex_vec: flexbuffers::VectorReader<&[u8]>,
	) -> Result<Self::InitData, flexbuffers::ReaderError>;
	fn from_path(client: Weak<Client>, path: &str) -> Self;
	fn node(&self) -> &Node;
}

#[async_trait]
pub trait ItemUIHandler<I: Item>: Send + Sync {
	async fn create(&self, item_id: &str, item: &Arc<I>, init_data: I::InitData);
	async fn destroy(&self, item_id: &str);
}

pub struct ItemUI<I: Item> {
	handler: HandlerWrapper<dyn ItemUIHandler<I>>,
	node: Arc<Node>,
	pub items: Mutex<FxHashMap<String, Arc<I>>>,
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
						if let Some(handler) = item_ui.handler.get_handler() {
							let flex_vec = flexbuffers::Reader::get_root(data)?.get_vector()?;
							let name = flex_vec.index(0)?.get_str()?.to_string();
							let item = I::from_path(
								item_ui.node.client.clone(),
								&format!("{}/item/{}", I::ROOT_PATH, name),
							);
							let item_ui = item_ui.clone();
							let init_data = I::parse_init_data(flex_vec.index(1)?.get_vector()?)?;
							tokio::task::spawn({
								async move {
									let mut items = item_ui.items.lock().await;
									items.insert(name.clone(), Arc::new(item));
									handler
										.create(&name, items.get(&name).unwrap(), init_data)
										.await
								}
							});
						}
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
									if let Some(handler) = item_ui.handler.get_handler() {
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
	type InitData = String;
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

	fn parse_init_data(
		flex_vec: flexbuffers::VectorReader<&[u8]>,
	) -> Result<String, flexbuffers::ReaderError> {
		Ok(flex_vec.index(0)?.get_str()?.to_string())
	}
}
impl Deref for EnvironmentItem {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[derive(Debug)]
pub struct PanelItemCursor {
	pub size: Vector2<u32>,
	pub hotspot: Vector2<i32>,
}

#[async_trait]
pub trait PanelItemHandler: Send + Sync {
	async fn resize(&self, size: Vector2<u32>);
	async fn set_cursor(&self, info: Option<PanelItemCursor>);
}

#[derive(Debug)]
pub struct PanelItemInitData {
	pub size: Vector2<u32>,
	pub cursor: Option<PanelItemCursor>,
}

pub struct PanelItem {
	pub spatial: Spatial,
	handler: HandlerWrapper<dyn PanelItemHandler>,
}
impl PanelItem {
	pub fn set_handler<T: PanelItemHandler + 'static>(&self, handler: &Arc<T>) {
		self.handler
			.set_handler(Arc::downgrade(handler) as Weak<dyn PanelItemHandler>)
	}

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

	pub async fn apply_cursor_material(
		&self,
		_cursor: &PanelItemCursor,
		model: &Model,
		material_index: u32,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal(
				"applyCursorMaterial",
				flex::flexbuffer_from_vector_arguments(|vec| {
					vec.push(model.node.get_path());
					vec.push(material_index);
				})
				.as_slice(),
			)
			.await
	}

	pub async fn pointer_deactivate(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("pointerDeactivate", &[]).await
	}

	pub async fn pointer_motion(&self, position: impl Into<Vector2<f32>>) -> Result<(), NodeError> {
		self.node
			.send_remote_signal(
				"pointerMotion",
				&flexbuffer_from_vector_arguments(|vec| {
					let position = position.into();
					vec.push(position.x);
					vec.push(position.y);
				}),
			)
			.await
	}

	pub async fn pointer_button(&self, button: u32, state: u32) -> Result<(), NodeError> {
		self.node
			.send_remote_signal(
				"pointerButton",
				&flexbuffer_from_vector_arguments(|vec| {
					vec.push(button);
					vec.push(state);
				}),
			)
			.await
	}

	pub async fn pointer_scroll(
		&self,
		scroll_distance: Vector2<f32>,
		scroll_steps: Vector2<f32>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal(
				"pointerScroll",
				&flexbuffer_from_vector_arguments(|vec| {
					push_to_vec!(vec, scroll_distance);
					push_to_vec!(vec, scroll_steps);
				}),
			)
			.await
	}
}
impl Item for PanelItem {
	type ItemType = PanelItem;
	type InitData = PanelItemInitData;
	const REGISTER_UI_FN: &'static str = "registerPanelItemUI";
	const ROOT_PATH: &'static str = "/item/panel";

	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn from_path(client: Weak<Client>, path: &str) -> Self {
		let panel_item = Self {
			spatial: Spatial {
				node: Node::from_path(client, path).unwrap(),
			},
			handler: HandlerWrapper::new(),
		};

		panel_item.node.local_signals.insert(
			"resize".to_string(),
			Box::new({
				let handler = panel_item.handler.clone();
				move |data| {
					if let Some(handler) = handler.get_handler() {
						let flex_vec = flexbuffers::Reader::get_root(data)?.get_vector()?;
						let x = flex_vec.idx(0).get_u64()? as u32;
						let y = flex_vec.idx(1).get_u64()? as u32;
						tokio::task::spawn(
							async move { handler.resize(Vector2::from([x, y])).await },
						);
					}
					Ok(())
				}
			}),
		);

		panel_item.node.local_signals.insert(
			"setCursor".to_string(),
			Box::new({
				let handler = panel_item.handler.clone();
				move |data| {
					if let Some(handler) = handler.get_handler() {
						let flex = flexbuffers::Reader::get_root(data)?;
						let data: Option<PanelItemCursor> = if !flex.flexbuffer_type().is_null() {
							let flex_vec = flex.get_vector()?;
							let size_vec = flex_vec.idx(0).get_vector()?;
							let size_x = size_vec.idx(0).get_u64()? as u32;
							let size_y = size_vec.idx(1).get_u64()? as u32;

							let hotspot_vec = flex_vec.idx(1).get_vector()?;
							let hotspot_x = hotspot_vec.idx(0).get_i64()? as i32;
							let hotspot_y = hotspot_vec.idx(1).get_i64()? as i32;
							Some(PanelItemCursor {
								size: Vector2::from([size_x, size_y]),
								hotspot: Vector2::from([hotspot_x, hotspot_y]),
							})
						} else {
							None
						};
						tokio::task::spawn(async move { handler.set_cursor(data).await });
					}
					Ok(())
				}
			}),
		);

		panel_item
	}

	fn parse_init_data(
		flex_vec: flexbuffers::VectorReader<&[u8]>,
	) -> Result<PanelItemInitData, flexbuffers::ReaderError> {
		let size_vec = flex_vec.index(0)?.get_vector()?;

		Ok(PanelItemInitData {
			size: Vector2::from([
				size_vec.idx(0).get_u64()? as u32,
				size_vec.idx(1).get_u64()? as u32,
			]),
			cursor: {
				let cursor = flex_vec.index(1)?;
				match cursor.flexbuffer_type() {
					flexbuffers::FlexBufferType::Null => None,
					flexbuffers::FlexBufferType::Vector => {
						let cursor_vec = cursor.get_vector()?;
						let cursor_size_vec = cursor_vec.idx(0).get_vector()?;
						let cursor_hotspot_vec = cursor_vec.idx(1).get_vector()?;
						Some(PanelItemCursor {
							size: Vector2::from([
								cursor_size_vec.index(0)?.get_u64()? as u32,
								cursor_size_vec.index(1)?.get_u64()? as u32,
							]),
							hotspot: Vector2::from([
								cursor_hotspot_vec.index(0)?.get_i64()? as i32,
								cursor_hotspot_vec.index(1)?.get_i64()? as i32,
							]),
						})
					}
					_ => return Err(flexbuffers::ReaderError::FlexbufferOutOfBounds),
				}
			},
		})
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
		async fn create(&self, item_id: &str, _item: &Arc<EnvironmentItem>, init_data: String) {
			println!("Environment item {item_id} created with path {init_data}");
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

	struct PanelUIDemo {
		tex_cube: Model,
		ui: Arc<ItemUI<PanelItem>>,
		panels: Mutex<FxHashMap<String, Arc<PanelItemUI>>>,
	}
	#[async_trait]
	impl ItemUIHandler<PanelItem> for PanelUIDemo {
		async fn create(&self, item_id: &str, item: &Arc<PanelItem>, init_data: PanelItemInitData) {
			println!("Panel item {item_id} created with size {:?}", init_data);
			let item_ui = Arc::new(PanelItemUI {
				item: Arc::downgrade(item),
			});
			item.set_handler(&item_ui);
			self.panels
				.lock()
				.await
				.insert(item_id.to_string(), item_ui);
			let _ = item.apply_surface_material(&self.tex_cube, 0).await;
			let _ = item
				.pointer_motion([(init_data.size.x / 2) as f32, (init_data.size.x / 2) as f32])
				.await;
		}
		async fn destroy(&self, item_id: &str) {
			println!("Panel item {item_id} destroyed");
		}
	}

	struct PanelItemUI {
		item: Weak<PanelItem>,
	}
	#[async_trait]
	impl PanelItemHandler for PanelItemUI {
		async fn resize(&self, size: Vector2<u32>) {
			println!("Got resize of {}, {}", size.x, size.y);
		}

		async fn set_cursor(&self, info: Option<PanelItemCursor>) {
			println!("Set cursor with info {:?}", info);
		}
	}

	let test = Arc::new(PanelUIDemo {
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
		panels: Mutex::new(FxHashMap::default()),
	});
	test.ui.set_handler(&test);

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}
