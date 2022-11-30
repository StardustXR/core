use crate::{
	fields::{Field, UnknownField},
	node::{ClientOwned, Node, NodeType},
	node::{HandledNodeType, NodeError},
	spatial::Spatial,
	HandlerWrapper,
};
use anyhow::Result;
use mint::{Quaternion, Vector3};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use rustc_hash::FxHashMap;
use serde::Deserialize;
use stardust_xr::{schemas::flex::deserialize, values::Transform};
use std::sync::Arc;

pub trait PulseSenderHandler: Send + Sync {
	fn new_receiver(&mut self, info: NewReceiverInfo, receiver: PulseReceiver, field: UnknownField);
	fn drop_receiver(&mut self, uid: &str);
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewReceiverInfo {
	pub uid: String,
	pub distance: f32,
	pub position: Vector3<f32>,
	pub rotation: Quaternion<f32>,
}

#[derive(Debug)]
pub struct PulseSender {
	pub spatial: Spatial,
	receivers: Arc<RwLock<FxHashMap<String, (PulseReceiver, UnknownField)>>>,
}
impl<'a> PulseSender {
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		mask: Vec<u8>,
	) -> Result<PulseSender, NodeError> {
		flexbuffers::Reader::get_root(mask.as_slice())
			.and_then(|f| f.get_map())
			.map_err(|_| NodeError::MapInvalid)?;
		let id = nanoid::nanoid!();
		Ok(PulseSender {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/data",
					"create_pulse_sender",
					"/data/sender",
					true,
					&id.clone(),
					(
						id,
						spatial_parent.node().get_path()?,
						Transform {
							position,
							rotation,
							scale: None,
						},
						mask,
					),
				)?,
			},
			receivers: Arc::new(RwLock::new(FxHashMap::default())),
		})
	}

	fn handle_new_receiver<H: PulseSenderHandler>(
		sender: Arc<PulseSender>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let (info, receiver, field) = sender.new_receiver(data)?;
		handler.lock().new_receiver(info, receiver, field);
		Ok(())
	}
	fn new_receiver(&self, data: &[u8]) -> Result<(NewReceiverInfo, PulseReceiver, UnknownField)> {
		let client = self.client()?;
		let info: NewReceiverInfo = deserialize(data)?;
		let receiver_stored = PulseReceiver {
			spatial: Spatial::from_path(&client, self.node().get_path()?, &info.uid, false),
		};
		let receiver = PulseReceiver {
			spatial: receiver_stored.spatial.alias(),
		};
		let field_stored = UnknownField {
			spatial: Spatial::from_path(
				&client,
				self.node().get_path()?,
				info.uid.clone() + "-field",
				false,
			),
		};
		let field = UnknownField {
			spatial: field_stored.spatial.alias(),
		};
		self.receivers
			.write()
			.insert(info.uid.clone(), (receiver_stored, field_stored));

		Ok((info, receiver, field))
	}

	fn handle_drop_receiver<H: PulseSenderHandler>(
		sender: Arc<PulseSender>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let uid: &str = deserialize(data)?;
		sender.receivers.write().remove(uid);
		handler.lock().drop_receiver(uid);
		Ok(())
	}

	pub fn send_data(&self, receiver: &PulseReceiver, data: &[u8]) -> Result<(), NodeError> {
		flexbuffers::Reader::get_root(data)
			.and_then(|f| f.get_map())
			.map_err(|_| NodeError::MapInvalid)?;

		self.node
			.send_remote_signal("send_data", &(receiver.node().get_name()?, data))
	}

	pub fn receivers(&self) -> RwLockReadGuard<FxHashMap<String, (PulseReceiver, UnknownField)>> {
		self.receivers.read()
	}

	pub fn wrap<H: PulseSenderHandler>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new(self, handler);
		handler_wrapper.add_handled_signal("new_receiver", Self::handle_new_receiver)?;
		handler_wrapper.add_handled_signal("drop_receiver", Self::handle_drop_receiver)?;
		Ok(handler_wrapper)
	}
}
impl NodeType for PulseSender {
	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn alias(&self) -> Self {
		PulseSender {
			spatial: self.spatial.alias(),
			receivers: self.receivers.clone(),
		}
	}
}
impl HandledNodeType for PulseSender {}
impl std::ops::Deref for PulseSender {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

pub trait PulseReceiverHandler: Send + Sync {
	fn data(&mut self, uid: &str, data: &[u8], data_reader: flexbuffers::MapReader<&[u8]>);
}
#[derive(Debug)]
pub struct PulseReceiver {
	pub spatial: Spatial,
}
impl<'a> PulseReceiver {
	pub fn create<Fi: Field + ClientOwned>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		field: &'a Fi,
		mask: Vec<u8>,
	) -> Result<Self, NodeError> {
		flexbuffers::Reader::get_root(mask.as_slice())
			.and_then(|f| f.get_map())
			.map_err(|_| NodeError::MapInvalid)?;

		let id = nanoid::nanoid!();
		Ok(PulseReceiver {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/data",
					"create_pulse_receiver",
					"/data/receiver",
					true,
					&id.clone(),
					(
						id,
						spatial_parent.node().get_path()?,
						Transform {
							position,
							rotation,
							scale: None,
						},
						&field.node().get_path()?,
						mask,
					),
				)?,
			},
		})
	}

	pub fn wrap<H: PulseReceiverHandler>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new(self, handler);

		handler_wrapper.add_handled_signal("data", move |_receiver, handler, data| {
			#[derive(Deserialize)]
			struct SendDataInfo<'a> {
				uid: &'a str,
				data: Vec<u8>,
			}
			let info: SendDataInfo = deserialize(data)?;
			let data_reader =
				flexbuffers::Reader::get_root(info.data.as_slice()).and_then(|f| f.get_map())?;
			handler
				.lock()
				.data(info.uid, info.data.as_slice(), data_reader);
			Ok(())
		})?;

		Ok(handler_wrapper)
	}
}
impl NodeType for PulseReceiver {
	fn node(&self) -> &Node {
		self.spatial.node()
	}

	fn alias(&self) -> Self {
		PulseReceiver {
			spatial: self.spatial.alias(),
		}
	}
}
impl HandledNodeType for PulseReceiver {}

#[tokio::test]
async fn fusion_pulses() {
	use super::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	struct PulseReceiverTest(Arc<Client>);
	impl PulseReceiverHandler for PulseReceiverTest {
		fn data(&mut self, uid: &str, data: &[u8], _data_reader: flexbuffers::MapReader<&[u8]>) {
			println!(
				"Pulse sender {} sent {}",
				uid,
				flexbuffers::Reader::get_root(data).unwrap()
			);
			self.0.stop_loop();
		}
	}
	struct PulseSenderTest {
		data: Vec<u8>,
		node: PulseSender,
	}
	impl PulseSenderHandler for PulseSenderTest {
		fn new_receiver(
			&mut self,
			info: NewReceiverInfo,
			receiver: PulseReceiver,
			field: UnknownField,
		) {
			println!(
				"New pulse receiver {:?} with field {:?} and info {:?}",
				receiver.node().get_path(),
				field.node().get_path(),
				info
			);
			self.node.send_data(&receiver, &self.data).unwrap();
		}
		fn drop_receiver(&mut self, uid: &str) {
			println!("Pulse receiver {} dropped", uid);
		}
	}

	let field = super::fields::SphereField::builder()
		.spatial_parent(client.get_root())
		.radius(0.1)
		.build()
		.unwrap();

	let mut mask = flexbuffers::Builder::default();
	let mut map = mask.start_map();
	map.push("test", true);
	map.end_map();

	let pulse_sender =
		PulseSender::create(client.get_root(), None, None, mask.view().to_vec()).unwrap();
	let pulse_sender_handler = PulseSenderTest {
		data: mask.view().to_vec(),
		node: pulse_sender.alias(),
	};
	let _pulse_sender_handler = pulse_sender.wrap(pulse_sender_handler).unwrap();
	let _pulse_receiver =
		PulseReceiver::create(client.get_root(), None, None, &field, mask.take_buffer())
			.unwrap()
			.wrap(PulseReceiverTest(client.clone()))
			.unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
