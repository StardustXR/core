use super::{scenegraph::Scenegraph, spatial::Spatial};
use crate::{client, messenger::Messenger};
use mio::net::UnixStream;
use mio::unix::pipe;
use mio::{Events, Interest, Poll, Token};
use parking_lot::Mutex;
use std::io::Write;
use std::sync::{Arc, Weak};

#[derive(Clone)]
pub struct ClientStopper(Arc<Mutex<pipe::Sender>>);

impl ClientStopper {
	pub fn stop(&self) -> Result<(), std::io::Error> {
		let _ = self.0.lock().write(&[0_u8; 1])?;
		Ok(())
	}
}

pub struct Client<'a> {
	pub messenger: Arc<Messenger>,
	pub scenegraph: Scenegraph<'a>,

	poll: Poll,
	stop_sender: ClientStopper,

	root: Option<Spatial<'a>>,
	hmd: Option<Spatial<'a>>,
}

const STOP_TOKEN: Token = Token(0);
const DISPATCH_TOKEN: Token = Token(1);

impl<'a> Client<'a> {
	pub fn connect() -> Result<Self, std::io::Error> {
		let connection = client::connect()?;
		Client::from_connection(connection)
	}
	pub fn from_connection(connection: UnixStream) -> Result<Self, std::io::Error> {
		let mut connection = connection;
		let poll = Poll::new()?;

		poll.registry()
			.register(&mut connection, DISPATCH_TOKEN, Interest::READABLE)?;

		let (stop_sender, mut stop_receiver) = pipe::new()?;
		poll.registry()
			.register(&mut stop_receiver, STOP_TOKEN, Interest::READABLE)?;

		let mut client = Client {
			scenegraph: Scenegraph::new(),
			messenger: Arc::new(Messenger::new(connection)),

			poll,
			stop_sender: ClientStopper(Arc::new(Mutex::new(stop_sender))),

			root: None,
			hmd: None,
		};

		client.root = Some(Spatial::from_path(&client, "/").unwrap());
		client.hmd = Some(Spatial::from_path(&client, "/hmd").unwrap());

		Ok(client)
	}
	pub fn dispatch(&self) -> Result<(), std::io::Error> {
		self.messenger.dispatch(&self.scenegraph)
	}

	pub fn get_weak_messenger(&self) -> Weak<Messenger> {
		Arc::downgrade(&self.messenger)
	}

	pub fn get_root(&self) -> &Spatial<'a> {
		self.root.as_ref().unwrap()
	}
	pub fn get_hmd(&self) -> &Spatial<'a> {
		self.hmd.as_ref().unwrap()
	}

	pub fn run_event_loop(
		&mut self,
		timeout: Option<core::time::Duration>,
	) -> Result<(), std::io::Error> {
		let mut events = Events::with_capacity(1024);
		'events: loop {
			self.poll.poll(&mut events, timeout)?;
			for event in &events {
				match event.token() {
					DISPATCH_TOKEN => 'dispatch: loop {
						match self.dispatch() {
							Ok(_) => continue,
							Err(e) => {
								if e.kind() == std::io::ErrorKind::WouldBlock {
									break 'dispatch;
								}
								return Err(e);
							}
						}
					},
					STOP_TOKEN => break 'events,
					Token(_) => break 'events,
				}
			}
		}

		self.messenger
			.send_remote_signal("/", "disconnect", &[0_u8; 0])?;
		Ok(())
	}

	pub fn get_cross_thread_stopper(&self) -> ClientStopper {
		self.stop_sender.clone()
	}

	pub fn stop_event_loop(&self) -> Result<(), std::io::Error> {
		self.stop_sender.stop()
	}
}

#[test]
fn fusion_client_connect() {
	let mut client = Client::connect().expect("Couldn't connect");
	let stopper = client.get_cross_thread_stopper();
	std::thread::spawn(move || {
		std::thread::sleep(core::time::Duration::from_secs(1));
		let _ = stopper.stop().and_then(|_| {
			println!("Successfully stopped");
			Ok(())
		});
	});
	client.run_event_loop(None).expect("Event loop failed");
}
