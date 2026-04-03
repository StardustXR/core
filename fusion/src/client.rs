//! Your connection to the Stardust server and other essentials.

use binderbinder::binder_object::BinderObject;
use gluon_wire::GluonSendError;
use stardust_xr_protocol::protocol::{
	audio::AudioInterface,
	client::{Client as ProtocolClient, ClientHandler, ClientState},
	dmatex::DmatexInterface,
	field::FieldInterface,
	lines::LinesInterface,
	model::ModelInterface,
	server::{Server, ServerInterface},
	sky::SkyInterface,
	spatial::{SpatialInterface, SpatialRef},
	spatial_query::SpatialQueryInterface,
	text::TextInterface,
};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::OnceCell;

#[derive(Error, Debug)]
pub enum ClientError {
	#[error("Could not connect to the stardust server")]
	ConnectionFailure,
	#[error("Gluon error: {0}")]
	GluonError(#[from] GluonSendError),
}

/// Your connection to the Stardust server.
pub struct StardustConnection {
	server: Server,
	root: SpatialRef,
	initial_state: ClientState,
	spatial_interface: OnceCell<SpatialInterface>,
	field_interface: OnceCell<FieldInterface>,
	dmatex_interface: OnceCell<DmatexInterface>,
	text_interface: OnceCell<TextInterface>,
	model_interface: OnceCell<ModelInterface>,
	lines_interface: OnceCell<LinesInterface>,
	sky_interface: OnceCell<SkyInterface>,
	audio_interface: OnceCell<AudioInterface>,
	spatial_query_interface: OnceCell<SpatialQueryInterface>,
}

impl StardustConnection {
	pub async fn connect<H: ClientHandler>(
		handler: Arc<BinderObject<H>>,
		resource_prefixes: Vec<String>,
	) -> Result<Self, ClientError> {
		let stardust_instance = std::env::var_os("STARDUST_INSTANCE")
			.map(|instance| {
				xdg::BaseDirectories::new()
					.runtime_dir
					.unwrap()
					.join(instance)
			})
			.unwrap_or_else(|| {
				let runtime_dir = xdg::BaseDirectories::new().runtime_dir.unwrap();
				for entry in runtime_dir.read_dir().unwrap() {
					let Ok(entry) = entry else { continue };
					if entry.file_name().starts_with("stardust-") {
						return entry.path();
					}

					return runtime_dir.join();
				}
			});

		Self::from_parts(server_interface, handler, resource_prefixes)
	}

	/// Create a client from an already-connected ServerInterface and handler.
	///
	/// The handler implements `ClientHandler` to receive frame, ping, and save_state events.
	pub async fn from_parts<H: ClientHandler>(
		server_interface: &ServerInterface,
		handler: Arc<BinderObject<H>>,
		resource_prefixes: Vec<String>,
	) -> Result<Self, ClientError> {
		let client = ProtocolClient::from_handler(&handler);
		let (server, initial_state) = server_interface
			.connect(client, resource_prefixes)
			.await
			.map_err(ClientError::GluonError)?;
		let root = initial_state.root.clone();
		Ok(StardustConnection {
			server,
			root,
			initial_state,
			spatial_interface: OnceCell::new(),
			field_interface: OnceCell::new(),
			dmatex_interface: OnceCell::new(),
			text_interface: OnceCell::new(),
			model_interface: OnceCell::new(),
			lines_interface: OnceCell::new(),
			sky_interface: OnceCell::new(),
			audio_interface: OnceCell::new(),
			spatial_query_interface: OnceCell::new(),
		})
	}

	/// The root spatial reference, positioned where the client was spawned.
	pub fn root(&self) -> &SpatialRef {
		&self.root
	}

	/// The server proxy for direct access to server methods.
	pub fn server(&self) -> &Server {
		&self.server
	}

	/// The initial client state returned on connection.
	pub fn initial_state(&self) -> &ClientState {
		&self.initial_state
	}

	// --- Interface accessors (lazily cached) ---

	pub async fn spatial_interface(&self) -> Result<&SpatialInterface, GluonSendError> {
		self.spatial_interface
			.get_or_try_init(|| self.server.spatial_interface())
			.await
	}

	pub async fn field_interface(&self) -> Result<&FieldInterface, GluonSendError> {
		self.field_interface
			.get_or_try_init(|| self.server.field_interface())
			.await
	}

	pub async fn dmatex_interface(&self) -> Result<&DmatexInterface, GluonSendError> {
		self.dmatex_interface
			.get_or_try_init(|| self.server.dmatex_interface())
			.await
	}

	pub async fn text_interface(&self) -> Result<&TextInterface, GluonSendError> {
		self.text_interface
			.get_or_try_init(|| self.server.text_interface())
			.await
	}

	pub async fn model_interface(&self) -> Result<&ModelInterface, GluonSendError> {
		self.model_interface
			.get_or_try_init(|| self.server.model_interface())
			.await
	}

	pub async fn lines_interface(&self) -> Result<&LinesInterface, GluonSendError> {
		self.lines_interface
			.get_or_try_init(|| self.server.lines_interface())
			.await
	}

	pub async fn sky_interface(&self) -> Result<&SkyInterface, GluonSendError> {
		self.sky_interface
			.get_or_try_init(|| self.server.sky_interface())
			.await
	}

	pub async fn audio_interface(&self) -> Result<&AudioInterface, GluonSendError> {
		self.audio_interface
			.get_or_try_init(|| self.server.audio_interface())
			.await
	}

	pub async fn spatial_query_interface(&self) -> Result<&SpatialQueryInterface, GluonSendError> {
		self.spatial_query_interface
			.get_or_try_init(|| self.server.spatial_query_interface())
			.await
	}
}
