pub mod resource;
#[macro_use]
pub mod node;

pub mod client;
pub mod data;
pub mod drawable;
pub mod field;
pub mod input;
pub mod input_action;
pub mod item;
pub mod scenegraph;
pub mod spatial;

use parking_lot::{Mutex, MutexGuard};
use std::sync::{Arc, Weak};

pub(crate) type WeakHandler<T> = Weak<Mutex<T>>;

pub struct HandlerWrapper<N: Sized, T: Send + Sync> {
	node: N,
	wrapped: Arc<Mutex<T>>,
}
impl<N: Sized, T: Send + Sync> HandlerWrapper<N, T> {
	pub(crate) fn new<F>(node: N, wrapper_handler_init: F) -> Self
	where
		F: FnOnce(WeakHandler<T>, &N) -> T,
	{
		Self {
			wrapped: Arc::new_cyclic(|weak| Mutex::new(wrapper_handler_init(weak.clone(), &node))),
			node,
		}
	}

	pub fn lock_wrapped(&self) -> MutexGuard<T> {
		self.wrapped.lock()
	}

	pub fn node(&self) -> &N {
		&self.node
	}

	pub(crate) fn weak_wrapped(&self) -> WeakHandler<T> {
		Arc::downgrade(&self.wrapped)
	}
}
