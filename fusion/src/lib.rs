#![allow(dead_code)]

pub use stardust_xr as core;

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

use self::node::HandledNodeType;
use anyhow::{anyhow, Result};
use client::LifeCycleHandler;
use input::InputHandlerHandler;
use items::panel::PanelItemHandler;
use node::NodeError;
use parking_lot::{Mutex, MutexGuard};
use spatial::ZoneHandler;
use std::sync::Arc;

#[derive(Debug)]
pub struct HandlerWrapper<N: HandledNodeType, H: Send + Sync + 'static> {
	node: Arc<N>,
	wrapped: Arc<Mutex<H>>,
}
impl<N: HandledNodeType, H: Send + Sync + 'static> HandlerWrapper<N, H> {
	pub(crate) fn new(node: N, handler: H) -> Self {
		Self {
			wrapped: Arc::new(Mutex::new(handler)),
			node: Arc::new(node),
		}
	}

	pub fn node(&self) -> &Arc<N> {
		&self.node
	}
	pub fn lock_wrapped(&self) -> MutexGuard<H> {
		self.wrapped.lock()
	}
	pub fn wrapped(&self) -> &Arc<Mutex<H>> {
		&self.wrapped
	}

	pub(crate) fn add_handled_signal(
		&self,
		name: &str,
		parse: fn(Arc<N>, Arc<Mutex<H>>, &[u8]) -> Result<()>,
	) -> Result<(), NodeError> {
		let node = Arc::downgrade(&self.node);
		let handler = Arc::downgrade(&self.wrapped);
		self.node.node().add_local_signal(name, move |data| {
			let Some(node) = node.upgrade() else { return Err(anyhow!("Node broken")) };
			let Some(handler) = handler.upgrade() else { return Err(anyhow!("Handler broken")) };
			parse(node, handler, data)
		})
	}
	// #[allow(clippy::type_complexity)]
	pub(crate) fn add_handled_method(
		&self,
		name: &str,
		parse: fn(Arc<N>, Arc<Mutex<H>>, &[u8]) -> Result<Vec<u8>>,
	) -> Result<(), NodeError> {
		let node = Arc::downgrade(&self.node);
		let handler = Arc::downgrade(&self.wrapped);
		self.node.node().add_local_method(name, move |data| {
			let Some(node) = node.upgrade() else { return Err(anyhow!("Node broken")) };
			let Some(handler) = handler.upgrade() else { return Err(anyhow!("Handler broken")) };
			parse(node, handler, data)
		})
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
impl PanelItemHandler for DummyHandler {
	fn resize(&mut self, _size: mint::Vector2<u32>) {}
	fn set_cursor(&mut self, _info: Option<items::panel::PanelItemCursor>) {}
}
impl ZoneHandler for DummyHandler {
	fn enter(&mut self, _uid: &str, _spatial: spatial::Spatial) {}
	fn capture(&mut self, _uid: &str, _spatial: spatial::Spatial) {}
	fn release(&mut self, _uid: &str) {}
	fn leave(&mut self, _uid: &str) {}
}
