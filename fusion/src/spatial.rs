//! Spatial types and interfaces for positioning objects in 3D space.

pub use stardust_xr_protocol::spatial::*;

use crate::{Result, client::Client};
use stardust_xr_protocol::client::ClientHandler;

pub trait SpatialExt {
	fn create<H: ClientHandler>(
		client: &Client<H>,
		parent: &SpatialRef,
		transform: Transform,
	) -> impl std::future::Future<Output = Result<(Spatial, SpatialRef)>> + Send;
}
impl SpatialExt for Spatial {
	async fn create<H: ClientHandler>(
		client: &Client<H>,
		parent: &SpatialRef,
		transform: Transform,
	) -> Result<(Spatial, SpatialRef)> {
		let created = client
			.spatial_interface()
			.create_spatial(parent.clone(), transform)
			.await??;

		Ok((created.spatial, created.spatial_ref))
	}
}
