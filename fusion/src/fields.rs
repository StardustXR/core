//! Analog SDFs to define boundaries for input, interaction, and behavior.

pub use stardust_xr_protocol::field::*;

use crate::{Result, client::Client};
use stardust_xr_protocol::{client::ClientHandler, spatial::Spatial};

pub trait FieldExt {
	fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		shape: Shape,
	) -> impl std::future::Future<Output = Result<(Field, FieldRef)>> + Send;
}
impl FieldExt for Field {
	async fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		shape: Shape,
	) -> Result<(Field, FieldRef)> {
		let created = client
			.field_interface()
			.create_field(spatial.clone(), shape)
			.await??;
		Ok((created.field, created.field_ref))
	}
}
