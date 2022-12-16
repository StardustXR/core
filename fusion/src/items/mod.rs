//! Self-contained nodes containing data and sometimes behavior.
//!
//! Examples of items
//! - Environment item (just holds a path to an equirectangular `.hdr` file, will be replaced with `File`).
//! - File item (Holds a file's data/path/url as well as its MIME type, planned).
//! - Panel item (Represents a toplevel Wayland surface aka window as well as its popups (context menus for example)).
//! - Lens item (Represents an OpenXR session, planned).
//!
//! Item acceptors are a way to temporarily take posession of (capture) an item.
//! They have an attached field to provide a logical counterpart for the visual element to the item UI.
//! Unlike 2D though, they often will want to create a UI of their own for the item, for example a panel shell is a client that forms a temporary UI around panel items using an item acceptor.
//! This allows client devs to create new 3D interfaces to 2D apps, augmenting their capabilities.
//! When an item is captured, the client with the `ItemUI` should hide its UI attached to the item to not visually disrupt the appearance, however UI disconnected from that item that influences it should work well, like a task bar for panel items.
//!
//! While items themselves have no UI of any kind naturally, the first client that registers as an `ItemUI<I: Item>` will get events when the item is created, captured, released, or destroyed.
//! This means it can create the UI that others will see, allowing devs to make new UIs that are easily hotswappable (kill the old client, summon a new one).
//! An item UI can also see all the acceptors of that item's type, and is responsible for capturing the items into acceptors whenever applicable (by distance after dropping the grabbable on the UI for example).
//!
//! The client that summoned the item (if applicable), item UI, and item acceptor can all call `release()` on the item to instantly release it from any acceptor that captured it.

mod environment;
pub use environment::*;

mod panel;
pub use panel::*;

use super::{
	client::Client,
	node::{Node, NodeError, NodeType},
};
use crate::{fields::Field, node::HandledNodeType, spatial::Spatial, HandlerWrapper};
use anyhow::{anyhow, Result};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use stardust_xr::{
	schemas::flex::{deserialize, serialize},
	values::Transform,
};
use std::{any::TypeId, ops::Deref, sync::Arc};

/// Base item trait, `release` and `uid` are the ones that client devs may want to use.
pub trait Item: NodeType + Send + Sync + 'static {
	type ItemType;
	type InitData: DeserializeOwned + Send;
	const TYPE_NAME: &'static str;

	fn uid(&self) -> Result<String, NodeError> {
		self.node().get_name()
	}
	fn release(&self) -> Result<(), NodeError> {
		self.node().send_remote_signal("release", &())
	}
	fn from_path(
		client: &Arc<Client>,
		parent_path: impl ToString,
		name: impl ToString,
		init_data: &Self::InitData,
	) -> Self;
}

/// Handler for the ItemUI item.
pub trait ItemUIHandler<I: Item>: Send + Sync + 'static {
	/// A new item of the `I` type has been created with the given init data and `uid`. `item` is an aliased node to the real item.
	fn item_created(&mut self, uid: &str, item: I, init_data: I::InitData);
	/// The item with `uid` has been captured by the item acceptor. `item` is an aliased node to the real item.
	fn item_captured(&mut self, uid: &str, acceptor_uid: &str, item: I);
	/// The item with `uid` has been released by the item acceptor. `item` is an aliased node to the real item.
	fn item_released(&mut self, uid: &str, acceptor_uid: &str, item: I);
	/// The item with `uid` has been destroyed.
	fn item_destroyed(&mut self, uid: &str);
	/// The item acceptor with `uid` has been created. `acceptor` is an aliased node to the acceptor.
	fn acceptor_created(&mut self, uid: &str, acceptor: ItemAcceptor<I>);
	/// The item acceptor with `uid` has been destroyed.
	fn acceptor_destroyed(&mut self, uid: &str);
}

/// Node to get all items and acceptors to make a UI around the items.
pub struct ItemUI<I: Item> {
	node: Node,
	items: Arc<RwLock<FxHashMap<String, I>>>,
	captured: Arc<RwLock<FxHashMap<String, I>>>,
	acceptors: Arc<RwLock<FxHashMap<String, ItemAcceptor<I>>>>,
}
impl<I: Item> ItemUI<I> {
	/// Attempt to register the ItemUI for this type of item. Will fail with `NodeError::OverrideSingleton` if it's already been registered.
	pub fn register(client: &Arc<Client>) -> Result<ItemUI<I>, NodeError> {
		if !client
			.registered_item_uis
			.lock()
			.contains(&TypeId::of::<I>())
		{
			Self::new_item_ui(client)
		} else {
			Err(NodeError::OverrideSingleton)
		}
	}
	fn new_item_ui(client: &Arc<Client>) -> Result<ItemUI<I>, NodeError> {
		let item_ui = ItemUI::<I> {
			node: Node::from_path(client, "/item", I::TYPE_NAME, true),
			items: Arc::new(RwLock::new(FxHashMap::default())),
			captured: Arc::new(RwLock::new(FxHashMap::default())),
			acceptors: Arc::new(RwLock::new(FxHashMap::default())),
		};

		item_ui.node.add_local_signal("create_acceptor", {
			let client = Arc::downgrade(client);
			let acceptors = item_ui.acceptors.clone();
			move |data| {
				let acceptor_uid: String = deserialize(data)?;

				let acceptor = ItemAcceptor::<I>::from_path(
					&client.upgrade().ok_or_else(|| anyhow!("Client dropped"))?,
					format!("/item/{}/acceptor", I::TYPE_NAME),
					&acceptor_uid,
				);
				acceptors.write().insert(acceptor_uid, acceptor);
				Ok(())
			}
		})?;
		item_ui.node.add_local_signal("destroy_acceptor", {
			let acceptors = item_ui.acceptors.clone();
			move |data| {
				let name: &str = deserialize(data)?;
				acceptors.write().remove(name);
				Ok(())
			}
		})?;

		client.registered_item_uis.lock().push(TypeId::of::<I>());

		item_ui
			.node
			.client()?
			.message_sender_handle
			.signal(
				"/item",
				"register_item_ui",
				&serialize([I::TYPE_NAME]).map_err(|_| NodeError::Serialization)?,
			)
			.map_err(|e| NodeError::MessengerError { e })?;
		Ok(item_ui)
	}

	/// Wrap this node and an `ItemUIHandler` in a `HandlerWrapper` to run code ASAP. Instead, you can also get the `items()` and `captured()` and `acceptors()` hashmaps.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: ItemUIHandler<I>>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new(self, handler);
		handler_wrapper.add_handled_signal("create_item", Self::handle_create_item)?;
		handler_wrapper.add_handled_signal("capture_item", Self::handle_capture_item)?;
		handler_wrapper.add_handled_signal("release_item", Self::handle_release_item)?;
		handler_wrapper.add_handled_signal("destroy_item", Self::handle_destroy_item)?;
		Ok(handler_wrapper)
	}

	fn handle_create_item<H: ItemUIHandler<I>>(
		ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let (uid, init_data): (&str, I::InitData) = deserialize(data)?;

		let item = I::from_path(
			&ui.client()?,
			format!("/item/{}/item", I::TYPE_NAME),
			&uid,
			&init_data,
		);
		let item_aliased = item.alias();
		ui.items.write().insert(uid.to_string(), item);
		handler.lock().item_created(uid, item_aliased, init_data);
		Ok(())
	}
	fn handle_capture_item<H: ItemUIHandler<I>>(
		ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let (item_uid, acceptor_uid): (&str, &str) = deserialize(data)?;
		let items = ui.items.read();
		let Some(item) = items.get(item_uid) else { return Ok(()) };
		ui.captured
			.write()
			.insert(item_uid.to_string(), item.alias());
		handler
			.lock()
			.item_captured(item_uid, acceptor_uid, item.alias());
		Ok(())
	}
	fn handle_release_item<H: ItemUIHandler<I>>(
		ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let (item_uid, acceptor_uid): (&str, &str) = deserialize(data)?;
		let Some(item) = ui.captured.write().remove(item_uid) else { return Ok(()) };
		handler.lock().item_released(item_uid, acceptor_uid, item);
		Ok(())
	}
	fn handle_destroy_item<H: ItemUIHandler<I>>(
		ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let uid: &str = deserialize(data)?;
		ui.items.write().remove(uid);
		handler.lock().item_destroyed(uid);
		Ok(())
	}

	fn handle_create_acceptor<H: ItemUIHandler<I>>(
		ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let uid: &str = deserialize(data)?;

		let acceptor: ItemAcceptor<I> = ItemAcceptor::from_path(
			&ui.client()?,
			format!("/item/{}/acceptor", I::TYPE_NAME),
			&uid,
		);
		let acceptor_aliased = acceptor.alias();
		ui.acceptors.write().insert(uid.to_string(), acceptor);
		handler.lock().acceptor_created(uid, acceptor_aliased);
		Ok(())
	}
	fn handle_destroy_acceptor<H: ItemUIHandler<I>>(
		ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let uid: &str = deserialize(data)?;
		ui.acceptors.write().remove(uid);
		handler.lock().acceptor_destroyed(uid);
		Ok(())
	}

	/// Get a read guard to all the items of the item ui's type that exist with the keys being their UIDs.
	pub fn items(&self) -> RwLockReadGuard<FxHashMap<String, I>> {
		self.items.read()
	}
	/// Get a read guard to all the items of the item ui's type that are captured in an acceptor with the keys being their UIDs.
	pub fn captured(&self) -> RwLockReadGuard<FxHashMap<String, I>> {
		self.captured.read()
	}
	/// Get a read guard to all the acceptors of the item ui's type that exist with the keys being their UIDs.
	pub fn acceptors(&self) -> RwLockReadGuard<FxHashMap<String, ItemAcceptor<I>>> {
		self.acceptors.read()
	}
}
impl<I: Item> NodeType for ItemUI<I> {
	fn node(&self) -> &Node {
		&self.node
	}

	fn alias(&self) -> Self {
		ItemUI {
			node: self.node.alias(),
			items: self.items.clone(),
			captured: self.captured.clone(),
			acceptors: self.acceptors.clone(),
		}
	}
}
impl<I: Item> HandledNodeType for ItemUI<I> {}
impl<I: Item> Drop for ItemUI<I> {
	fn drop(&mut self) {
		let type_id = TypeId::of::<I>();
		if let Ok(client) = self.node.client() {
			let mut registered_item_uis = client.registered_item_uis.lock();
			if let Ok(type_id_loc) = registered_item_uis.binary_search(&type_id) {
				registered_item_uis.remove(type_id_loc);
			}
		}
	}
}

/// Handler for the ItemAcceptor node.
pub trait ItemAcceptorHandler<I: Item>: Send + Sync + 'static {
	/// Item `item` with unique ID `uid` has been captured into this acceptor with `init_data`.
	fn captured(&mut self, uid: &str, item: I, init_data: I::InitData);
	/// Item with unique ID `uid` has been released from this acceptor.
	fn released(&mut self, uid: &str);
}

/// Node that can borrow items for a bit (capturing).
pub struct ItemAcceptor<I: Item> {
	spatial: Spatial,
	captured_items: Arc<RwLock<FxHashMap<String, I>>>,
}
impl<I: Item> ItemAcceptor<I> {
	/// Create a new item acceptor. Field can be dropped and the acceptor will still work.
	pub fn create<'a, Fi: Field>(
		spatial_parent: &'a Spatial,
		transform: Transform,
		field: &'a Fi,
	) -> Result<ItemAcceptor<I>, NodeError> {
		let id = nanoid::nanoid!();
		let item_acceptor = ItemAcceptor::<I> {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/item",
					"create_item_acceptor",
					&format!("/item/{}/acceptor", I::TYPE_NAME),
					true,
					&id,
					(
						&id,
						spatial_parent.node().get_path()?,
						transform,
						field.node().get_path()?,
						I::TYPE_NAME,
					),
				)?,
			},
			captured_items: Arc::new(RwLock::new(FxHashMap::default())),
		};

		item_acceptor.node().add_local_signal("capture", {
			let client = Arc::downgrade(&spatial_parent.node().client()?);
			let items = item_acceptor.captured_items.clone();
			move |data| {
				let (item_uid, init_data): (&str, I::InitData) = deserialize(data)?;

				let item = I::from_path(
					&client.upgrade().ok_or_else(|| anyhow!("Client dropped"))?,
					&format!("/item/{}/item", I::TYPE_NAME),
					item_uid,
					&init_data,
				);
				items.write().insert(item_uid.to_string(), item);
				Ok(())
			}
		})?;
		item_acceptor.node().add_local_signal("release", {
			let items = item_acceptor.captured_items.clone();
			move |data| {
				let name: &str = deserialize(data)?;
				items.write().remove(name);
				Ok(())
			}
		})?;

		Ok(item_acceptor)
	}

	fn from_path(
		client: &Arc<Client>,
		parent: impl ToString,
		name: impl ToString,
	) -> ItemAcceptor<I> {
		ItemAcceptor {
			spatial: Spatial {
				node: Node::from_path(client, parent, name, false),
			},
			captured_items: Arc::new(RwLock::new(FxHashMap::default())),
		}
	}

	/// Wrap this node and an `ItemAcceptorHandler` in a `HandlerWrapper` to run code ASAP. Instead, you can also get the `items()` hashmap.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: ItemAcceptorHandler<I>>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new(self, handler);
		handler_wrapper.add_handled_signal("capture", Self::handle_capture_item)?;
		handler_wrapper.add_handled_signal("release", Self::handle_release_item)?;
		Ok(handler_wrapper)
	}

	fn handle_capture_item<H: ItemAcceptorHandler<I>>(
		acceptor: Arc<ItemAcceptor<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let (uid, init_data): (&str, I::InitData) = deserialize(data)?;
		let item = I::from_path(
			&acceptor.client()?,
			&acceptor.node().get_path()?,
			uid,
			&init_data,
		);
		let item_aliased = item.alias();
		acceptor
			.captured_items
			.write()
			.insert(uid.to_string(), item);
		handler.lock().captured(uid, item_aliased, init_data);
		Ok(())
	}
	fn handle_release_item<H: ItemAcceptorHandler<I>>(
		acceptor: Arc<ItemAcceptor<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let uid: &str = deserialize(data)?;
		acceptor.captured_items.write().remove(uid);
		handler.lock().released(uid);
		Ok(())
	}

	/// Get all the captured items
	pub fn captured_items(&self) -> RwLockReadGuard<FxHashMap<String, I>> {
		self.captured_items.read()
	}
	/// Capture an item into the acceptor
	pub fn capture(&self, item: &I) -> Result<(), NodeError> {
		self.node()
			.send_remote_signal("capture", &item.node().get_path()?)
	}
}
impl<I: Item> NodeType for ItemAcceptor<I> {
	fn node(&self) -> &Node {
		&self.spatial.node
	}
	fn alias(&self) -> Self {
		ItemAcceptor {
			spatial: self.spatial.alias(),
			captured_items: self.captured_items.clone(),
		}
	}
}
impl<I: Item> HandledNodeType for ItemAcceptor<I> {}
impl<I: Item> Deref for ItemAcceptor<I> {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}
