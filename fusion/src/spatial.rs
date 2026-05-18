//! Spatial types and interfaces for positioning objects in 3D space.

use stardust_xr_protocol::client::ClientHandler;
pub use stardust_xr_protocol::spatial::*;

use crate::{client::Client, error::ServerError};

pub trait SpatialExt {
	fn create<H: ClientHandler>(
		client: &Client<H>,
		parent: &SpatialRef,
		transform: Transform,
	) -> impl std::future::Future<Output = Result<Spatial, ServerError>> + Send;
}
impl SpatialExt for Spatial {
	async fn create<H: ClientHandler>(
		client: &Client<H>,
		parent: &SpatialRef,
		transform: Transform,
	) -> Result<Spatial, ServerError> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.spatial_interface()
			.create_spatial(parent.clone(), transform)
			.await?)
	}
}
