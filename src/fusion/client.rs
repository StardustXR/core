use super::{scenegraph::Scenegraph, spatial::Spatial};
use crate::{client, messenger::Messenger};
use std::{
	os::unix::net::UnixStream,
	rc::{Rc, Weak},
};

pub struct Client<'a> {
	pub messenger: Rc<Messenger<'a>>,
	pub scenegraph: Scenegraph<'a>,
	root: Option<Spatial<'a>>,
	hmd: Option<Spatial<'a>>,
}

impl<'a> Client<'a> {
	pub fn connect() -> Option<Self> {
		let connection = client::connect()?;
		Client::from_connection(connection)
	}
	pub fn from_connection(connection: UnixStream) -> Option<Self> {
		let mut client = Client {
			scenegraph: Scenegraph::new(),
			messenger: Rc::new(Messenger::new(connection)),
			root: None,
			hmd: None,
		};

		client.root = Some(Spatial::from_path(&client, "/").ok()?);
		client.hmd = Some(Spatial::from_path(&client, "/hmd").ok()?);

		Some(client)
	}
	pub fn dispatch(&self) -> Result<(), std::io::Error> {
		self.messenger.dispatch(&self.scenegraph)
	}

	pub fn get_weak_messenger(&self) -> Weak<Messenger<'a>> {
		Rc::downgrade(&self.messenger)
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
