//! Self-contained nodes containing data and sometimes behavior.
//!
//! Examples of items
//! - Panel item (Represents a toplevel Wayland surface aka window as well as its popups (context menus for example)).
//! - Lens item (Represents an OpenXR session, planned).
//! - File item (Holds a file's data/path/url as well as its MIME type, planned).
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

use crate::{
	client::Client,
	drawable::ModelPartAspect,
	fields::{FieldAspect, UnknownField},
	node::{NodeAspect, NodeError, NodeResult, NodeType},
	spatial::{SpatialAspect, Transform},
	HandlerWrapper,
};
use nanoid::nanoid;
use parking_lot::Mutex;
use serde::de::DeserializeOwned;
use std::{any::TypeId, marker::PhantomData, sync::Arc};

pub use acceptor::*;
pub use internal::{ItemAspect, ItemUiAspect};
pub use ui_handler::*;

mod internal {
	use super::*;
	pub(super) fn _register_item_ui(
		client: &Arc<Client>,
		item_type: &str,
	) -> NodeResult<UnknownItemUi> {
		register_item_ui(client, item_type)
	}
	stardust_xr_fusion_codegen::codegen_item_client_protocol!();
	impl ItemAspect for CameraItem {}
	impl ItemAspect for PanelItem {}
}
pub mod panel {
	pub use super::internal::{
		ChildInfo, Geometry, PanelItem, PanelItemAspect, PanelItemHandler, PanelItemInitData,
		SurfaceId, ToplevelInfo,
	};
}
pub mod camera {
	pub use super::internal::{CameraItem, CameraItemAspect};
}

/// Base item trait, `release` and `uid` are the ones that client devs may want to use.
pub trait Item: ItemAspect + Send + Sync + 'static {
	type InitData: DeserializeOwned + Send;
	const TYPE_NAME: &'static str;
	fn from_unknown(data: internal::UnknownItem) -> Self;
	fn get_item_data(data: internal::InitialItemData) -> Option<Self::InitData>;
}

mod ui_handler {
	use super::acceptor::ItemAcceptor;
	use super::*;

	/// Handler for the ItemUI item.
	pub trait ItemUIHandler<I: Item>: Send + Sync + 'static {
		/// A new item of the `I` type has been created with the given init data and `uid`. `item` is an aliased node to the real item.
		fn create_item(&mut self, item_uid: String, item: I, init_data: I::InitData);
		/// The item with `uid` has been captured by the item acceptor. `item` is an aliased node to the real item.
		fn capture_item(&mut self, item_uid: String, acceptor_uid: String);
		/// The item with `uid` has been released by the item acceptor. `item` is an aliased node to the real item.
		fn release_item(&mut self, item_uid: String, acceptor_uid: String);
		/// The item with `uid` has been destroyed.
		fn destroy_item(&mut self, item_uid: String);
		/// The item acceptor with `uid` has been created. `acceptor` is an aliased node to the acceptor.
		fn create_acceptor(
			&mut self,
			acceptor_uid: String,
			acceptor: ItemAcceptor<I>,
			field: UnknownField,
		);
		/// The item acceptor with `uid` has been destroyed.
		fn destroy_acceptor(&mut self, acceptor_uid: String);
	}
	/// Node to get all items and acceptors to make a UI around the items.
	pub struct ItemUI<I: Item> {
		pub(crate) ty: PhantomData<I>,
		pub(crate) base: internal::UnknownItemUi,
	}
	impl<I: Item> ItemUI<I> {
		/// Attempt to register the ItemUI for this type of item. Will fail with `NodeError::OverrideSingleton` if it's already been registered.
		pub fn register(client: &Arc<Client>) -> NodeResult<ItemUI<I>> {
			if !client
				.registered_item_uis
				.lock()
				.contains(&TypeId::of::<I>())
			{
				let unknown = internal::_register_item_ui(client, I::TYPE_NAME)?;
				Ok(ItemUI {
					ty: PhantomData,
					base: unknown,
				})
			} else {
				Err(NodeError::OverrideSingleton)
			}
		}
	}
	impl<I: Item> NodeType for ItemUI<I> {
		fn node(&self) -> &crate::node::Node {
			&self.base.node()
		}

		fn from_path(client: &Arc<Client>, path: String, destroyable: bool) -> Self {
			ItemUI {
				ty: PhantomData,
				base: internal::UnknownItemUi::from_path(client, path, destroyable),
			}
		}

		fn alias(&self) -> Self
		where
			Self: Sized,
		{
			ItemUI {
				ty: PhantomData,
				base: self.base.alias(),
			}
		}
	}
	impl<I: Item> internal::ItemUiAspect<I> for ItemAcceptor<I> {}
	impl<I: Item, H: ItemUIHandler<I>> internal::ItemUiHandler<I> for H {
		fn create_item(
			&mut self,
			uid: String,
			item: internal::UnknownItem,
			initial_data: internal::InitialItemData,
		) {
			#[allow(irrefutable_let_patterns)]
			let Some(data) = I::get_item_data(initial_data) else {
				return;
			};
			self.create_item(uid, I::from_unknown(item), data)
		}
		fn capture_item(&mut self, item_uid: String, acceptor_uid: String) {
			self.capture_item(item_uid, acceptor_uid)
		}
		fn release_item(&mut self, item_uid: String, acceptor_uid: String) {
			self.release_item(item_uid, acceptor_uid)
		}
		fn destroy_item(&mut self, uid: String) {
			self.destroy_item(uid)
		}

		fn create_acceptor(
			&mut self,
			uid: String,
			acceptor: internal::UnknownItemAcceptor,
			acceptor_field: UnknownField,
		) {
			self.create_acceptor(
				uid,
				super::acceptor::ItemAcceptor {
					ty: PhantomData,
					base: acceptor,
				},
				acceptor_field,
			)
		}
		fn destroy_acceptor(&mut self, uid: String) {
			self.destroy_item(uid)
		}
	}
}

mod acceptor {
	use super::*;
	/// Handler for the ItemAcceptor node.
	pub trait ItemAcceptorHandler<I: Item>: Send + Sync + 'static {
		/// Item `item` with unique ID `uid` has been captured into this acceptor with `init_data`.
		fn captured(&mut self, uid: String, item: I, init_data: I::InitData);
		/// Item with unique ID `uid` has been released from this acceptor.
		fn released(&mut self, uid: String);
	}

	/// Node that can borrow items for a bit (capturing).
	pub struct ItemAcceptor<I: Item> {
		pub(crate) ty: PhantomData<I>,
		pub(crate) base: internal::UnknownItemAcceptor,
	}
	impl<I: Item> NodeType for ItemAcceptor<I> {
		fn node(&self) -> &crate::node::Node {
			&self.base.node()
		}

		fn from_path(client: &Arc<Client>, path: String, destroyable: bool) -> Self {
			ItemAcceptor {
				ty: PhantomData,
				base: internal::UnknownItemAcceptor::from_path(client, path, destroyable),
			}
		}

		fn alias(&self) -> Self
		where
			Self: Sized,
		{
			ItemAcceptor {
				ty: PhantomData,
				base: self.base.alias(),
			}
		}
	}
	impl<I: Item> NodeAspect for ItemAcceptor<I> {}
	impl<I: Item> internal::ItemAcceptorAspect<I> for ItemAcceptor<I> {}
	impl<I: Item> ItemAcceptor<I> {
		pub fn create(
			spatial_parent: &impl SpatialAspect,
			transform: Transform,
			field: &impl FieldAspect,
		) -> NodeResult<Self> {
			let client = spatial_parent.client()?;
			let name = nanoid!();
			internal::create_item_acceptor(
				&client,
				&name,
				spatial_parent,
				transform,
				I::TYPE_NAME,
				field,
			)?;
			Ok(ItemAcceptor::<I>::from_path(
				&client,
				format!("/item/{}/acceptor/{name}", I::TYPE_NAME),
				true,
			))
		}
		pub fn wrap<H: ItemAcceptorHandler<I> + internal::ItemAcceptorHandler<I>>(
			self,
			handler: H,
		) -> NodeResult<HandlerWrapper<Self, H>> {
			<Self as internal::ItemAcceptorAspect<I>>::wrap(self, handler)
		}
		pub fn wrap_raw<H: ItemAcceptorHandler<I> + internal::ItemAcceptorHandler<I>>(
			self,
			handler: Arc<Mutex<H>>,
		) -> NodeResult<HandlerWrapper<Self, H>> {
			<Self as internal::ItemAcceptorAspect<I>>::wrap_raw(self, handler)
		}
	}
	impl<I: Item, H: ItemAcceptorHandler<I>> internal::ItemAcceptorHandler<I> for H {
		fn capture_item(
			&mut self,
			item_uid: String,
			item: internal::UnknownItem,
			initial_data: internal::InitialItemData,
		) {
			#[allow(irrefutable_let_patterns)]
			let Some(data) = I::get_item_data(initial_data) else {
				return;
			};
			self.captured(item_uid, I::from_unknown(item), data);
		}
		fn release_item(&mut self, item_uid: String) {
			self.released(item_uid)
		}
	}
}

#[tokio::test]
async fn fusion_panel_ui() {
	use crate::items::*;
	color_eyre::install().unwrap();
	use manifest_dir_macros::directory_relative_path;
	use rustc_hash::FxHashMap;
	let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();
	client.set_base_prefixes(&[directory_relative_path!("res")]);

	struct PanelItemManager(FxHashMap<String, HandlerWrapper<PanelItem, PanelItemUI>>);
	impl ItemUIHandler<PanelItem> for PanelItemManager {
		fn create_item(
			&mut self,
			item_uid: String,
			item: PanelItem,
			init_data: <PanelItem as Item>::InitData,
		) {
			item.set_toplevel_focused_visuals(true).unwrap();
			item.auto_size_toplevel().unwrap();
			self.0.insert(
				item_uid.to_string(),
				item.wrap(PanelItemUI::new(init_data)).unwrap(),
			);
		}
		fn capture_item(&mut self, item_uid: String, acceptor_uid: String) {
			println!("Acceptor {acceptor_uid} captured panel item {item_uid}");
		}
		fn destroy_item(&mut self, item_uid: String) {}
		fn release_item(&mut self, item_uid: String, acceptor_uid: String) {
			println!("Acceptor {acceptor_uid} released panel item {item_uid}");
		}
		fn create_acceptor(
			&mut self,
			acceptor_uid: String,
			acceptor: ItemAcceptor<PanelItem>,
			field: UnknownField,
		) {
		}
		fn destroy_acceptor(&mut self, acceptor_uid: String) {}
	}
	struct PanelItemUI;
	impl PanelItemUI {
		fn new(init_data: PanelItemInitData) -> Self {
			println!("Panel item created with {:?}", init_data);
			PanelItemUI
		}
	}

	impl PanelItemHandler for PanelItemUI {
		fn set_cursor(&mut self, cursor_info: Geometry) {
			dbg!(cursor_info);
		}

		fn toplevel_size_changed(&mut self, size: mint::Vector2<u32>) {
			dbg!(size);
		}

		fn create_child(&mut self, uid: String, info: ChildInfo) {
			dbg!(uid);
			dbg!(info);
		}
		fn reposition_child(&mut self, uid: String, geometry: Geometry) {
			dbg!(uid);
			dbg!(geometry);
		}
		fn drop_child(&mut self, uid: String) {
			dbg!(uid);
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
