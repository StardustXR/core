pub mod environment;
pub mod panel;

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
use std::{any::TypeId, sync::Arc};

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

pub trait ItemUIHandler<I: Item>: Send + Sync + 'static {
	fn item_created(&mut self, uid: &str, item: I, init_data: I::InitData);
	fn item_captured(&mut self, uid: &str, acceptor_uid: &str, item: I);
	fn item_released(&mut self, uid: &str, acceptor_uid: &str, item: I);
	fn item_destroyed(&mut self, uid: &str);
	fn acceptor_created(&mut self, uid: &str, acceptor: ItemAcceptor<I>);
	fn acceptor_destroyed(&mut self, uid: &str);
}

pub struct ItemUI<I: Item> {
	node: Node,
	items: Arc<RwLock<FxHashMap<String, I>>>,
	captured_items: Arc<RwLock<FxHashMap<String, I>>>,
	acceptors: Arc<RwLock<FxHashMap<String, ItemAcceptor<I>>>>,
}
impl<I: Item> ItemUI<I> {
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
			captured_items: Arc::new(RwLock::new(FxHashMap::default())),
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
		ui.captured_items
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
		let Some(item) = ui.captured_items.write().remove(item_uid) else { return Ok(()) };
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

	pub fn items(&self) -> RwLockReadGuard<FxHashMap<String, I>> {
		self.items.read()
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
			captured_items: self.captured_items.clone(),
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

pub trait ItemAcceptorHandler<I: Item>: Send + Sync + 'static {
	fn captured(&mut self, uid: &str, item: I, init_data: I::InitData);
	fn released(&mut self, uid: &str);
}

pub struct ItemAcceptor<I: Item> {
	pub spatial: Spatial,
	items: Arc<RwLock<FxHashMap<String, I>>>,
}
impl<I: Item> ItemAcceptor<I> {
	pub fn create<'a, Fi: Field>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
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
						Transform {
							position,
							rotation,
							scale: None,
						},
						field.node().get_path()?,
						I::TYPE_NAME,
					),
				)?,
			},
			items: Arc::new(RwLock::new(FxHashMap::default())),
		};

		item_acceptor.node().add_local_signal("capture", {
			let client = Arc::downgrade(&spatial_parent.node().client()?);
			let items = item_acceptor.items.clone();
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
			let items = item_acceptor.items.clone();
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
			items: Arc::new(RwLock::new(FxHashMap::default())),
		}
	}

	pub fn items(&self) -> RwLockReadGuard<FxHashMap<String, I>> {
		self.items.read()
	}

	pub fn capture(&self, item: &I) -> Result<(), NodeError> {
		self.node()
			.send_remote_signal("capture", &item.node().get_path()?)
	}

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
		acceptor.items.write().insert(uid.to_string(), item);
		handler.lock().captured(uid, item_aliased, init_data);
		Ok(())
	}
	fn handle_release_item<H: ItemAcceptorHandler<I>>(
		acceptor: Arc<ItemAcceptor<I>>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let uid: &str = deserialize(data)?;
		acceptor.items.write().remove(uid);
		handler.lock().released(uid);
		Ok(())
	}
}
impl<I: Item> NodeType for ItemAcceptor<I> {
	fn node(&self) -> &Node {
		&self.spatial.node
	}
	fn alias(&self) -> Self {
		ItemAcceptor {
			spatial: self.spatial.alias(),
			items: self.items.clone(),
		}
	}
}
impl<I: Item> HandledNodeType for ItemAcceptor<I> {}
