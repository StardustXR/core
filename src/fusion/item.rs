use super::{
	client::Client,
	drawable::Model,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
	HandlerWrapper, WeakHandler,
};
use crate::{
	flex::{self, flexbuffer_from_vector_arguments},
	values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ZERO},
};
use mint::Vector2;
use rustc_hash::FxHashMap;
use std::{
	ops::Deref,
	path::Path,
	sync::{Arc, Weak},
};
use tokio::sync::Mutex;
use xkbcommon::xkb::{self, Keymap, KEYMAP_FORMAT_TEXT_V1};

pub trait Item: Sized + Send + Sync {
	type ItemType;
	type InitData: Send;
	const REGISTER_UI_FN: &'static str;
	const ROOT_PATH: &'static str;

	fn parse_init_data(
		flex_vec: flexbuffers::VectorReader<&[u8]>,
	) -> Result<Self::InitData, flexbuffers::ReaderError>;
	fn node(&self) -> &Node;
}

pub struct ItemUI<I: Item, T: Send + Sync + 'static> {
	node: Arc<Node>,
	items: Mutex<FxHashMap<String, HandlerWrapper<I, T>>>,
}

pub trait ItemUIType<T: Send + Sync + 'static>: Sized {
	type Item: Item + 'static;

	fn register<F>(
		client: &Arc<Client>,
		item_ui_init: F,
	) -> Result<Arc<ItemUI<Self::Item, T>>, NodeError>
	where
		F: FnMut(<<Self as ItemUIType<T>>::Item as Item>::InitData, &Self::Item) -> T
			+ Clone
			+ Send
			+ Sync
			+ 'static,
	{
		if client
			.item_uis
			.lock()
			.contains::<Arc<ItemUI<Self::Item, T>>>()
		{
			Ok(client
				.item_uis
				.lock()
				.get::<Arc<ItemUI<Self::Item, T>>>()
				.unwrap()
				.clone())
		} else {
			Self::new_item_ui(client, item_ui_init)
		}
	}

	fn new_item_ui<F>(
		client: &Arc<Client>,
		item_ui_init: F,
	) -> Result<Arc<ItemUI<Self::Item, T>>, NodeError>
	where
		F: FnMut(<<Self as ItemUIType<T>>::Item as Item>::InitData, &Self::Item) -> T
			+ Clone
			+ Send
			+ Sync
			+ 'static,
	{
		let item_ui = Arc::new(ItemUI::<Self::Item, T> {
			node: Node::from_path(Arc::downgrade(client), Self::Item::ROOT_PATH).unwrap(),
			items: Mutex::new(FxHashMap::default()),
		});

		item_ui
			.node
			.client
			.upgrade()
			.unwrap()
			.messenger
			.send_remote_signal("/item", Self::Item::REGISTER_UI_FN, &[])
			.map_err(|_| NodeError::ServerCreationFailed)?;

		item_ui.node.local_signals.insert(
			"create".to_string(),
			Box::new({
				let item_ui = item_ui.clone();
				move |data| {
					let flex_vec = flexbuffers::Reader::get_root(data)?.get_vector()?;
					let name = flex_vec.index(0)?.get_str()?.to_string();
					let init_data = Self::Item::parse_init_data(flex_vec.index(1)?.get_vector()?)?;

					let item = Self::from_path(
						item_ui.node.client.clone(),
						&format!("{}/item/{}", Self::Item::ROOT_PATH, name),
						init_data,
						item_ui_init.clone(),
					);
					tokio::task::spawn({
						let item_ui = item_ui.clone();
						async move {
							item_ui.items.lock().await.insert(name.clone(), item);
						}
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
					item_ui.items.blocking_lock().remove(name);
					Ok(())
				}
			}),
		);

		client.item_uis.lock().insert(item_ui.clone());
		Ok(item_ui)
	}

	fn from_path<F>(
		client: Weak<Client>,
		path: &str,
		init_data: <<Self as ItemUIType<T>>::Item as Item>::InitData,
		ui_init_fn: F,
	) -> HandlerWrapper<Self::Item, T>
	where
		F: FnMut(<<Self as ItemUIType<T>>::Item as Item>::InitData, &Self::Item) -> T
			+ Clone
			+ Send
			+ Sync
			+ 'static,
		T: Send + Sync + 'static;
}

pub struct EnvironmentItem {
	pub spatial: Spatial,
}

#[buildstructor::buildstructor]
impl<'a> EnvironmentItem {
	#[builder(entry = "builder")]
	pub fn create(
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

	fn parse_init_data(
		flex_vec: flexbuffers::VectorReader<&[u8]>,
	) -> Result<String, flexbuffers::ReaderError> {
		Ok(flex_vec.index(0)?.get_str()?.to_string())
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
		F: FnMut(String, &Self::Item) -> T + Clone + Send + Sync + 'static,
		T: Send + Sync + 'static,
	{
		let item = EnvironmentItem {
			spatial: Spatial {
				node: Node::from_path(client, path).unwrap(),
			},
		};
		HandlerWrapper::new(item, |_, f| ui_init_fn(init_data, f))
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

pub trait PanelItemHandler: Send + Sync {
	fn resize(&self, size: Vector2<u32>);
	fn set_cursor(&self, info: Option<PanelItemCursor>);
}

#[derive(Debug)]
pub struct PanelItemInitData {
	pub size: Vector2<u32>,
	pub cursor: Option<PanelItemCursor>,
}

pub struct PanelItem {
	pub spatial: Spatial,
}
impl PanelItem {
	pub fn apply_surface_material(
		&self,
		model: &Model,
		material_index: u32,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"applySurfaceMaterial",
			flex::flexbuffer_from_vector_arguments(|vec| {
				vec.push(model.node.get_path());
				vec.push(material_index);
			})
			.as_slice(),
		)
	}

	pub fn apply_cursor_material(
		&self,
		_cursor: &PanelItemCursor,
		model: &Model,
		material_index: u32,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"applyCursorMaterial",
			flex::flexbuffer_from_vector_arguments(|vec| {
				vec.push(model.node.get_path());
				vec.push(material_index);
			})
			.as_slice(),
		)
	}

	pub fn pointer_deactivate(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("pointerDeactivate", &[])
	}

	pub fn pointer_motion(&self, position: impl Into<Vector2<f32>>) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"pointerMotion",
			&flexbuffer_from_vector_arguments(|vec| {
				let position = position.into();
				vec.push(position.x);
				vec.push(position.y);
			}),
		)
	}

	pub fn pointer_button(&self, button: u32, state: u32) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"pointerButton",
			&flexbuffer_from_vector_arguments(|vec| {
				vec.push(button);
				vec.push(state);
			}),
		)
	}

	pub fn pointer_scroll(
		&self,
		scroll_distance: Vector2<f32>,
		scroll_steps: Vector2<f32>,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"pointerScroll",
			&flexbuffer_from_vector_arguments(|vec| {
				push_to_vec!(vec, scroll_distance);
				push_to_vec!(vec, scroll_steps);
			}),
		)
	}

	pub fn keyboard_activate(&self, keymap: &str) -> Result<(), NodeError> {
		Keymap::new_from_string(
			&xkb::Context::new(0),
			keymap.to_string(),
			KEYMAP_FORMAT_TEXT_V1,
			0,
		)
		.ok_or(NodeError::InvalidPath)?;
		let data = flexbuffers::singleton(keymap);
		self.node
			.send_remote_signal("keyboardActivateString", &data)
	}
	pub fn keyboard_deactivate(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("keyboardDeactivate", &[])
	}
	pub fn keyboard_key_state(&self, key: u32, state: bool) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"keyboardKeyState",
			&flexbuffer_from_vector_arguments(|vec| {
				vec.push(key);
				vec.push(state as u32);
			}),
		)
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
impl<T: PanelItemHandler + Send + Sync + 'static> ItemUIType<T> for ItemUI<PanelItem, T> {
	type Item = PanelItem;

	fn from_path<F>(
		client: Weak<Client>,
		path: &str,
		init_data: PanelItemInitData,
		mut ui_init_fn: F,
	) -> HandlerWrapper<PanelItem, T>
	where
		F: FnMut(PanelItemInitData, &PanelItem) -> T + Clone + Send + Sync + 'static,
		T: Send + Sync + 'static,
	{
		let item = PanelItem {
			spatial: Spatial {
				node: Node::from_path(client, path).unwrap(),
			},
		};

		HandlerWrapper::new(item, |handler: WeakHandler<T>, item| {
			item.node.local_signals.insert(
				"resize".to_string(),
				Box::new({
					let handler = handler.clone();
					move |data| {
						if let Some(handler) = handler.upgrade() {
							let flex_vec = flexbuffers::Reader::get_root(data)?.get_vector()?;
							let x = flex_vec.idx(0).get_u64()? as u32;
							let y = flex_vec.idx(1).get_u64()? as u32;
							handler.lock().resize(Vector2::from([x, y]))
						}
						Ok(())
					}
				}),
			);

			item.node.local_signals.insert(
				"setCursor".to_string(),
				Box::new({
					move |data| {
						if let Some(handler) = handler.upgrade() {
							let flex = flexbuffers::Reader::get_root(data)?;
							let data: Option<PanelItemCursor> = if !flex.flexbuffer_type().is_null()
							{
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
							handler.lock().set_cursor(data)
						}
						Ok(())
					}
				}),
			);
			ui_init_fn(init_data, item)
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
		.unwrap();
}

#[tokio::test]
async fn fusion_environment_ui() -> anyhow::Result<()> {
	let (client, event_loop) = Client::connect_with_async_loop().await?;

	struct EnvironmentUI {
		path: String,
	}
	impl EnvironmentUI {
		pub fn new(path: String) -> Self {
			println!("Environment item with path {path} created");
			EnvironmentUI { path }
		}
	}
	impl Drop for EnvironmentUI {
		fn drop(&mut self) {
			println!("Environment item with path {} destroyed", self.path)
		}
	}

	let _item_ui = ItemUI::register(&client, |init_data, _item: &EnvironmentItem| {
		EnvironmentUI::new(init_data)
	})?;

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}

#[tokio::test]
async fn fusion_panel_ui() -> anyhow::Result<()> {
	use manifest_dir_macros::directory_relative_path;
	let (client, event_loop) = Client::connect_with_async_loop().await?;
	client.set_base_prefixes(&[directory_relative_path!("res")])?;

	struct PanelItemUI;
	impl PanelItemUI {
		fn new(init_data: PanelItemInitData) -> Self {
			println!("Panel item created with {:?}", init_data);
			PanelItemUI
		}
	}
	impl PanelItemHandler for PanelItemUI {
		fn resize(&self, size: Vector2<u32>) {
			println!("Got resize of {}, {}", size.x, size.y);
		}

		fn set_cursor(&self, info: Option<PanelItemCursor>) {
			println!("Set cursor with info {:?}", info);
		}
	}
	impl Drop for PanelItemUI {
		fn drop(&mut self) {
			println!("Panel item destroyed");
		}
	}

	let _item_ui =
		ItemUI::<PanelItem, PanelItemUI>::register(&client, |init_data, _item: &PanelItem| {
			PanelItemUI::new(init_data)
		})?;

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}
