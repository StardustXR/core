//! Spatial query system

use crate::{client::Client, error::ServerError};
use stardust_xr_protocol::{client::ClientHandler, field::FieldRef};
pub use stardust_xr_protocol::{query::QueryableObject, spatial::SpatialRef, spatial_query::*};

pub trait QueryExt {
	fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: SpatialRef,
		field: FieldRef,
	) -> impl std::future::Future<Output = Result<QueryableObject, ServerError>> + Send;
}
impl QueryExt for QueryableObject {
	async fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: SpatialRef,
		field: FieldRef,
	) -> Result<QueryableObject, ServerError> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.query_interface()
			.register_queryable(spatial, field)
			.await?)
	}
}
