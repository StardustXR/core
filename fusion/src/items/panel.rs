use super::{HandledItem, Item};
use crate::{
	client::Client,
	drawable::Model,
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
	HandlerWrapper,
};
use mint::Vector2;
use parking_lot::Mutex;
use serde::Deserialize;
use stardust_xr::schemas::flex::deserialize;
use std::{
	ops::Deref,
	sync::{Arc, Weak},
};
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

	fn handle_resize<T: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<T>>,
		data: &[u8],
	) -> anyhow::Result<()> {
		handler.lock().resize(deserialize(data)?);
		Ok(())
	}

	fn handle_set_cursor<T: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<T>>,
		data: &[u8],
	) -> anyhow::Result<()> {
		handler.lock().set_cursor(deserialize(data)?);
		Ok(())
	}
}
impl NodeType for PanelItem {
	fn node(&self) -> &Node {
		&self.spatial.node
	}
}
impl Item for PanelItem {
	type ItemType = PanelItem;
	type InitData = PanelItemInitData;
	const TYPE_NAME: &'static str = "panel";
}
impl<T: PanelItemHandler + 'static> HandledItem<T> for PanelItem {
	fn from_path<F>(
		client: &Arc<Client>,
		parent_path: impl ToString,
		name: impl ToString,
		init_data: Self::InitData,
		mut ui_init_fn: F,
	) -> HandlerWrapper<Self, T>
	where
		F: FnMut(Self::InitData, Weak<Mutex<T>>, &Arc<Self>) -> T + Clone + Send + Sync + 'static,
	{
		let item = PanelItem {
			spatial: Spatial {
				node: Node::from_path(client, parent_path, name, false),
			},
		};

		let handler_wrapper = HandlerWrapper::new(item, |handler: Weak<Mutex<T>>, item| {
			ui_init_fn(init_data, handler, item)
		});
		handler_wrapper
			.add_handled_signal("resize", Self::handle_resize)
			.unwrap();
		handler_wrapper
			.add_handled_signal("set_cursor", Self::handle_set_cursor)
			.unwrap();

		handler_wrapper
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
	impl crate::items::ItemHandler<PanelItem> for PanelItemUI {
		fn captured(&mut self, item: &PanelItem, acceptor_uid: &str) {
			println!(
				"Acceptor {} captured panel item {}",
				acceptor_uid,
				item.node().get_name().unwrap()
			);
		}
		fn released(&mut self, item: &PanelItem, acceptor_uid: &str) {
			println!(
				"Acceptor {} released panel item {}",
				acceptor_uid,
				item.node().get_name().unwrap()
			);
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

	let _item_ui = crate::items::ItemUI::<PanelItem, PanelItemUI>::register(
		&client,
		|init_data, _weak_wrapped, _node_ref| PanelItemUI::new(init_data),
	)?;

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => Err(anyhow::anyhow!("Timed Out"))?,
		e = event_loop => e??,
	}
	Ok(())
}
