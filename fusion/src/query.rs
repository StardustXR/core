//! Spatial query system

use gluon::Interface;
pub use stardust_xr_protocol::query::*;

use crate::{Result, client::Client};
use stardust_xr_protocol::{client::ClientHandler, field::Field, spatial::Spatial};

pub trait QueryableExt {
	fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: Spatial,
		field: Field,
	) -> impl std::future::Future<Output = Result<QueryableObject>> + Send;

	fn add_interface<I: Interface>(
		&self,
		interface: &I,
	) -> impl std::future::Future<Output = Result<QueryableInterfaceGuard>> + Send;
}
impl QueryableExt for QueryableObject {
	async fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: Spatial,
		field: Field,
	) -> Result<QueryableObject> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.query_interface()
			.register_queryable(spatial, field)
			.await??)
	}

	async fn add_interface<I: Interface>(&self, interface: &I) -> Result<QueryableInterfaceGuard> {
		Ok(self.add_interface(interface, I::ID).await?)
	}
}
