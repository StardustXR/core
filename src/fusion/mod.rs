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

use anyhow::Result;
use parking_lot::{Mutex, MutexGuard};
use std::sync::{Arc, Weak};

use self::node::NodeType;

pub type WeakWrapped<T> = Weak<Mutex<T>>;

pub struct WeakNodeRef<N: NodeType + Sized>(pub(crate) Weak<N>);
impl<N: NodeType + Sized> WeakNodeRef<N> {
	pub fn empty() -> Self {
		WeakNodeRef(Weak::new())
	}
	pub fn with_node<F, O>(&self, f: F) -> Option<O>
	where
		F: FnOnce(&N) -> O,
	{
		self.0.upgrade().as_deref().map(f)
	}
}
impl<N: NodeType + Sized> Clone for WeakNodeRef<N> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

pub struct HandlerWrapper<N: NodeType, T: Send + Sync + 'static> {
	node: Arc<N>,
	wrapped: Arc<Mutex<T>>,
}
impl<N: NodeType, T: Send + Sync + 'static> HandlerWrapper<N, T> {
	pub fn new<F>(node: N, wrapper_handler_init: F) -> Self
	where
		F: FnOnce(WeakWrapped<T>, WeakNodeRef<N>, &N) -> T,
	{
		let node = Arc::new(node);
		Self {
			wrapped: Arc::new_cyclic(|weak| {
				Mutex::new(wrapper_handler_init(
					weak.clone(),
					WeakNodeRef(Arc::downgrade(&node)),
					&node,
				))
			}),
			node,
		}
	}

	pub fn lock_inner(&self) -> MutexGuard<T> {
		self.wrapped.lock()
	}

	pub fn node(&self) -> &N {
		&self.node
	}
	pub fn weak_node_ref(&self) -> WeakNodeRef<N> {
		WeakNodeRef(Arc::downgrade(&self.node))
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
		self.node.node().local_signals.insert(
			name.to_string(),
			Box::new(move |data| {
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
