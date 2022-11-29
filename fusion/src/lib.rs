#![allow(dead_code)]

pub mod resource;
#[macro_use]
pub mod node;

pub mod client;
pub mod data;
pub mod drawable;
pub mod fields;
pub mod input;
pub mod items;
pub mod scenegraph;
pub mod spatial;
pub mod startup_settings;

use self::node::{Node, NodeType};
use anyhow::Result;
use client::LifeCycleHandler;
use input::InputHandlerHandler;
use items::{panel::PanelItemHandler, Item, ItemHandler};
use parking_lot::{Mutex, MutexGuard};
use spatial::ZoneHandler;
use std::sync::{Arc, Weak};

pub type WeakWrapped<T> = Weak<Mutex<T>>;

#[derive(Debug)]
pub struct HandlerWrapper<N: NodeType, T: Send + Sync + 'static> {
	node: Arc<N>,
	wrapped: Arc<Mutex<T>>,
}
impl<N: NodeType, T: Send + Sync + 'static> HandlerWrapper<N, T> {
	pub fn new<F>(node: N, wrapper_handler_init: F) -> Self
	where
		F: FnOnce(WeakWrapped<T>, &Arc<N>) -> T,
	{
		let node = Arc::new(node);
		Self {
			wrapped: Arc::new_cyclic(|weak| Mutex::new(wrapper_handler_init(weak.clone(), &node))),
			node,
		}
	}

	pub fn lock_inner(&self) -> MutexGuard<T> {
		self.wrapped.lock()
	}

	pub fn node(&self) -> &N {
		&self.node
	}

	pub fn weak_wrapped(&self) -> WeakWrapped<T> {
		Arc::downgrade(&self.wrapped)
	}

	pub(crate) fn add_handled_signal(
		&self,
		name: &str,
		parse: fn(Arc<Mutex<T>>, &[u8]) -> Result<()>,
	) {
		let handler = self.weak_wrapped();
		self.node.node().local_signals.lock().insert(
			name.to_string(),
			Arc::new(move |data| {
				if let Some(handler) = handler.upgrade() {
					parse(handler, data)?
				}
				Ok(())
			}),
		);
	}
	// #[allow(clippy::type_complexity)]
	// pub(crate) fn add_handled_method(
	// 	&self,
	// 	name: &str,
	// 	parse: fn(Arc<Mutex<T>>, &[u8]) -> Result<Vec<u8>>,
	// ) {
	// 	let handler = wrapper.weak_inner();
	// 	self.node.local_methods.insert(
	// 		name.to_string(),
	// 		Box::new(move |data| {
	// 			let handler = handler
	// 				.upgrade()
	// 				.ok_or_else(|| anyhow::anyhow!("No handler for this method"))?;
	// 			parse(handler, data)
	// 		}),
	// 	);
	// }
}

impl<N: NodeType, T: Send + Sync + 'static> NodeType for HandlerWrapper<N, T> {
	fn node(&self) -> &Node {
		self.node().node()
	}
}

impl<N: NodeType, T: Send + Sync + 'static> Clone for HandlerWrapper<N, T> {
	fn clone(&self) -> Self {
		Self {
			node: self.node.clone(),
			wrapped: self.wrapped.clone(),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct DummyHandler;
impl LifeCycleHandler for DummyHandler {
	fn logic_step(&mut self, _info: client::LogicStepInfo) {}
}
impl InputHandlerHandler for DummyHandler {
	fn input(&mut self, _input: stardust_xr::schemas::flat::InputData) -> bool {
		false
	}
}
impl<I: Item> ItemHandler<I> for DummyHandler {
	fn captured(&mut self, _item: &I, _acceptor_uid: &str) {}
	fn released(&mut self, _item: &I, _acceptor_uid: &str) {}
}
impl PanelItemHandler for DummyHandler {
	fn resize(&mut self, _size: mint::Vector2<u32>) {}
	fn set_cursor(&mut self, _info: Option<items::panel::PanelItemCursor>) {}
}
impl ZoneHandler for DummyHandler {
	fn enter(&mut self, _uid: &str, _spatial: &spatial::Spatial) {}
	fn capture(&mut self, _uid: &str, _spatial: &spatial::Spatial) {}
	fn release(&mut self, _uid: &str) {}
	fn leave(&mut self, _uid: &str) {}
}
