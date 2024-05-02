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

pub mod camera;
pub mod panel;
use parking_lot::Mutex;

use super::{
	client::Client,
	node::{Node, NodeError, NodeType},
};
use crate::{
	fields::{Field, FieldAspect},
	node::OwnedAspect,
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
	HandlerWrapper,
};
use color_eyre::eyre::Result;
use serde::de::DeserializeOwned;
use stardust_xr::schemas::flex::{deserialize, serialize, Datamap};
use std::{any::TypeId, marker::PhantomData, os::fd::OwnedFd, sync::Arc};

/// Base item trait, `release` and `uid` are the ones that client devs may want to use.
pub trait ItemAspect: NodeType + Send + Sync + 'static {
	type InitData: DeserializeOwned + Send;
	const TYPE_NAME: &'static str;

	fn release(&self) -> Result<(), NodeError> {
		self.node().send_remote_signal("release", &())
	}
}

/// Handler for the ItemUI item.
#[allow(unused_variables)]
pub trait ItemUIHandler<I: ItemAspect>: Send + Sync + 'static {
	/// A new item of the `I` type has been created with the given init data and `uid`. `item` is an aliased node to the real item.
	fn item_created(&mut self, item_uid: String, item: I, init_data: I::InitData) {}
	/// The item with `uid` has been captured by the item acceptor. `item` is an aliased node to the real item.
	fn item_captured(&mut self, item_uid: String, acceptor_uid: String) {}
	/// The item with `uid` has been released by the item acceptor. `item` is an aliased node to the real item.
	fn item_released(&mut self, item_uid: String, acceptor_uid: String) {}
	/// The item with `uid` has been destroyed.
	fn item_destroyed(&mut self, item_uid: String) {}
	/// The item acceptor with `uid` has been created. `acceptor` is an aliased node to the acceptor.
	fn acceptor_created(&mut self, acceptor_uid: String, acceptor: ItemAcceptor<I>, field: Field) {}
	/// The item acceptor with `uid` has been destroyed.
	fn acceptor_destroyed(&mut self, acceptor_uid: String) {}
}

#[allow(unused_variables)]
pub trait InputItemHandler: Send + Sync {
	/// The input is(n't) able to be tracked at the moment
	fn track_status(&mut self, tracked: bool);
	/// The datamap (abstract data like pinch strength, grab strength, etc.) has been updated
	fn datamap_updated(&mut self, datamap: Datamap);
}

/// Node to get all items and acceptors to make a UI around the items.
pub struct ItemUI<I: ItemAspect> {
	ty: PhantomData<I>,
	node: Node,
}
impl<I: ItemAspect> ItemUI<I> {
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
			ty: PhantomData::default(),
			node: Node::from_parent_name(client, "/item", I::TYPE_NAME, true),
		};

		client.registered_item_uis.lock().insert(TypeId::of::<I>());

		item_ui
			.node
			.client()?
			.message_sender_handle
			.signal(
				"/item",
				"register_item_ui",
				&serialize([I::TYPE_NAME]).map_err(|_| NodeError::Serialization)?,
				Vec::new(),
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
		self.wrap_raw(Arc::new(Mutex::new(handler)))
	}
	/// Wrap this node and an `ItemUIHandler` in a `HandlerWrapper` to run code ASAP. Instead, you can also get the `items()` and `captured()` and `acceptors()` hashmaps.
	#[must_use = "Dropping this handler wrapper would immediately drop the node"]
	pub fn wrap_raw<H: ItemUIHandler<I>>(
		self,
		handler: Arc<Mutex<H>>,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new_raw(self, handler);
		handler_wrapper.add_handled_signal("create_item", Self::handle_create_item)?;
		handler_wrapper.add_handled_signal("capture_item", Self::handle_capture_item)?;
		handler_wrapper.add_handled_signal("release_item", Self::handle_release_item)?;
		handler_wrapper.add_handled_signal("destroy_item", Self::handle_destroy_item)?;
		handler_wrapper.add_handled_signal("create_acceptor", Self::handle_create_acceptor)?;
		handler_wrapper.add_handled_signal("destroy_acceptor", Self::handle_destroy_acceptor)?;
		Ok(handler_wrapper)
	}

	fn handle_create_item<H: ItemUIHandler<I>>(
		ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> Result<()> {
		let (uid, init_data): (String, I::InitData) = deserialize(data)?;

		let item = I::from_parent_name(
			&ui.client()?,
			&format!("/item/{}/item", I::TYPE_NAME),
			&uid,
			false,
		);
		handler.lock().item_created(uid, item, init_data);
		Ok(())
	}
	fn handle_capture_item<H: ItemUIHandler<I>>(
		_ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> Result<()> {
		let (item_uid, acceptor_uid): (String, String) = deserialize(data)?;
		handler.lock().item_captured(item_uid, acceptor_uid);
		Ok(())
	}
	fn handle_release_item<H: ItemUIHandler<I>>(
		_ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> Result<()> {
		let (item_uid, acceptor_uid): (String, String) = deserialize(data)?;
		handler.lock().item_released(item_uid, acceptor_uid);
		Ok(())
	}
	fn handle_destroy_item<H: ItemUIHandler<I>>(
		_ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> Result<()> {
		let uid: String = deserialize(data)?;
		handler.lock().item_destroyed(uid);
		Ok(())
	}

	fn handle_create_acceptor<H: ItemUIHandler<I>>(
		ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> Result<()> {
		let uid: String = deserialize(data)?;

		let client = ui.client()?;
		let acceptor: ItemAcceptor<I> = ItemAcceptor::from_parent_name(
			&client,
			&format!("/item/{}/acceptor", I::TYPE_NAME),
			&uid,
			false,
		);
		let field = Field::from_parent_name(&client, acceptor.node().get_path()?, "field", false);
		handler.lock().acceptor_created(uid, acceptor, field);
		Ok(())
	}
	fn handle_destroy_acceptor<H: ItemUIHandler<I>>(
		_ui: Arc<ItemUI<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> Result<()> {
		let uid: String = deserialize(data)?;
		handler.lock().acceptor_destroyed(uid);
		Ok(())
	}
}
impl<I: ItemAspect> NodeType for ItemUI<I> {
	fn node(&self) -> &Node {
		&self.node
	}

	fn alias(&self) -> Self {
		ItemUI {
			ty: Default::default(),
			node: self.node.alias(),
		}
	}

	fn from_path(client: &Arc<Client>, path: String, destroyable: bool) -> Self {
		ItemUI {
			ty: Default::default(),
			node: Node::from_path(client, path, destroyable),
		}
	}
}
impl<I: ItemAspect> Drop for ItemUI<I> {
	fn drop(&mut self) {
		let type_id = TypeId::of::<I>();
		if let Ok(client) = self.node.client() {
			let mut registered_item_uis = client.registered_item_uis.lock();
			registered_item_uis.remove(&type_id);
		}
	}
}

/// Handler for the ItemAcceptor node.
#[allow(unused_variables)]
pub trait ItemAcceptorHandler<I: ItemAspect>: Send + Sync + 'static {
	/// Item `item` with unique ID `uid` has been captured into this acceptor with `init_data`.
	fn captured(&mut self, uid: String, item: I, init_data: I::InitData) {}
	/// Item with unique ID `uid` has been released from this acceptor.
	fn released(&mut self, uid: String) {}
}

/// Node that can borrow items for a bit (capturing).
pub struct ItemAcceptor<I: ItemAspect> {
	node: Node,
	ty: PhantomData<I>,
}
impl<I: ItemAspect> ItemAcceptor<I> {
	/// Create a new item acceptor. Field can be dropped and the acceptor will still work.
	pub fn create<'a>(
		spatial_parent: &'a impl SpatialAspect,
		transform: Transform,
		field: &'a impl FieldAspect,
	) -> Result<ItemAcceptor<I>, NodeError> {
		let id = nanoid::nanoid!();
		let client = spatial_parent.client()?;
		client.message_sender_handle.signal(
			"/item",
			"create_item_acceptor",
			&serialize((
				&id,
				spatial_parent.node().get_path()?,
				transform,
				field.node().get_path()?,
				I::TYPE_NAME,
			))?,
			Vec::new(),
		)?;
		let item_acceptor = ItemAcceptor::<I> {
			node: Node::from_parent_name(
				&spatial_parent.client()?,
				&format!("/item/{}/acceptor", I::TYPE_NAME),
				&id,
				true,
			),

			ty: PhantomData::default(),
		};
		Ok(item_acceptor)
	}

	/// Wrap this node and an `ItemAcceptorHandler` in a `HandlerWrapper` to run code ASAP. Instead, you can also get the `items()` hashmap.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: ItemAcceptorHandler<I>>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		self.wrap_raw(Arc::new(Mutex::new(handler)))
	}
	/// Wrap this node and an `ItemAcceptorHandler` in a `HandlerWrapper` to run code ASAP. Instead, you can also get the `items()` hashmap.
	#[must_use = "Dropping this handler wrapper would immediately drop the node"]
	pub fn wrap_raw<H: ItemAcceptorHandler<I>>(
		self,
		handler: Arc<Mutex<H>>,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new_raw(self, handler);
		handler_wrapper.add_handled_signal("capture", Self::handle_capture_item)?;
		handler_wrapper.add_handled_signal("release", Self::handle_release_item)?;
		Ok(handler_wrapper)
	}

	fn handle_capture_item<H: ItemAcceptorHandler<I>>(
		acceptor: Arc<ItemAcceptor<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> Result<()> {
		let (uid, init_data): (String, I::InitData) = deserialize(data)?;
		let item = I::from_parent_name(
			&acceptor.client()?,
			&acceptor.node().get_path()?,
			&uid,
			false,
		);
		let item_aliased = item.alias();
		handler.lock().captured(uid, item_aliased, init_data);
		Ok(())
	}
	fn handle_release_item<H: ItemAcceptorHandler<I>>(
		_acceptor: Arc<ItemAcceptor<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> Result<()> {
		let uid: String = deserialize(data)?;
		handler.lock().released(uid);
		Ok(())
	}

	/// Capture an item into the acceptor
	pub fn capture(&self, item: &I) -> Result<(), NodeError> {
		self.node()
			.send_remote_signal("capture", &item.node().get_path()?)
	}
}
impl<I: ItemAspect> NodeType for ItemAcceptor<I> {
	fn node(&self) -> &Node {
		&self.node
	}
	fn alias(&self) -> Self {
		ItemAcceptor {
			node: self.node.alias(),
			ty: PhantomData::default(),
		}
	}

	fn from_path(client: &Arc<Client>, path: String, destroyable: bool) -> Self {
		ItemAcceptor {
			node: Node::from_path(client, path, destroyable),
			ty: PhantomData::default(),
		}
	}
}
impl<I: ItemAspect> OwnedAspect for ItemAcceptor<I> {}
impl<I: ItemAspect> SpatialRefAspect for ItemAcceptor<I> {}
impl<I: ItemAspect> SpatialAspect for ItemAcceptor<I> {}
