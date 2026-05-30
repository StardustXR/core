//! Anything the user can see such as lines, models and text.

#![allow(ambiguous_glob_reexports)]

pub use stardust_xr_protocol::lines::*;
pub use stardust_xr_protocol::model::*;
pub use stardust_xr_protocol::sky::*;
pub use stardust_xr_protocol::text::*;

use crate::{Result, client::Client};
use stardust_xr_protocol::{client::ClientHandler, spatial::Spatial, types::Resource};

pub trait LinesExt {
	fn create<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		lines: Vec<Line>,
	) -> impl std::future::Future<Output = Result<Lines>> + Send;
}
impl LinesExt for Lines {
	async fn create<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		lines: Vec<Line>,
	) -> Result<Lines> {
		Ok(client
			.lines_interface()
			.create_lines(spatial.clone(), lines)
			.await??)
	}
}

pub trait ModelExt {
	fn create<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		model: Resource,
	) -> impl std::future::Future<Output = Result<Model>> + Send;
}
impl ModelExt for Model {
	async fn create<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		model: Resource,
	) -> Result<Model> {
		Ok(client
			.model_interface()
			.load_model(spatial.clone(), model)
			.await??)
	}
}

pub trait TextExt {
	fn create<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		text: String,
		style: TextStyle,
	) -> impl std::future::Future<Output = Result<Text>> + Send;
}
impl TextExt for Text {
	async fn create<H: ClientHandler>(
		client: &Client<H>,
		spatial: &Spatial,
		text: String,
		style: TextStyle,
	) -> Result<Text> {
		Ok(client
			.text_interface()
			.create_text(spatial.clone(), text, style)
			.await??)
	}
}
