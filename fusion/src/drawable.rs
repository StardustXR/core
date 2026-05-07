//! Anything the user can see such as lines, models and text.

#![allow(ambiguous_glob_reexports)]

use stardust_xr_protocol::client::ClientHandler;
pub use stardust_xr_protocol::lines::*;
pub use stardust_xr_protocol::model::*;
pub use stardust_xr_protocol::sky::*;
pub use stardust_xr_protocol::text::*;

use stardust_xr_protocol::spatial::Spatial;
use stardust_xr_protocol::types::Resource;
use stardust_xr_protocol::types::Vec3F;

use crate::{client::Client, error::ServerError};

pub trait LinesExt {
	fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		lines: Vec<Line>,
	) -> impl std::future::Future<Output = Result<Lines, ServerError>> + Send;
}
impl LinesExt for Lines {
	async fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		lines: Vec<Line>,
	) -> Result<Lines, ServerError> {
		Ok(client
			.lines_interface()
			.create_lines(spatial.clone(), lines)
			.await?)
	}
}

pub trait ModelExt {
	fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		model: Resource,
		model_scale: impl Into<Vec3F> + Send,
	) -> impl std::future::Future<Output = Result<Model, ServerError>> + Send;
}
impl ModelExt for Model {
	async fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		model: Resource,
		model_scale: impl Into<Vec3F> + Send,
	) -> Result<Model, ServerError> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.model_interface()
			.load_model(spatial.clone(), model, model_scale.into())
			.await?)
	}
}

pub trait TextExt {
	fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		text: String,
		style: TextStyle,
	) -> impl std::future::Future<Output = Result<Text, ServerError>> + Send;
}
impl TextExt for Text {
	async fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		text: String,
		style: TextStyle,
	) -> Result<Text, ServerError> {
		// TODO: actually handle invalid handles at the protocol level
		Ok(client
			.text_interface()
			.create_text(spatial.clone(), text, style)
			.await?)
	}
}
