use super::scenegraph::Scenegraph;
use crate::client;
use crate::messenger::Messenger;

use super::spatial::Spatial;

use std::rc;

pub struct Client<'a> {
	pub messenger: rc::Rc<Messenger<'a>>,
	pub scenegraph: Scenegraph<'a>,
	root: Option<Spatial<'a>>,
	hmd: Option<Spatial<'a>>,
}

impl<'a> Client<'a> {
	pub fn connect() -> Option<Self> {
		let connection = client::connect()?;
		let mut client = Client {
			scenegraph: Scenegraph::new(),
			messenger: rc::Rc::new(Messenger::new(connection)),
			root: None,
			hmd: None,
		};

		client.root = Some(Spatial::from_path(&client, "/").ok()?);
		client.hmd = Some(Spatial::from_path(&client, "/hmd").ok()?);

		Some(client)
	}

	pub fn get_weak_messenger(&self) -> rc::Weak<Messenger<'a>> {
		rc::Rc::downgrade(&self.messenger)
	}

	pub fn get_root(&self) -> &Spatial<'a> {
		self.root.as_ref().unwrap()
	}
	pub fn get_hmd(&self) -> &Spatial<'a> {
		self.hmd.as_ref().unwrap()
	}
}

#[test]
fn connect() {
	Client::connect().expect("Couldn't connect");
}
