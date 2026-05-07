//! Analog SDFs to define boundaries for input, interaction, and behavior.

pub use stardust_xr_protocol::field::*;
use stardust_xr_protocol::{client::ClientHandler, spatial::Spatial};

use crate::{client::Client, error::ServerError};

pub trait FieldExt {
	fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		shape: Shape,
	) -> impl std::future::Future<Output = Result<Field, ServerError>> + Send;
}
impl FieldExt for Field {
	async fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		shape: Shape,
	) -> Result<Field, ServerError> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.field_interface()
			.create_field(spatial.clone(), shape)
			.await?)
	}
}
