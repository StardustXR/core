//! Anything the user can see such as lines, models and text.

#![allow(ambiguous_glob_reexports)]

use stardust_xr_protocol::client::ClientHandler;
pub use stardust_xr_protocol::lines::*;
use stardust_xr_protocol::model;
pub use stardust_xr_protocol::model::*;
pub use stardust_xr_protocol::sky::*;
pub use stardust_xr_protocol::text::*;

use stardust_xr_protocol::spatial::Spatial;
use stardust_xr_protocol::types::Resource;

use crate::{client::Client, error::ServerError};
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum ModelLoadError {
	#[error("Some verified handle wasn't owned by the server")]
	NotFound,
	#[error("Spatial wasn't owned by the server")]
	InvalidSpatial,
	#[error("Gluon error: {0}")]
	GluonError(#[from] gluon::SendError),
}

pub trait ModelExt {
	fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		model: Resource,
	) -> impl std::future::Future<Output = Result<Model, ModelLoadError>> + Send;
}
impl ModelExt for Model {
	async fn new<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		model: Resource,
	) -> Result<Model, ModelLoadError> {
		client
			.model_interface()
			.load_model(spatial.clone(), model)
			.await?
			.map_err(|err| match err {
				model::ModelLoadError::NotFound => ModelLoadError::NotFound,
				model::ModelLoadError::InvalidSpatial => ModelLoadError::InvalidSpatial,
			})
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
