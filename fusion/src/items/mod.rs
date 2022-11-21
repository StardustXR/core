pub mod environment;
pub mod panel;

use crate::{fields::Field, spatial::Spatial, DummyHandler};

use super::{
	client::Client,
	node::{Node, NodeError, NodeType},
	HandlerWrapper, WeakNodeRef, WeakWrapped,
};
use parking_lot::{Mutex, MutexGuard};
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use stardust_xr::{
	schemas::flex::{deserialize, serialize},
	values::Transform,
};
use std::{
	any::TypeId,
	sync::{Arc, Weak},
};

pub trait Item: NodeType + Send + Sync + 'static {
	type ItemType;
	type InitData: DeserializeOwned + Send;
	const TYPE_NAME: &'static str;

	fn uid(&self) -> &str {
		self.node().get_name()
	}

	fn release(&self) -> Result<(), NodeError> {
		self.node().send_remote_signal("release", &())
	}
}
pub trait HandledItem<H: Send + Sync + 'static>: Item {
	fn from_path<F>(
		client: Weak<Client>,
		path: &str,
		init_data: Self::InitData,
		ui_init_fn: F,
	) -> HandlerWrapper<Self, H>
	where
		F: FnMut(Self::InitData, WeakWrapped<H>, WeakNodeRef<Self>, &Self) -> H
			+ Clone
			+ Send
			+ Sync
			+ 'static;
}

pub trait ItemHandler<I: Item>: Send + Sync + 'static {
	fn captured(&mut self, item: &I, acceptor_uid: &str);
	fn released(&mut self, item: &I, acceptor_uid: &str);
}

pub struct ItemUI<I: HandledItem<H> + HandledItem<DummyHandler>, H: ItemHandler<I>> {
	node: Arc<Node>,
	items: Arc<Mutex<FxHashMap<String, HandlerWrapper<I, H>>>>,
	captured_items: Arc<Mutex<FxHashMap<String, HandlerWrapper<I, H>>>>,
	acceptors: Arc<Mutex<FxHashMap<String, ItemAcceptor<I, DummyHandler>>>>,
}
impl<I: HandledItem<H> + HandledItem<DummyHandler>, H: ItemHandler<I>> ItemUI<I, H> {
	pub fn register<F>(client: &Arc<Client>, item_ui_init: F) -> Result<ItemUI<I, H>, NodeError>
	where
		F: FnMut(I::InitData, WeakWrapped<H>, WeakNodeRef<I>, &I) -> H
			+ Clone
			+ Send
			+ Sync
			+ 'static,
	{
		if !client
			.registered_item_uis
			.lock()
			.contains(&TypeId::of::<I>())
		{
			Self::new_item_ui(client, item_ui_init)
		} else {
			Err(NodeError::OverrideSingleton)
		}
	}

	fn new_item_ui<F>(client: &Arc<Client>, item_ui_init: F) -> Result<ItemUI<I, H>, NodeError>
	where
		F: FnMut(I::InitData, WeakWrapped<H>, WeakNodeRef<I>, &I) -> H
			+ Clone
			+ Send
			+ Sync
			+ 'static,
	{
		let item_ui = ItemUI::<I, H> {
			node: Node::from_path(
				Arc::downgrade(client),
				format!("/item/{}", I::TYPE_NAME),
				true,
			)
			.unwrap(),
			items: Arc::new(Mutex::new(FxHashMap::default())),
			captured_items: Arc::new(Mutex::new(FxHashMap::default())),
			acceptors: Arc::new(Mutex::new(FxHashMap::default())),
		};

		item_ui.node.local_signals.lock().insert(
			"create_item".to_string(),
			Arc::new({
				let client = Arc::downgrade(client);
				let items = item_ui.items.clone();
				move |data| {
					let (item_uid, init_data): (String, I::InitData) = deserialize(data)?;

					let item = I::from_path(
						client.clone(),
						&format!("/item/{}/item/{}", I::TYPE_NAME, item_uid),
						init_data,
						item_ui_init.clone(),
					);
					items.lock().insert(item_uid, item);
					Ok(())
				}
			}),
		);
		item_ui.node.local_signals.lock().insert(
			"capture_item".to_string(),
			Arc::new({
				let items = item_ui.items.clone();
				let captured_items = item_ui.captured_items.clone();
				move |data| {
					let (item_uid, acceptor_uid): (&str, &str) = deserialize(data)?;
					let items = items.lock();
					let Some(item) = items.get(item_uid) else { return Ok(()) };
					item.wrapped.lock().captured(&item.node, acceptor_uid);
					captured_items
						.lock()
						.insert(item_uid.to_string(), item.clone());
					Ok(())
				}
			}),
		);
		item_ui.node.local_signals.lock().insert(
			"release_item".to_string(),
			Arc::new({
				let captured_items = item_ui.captured_items.clone();
				move |data| {
					let (item_uid, acceptor_uid): (&str, &str) = deserialize(data)?;
					let Some(item) = captured_items.lock().remove(item_uid) else { return Ok(()) };
					item.wrapped.lock().captured(&item.node, acceptor_uid);
					Ok(())
				}
			}),
		);
		item_ui.node.local_signals.lock().insert(
			"destroy_item".to_string(),
			Arc::new({
				let items = item_ui.items.clone();
				move |data| {
					let name: &str = deserialize(data)?;
					items.lock().remove(name);
					Ok(())
				}
			}),
		);

		item_ui.node.local_signals.lock().insert(
			"create_acceptor".to_string(),
			Arc::new({
				let client = Arc::downgrade(client);
				let acceptors = item_ui.acceptors.clone();
				move |data| {
					let acceptor_uid: String = deserialize(data)?;

					let acceptor = ItemAcceptor::<I, DummyHandler>::from_path(
						client.clone(),
						format!("/item/{}/acceptor/{}", I::TYPE_NAME, acceptor_uid),
					)?;
					acceptors.lock().insert(acceptor_uid, acceptor);
					Ok(())
				}
			}),
		);
		item_ui.node.local_signals.lock().insert(
			"destroy_acceptor".to_string(),
			Arc::new({
				let acceptors = item_ui.acceptors.clone();
				move |data| {
					let name: &str = deserialize(data)?;
					acceptors.lock().remove(name);
					Ok(())
				}
			}),
		);

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

	pub fn items(&self) -> MutexGuard<FxHashMap<String, HandlerWrapper<I, H>>> {
		self.items.lock()
	}
}
impl<I: HandledItem<H> + HandledItem<DummyHandler>, H: ItemHandler<I>> Drop for ItemUI<I, H> {
	fn drop(&mut self) {
		let type_id = TypeId::of::<I>();
		if let Some(client) = self.node.client.upgrade() {
			let mut registered_item_uis = client.registered_item_uis.lock();
			if let Ok(type_id_loc) = registered_item_uis.binary_search(&type_id) {
				registered_item_uis.remove(type_id_loc);
			}
		}
	}
}

pub struct ItemAcceptor<I: HandledItem<H> + HandledItem<DummyHandler>, H: Send + Sync + 'static> {
	pub spatial: Spatial,
	items: Arc<Mutex<FxHashMap<String, HandlerWrapper<I, H>>>>,
}
impl<I: HandledItem<H> + HandledItem<DummyHandler>, H: Send + Sync> ItemAcceptor<I, H> {
	pub fn create<'a, F, Fi: Field>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		field: &'a Fi,
		item_acceptor_init: F,
	) -> Result<ItemAcceptor<I, H>, NodeError>
	where
		F: FnMut(I::InitData, WeakWrapped<H>, WeakNodeRef<I>, &I) -> H
			+ Clone
			+ Send
			+ Sync
			+ 'static,
	{
		let id = nanoid::nanoid!();
		let item_acceptor = ItemAcceptor::<I, H> {
			spatial: Spatial {
				node: Node::new(
					spatial_parent.node().client.clone(),
					"/item",
					"create_item_acceptor",
					&format!("/item/{}/acceptor", I::TYPE_NAME),
					true,
					&id,
					(
						&id,
						spatial_parent,
						Transform {
							position,
							rotation,
							scale: None,
						},
						&field.node(),
						I::TYPE_NAME,
					),
				)?,
			},
			items: Arc::new(Mutex::new(FxHashMap::default())),
		};

		item_acceptor.node().local_signals.lock().insert(
			"capture".to_string(),
			Arc::new({
				let client = spatial_parent.node().client.clone();
				let items = item_acceptor.items.clone();
				move |data| {
					let (item_acceptord, init_data): (String, I::InitData) = deserialize(data)?;

					let item = I::from_path(
						client.clone(),
						&format!("/item/{}/item/{}", I::TYPE_NAME, item_acceptord),
						init_data,
						item_acceptor_init.clone(),
					);
					items.lock().insert(item_acceptord, item);
					Ok(())
				}
			}),
		);
		item_acceptor.node().local_signals.lock().insert(
			"release".to_string(),
			Arc::new({
				let items = item_acceptor.items.clone();
				move |data| {
					let name: &str = deserialize(data)?;
					items.lock().remove(name);
					Ok(())
				}
			}),
		);

		Ok(item_acceptor)
	}

	fn from_path(
		client: Weak<Client>,
		path: String,
	) -> Result<ItemAcceptor<I, DummyHandler>, NodeError> {
		Ok(ItemAcceptor {
			spatial: Spatial {
				node: Node::from_path(client, path, false)?,
			},
			items: Arc::new(Mutex::new(FxHashMap::default())),
		})
	}

	pub fn items(&self) -> MutexGuard<FxHashMap<String, HandlerWrapper<I, H>>> {
		self.items.lock()
	}

	pub fn capture(&self, item: &I) -> Result<(), NodeError> {
		self.node()
			.send_remote_signal("capture", &item.node().get_path())
	}
}
impl<I: HandledItem<H> + HandledItem<DummyHandler>, H: Send + Sync> NodeType
	for ItemAcceptor<I, H>
{
	fn node(&self) -> &Node {
		&self.spatial.node
	}
}
