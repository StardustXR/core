//! Spatial types and interfaces for positioning objects in 3D space.

use crate::{client::Client, error::ServerError};
use stardust_xr_protocol::{client::ClientHandler, types::CreateError};

pub use stardust_xr_protocol::spatial::*;

pub trait SpatialExt {
	fn create<H: ClientHandler>(
		client: &Client<H>,
		parent: &SpatialRef,
		transform: Transform,
	) -> impl std::future::Future<Output = Result<Result<CreatedSpatial, CreateError>, ServerError>> + Send;
}
impl SpatialExt for Spatial {
	async fn create<H: ClientHandler>(
		client: &Client<H>,
		parent: &SpatialRef,
		transform: Transform,
	) -> Result<Result<CreatedSpatial, CreateError>, ServerError> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.spatial_interface()
			.create_spatial(parent.clone(), transform)
			.await?)
	}
}
