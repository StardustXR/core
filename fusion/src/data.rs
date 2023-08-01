//! Module containing pulse senders/receivers, the way to send non-spatial data through 3D space.
//!
//! Uses include:
//! - Keyboard (virtual or physical) events
//! - Controller inputs
//! - Hardware mouse/trackball events (when mapping it to 3D space and back is impractical)
//! - Actions such as copy/paste, duplicate, etc.
//! - Quit requests
//!
//! Pulse senders and receivers both have a mask, a set of keys and values (using flexbuffers maps) that are used to filter specific protocols of information.
//!
//! Pulse senders can see all the pulse receivers that match their mask (contain at least the same keys/values).
//! Each receiver has its own UID to identify it for "connecting" the sender to it visually or such.
//! Pulse senders can send any message that matches the receiver's mask (contain at least the same keys/values).
//!
//! Pulse receivers have an attached field that can be used to make pulse senders aware of their bounds better, such as a panel with a box field and a pulse receiver for keyboard input.
//! The position/rotation of pulse receivers should be the exact point a visual indicator of connection would connect to, and the forward direction should be away from the body it's part of design-wise.
//! Pulse receivers cannot see the pulse senders, but any time data is sent to them they get the UID of the sender to allow keymap switching or such.

use crate::{
	fields::{Field, UnknownField},
	node::{HandledNodeType, NodeError},
	node::{Node, NodeType},
	spatial::Spatial,
	HandlerWrapper,
};

use mint::{Quaternion, Vector3};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use rustc_hash::FxHashMap;
use serde::Deserialize;
use stardust_xr::{
	schemas::flex::{deserialize, flexbuffers},
	values::Transform,
};
use std::{ops::Deref, os::fd::OwnedFd, sync::Arc};

/// Trait for handling when pulse receivers matching the sender's mask are created/destroyed on the server.
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

/// Node to send non-spatial data through 3D space.
///
/// # Example
/// ```
/// struct PulseSenderTest {
/// 	data: Vec<u8>,
/// 	node: PulseSender,
/// }
/// impl PulseSenderHandler for PulseSenderTest {
/// 	fn new_receiver(
/// 		&mut self,
/// 		info: NewReceiverInfo,
/// 		receiver: PulseReceiver,
/// 		field: UnknownField,
/// 	) {
/// 		println!(
/// 			"New pulse receiver {:?} with field {:?} and info {:?}",
/// 			receiver.node().get_path(),
/// 			field.node().get_path(),
/// 			info
/// 		);
/// 		self.node.send_data(&receiver, &self.data).unwrap();
/// 	}
/// 	fn drop_receiver(&mut self, uid: &str) {
/// 		println!("Pulse receiver {} dropped", uid);
/// 	}
/// }
///
/// let mask = {
/// 	let mut fbb = flexbuffers::Builder::default();
/// 	let mut map = fbb.start_map();
/// 	map.push("test", true);
/// 	map.end_map();
/// 	fbb.take_buffer()
/// };
///
/// let pulse_sender =
/// 	PulseSender::create(client.get_root(), None, None, &mask).unwrap();
/// let pulse_sender_handler = PulseSenderTest {
/// 	data: mask,
/// 	node: pulse_sender.alias(),
/// };
/// let _pulse_sender_handler = pulse_sender.wrap(pulse_sender_handler).unwrap();
/// ```
#[derive(Debug)]
pub struct PulseSender {
	spatial: Spatial,
	receivers: Arc<RwLock<FxHashMap<String, (PulseReceiver, UnknownField)>>>,
}
impl PulseSender {
	/// Create a pulse receiver node. The mask must be a flexbuffers serialized map at its root.
	pub fn create(
		spatial_parent: &Spatial,
		transform: Transform,
		mask: &[u8],
	) -> Result<PulseSender, NodeError> {
		flexbuffers::Reader::get_root(mask)
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
					(id, spatial_parent.node().get_path()?, transform, mask),
				)?,
			},
			receivers: Arc::new(RwLock::new(FxHashMap::default())),
		})
	}

	fn handle_new_receiver<H: PulseSenderHandler>(
		sender: Arc<PulseSender>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> color_eyre::eyre::Result<()> {
		let client = sender.client()?;
		let info: NewReceiverInfo = deserialize(data)?;
		let receiver_stored = PulseReceiver {
			spatial: Spatial::from_path(&client, sender.node().get_path()?, &info.uid, false),
		};
		let receiver = receiver_stored.alias();
		let field_stored = UnknownField {
			spatial: Spatial::from_path(
				&client,
				sender.node().get_path()?,
				info.uid.clone() + "-field",
				false,
			),
		};
		let field = field_stored.alias();
		sender
			.receivers
			.write()
			.insert(info.uid.clone(), (receiver_stored, field_stored));
		handler.lock().new_receiver(info, receiver, field);
		Ok(())
	}

	fn handle_drop_receiver<H: PulseSenderHandler>(
		sender: Arc<PulseSender>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
		_fds: Vec<OwnedFd>,
	) -> color_eyre::eyre::Result<()> {
		let uid: &str = deserialize(data)?;
		sender.receivers.write().remove(uid);
		handler.lock().drop_receiver(uid);
		Ok(())
	}

	/// Send data to the receiver.
	/// This message must be a flexbuffers serialized map with all the keys/values from the receiver's mask present.
	/// You can use `stardust_xr_fusion::core::schemas::flex::flexbuffers::FlexbufferSerializer` with the flexbuffers crate feature `serialize_human_readable` to serialize structs into maps.
	pub fn send_data(&self, receiver: &PulseReceiver, data: &[u8]) -> Result<(), NodeError> {
		flexbuffers::Reader::get_root(data)
			.and_then(|f| f.get_map())
			.map_err(|_| NodeError::MapInvalid)?;

		self.node
			.send_remote_signal("send_data", &(receiver.node().get_path()?, data))
	}

	/// Get a read only guard to the receivers list. This can be used instead of the handler if you want.
	pub fn receivers(&self) -> RwLockReadGuard<FxHashMap<String, (PulseReceiver, UnknownField)>> {
		self.receivers.read()
	}

	/// Wrap this node and a pulse sender handler struct into a `HandlerWrapper` struct. You can use `PulseSender::receivers()` instead.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: PulseSenderHandler>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		self.wrap_raw(Arc::new(Mutex::new(handler)))
	}

	/// Wrap this node and a pulse sender handler struct into a `HandlerWrapper` struct. You can use `PulseSender::receivers()` instead.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap_raw<H: PulseSenderHandler>(
		self,
		handler: Arc<Mutex<H>>,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new_raw(self, handler);
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

/// Trait for handling when data is sent to this pulse receiver.
pub trait PulseReceiverHandler: Send + Sync {
	/// `data` and `data_reader` point to the same data, so feel free to use one or the other.
	fn data(&mut self, uid: &str, data: &[u8], data_reader: flexbuffers::MapReader<&[u8]>);
}

/// Node to receive non-spatial data through 3D space.
///
/// # Example
/// ```
/// use stardust_xr_fusion::data::PulseReceiverHandler;
/// struct PulseReceiverTest;
/// impl PulseReceiverHandler for PulseReceiverTest {
/// 	fn data(&mut self, uid: &str, data: &[u8], _data_reader: flexbuffers::MapReader<&[u8]>) {
/// 		println!(
/// 			"Pulse sender {} sent {}",
/// 			uid,
/// 			flexbuffers::Reader::get_root(data).unwrap()
/// 		);
/// 	}
/// }
///
/// let mask = {
/// 	let mut fbb = flexbuffers::Builder::default();
/// 	let mut map = fbb.start_map();
/// 	map.push("test", true);
/// 	map.end_map();
/// 	fbb.take_buffer()
/// };
///
/// use stardust_xr_fusion::fields::SphereField;
/// let field = SphereField::builder()
/// 	.spatial_parent(client.get_root())
/// 	.radius(0.1)
/// 	.build()
/// 	.unwrap();
///
/// use stardust_xr_fusion::data::PulseReceiver;
/// let _pulse_receiver =
/// 	PulseReceiver::create(client.get_root(), None, None, &field, &mask)
/// 		.unwrap()
/// 		.wrap(PulseReceiverTest)
/// 		.unwrap();
/// ```
#[derive(Debug)]
pub struct PulseReceiver {
	spatial: Spatial,
}
impl PulseReceiver {
	/// Create a pulse receiver node. The field will remain intact even if its node is dropped.
	pub fn create<Fi: Field>(
		spatial_parent: &Spatial,
		transform: Transform,
		field: &Fi,
		mask: &[u8],
	) -> Result<Self, NodeError> {
		flexbuffers::Reader::get_root(mask)
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
						transform,
						&field.node().get_path()?,
						mask,
					),
				)?,
			},
		})
	}

	/// Wrap this struct and a handler into a `HandlerWrapper` if possible.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: PulseReceiverHandler>(
		self,
		handler: H,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		self.wrap_raw(Arc::new(Mutex::new(handler)))
	}
	/// Wrap this struct and a handler into a `HandlerWrapper` if possible.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap_raw<H: PulseReceiverHandler>(
		self,
		handler: Arc<Mutex<H>>,
	) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new_raw(self, handler);

		handler_wrapper.add_handled_signal("data", move |_receiver, handler, data, _fds| {
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
impl Deref for PulseReceiver {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_pulses() {
	use super::client::Client;
	color_eyre::install().unwrap();
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

	let field = super::fields::SphereField::create(client.get_root(), Vector3::from([0.0; 3]), 0.1)
		.unwrap();

	let mask = {
		let mut fbb = flexbuffers::Builder::default();
		let mut map = fbb.start_map();
		map.push("test", true);
		map.end_map();
		fbb.take_buffer()
	};

	let pulse_sender = PulseSender::create(client.get_root(), Transform::default(), &mask).unwrap();
	let pulse_sender_handler = PulseSenderTest {
		data: mask.clone(),
		node: pulse_sender.alias(),
	};
	let _pulse_sender_handler = pulse_sender.wrap(pulse_sender_handler).unwrap();
	let _pulse_receiver =
		PulseReceiver::create(client.get_root(), Transform::default(), &field, &mask)
			.unwrap()
			.wrap(PulseReceiverTest(client.clone()))
			.unwrap();

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(1)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	}
}
