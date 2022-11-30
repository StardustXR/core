use super::Item;
use crate::{
	client::Client,
	drawable::Model,
	node::{HandledNodeType, Node, NodeError, NodeType},
	spatial::Spatial,
	HandlerWrapper,
};
use mint::Vector2;
use parking_lot::Mutex;
use serde::Deserialize;
use stardust_xr::schemas::flex::deserialize;
use std::{ops::Deref, sync::Arc};
use xkbcommon::xkb::{self, Keymap, KEYMAP_FORMAT_TEXT_V1};

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
		self.node.send_remote_signal(
			"apply_surface_material",
			&(model.spatial.node().get_path()?, material_index),
		)
	}

	pub fn apply_cursor_material(
		&self,
		_cursor: &PanelItemCursor,
		model: &Model,
		material_index: u32,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"apply_cursor_material",
			&(model.spatial.node().get_path()?, material_index),
		)
	}

	pub fn pointer_deactivate(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("pointer_deactivate", &())
	}

	pub fn pointer_motion(&self, position: impl Into<Vector2<f32>>) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointer_motion", &position.into())
	}

	pub fn pointer_button(&self, button: u32, state: u32) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointer_button", &(button, state))
	}

	pub fn pointer_scroll(
		&self,
		scroll_distance: Vector2<f32>,
		scroll_steps: Vector2<f32>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointer_scroll", &(scroll_distance, scroll_steps))
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
			.send_remote_signal("keyboard_activate_string", &keymap)
	}
	pub fn keyboard_deactivate(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("keyboard_deactivate", &())
	}
	pub fn keyboard_key_state(&self, key: u32, state: bool) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("keyboard_key_state", &(key, state as u32))
	}
	pub fn resize(&self, width: u32, height: u32) -> Result<(), NodeError> {
		self.node.send_remote_signal("resize", &(width, height))
	}

	fn handle_resize<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> anyhow::Result<()> {
		handler.lock().resize(deserialize(data)?);
		Ok(())
	}

	fn handle_set_cursor<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> anyhow::Result<()> {
		handler.lock().set_cursor(deserialize(data)?);
		Ok(())
	}

	pub fn wrap<H: PanelItemHandler>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new(self, handler);
		handler_wrapper
			.add_handled_signal("resize", Self::handle_resize)
			.unwrap();
		handler_wrapper
			.add_handled_signal("set_cursor", Self::handle_set_cursor)
			.unwrap();
		Ok(handler_wrapper)
	}
}
impl NodeType for PanelItem {
	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn alias(&self) -> Self {
		PanelItem {
			spatial: self.spatial.alias(),
		}
	}
}
impl HandledNodeType for PanelItem {}
impl Item for PanelItem {
	type ItemType = PanelItem;
	type InitData = PanelItemInitData;
	const TYPE_NAME: &'static str = "panel";

	fn from_path(
		client: &Arc<Client>,
		parent_path: impl ToString,
		name: impl ToString,
		_init_data: &PanelItemInitData,
	) -> Self {
		// let handler_wrapper = HandlerWrapper::new(item, |handler: Weak<Mutex<T>>, item| {
		// 	ui_init_fn(init_data, handler, item)
		// });
		// handler_wrapper
		// 	.add_handled_signal("resize", Self::handle_resize)
		// 	.unwrap();
		// handler_wrapper
		// 	.add_handled_signal("set_cursor", Self::handle_set_cursor)
		// 	.unwrap();
		PanelItem {
			spatial: Spatial {
				node: Node::from_path(client, parent_path, name, false),
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
async fn fusion_panel_ui() {
	use manifest_dir_macros::directory_relative_path;
	use rustc_hash::FxHashMap;
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();
	client.set_base_prefixes(&[directory_relative_path!("res")]);

	struct PanelItemManager(FxHashMap<String, HandlerWrapper<PanelItem, PanelItemUI>>);
	impl crate::items::ItemUIHandler<PanelItem> for PanelItemManager {
		fn item_created(&mut self, uid: &str, item: PanelItem, init_data: PanelItemInitData) {
			self.0.insert(
				uid.to_string(),
				item.wrap(PanelItemUI::new(init_data)).unwrap(),
			);
		}
		fn item_captured(&mut self, _uid: &str, acceptor_uid: &str, item: PanelItem) {
			println!(
				"Acceptor {} captured panel item {}",
				acceptor_uid,
				item.node().get_name().unwrap()
			);
		}
		fn item_released(&mut self, _uid: &str, acceptor_uid: &str, item: PanelItem) {
			println!(
				"Acceptor {} released panel item {}",
				acceptor_uid,
				item.node().get_name().unwrap()
			);
		}
		fn item_destroyed(&mut self, _uid: &str) {}
		fn acceptor_created(
			&mut self,
			_uid: &str,
			_acceptorr: crate::items::ItemAcceptor<PanelItem>,
		) {
		}
		fn acceptor_destroyed(&mut self, _uid: &str) {}
	}
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

	let _item_ui = crate::items::ItemUI::<PanelItem>::register(&client)
		.unwrap()
		.wrap(PanelItemManager(FxHashMap::default()))
		.unwrap();

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(1)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	}
}
