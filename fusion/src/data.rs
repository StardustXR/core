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
	spatial::{Spatial, SpatialAspect, SpatialRef, SpatialRefAspect, Transform},
};
use stardust_xr::values::*;

pub use xkbcommon::xkb;
use xkbcommon::xkb::{Context, Keymap, FORMAT_TEXT_V1, KEYMAP_COMPILE_NO_FLAGS};
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
