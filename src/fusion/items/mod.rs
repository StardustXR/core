pub mod environment;
pub mod panel;

use super::{
	client::Client,
	node::{Node, NodeError, NodeType},
	HandlerWrapper, WeakNodeRef, WeakWrapped,
};
use parking_lot::{Mutex, MutexGuard};
use rustc_hash::FxHashMap;
use std::{
	any::TypeId,
	sync::{Arc, Weak},
};

pub trait Item: NodeType + Sized + Send + Sync {
	type ItemType;
	type InitData: Send;
	const REGISTER_UI_FN: &'static str;
	const ROOT_PATH: &'static str;

	fn parse_init_data(
		flex_vec: flexbuffers::VectorReader<&[u8]>,
	) -> Result<Self::InitData, flexbuffers::ReaderError>;
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
			node: Node::from_path(Arc::downgrade(client), Self::Item::ROOT_PATH).unwrap(),
			items: Arc::new(Mutex::new(FxHashMap::default())),
		};

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
				let client = Arc::downgrade(client);
				let items = item_ui.items.clone();
				move |data| {
					let flex_vec = flexbuffers::Reader::get_root(data)?.get_vector()?;
					let name = flex_vec.index(0)?.get_str()?.to_string();
					let init_data = Self::Item::parse_init_data(flex_vec.index(1)?.get_vector()?)?;

					let item = Self::from_path(
						client.clone(),
						&format!("{}/item/{}", Self::Item::ROOT_PATH, name),
						init_data,
						item_ui_init.clone(),
					);
					items.lock().insert(name, item);
					Ok(())
				}
			}),
		);
		item_ui.node.local_signals.insert(
			"destroy".to_string(),
			Box::new({
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
