//! Anything the user can see such as lines, models and text.

mod lines;
mod model;
mod text;

// pub use lines::*;
pub use model::*;
use stardust_xr::schemas::flex::serialize;
// pub use text::*;

use crate::{client::Client, node::NodeError};
use std::path::Path;

/// Set only the sky texture to this equirectangular `.hdr` file.
pub fn set_sky_tex(client: &Client, file: &impl AsRef<Path>) -> Result<(), NodeError> {
	set_sky(client, file, true, false)
}
/// Set only the sky lighting to this equirectangular `.hdr` file.
pub fn set_sky_light(client: &Client, file: &impl AsRef<Path>) -> Result<(), NodeError> {
	set_sky(client, file, false, true)
}
/// Set the sky texture and lighting to this equirectangular `.hdr` file.
pub fn set_sky_tex_light(client: &Client, file: &impl AsRef<Path>) -> Result<(), NodeError> {
	set_sky(client, file, true, true)
}

fn set_sky(
	client: &Client,
	file: &impl AsRef<Path>,
	tex: bool,
	light: bool,
) -> Result<(), NodeError> {
	if !file.as_ref().exists() {
		return Err(NodeError::InvalidPath);
	}
	let file_str = file.as_ref().to_str().ok_or(NodeError::InvalidPath)?;
	client
		.message_sender_handle
		.signal(
			"/drawable",
			"set_sky_file",
			&serialize(&(file_str, tex, light)).map_err(|_| NodeError::Serialization)?,
			Vec::new(),
		)
		.map_err(|e| NodeError::MessengerError { e })
}

#[tokio::test]
async fn fusion_sky() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let sky_path = std::path::PathBuf::from(manifest_dir_macros::file_relative_path!(
		"res/fusion/sky.hdr"
	));

	set_sky_tex(&client, &sky_path).unwrap();
	set_sky_light(&client, &sky_path).unwrap();
	set_sky_tex_light(&client, &sky_path).unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(5)).await;
}
