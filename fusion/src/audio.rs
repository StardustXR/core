//! Audio types and interfaces.

pub use stardust_xr_protocol::audio::*;

use crate::{Result, client::Client};
use stardust_xr_protocol::{client::ClientHandler, spatial::Spatial, types::Resource};

pub trait SoundExt {
	fn create<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		sound: Resource,
	) -> impl std::future::Future<Output = Result<Sound>> + Send;
}
impl SoundExt for Sound {
	async fn create<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		sound: Resource,
	) -> Result<Sound> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.audio_interface()
			.create_sound(spatial.clone(), sound)
			.await??)
	}
}
