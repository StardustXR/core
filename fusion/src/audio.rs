//! Audio types and interfaces.

pub use stardust_xr_protocol::audio::*;
use stardust_xr_protocol::{spatial::Spatial, types::Resource};

use crate::{client::Client, error::ServerError};

pub trait SoundExt {
	fn new(
		client: &Client,
        spatial: &Spatial,
        sound: Resource,
	) -> impl std::future::Future<Output = Result<Sound, ServerError>> + Send;
}
impl SoundExt for Sound {
	async fn new(
		client: &Client,
        spatial: &Spatial,
        sound: Resource,
	) -> Result<Sound, ServerError> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.audio_interface()
			.create_sound(spatial.clone(), sound)
			.await?)
	}
}
