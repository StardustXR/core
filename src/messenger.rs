#[allow(dead_code, unused_imports)]
#[path = "schemas/message.rs"]
mod message;
use message::stardust_xr::{MessageArgs, Message};

#[path = "scenegraph.rs"]
mod scenegraph;

use bincode::serialize;

use std::collections::HashMap;
use std::io::{Result, Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::Mutex;

type Callback = fn(&flexbuffers::Reader<&[u8]>);

pub struct Messenger {
	connection: Mutex<UnixStream>,
	pending_callbacks: Mutex<HashMap<u32, Callback>>,
}

impl Messenger {
	pub fn new(connection: UnixStream) -> Self {
		Self {
			connection: Mutex::new(connection),
			pending_callbacks: Mutex::new(HashMap::new())
		}
	}

	fn generate_message_id(&mut self) -> u32 {
		let mut id:u32 = 0;
		while !self.pending_callbacks.lock().unwrap().contains_key(&id) {
			id+=1;
		}
		return id;
	}

	pub fn error(&mut self, object: &str, method: &str, err: &str) -> Result<()> {
		self.send_call(0, None, object, method, Some(err), None)?;
		Ok(())
	}
	pub fn send_signal<T>(&mut self, object: &str, method: &str, args_constructor: T) -> Result<()> where T: Fn(&mut flexbuffers::Builder) {
		let mut fbb = flexbuffers::Builder::default();
		args_constructor(&mut fbb);
		self.send_signal_raw(object, method, fbb.view())?;
		Ok(())
	}
	pub fn send_signal_raw(&mut self, object: &str, method: &str, data: &[u8]) -> Result<()> {
		self.send_call(1, None, object, method, None, Some(data))?;
		Ok(())
	}

	pub fn execute_remote_method<T>(&mut self, object: &str, method: &str, args_constructor: T, callback: Callback) -> Result<()> where T: Fn(&mut flexbuffers::Builder) {
		let mut fbb = flexbuffers::Builder::default();
		args_constructor(&mut fbb);
		self.execute_remote_method_raw(object, method, fbb.view(), callback)?;
		Ok(())
	}
	pub fn execute_remote_method_raw(&mut self, object: &str, method: &str, data: &[u8], callback: Callback) -> Result<()> {
		let id = self.generate_message_id();
		self.pending_callbacks.lock().unwrap().insert(id, callback);
		self.send_call(1, None, object, method, None, Some(data))?;
		Ok(())
	}

	fn send_call(&mut self, call_type: u8, id: Option<u32>, path: &str, method: &str, err: Option<&str>, data: Option<&[u8]>) -> Result<()> {
		let mut fbb     = flatbuffers::FlatBufferBuilder::with_capacity(1024);
		let flex_path   = fbb.create_string(path);
		let flex_method = fbb.create_string(method);
		let flex_err    = err.map(|s| fbb.create_string(s));
		let flex_data   = data.map(|s| fbb.create_vector(s));

		let message_constructed = message::stardust_xr::Message::create(&mut fbb, &MessageArgs{
			type_: call_type,
			id: id.unwrap_or(0),
			object: Some(flex_path),
			method: Some(flex_method),
			error: flex_err,
			data: flex_data,
		});
		fbb.finish(message_constructed, None);

//		let mut flbb = flexbuffers::Builder::default();
//		flbb.build_singleton(fbb.finished_data().len() as u32);
		print!("Message length's flexbuffer size is {}", fbb.finished_data().len());
		let message_length = fbb.finished_data().len() as u32;
		self.connection.lock().unwrap().write_all(&message_length.to_ne_bytes())?;

		self.connection.lock().unwrap().write_all(fbb.finished_data())?;
		Ok(())
	}

	fn handle_message<S>(&mut self, message: &Message, scenegraph: S) -> Result<()> where S: scenegraph::Scenegraph {
		let message_type = message.type_();
		match message_type {
			0 => println!("[Stardust XR][{:?}:{:?}] {:?}", message.object(), message.method(), message.error()),
			1 => {
				let data_root = flexbuffers::Reader::get_root(message.data().unwrap());
				scenegraph.send_signal(message.object().unwrap(), message.method().unwrap(), &data_root.unwrap());
			},
			2 => {
				let data_root = flexbuffers::Reader::get_root(message.data().unwrap());
				let return_value = scenegraph.execute_method(message.object().unwrap(), message.method().unwrap(), &data_root.unwrap());
				self.send_call(3, Some(message.id()), message.object().unwrap(), message.method().unwrap(), None, Some(&return_value))?;
			},
			3 => {
				if self.pending_callbacks.lock().unwrap().contains_key(&message.id()) {
					let callback_opt = self.pending_callbacks.lock().unwrap().remove(&message.id());
					match callback_opt {
						None => {
							println!("The method callback on node \"{}\" with method \"{}\" and id {} is not pending",
							          message.object().unwrap(), message.method().unwrap(), message.id());
						},
						Some(callback) => {
							let flex_root = flexbuffers::Reader::get_root(message.data().unwrap()).unwrap();
							callback(&flex_root);
						}
					}
				}
			},
			_ => println!("Type is wayyy off"),
		}
		Ok(())
	}

	pub fn dispatch<S>(&mut self, scenegraph: S) -> Result<()> where S: scenegraph::Scenegraph {
		let mut message_length_buffer: [u8; 8] = [0; 8];
		self.connection.lock().unwrap().read_exact(&mut message_length_buffer)?;
		let root = flexbuffers::Reader::get_root(&message_length_buffer as &[u8]).unwrap();
		let message_length: u32 = root.as_u32();

		let mut message_buffer: Vec<u8> = Vec::with_capacity(message_length as usize);
		self.connection.lock().unwrap().read_exact(message_buffer.as_mut_slice())?;
		let message_root = message::stardust_xr::root_as_message(&message_buffer);
		self.handle_message(&message_root.unwrap(), scenegraph)?;
		Ok(())
	}
}

