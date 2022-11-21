mod lines;
mod model;
mod text;

pub use lines::*;
pub use model::*;
use stardust_xr::schemas::flex::serialize;
pub use text::*;

use crate::{client::Client, node::NodeError};
use anyhow::Result;
use std::path::Path;

impl Client {
	pub fn set_sky_tex(&self, file: &impl AsRef<Path>) -> Result<(), NodeError> {
		self.set_sky(file, true, false)
	}
	pub fn set_sky_light(&self, file: &impl AsRef<Path>) -> Result<(), NodeError> {
		self.set_sky(file, false, true)
	}
	pub fn set_sky_tex_light(&self, file: &impl AsRef<Path>) -> Result<(), NodeError> {
		self.set_sky(file, true, true)
	}

	fn set_sky(&self, file: &impl AsRef<Path>, tex: bool, light: bool) -> Result<(), NodeError> {
		if !file.as_ref().exists() {
			return Err(NodeError::InvalidPath);
		}
		let file_str = file.as_ref().to_str().ok_or(NodeError::InvalidPath)?;
		self.message_sender_handle
			.signal(
				"/drawable",
				"set_sky_file",
				&serialize(&(file_str, tex, light)).map_err(|_| NodeError::Serialization)?,
			)
			.map_err(|e| NodeError::MessengerError { e })
	}
}

#[tokio::test]
async fn fusion_sky() {
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let sky_path = std::path::PathBuf::from(manifest_dir_macros::file_relative_path!(
		"res/fusion/sky.hdr"
	));

	client.set_sky_tex(&sky_path).unwrap();
	client.set_sky_light(&sky_path).unwrap();
	client.set_sky_tex_light(&sky_path).unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(5)).await;
}
