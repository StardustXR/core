use crate::{
	fields::{Field, UnknownField},
	node::NodeError,
	node::{ClientOwned, Node, NodeType},
	spatial::Spatial,
	HandlerWrapper, WeakWrapped,
};
use mint::{Quaternion, Vector3};
use parking_lot::{RwLock, RwLockReadGuard};
use rustc_hash::FxHashMap;
use serde::Deserialize;
use stardust_xr::{schemas::flex::deserialize, values::Transform};
use std::sync::Arc;

pub trait PulseSenderHandler: Send + Sync {
	fn new_receiver(
		&mut self,
		receiver: &PulseReceiver,
		field: &UnknownField,
		info: NewReceiverInfo,
	);
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
	receivers: RwLock<FxHashMap<String, (PulseReceiver, UnknownField)>>,
}
impl<'a> PulseSender {
	pub fn create<F, T>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		mask: Vec<u8>,
		wrapped_init: F,
	) -> Result<HandlerWrapper<PulseSender, T>, NodeError>
	where
		F: FnOnce(&Arc<PulseSender>) -> T,
		T: PulseSenderHandler + 'static,
	{
		flexbuffers::Reader::get_root(mask.as_slice())
			.and_then(|f| f.get_map())
			.map_err(|_| NodeError::MapInvalid)?;
		let id = nanoid::nanoid!();
		let sender = PulseSender {
			spatial: Spatial {
				node: Node::new(
					spatial_parent.node.client.clone(),
					"/data",
					"create_pulse_sender",
					"/data/sender",
					true,
					&id.clone(),
					(
						id,
						spatial_parent,
						Transform {
							position,
							rotation,
							scale: None,
						},
						mask,
					),
				)?,
			},
			receivers: RwLock::new(FxHashMap::default()),
		};

		let handler_wrapper = HandlerWrapper::new(sender, |weak_handler, sender| {
			sender.node().local_signals.lock().insert(
				"new_receiver".to_string(),
				Arc::new({
					let weak_handler: WeakWrapped<dyn PulseSenderHandler> = weak_handler.clone();
					let node_ref = sender.clone();
					move |data| {
						let info: NewReceiverInfo = deserialize(data)?;
						let receiver = PulseReceiver {
							spatial: Spatial::from_path(
								node_ref.node().client.clone(),
								&(node_ref.node().get_path().to_string() + "/" + &info.uid),
								false,
							)?,
						};
						let field = UnknownField {
							spatial: Spatial::from_path(
								node_ref.node().client.clone(),
								&(node_ref.node().get_path().to_string()
									+ "/" + &info.uid + "-field"),
								false,
							)?,
						};
						node_ref
							.receivers
							.write()
							.insert(info.uid.clone(), (receiver, field));
						if let Some(handler) = weak_handler.upgrade() {
							let receivers = node_ref.receivers.read();
							let (receiver, field) = receivers.get(&info.uid).unwrap();
							handler.lock().new_receiver(receiver, field, info);
							// handler.lock().enter(, spatial)
						}
						Ok(())
					}
				}),
			);
			sender.node().local_signals.lock().insert(
				"drop_receiver".to_string(),
				Arc::new({
					let weak_handler: WeakWrapped<dyn PulseSenderHandler> = weak_handler;
					let sender = sender.clone();
					move |data| {
						let uid: &str = deserialize(data)?;
						sender.receivers.write().remove(uid);
						if let Some(handler) = weak_handler.upgrade() {
							handler.lock().drop_receiver(uid);
						}
						Ok(())
					}
				}),
			);
			wrapped_init(sender)
		});

		// handler_wrapper.
		Ok(handler_wrapper)
	}

	pub fn send_data(&self, receiver: &PulseReceiver, data: &[u8]) -> Result<(), NodeError> {
		flexbuffers::Reader::get_root(data)
			.and_then(|f| f.get_map())
			.map_err(|_| NodeError::MapInvalid)?;

		self.node
			.send_remote_signal("send_data", &(receiver.node().get_name(), data))
	}

	pub fn receivers<'r>(
		&'r self,
	) -> RwLockReadGuard<'r, FxHashMap<String, (PulseReceiver, UnknownField)>> {
		self.receivers.read()
	}
}
impl NodeType for PulseSender {
	fn node(&self) -> &Node {
		&self.spatial.node
	}
}
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
	pub fn create<F, Fi: Field + ClientOwned, T>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		field: &'a Fi,
		mask: Vec<u8>,
		wrapped_init: F,
	) -> Result<HandlerWrapper<Self, T>, NodeError>
	where
		F: FnOnce(&Arc<PulseReceiver>) -> T,
		T: PulseReceiverHandler + 'static,
	{
		flexbuffers::Reader::get_root(mask.as_slice())
			.and_then(|f| f.get_map())
			.map_err(|_| NodeError::MapInvalid)?;

		let id = nanoid::nanoid!();
		let pulse_rx = PulseReceiver {
			spatial: Spatial {
				node: Node::new(
					spatial_parent.node.client.clone(),
					"/data",
					"create_pulse_receiver",
					"/data/receiver",
					true,
					&id.clone(),
					(
						id,
						spatial_parent,
						Transform {
							position,
							rotation,
							scale: None,
						},
						&field.node(),
						mask,
					),
				)?,
			},
		};

		Ok(HandlerWrapper::new(pulse_rx, |weak_handler, node_ref| {
			node_ref.node().local_signals.lock().insert(
				"data".to_string(),
				Arc::new({
					let weak_handler: WeakWrapped<dyn PulseReceiverHandler> = weak_handler;
					#[derive(Deserialize)]
					struct SendDataInfo<'a> {
						uid: &'a str,
						data: Vec<u8>,
					}
					move |data| {
						let info: SendDataInfo = deserialize(data)?;
						let data_reader = flexbuffers::Reader::get_root(info.data.as_slice())
							.and_then(|f| f.get_map())?;
						if let Some(handler) = weak_handler.upgrade() {
							handler
								.lock()
								.data(info.uid, info.data.as_slice(), data_reader);
						}
						Ok(())
					}
				}),
			);
			wrapped_init(node_ref)
		}))
	}
}
impl NodeType for PulseReceiver {
	fn node(&self) -> &Node {
		self.spatial.node()
	}
}

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
		node: Arc<PulseSender>,
	}
	impl PulseSenderHandler for PulseSenderTest {
		fn new_receiver(
			&mut self,
			receiver: &PulseReceiver,
			field: &UnknownField,
			info: NewReceiverInfo,
		) {
			println!(
				"New pulse receiver {:?} with field {:?} and info {:?}",
				receiver.node().get_path(),
				field.node().get_path(),
				info
			);
			self.node.send_data(receiver, &self.data).unwrap();
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
	let _pulse_sender = PulseSender::create(
		client.get_root(),
		None,
		None,
		mask.view().to_vec(),
		|node| PulseSenderTest {
			data: mask.view().to_vec(),
			node: node.clone(),
		},
	)
	.unwrap();
	let _pulse_receiver = PulseReceiver::create(
		client.get_root(),
		None,
		None,
		&field,
		mask.take_buffer(),
		|_| PulseReceiverTest(client.clone()),
	)
	.unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
