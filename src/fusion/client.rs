use super::scenegraph::Scenegraph;
use crate::client;
use crate::messenger::Messenger;

use super::spatial::Spatial;

use std::{
	cell,
	rc,
};

pub struct Client<'a> {
	pub messenger: rc::Rc<Messenger<'a>>,
	pub scenegraph: Scenegraph<'a>,
	root: Option<Spatial<'a>>,
}

impl<'a> Client<'a> {
	pub fn connect() -> Option<Self> {
		let connection = client::connect()?;
		let mut client = Client {
			scenegraph: Scenegraph::new(),
			messenger: rc::Rc::new(Messenger::new(connection)),
			root: None,
		};

		client.root = Some(Spatial::from_path(&client, "/").unwrap());

		Some(client)
	}

	pub fn get_weak_messenger(&self) -> rc::Weak<Messenger<'a>> {
		rc::Rc::downgrade(&self.messenger)
	}

	pub fn get_root(&'a self) -> &'a Spatial<'a> {
		self.root.as_ref().unwrap()
	}
}

