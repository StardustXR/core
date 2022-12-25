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
use serde_repr::{Deserialize_repr, Serialize_repr};
use stardust_xr::schemas::flex::deserialize;
use std::{ops::Deref, sync::Arc};
pub use xkbcommon::xkb;
use xkbcommon::xkb::{Keymap, KEYMAP_FORMAT_TEXT_V1};

/// Handler for the `panel` item.
pub trait PanelItemHandler: Send + Sync {
	/// The toplevel surface is being resized to `size` (in pixels).
	fn commit_toplevel(&mut self, state: Option<PanelItemToplevel>);

	/// The cursor is being changed.
	///
	/// The cursor's material will automatically update, you just need to hide/show the cursor and account for the new size/hotspot.
	fn set_cursor(&mut self, info: Option<PanelItemCursor>);
}

/// An updated cursor.
#[derive(Debug, Clone, Deserialize)]
pub struct PanelItemCursor {
	/// Size of the cursor in pixels.
	pub size: Vector2<u32>,
	/// Hotspot position in pixels. This is the point relative to the top left where the cursor matches the 2D pointer.
	pub hotspot: Vector2<i32>,
}
/// The state of the panel item's toplevel.
#[derive(Debug, Clone, Deserialize)]
pub struct PanelItemToplevel {
	/// Equivalent to the window title.
	pub title: String,
	/// Application identifier, see https://standards.freedesktop.org/desktop-entry-spec/
	pub app_id: String,
	/// Current size in pixels
	pub size: Vector2<u32>,
	/// Recommended maximum size in pixels
	pub max_size: Vector2<u32>,
	/// Recommended minimum size in pixels
	pub min_size: Vector2<u32>,
	/// Array of states
	pub states: Vec<PanelItemState>,
}
/// The states the toplevel can be in.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr)]
pub enum PanelItemState {
	/// The surface is maximized.
	///
	/// The window geometry specified in the configure event must be obeyed by the client.
	/// The client should draw without shadow or other decoration outside of the window geometry.
	Maximized = 1,
	/// The surface is fullscreen.
	///
	/// The window geometry specified in the configure event is a maximum; the client cannot resize beyond it.
	/// For a surface to cover the whole fullscreened area, the geometry dimensions must be obeyed by the client.
	Fullscreen = 2,
	/// The surface is being resized.
	///
	/// The window geometry specified in the configure event is a maximum; the client cannot resize beyond it.
	/// Clients that have aspect ratio or cell sizing configuration can use a smaller size, however.
	Resizing = 3,
	/// Client window decorations should be painted as if the window is active.
	///
	/// This does not mean that the window actually has keyboard or pointer focus.
	Activated = 4,
	/// The window is currently in a tiled layout and the left edge is considered to be adjacent to another part of the tiling grid.
	TiledLeft = 5,
	/// The window is currently in a tiled layout and the right edge is considered to be adjacent to another part of the tiling grid.
	TiledRight = 6,
	/// The window is currently in a tiled layout and the top edge is considered to be adjacent to another part of the tiling grid.
	TiledTop = 7,
	/// The window is currently in a tiled layout and the bottom edge is considered to be adjacent to another part of the tiling grid.
	TiledBottom = 8,
}
/// A capability the panel item UI has.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Deserialize_repr, Serialize_repr)]
pub enum PanelItemCapability {
	/// Wayland clients can tell this stardust client to open a context menu for window management options.
	WindowMenu = 1,
	/// Maximize is supported, no shadows and the panel item UI controls the size of the window entirely.
	Maximize = 2,
	/// Fullscreen is supported, no shadows or title bar and the panel item UI controls the size of the window entirely.
	Fullscreen = 3,
	/// Minimize is supported, this just makes the button send the panel item handler an event when it's clicked.
	Minimize = 4,
}
/// The init data for the panel item.
#[derive(Debug, Clone, Deserialize)]
pub struct PanelItemInitData {
	/// Size of the toplevel surface in pixels.
	pub toplevel: Option<PanelItemToplevel>,
	/// The cursor, if applicable.
	pub cursor: Option<PanelItemCursor>,
}

/// An item that represents a toplevel wayland surface (base window) and all its popups (context menus, modals, etc.).
#[derive(Debug)]
pub struct PanelItem {
	spatial: Spatial,
}
impl PanelItem {
	/// Apply the toplevel surface's visuals as a material to a model.
	///
	/// This material is unlit with the [Simula text shader](https://github.com/SimulaVR/Simula/blob/master/addons/godot-haskell-plugin/TextShader.tres) ported on the server.
	/// The material index is global across the whole model for now, just play around with it a bit.
	pub fn apply_toplevel_material(
		&self,
		model: &Model,
		material_index: u32,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"apply_toplevel_material",
			&(model.node().get_path()?, material_index),
		)
	}

	/// Apply the cursor's visuals as a material to a model.
	///
	/// This material is unlit with the [Simula text shader](https://github.com/SimulaVR/Simula/blob/master/addons/godot-haskell-plugin/TextShader.tres) ported on the server.
	/// The material index is global across the whole model for now, just play around with it a bit.
	pub fn apply_cursor_material(
		&self,
		_cursor: &PanelItemCursor,
		model: &Model,
		material_index: u32,
	) -> Result<(), NodeError> {
		self.node.send_remote_signal(
			"apply_cursor_material",
			&(model.node().get_path()?, material_index),
		)
	}

	/// Send an event to set the pointer's position (in pixels, relative to top-left of surface). This will activate the pointer.
	pub fn pointer_motion(&self, position: impl Into<Vector2<f32>>) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointer_motion", &position.into())
	}
	/// Send an event to set a pointer button's state if the pointer's active.
	///
	/// The `button` is from the `input_event_codes` crate (e.g. BTN_LEFT for left click) and the state is `false` for released, `true` for pressed.
	pub fn pointer_button(&self, button: u32, state: bool) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointer_button", &(button, state as u32))
	}
	/// Send an event to scroll the pointer if it's active.
	///
	/// Scroll distance is a value in pixels corresponding to the "distance" the surface should be scrolled.
	///
	/// Scroll steps is a value in columns/rows corresponding to the wheel clicks of a mouse or such.  This also supports fractions of a wheel click.
	pub fn pointer_scroll(
		&self,
		scroll_distance: Vector2<f32>,
		scroll_steps: Vector2<f32>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("pointer_scroll", &(scroll_distance, scroll_steps))
	}
	/// Deactivate the pointer, for example whenever nothing is pointing at the panel item's UI.
	pub fn pointer_deactivate(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("pointer_deactivate", &())
	}

	/// Activate the keyboard with a given `xkb` keymap.
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
	/// Set a key's state if the keyboard is active.
	///
	/// `key` is a raw keycode that corresponds to the given keymap.
	pub fn keyboard_key_state(&self, key: u32, state: bool) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("keyboard_key_state", &(key, state as u32))
	}
	/// Deactivate the keyboard.
	pub fn keyboard_deactivate(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("keyboard_deactivate", &())
	}

	/// Request a resize of the surface (in pixels).
	///
	/// The surface's actual size after being resized will be given if the panel item is wrapped as `PanelItemHandler::resize`.
	pub fn configure_toplevel(
		&self,
		size: Option<Vector2<u32>>,
		states: &[PanelItemState],
		bounds: Option<Vector2<u32>>,
	) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("configure_toplevel", &(size, states, bounds))
	}

	fn handle_commit_toplevel<H: PanelItemHandler>(
		_panel_item: Arc<PanelItem>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> anyhow::Result<()> {
		handler.lock().commit_toplevel(deserialize(data)?);
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

	/// Wrap the panel item and `PanelItemHandler` in a `HandlerWrapper` to receive resize and cursor events.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: PanelItemHandler>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new(self, handler);
		handler_wrapper
			.add_handled_signal("commit_toplevel", Self::handle_commit_toplevel)
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
			item.configure_toplevel(
				Some(Vector2::from([1000; 2])),
				&[PanelItemState::Activated],
				Some(Vector2::from([500; 2])),
			)
			.unwrap();
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
	}
	struct PanelItemUI;
	impl PanelItemUI {
		fn new(init_data: PanelItemInitData) -> Self {
			println!("Panel item created with {:?}", init_data);
			PanelItemUI
		}
	}
	impl PanelItemHandler for PanelItemUI {
		fn commit_toplevel(&mut self, toplevel_state: Option<PanelItemToplevel>) {
			dbg!(toplevel_state);
		}

		fn set_cursor(&mut self, cursor_info: Option<PanelItemCursor>) {
			dbg!(cursor_info);
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
		_ = tokio::time::sleep(core::time::Duration::from_secs(60)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	}
}
