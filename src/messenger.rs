use mio::net::UnixStream;
use std::{
	io::{Read, Result, Write},
	sync::Mutex,
};

use core::hash::BuildHasherDefault;
use dashmap::DashMap;
use rustc_hash::FxHasher;

use crate::{
	scenegraph,
	schemas::message::{
		self,
		stardust_xr::{Message, MessageArgs},
	},
};

pub type RawCallback = dyn Fn(&[u8]) + Send;
pub type Callback = dyn Fn(&flexbuffers::Reader<&[u8]>);

/// if you send a method call and expect a response back, you need to queue the callback so whenever you handle all the messages the callback can be called
/// so pending_callbacks is the queue
pub struct Messenger {
	connection: Mutex<UnixStream>,
	pending_callbacks: Mutex<DashMap<u32, Box<RawCallback>, BuildHasherDefault<FxHasher>>>,
}

impl Messenger {
	pub fn new(connection: UnixStream) -> Self {
		Self {
			connection: Mutex::new(connection),
			pending_callbacks: Default::default(),
		}
	}

	/// This makes sure that there are no repeat id's, but every id is filled.
	/// for example if a id like 2, finished, but you still had 1, 3, 4, and 5 waiting
	/// then you could reuse 2
	pub fn generate_message_id(&self) -> u32 {
		let mut id: u32 = 0;
		while self.pending_callbacks.lock().unwrap().contains_key(&id) {
			id += 1;
		}
		id
	}

	//let flex_root = flexbuffers::Reader::get_root(message.unwrap()).unwrap();
	pub fn error<T: std::fmt::Display>(&self, object: &str, method: &str, err: T) -> Result<()> {
		let error = format!("{}", err);
		self.send_call(0, None, object, method, Some(error.as_str()), None)
	}
	pub fn send_remote_signal(&self, object: &str, method: &str, data: &[u8]) -> Result<()> {
		self.send_call(1, None, object, method, None, Some(data))
	}
	pub fn execute_remote_method(
		&self,
		object: &str,
		method: &str,
		data: &[u8],
		callback: Box<RawCallback>,
	) -> Result<()> {
		let id = self.generate_message_id();
		self.pending_callbacks.lock().unwrap().insert(id, callback);
		self.send_call(1, None, object, method, None, Some(data))
	}
	fn send_call(
		&self,
		call_type: u8,
		id: Option<u32>,
		path: &str,
		method: &str,
		err: Option<&str>,
		data: Option<&[u8]>,
	) -> Result<()> {
		let mut fbb = flatbuffers::FlatBufferBuilder::with_capacity(1024);
		let flex_path = fbb.create_string(path);
		let flex_method = fbb.create_string(method);
		let flex_err = err.map(|s| fbb.create_string(s));
		let flex_data = data.map(|s| fbb.create_vector(s));

		let message_constructed = message::stardust_xr::Message::create(
			&mut fbb,
			&MessageArgs {
				type_: call_type,
				id: id.unwrap_or(0),
				object: Some(flex_path),
				method: Some(flex_method),
				error: flex_err,
				data: flex_data,
			},
		);
		fbb.finish(message_constructed, None);

		let message_length = fbb.finished_data().len() as u32;
		self.connection
			.lock()
			.unwrap()
			.write_all(&message_length.to_ne_bytes())?;

		self.connection
			.lock()
			.unwrap()
			.write_all(fbb.finished_data())?;
		Ok(())
	}

	fn handle_message(
		&self,
		message: &Message,
		scenegraph: &impl scenegraph::Scenegraph,
	) -> Result<()> {
		let message_type = message.type_();
		match message_type {
			// Errors
			0 => eprintln!(
				"[Stardust XR][{}:{}] {}",
				message.object().unwrap_or("unknown"),
				message.method().unwrap_or("unknown"),
				message.error().unwrap_or("unknown"),
			),
			// Signals
			1 => {
				scenegraph
					.send_signal(
						message.object().unwrap(),
						message.method().unwrap(),
						message.data().unwrap(),
					)
					.unwrap_or_else(|error| {
						self.error(message.object().unwrap(), message.method().unwrap(), error)
							.ok();
					});
			}
			// Method called
			2 => {
				let method_result = scenegraph.execute_method(
					message.object().unwrap(),
					message.method().unwrap(),
					message.data().unwrap(),
				);
				match method_result {
					Ok(return_value) => self.send_call(
						3,
						Some(message.id()),
						message.object().unwrap(),
						message.method().unwrap(),
						None,
						Some(&return_value),
					)?,
					Err(error) => {
						self.error(message.object().unwrap(), message.method().unwrap(), error)
							.ok();
					}
				};
			}
			// Method return
			3 => {
				if self
					.pending_callbacks
					.lock()
					.unwrap()
					.contains_key(&message.id())
				{
					let callback_opt = self.pending_callbacks.lock().unwrap().remove(&message.id());
					match callback_opt {
						None => println!("The method callback on node \"{}\" with method \"{}\" and id {} is not pending",
							  message.object().unwrap(), message.method().unwrap(), message.id()),
						Some((_, callback)) => callback(message.data().unwrap())
					}
				}
			}
			_ => println!("Type is wayyy off"),
		}
		Ok(())
	}

	pub fn dispatch(&self, scenegraph: &impl scenegraph::Scenegraph) -> Result<()> {
		let mut message_length_buffer: [u8; 4] = [0; 4];
		self.connection
			.lock()
			.unwrap()
			.read_exact(&mut message_length_buffer)?;
		let message_length: u32 = u32::from_ne_bytes(message_length_buffer);

		let mut message_buffer: Vec<u8> = std::vec::from_elem(0_u8, message_length as usize);
		self.connection
			.lock()
			.unwrap()
			.read_exact(message_buffer.as_mut_slice())?;
		let message_root = message::stardust_xr::root_as_message(&message_buffer);
		self.handle_message(&message_root.unwrap(), scenegraph)?;
		Ok(())
	}
}
