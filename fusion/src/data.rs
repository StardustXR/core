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

use crate::{fields::{FieldAspect, Field}, node::NodeResult, node::{NodeAspect, NodeType}, spatial::{SpatialAspect, Transform}, impl_aspects};
use nanoid::nanoid;
use stardust_xr::values::Datamap;

stardust_xr_fusion_codegen::codegen_data_protocol!();

impl_aspects!(PulseSender: NodeAspect, SpatialAspect);
impl PulseSender {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		mask: &Datamap,
	) -> NodeResult<Self> {
		create_pulse_sender(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			mask,
		)
	}
}

impl_aspects!(PulseReceiver: NodeAspect, SpatialAspect);
impl PulseReceiver {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		field: &impl FieldAspect,
		mask: &Datamap,
	) -> NodeResult<Self> {
		create_pulse_receiver(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			field,
			mask,
		)
	}
}

#[cfg(feature = "keymap")]
pub use xkbcommon::xkb;
#[cfg(feature = "keymap")]
use xkbcommon::xkb::{Context, Keymap, FORMAT_TEXT_V1, KEYMAP_COMPILE_NO_FLAGS};
#[cfg(feature = "keymap")]
impl crate::client::Client {
	pub fn register_xkb_keymap(
		&self,
		keymap_string: String,
	) -> NodeResult<impl std::future::Future<Output = NodeResult<String>> + Send + Sync> {
		let client = self.get_root().client();
		Keymap::new_from_string(
			&Context::new(0),
			keymap_string.clone(),
			FORMAT_TEXT_V1,
			KEYMAP_COMPILE_NO_FLAGS,
		)
		.ok_or_else(|| crate::node::NodeError::ReturnedError {
			e: "Invalid keymap".to_string(),
		})?;
		Ok(async move { register_keymap(&client?, &keymap_string).await })
	}
	pub async fn get_xkb_keymap(&self, keymap_id: &str) -> NodeResult<Keymap> {
		let keymap_str = get_keymap(&self.get_root().client()?, keymap_id).await?;

		Keymap::new_from_string(
			&Context::new(0),
			keymap_str,
			FORMAT_TEXT_V1,
			KEYMAP_COMPILE_NO_FLAGS,
		)
		.ok_or_else(|| crate::node::NodeError::InvalidPath)
	}
}

#[tokio::test]
async fn fusion_pulses() {
	use super::client::Client;
	color_eyre::install().unwrap();
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
	struct Test {
		test: (),
	}

	struct PulseReceiverTest(std::sync::Arc<Client>);
	impl PulseReceiverHandler for PulseReceiverTest {
		fn data(&mut self, uid: String, data: Datamap) {
			println!("Pulse sender {uid} sent {data:?}");
			self.0.stop_loop();
		}
	}
	struct PulseSenderTest {
		data: Datamap,
		node: PulseSender,
	}
	impl PulseSenderHandler for PulseSenderTest {
		fn new_receiver(&mut self, uid: String, receiver: PulseReceiver, field: Field) {
			println!(
				"New pulse receiver {:?} with field {:?} and uid {:?}",
				receiver, field, uid
			);
			receiver.send_data(&self.node, &self.data).unwrap();
		}
		fn drop_receiver(&mut self, uid: String) {
			println!("Pulse receiver {} dropped", uid);
		}
	}

	let field =
		super::fields::SphereField::create(client.get_root(), mint::Vector3::from([0.0; 3]), 0.1)
			.unwrap();

	let data = Datamap::from_typed(Test::default()).unwrap();
	let pulse_sender = PulseSender::create(client.get_root(), Transform::none(), &data).unwrap();
	let pulse_sender_handler = PulseSenderTest {
		data: data.clone(),
		node: pulse_sender.alias(),
	};
	let _pulse_sender_handler = pulse_sender.wrap(pulse_sender_handler).unwrap();
	let _pulse_receiver =
		PulseReceiver::create(client.get_root(), Transform::none(), &field, &data)
			.unwrap()
			.wrap(PulseReceiverTest(client.clone()))
			.unwrap();

	tokio::select! {
		_ = tokio::time::sleep(core::time::Duration::from_secs(1)) => panic!("Timed Out"),
		e = event_loop => e.unwrap().unwrap(),
	}
}
