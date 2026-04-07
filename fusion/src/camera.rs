//! Camera types and interfaces.

pub use stardust_xr_protocol::camera::*;

// use crate::{client::Client, error::ServerError};
//
// pub trait CameraExt {
// 	fn new(
// 		client: &Client,
// 		parent: &SpatialRef,
// 		transform: Transform,
// 	) -> impl std::future::Future<Output = Result<Spatial, ServerError>> + Send;
// }
// impl SpatialExt for Spatial {
// 	async fn new(
// 		client: &Client,
// 		parent: &SpatialRef,
// 		transform: Transform,
// 	) -> Result<Spatial, ServerError> {
// 		// TODO: actually handle invalid handles at the protocol level
// 		Ok(client
// 			.spatial_interface()
// 			.create_spatial(parent.clone(), transform)
// 			.await?)
// 	}
// }
