pub mod environment;
pub mod panel;

use super::{
	client::Client,
	node::{Node, NodeError, NodeType},
	HandlerWrapper, WeakNodeRef, WeakWrapped,
};
use parking_lot::{Mutex, MutexGuard};
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use stardust_xr::schemas::flex::deserialize;
use std::{
	any::TypeId,
	sync::{Arc, Weak},
};

pub trait Item: NodeType + Sized + Send + Sync {
	type ItemType;
	type InitData: DeserializeOwned + Send;
	const REGISTER_UI_FN: &'static str;
	const ROOT_PATH: &'static str;

	fn node(&self) -> &Node;
}

pub struct ItemUI<I: Item + 'static, T: Send + Sync + 'static> {
	node: Arc<Node>,
	items: Arc<Mutex<FxHashMap<String, HandlerWrapper<I, T>>>>,
}
pub trait ItemUIType<T: Send + Sync + 'static>: Sized {
	type Item: Item + 'static;

	fn register<F>(
		client: &Arc<Client>,
		item_ui_init: F,
	) -> Result<ItemUI<Self::Item, T>, NodeError>
	where
		F: FnMut(
				<<Self as ItemUIType<T>>::Item as Item>::InitData,
				WeakWrapped<T>,
				WeakNodeRef<Self::Item>,
				&Self::Item,
			) -> T
			+ Clone
			+ Send
			+ Sync
			+ 'static,
	{
		if !client
			.registered_item_uis
			.lock()
			.contains(&TypeId::of::<Self::Item>())
		{
			Self::new_item_ui(client, item_ui_init)
		} else {
			Err(NodeError::OverrideSingleton)
		}
	}

	fn new_item_ui<F>(
		client: &Arc<Client>,
		item_ui_init: F,
	) -> Result<ItemUI<Self::Item, T>, NodeError>
	where
		F: FnMut(
				<<Self as ItemUIType<T>>::Item as Item>::InitData,
				WeakWrapped<T>,
				WeakNodeRef<Self::Item>,
				&Self::Item,
			) -> T
			+ Clone
			+ Send
			+ Sync
			+ 'static,
	{
		let item_ui = ItemUI::<Self::Item, T> {
			node: Node::from_path(Arc::downgrade(client), Self::Item::ROOT_PATH.to_string())
				.unwrap(),
			items: Arc::new(Mutex::new(FxHashMap::default())),
		};

		item_ui.node.local_signals.lock().insert(
			"create".to_string(),
			Arc::new({
				let client = Arc::downgrade(client);
				let items = item_ui.items.clone();
				move |data| {
					let (item_uid, init_data): (
						String,
						<<Self as ItemUIType<T>>::Item as Item>::InitData,
					) = deserialize(data)?;

					let item = Self::from_path(
						client.clone(),
						&format!("{}/item/{}", Self::Item::ROOT_PATH, item_uid),
						init_data,
						item_ui_init.clone(),
					);
					items.lock().insert(item_uid, item);
					Ok(())
				}
			}),
		);
		item_ui.node.local_signals.lock().insert(
			"destroy".to_string(),
			Arc::new({
				let items = item_ui.items.clone();
				move |data| {
					let name = flexbuffers::Reader::get_root(data)?.get_str()?;
					items.lock().remove(name);
					Ok(())
				}
			}),
		);

		client
			.registered_item_uis
			.lock()
			.push(TypeId::of::<Self::Item>());

		item_ui
			.node
			.client
			.upgrade()
			.unwrap()
			.messenger
			.send_remote_signal("/item", Self::Item::REGISTER_UI_FN, &[]);
		Ok(item_ui)
	}

	fn from_path<F>(
		client: Weak<Client>,
		path: &str,
		init_data: <<Self as ItemUIType<T>>::Item as Item>::InitData,
		ui_init_fn: F,
	) -> HandlerWrapper<Self::Item, T>
	where
		F: FnMut(
				<<Self as ItemUIType<T>>::Item as Item>::InitData,
				WeakWrapped<T>,
				WeakNodeRef<Self::Item>,
				&Self::Item,
			) -> T
			+ Clone
			+ Send
			+ Sync
			+ 'static,
		T: Send + Sync + 'static;
}
impl<I: Item + 'static, T: Send + Sync> ItemUI<I, T> {
	pub fn items(&self) -> MutexGuard<FxHashMap<String, HandlerWrapper<I, T>>> {
		self.items.lock()
	}
}
impl<I: Item + 'static, T: Send + Sync> Drop for ItemUI<I, T> {
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
