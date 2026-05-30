//! Analog SDFs to define boundaries for input, interaction, and behavior.

pub use stardust_xr_protocol::field::*;
use stardust_xr_protocol::{client::ClientHandler, spatial::Spatial, types::CreateError};

use crate::{client::Client, error::ServerError};

pub trait FieldExt {
	fn create<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		shape: Shape,
	) -> impl std::future::Future<Output = Result<Result<CreatedField, CreateError>, ServerError>> + Send;
}
impl FieldExt for Field {
	async fn create<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		shape: Shape,
	) -> Result<Result<CreatedField, CreateError>, ServerError> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.field_interface()
			.create_field(spatial.clone(), shape)
			.await?)
	}
}
