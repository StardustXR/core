use std::{ops::Deref, sync::Weak};

use mint::Vector2;
use serde::Deserialize;
use stardust_xr::schemas::flex::deserialize;
use xkbcommon::xkb::{self, Keymap, KEYMAP_FORMAT_TEXT_V1};

use crate::{
	client::Client,
	drawable::Model,
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
	HandlerWrapper, WeakNodeRef, WeakWrapped,
};

use super::{Item, ItemUI, ItemUIType};

pub trait PanelItemHandler: Send + Sync {
	fn resize(&mut self, size: Vector2<u32>);
	fn set_cursor(&mut self, info: Option<PanelItemCursor>);
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct PanelItemCursor {
	pub size: Vector2<u32>,
	pub hotspot: Vector2<i32>,
}
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct PanelItemInitData {
	pub size: Vector2<u32>,
	pub cursor: Option<PanelItemCursor>,
}

#[derive(Debug)]
pub struct PanelItem {
	pub spatial: Spatial,
}
impl PanelItem {
	pub fn apply_surface_material(
		&self,
		model: &Model,
		material_index: u32,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("applySurfaceMaterial", &(&model.spatial, material_index))
	}

	pub fn apply_cursor_material(
		&self,
		_cursor: &PanelItemCursor,
		model: &Model,
		material_index: u32,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("applyCursorMaterial", &(&model.spatial, material_index))
	}

	pub fn pointer_deactivate(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("pointerDeactivate", &())
	}

	pub fn pointer_motion(&self, position: impl Into<Vector2<f32>>) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointerMotion", &position.into())
	}

	pub fn pointer_button(&self, button: u32, state: u32) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointerButton", &(button, state))
	}

	pub fn pointer_scroll(
		&self,
		scroll_distance: Vector2<f32>,
		scroll_steps: Vector2<f32>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointerScroll", &(scroll_distance, scroll_steps))
	}

	pub fn keyboard_activate(&self, keymap: &str) -> Result<(), NodeError> {
		Keymap::new_from_string(
			&xkb::Context::new(0),
			keymap.to_string(),
			KEYMAP_FORMAT_TEXT_V1,
			0,
		)
		.ok_or(NodeError::InvalidPath)?;
		self.node
			.send_remote_signal("keyboardActivateString", &keymap)
	}
	pub fn keyboard_deactivate(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("keyboardDeactivate", &())
	}
	pub fn keyboard_key_state(&self, key: u32, state: bool) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("keyboardKeyState", &(key, state as u32))
	}
	pub fn resize(&self, width: u32, height: u32) -> Result<(), NodeError> {
		self.node.send_remote_signal("resize", &(width, height))
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
}
impl NodeType for PanelItem {
	fn node(&self) -> &Node {
		&self.spatial.node
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
		F: FnMut(PanelItemInitData, WeakWrapped<T>, WeakNodeRef<PanelItem>, &PanelItem) -> T
			+ Clone
			+ Send
			+ Sync
			+ 'static,
		T: Send + Sync + 'static,
	{
		let item = PanelItem {
			spatial: Spatial {
				node: Node::from_path(client, path.to_string()).unwrap(),
			},
		};

		HandlerWrapper::new(item, |handler: WeakWrapped<T>, weak_node_ref, item| {
			item.node.local_signals.lock().insert(
				"resize".to_string(),
				Box::new({
					let handler = handler.clone();
					move |data| {
						if let Some(handler) = handler.upgrade() {
							handler.lock().resize(deserialize(data)?)
						}
						Ok(())
					}
				}),
			);

			item.node.local_signals.lock().insert(
				"setCursor".to_string(),
				Box::new({
					let handler = handler.clone();
					move |data| {
						if let Some(handler) = handler.upgrade() {
							handler.lock().set_cursor(deserialize(data)?)
						}
						Ok(())
					}
				}),
			);
			ui_init_fn(init_data, handler, weak_node_ref, item)
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
async fn fusion_panel_ui() -> anyhow::Result<()> {
	use manifest_dir_macros::directory_relative_path;
	let (client, event_loop) = Client::connect_with_async_loop().await?;
	client.set_base_prefixes(&[directory_relative_path!("res")]);

	struct PanelItemUI;
	impl PanelItemUI {
		fn new(init_data: PanelItemInitData) -> Self {
			println!("Panel item created with {:?}", init_data);
			PanelItemUI
		}
	}
	impl PanelItemHandler for PanelItemUI {
		fn resize(&mut self, size: Vector2<u32>) {
			println!("Got resize of {}, {}", size.x, size.y);
		}

		fn set_cursor(&mut self, info: Option<PanelItemCursor>) {
			println!("Set cursor with info {:?}", info);
		}
	}
	impl Drop for PanelItemUI {
		fn drop(&mut self) {
			println!("Panel item destroyed");
		}
	}

	let _item_ui = ItemUI::<PanelItem, PanelItemUI>::register(
		&client,
		|init_data, _weak_wrapped, _weak_node_ref, _item: &PanelItem| PanelItemUI::new(init_data),
	)?;

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out")),
		_ = event_loop => Ok(()),
	}
}
