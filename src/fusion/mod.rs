pub mod resource;
#[macro_use]
pub mod node;

pub mod client;
pub mod data;
pub mod drawable;
pub mod field;
pub mod scenegraph;
pub mod spatial;

pub use async_trait::async_trait;

use parking_lot::Mutex;
use std::sync::{Arc, Weak};

#[derive(Debug)]
struct HandlerWrapper<H: ?Sized>(Arc<Mutex<Option<Weak<H>>>>);
impl<H: ?Sized> HandlerWrapper<H> {
	pub fn new() -> Self {
		HandlerWrapper(Arc::new(Mutex::new(None)))
	}
	pub fn set_handler(&self, handler: Weak<H>) {
		self.0.lock().replace(handler);
	}
	pub fn handle<F, O>(&self, closure: F) -> Option<O>
	where
		F: FnOnce(Arc<H>) -> O,
	{
		self.0
			.lock()
			.clone()
			.and_then(|handler| handler.upgrade())
			.map(closure)
	}
}
impl<H: ?Sized> Clone for HandlerWrapper<H> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}
