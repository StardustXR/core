//! Spatial query system

use crate::{client::Client, error::ServerError};
use stardust_xr_protocol::{
	client::ClientHandler, field::Field, query::QueryableError, spatial::Spatial,
};
pub use stardust_xr_protocol::{query::QueryableObject, spatial::SpatialRef, spatial_query::*};

pub trait QueryExt {
	fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: Spatial,
		field: Field,
	) -> impl std::future::Future<Output = Result<Result<QueryableObject, QueryableError>, ServerError>>
	+ Send;
}
impl QueryExt for QueryableObject {
	async fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: Spatial,
		field: Field,
	) -> Result<Result<QueryableObject, QueryableError>, ServerError> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.query_interface()
			.register_queryable(spatial, field)
			.await?)
	}
}
