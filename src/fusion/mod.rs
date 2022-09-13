pub mod resource;
#[macro_use]
pub mod node;

pub mod client;
pub mod data;
pub mod drawable;
pub mod field;
pub mod input;
pub mod item;
pub mod scenegraph;
pub mod spatial;

pub use async_trait::async_trait;

use parking_lot::Mutex;
use std::sync::{Arc, Weak};

#[derive(Debug)]
pub(crate) struct HandlerWrapper<H: ?Sized>(Arc<Mutex<Option<Weak<H>>>>);
impl<H: ?Sized> HandlerWrapper<H> {
	pub fn new() -> Self {
		HandlerWrapper(Arc::new(Mutex::new(None)))
	}
	pub fn set_handler(&self, handler: Weak<H>) {
		self.0.lock().replace(handler);
	}
	pub fn get_handler(&self) -> Option<Arc<H>> {
		self.0.lock().clone().and_then(|handler| handler.upgrade())
	}
}
impl<H: ?Sized> Clone for HandlerWrapper<H> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}
