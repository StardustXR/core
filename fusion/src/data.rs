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
	fields::{Field, FieldAspect},
	impl_aspects,
	node::NodeResult,
	node::OwnedAspect,
	spatial::{SpatialAspect, SpatialRef, SpatialRefAspect, Transform},
};
use stardust_xr::values::*;

stardust_xr_fusion_codegen::codegen_data_protocol!();

impl_aspects!(PulseSender: OwnedAspect, SpatialRefAspect, SpatialAspect);
impl PulseSender {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		mask: &Datamap,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_pulse_sender(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			mask,
		)
	}
}

impl_aspects!(PulseReceiver: OwnedAspect, SpatialRefAspect, SpatialAspect);
impl PulseReceiver {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		field: &impl FieldAspect,
		mask: &Datamap,
	) -> NodeResult<Self> {
		let client = spatial_parent.client()?;
		create_pulse_receiver(
			&client,
			client.generate_id(),
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
impl crate::client::ClientHandle {
	pub fn register_xkb_keymap(
		&self,
		keymap_string: String,
	) -> NodeResult<impl std::future::Future<Output = NodeResult<u64>> + Send + Sync> {
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
	pub async fn get_xkb_keymap(&self, keymap_id: u64) -> NodeResult<Keymap> {
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
	use crate::*;
	let mut client = Client::connect().await.expect("Couldn't connect");

	#[derive(Debug, Default, Clone, Copy, serde::Serialize, serde::Deserialize)]
	struct Test {
		test: (),
	}

	let field = super::fields::Field::create(
		client.get_root(),
		Transform::identity(),
		crate::fields::Shape::Sphere(0.1),
	)
	.unwrap();
	let data = Datamap::from_typed(Test::default()).unwrap();
	let pulse_sender = PulseSender::create(client.get_root(), Transform::none(), &data).unwrap();
	let pulse_receiver =
		PulseReceiver::create(client.get_root(), Transform::none(), &field, &data).unwrap();

	let event_loop = client.event_loop(|_client, stop| {
		while let Some(sender_event) = pulse_sender.recv_event() {
			match sender_event {
				PulseSenderEvent::NewReceiver { receiver, field } => {
					println!("New pulse receiver {:?} with field {:?}", receiver, field);
					receiver.send_data(&pulse_sender, &data).unwrap();
				}
				PulseSenderEvent::DropReceiver { id } => {
					println!("Pulse receiver {} dropped", id);
				}
			}
		}
		while let Some(receiver_event) = pulse_receiver.recv_event() {
			match receiver_event {
				PulseReceiverEvent::Data { sender, data } => {
					println!("Pulse sender {sender:?} sent {data:?}");
					stop.stop();
				}
			}
		}
	});

	tokio::time::timeout(std::time::Duration::from_secs(1), event_loop)
		.await
		.unwrap()
		.unwrap()
}
