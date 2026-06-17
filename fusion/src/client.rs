//! Your connection to the Stardust server and other essentials.

pub use stardust_xr_protocol::client::{ClientHandler, FrameInfo};

use crate::error::Error;
use gluon::Object;
use pion_binder::PionBinderDevice;
use stardust_xr_protocol::{
	audio::AudioInterface,
	client::Client as ProtocolClient,
	dir::find_pion_file,
	dmatex::DmatexInterface,
	field::FieldInterface,
	lines::LinesInterface,
	model::ModelInterface,
	query::QueryInterface,
	server::{Server, ServerInterface},
	sky::SkyInterface,
	spatial::{SpatialInterface, SpatialRef},
	spatial_query::SpatialQueryInterface,
	text::TextInterface,
};
use std::{env, fs, path::Path, sync::Arc};
use tokio::sync::broadcast;

#[macro_export]
macro_rules! project_local_resources {
	($relative_path:expr) => {
		std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join($relative_path)
	};
}

/// Your connection to the Stardust server.
pub struct Client<H: ClientHandler> {
	pion_dev: PionBinderDevice,
	handler: Object<H>,
	server: Server,
	root: SpatialRef,
	server_interface: ServerInterface,
	spatial_interface: SpatialInterface,
	field_interface: FieldInterface,
	dmatex_interface: DmatexInterface,
	text_interface: TextInterface,
	model_interface: ModelInterface,
	lines_interface: LinesInterface,
	sky_interface: SkyInterface,
	audio_interface: AudioInterface,
	query_interface: QueryInterface,
	spatial_query_interface: SpatialQueryInterface,
}

impl Client<DefaultHandler> {
	pub async fn auto_connect(resource_prefixes: &[&Path]) -> Result<(Self, SpatialRef), Error> {
		let dev = PionBinderDevice::default();
		Self::manual_connect(&dev, resource_prefixes).await
	}
	pub async fn manual_connect(
		pion_device: &PionBinderDevice,
		resource_prefixes: &[&Path],
	) -> Result<(Self, SpatialRef), Error> {
		// TODO: do proper checks to make sure this is actually a server interface
		let handler = pion_device.register_object(DefaultHandler {
			frame_sender: broadcast::channel(8).0,
		});
		Self::manual_connect_with_handler(pion_device, handler, resource_prefixes).await
	}
	pub fn frame_receiver(&self) -> broadcast::Receiver<FrameInfo> {
		self.handler().frame_sender.subscribe()
	}
}

impl<H: ClientHandler> Client<H> {
	pub async fn auto_connect_with_handler(
		handler: Object<H>,
		resource_prefixes: &[&Path],
	) -> Result<(Self, SpatialRef), Error> {
		let dev = PionBinderDevice::default();
		Self::manual_connect_with_handler(&dev, handler, resource_prefixes).await
	}

	pub async fn manual_connect_with_handler(
		pion_device: &PionBinderDevice,
		handler: Object<H>,
		resource_prefixes: &[&Path],
	) -> Result<(Client<H>, SpatialRef), Error> {
		let server_path = find_pion_file("stardust-server").ok_or(Error::NoServerFile)?;

		let paths = resource_prefixes
			.iter()
			.map(|p| p.to_string_lossy().to_string());
		let runtime_prefixes = std::env::var("STARDUST_RES_PREFIXES").ok();
		let env_prefixes = runtime_prefixes
			.as_deref()
			.or(option_env!("STARDUST_RES_PREFIXES"))
			.into_iter()
			.flat_map(|f| f.split(':'))
			.map(|p| p.to_string());

		let prefixes = env_prefixes.chain(paths).collect::<Vec<String>>();

		let file = fs::OpenOptions::new()
			.read(true)
			.write(true)
			.create(false)
			.open(&server_path)
			.map_err(Error::PionFile)?;
		let interface = pion_device
			.get_binder_ref_from_file(file)
			.await
			.map_err(|_| Error::ConnectionFailure)?;
		// TODO: do proper checks to make sure this is actually a server interface
		let server_interface = ServerInterface::from_object_or_ref(interface);
		let client = ProtocolClient::from_handler(&handler);
		let state_token = env::var("STARDUST_STARTUP_TOKEN").ok();
		let (server, root) = server_interface
			.connect(client, state_token, prefixes)
			.await
			.map_err(Error::Gluon)?;
		Ok((
			Client {
				pion_dev: pion_device.clone(),
				root: root.clone(),
				handler,
				spatial_interface: server.spatial_interface().await?,
				field_interface: server.field_interface().await?,
				dmatex_interface: server.dmatex_interface().await?,
				text_interface: server.text_interface().await?,
				model_interface: server.model_interface().await?,
				lines_interface: server.lines_interface().await?,
				sky_interface: server.sky_interface().await?,
				audio_interface: server.audio_interface().await?,
				query_interface: server.query_interface().await?,
				spatial_query_interface: server.spatial_query_interface().await?,
				server,
				server_interface,
			},
			root,
		))
	}

	/// The root spatial reference, positioned where the client was spawned.
	pub fn root(&self) -> &SpatialRef {
		&self.root
	}

	/// The server proxy for direct access to server methods.
	pub fn server(&self) -> &Server {
		&self.server
	}
	/// Get a SpatialRef for a specific startup token
	pub async fn startup_token_spatial(&self, token: impl Into<String>) -> Option<SpatialRef> {
		self.server_interface
			.startup_spatial(token)
			.await
			.ok()
			.flatten()
	}

	pub fn pion_device(&self) -> &PionBinderDevice {
		&self.pion_dev
	}

	pub fn handler(&self) -> &Arc<H> {
		self.handler.handler_arc()
	}

	// --- Interface accessors (cached) ---

	pub fn spatial_interface(&self) -> &SpatialInterface {
		&self.spatial_interface
	}

	pub fn field_interface(&self) -> &FieldInterface {
		&self.field_interface
	}

	pub fn dmatex_interface(&self) -> &DmatexInterface {
		&self.dmatex_interface
	}

	pub fn text_interface(&self) -> &TextInterface {
		&self.text_interface
	}

	pub fn model_interface(&self) -> &ModelInterface {
		&self.model_interface
	}

	pub fn lines_interface(&self) -> &LinesInterface {
		&self.lines_interface
	}

	pub fn sky_interface(&self) -> &SkyInterface {
		&self.sky_interface
	}

	pub fn audio_interface(&self) -> &AudioInterface {
		&self.audio_interface
	}

	pub fn query_interface(&self) -> &QueryInterface {
		&self.query_interface
	}

	pub fn spatial_query_interface(&self) -> &SpatialQueryInterface {
		&self.spatial_query_interface
	}
}

#[derive(Debug, gluon::Handler)]
pub struct DefaultHandler {
	frame_sender: broadcast::Sender<FrameInfo>,
}
impl ClientHandler for DefaultHandler {
	async fn frame(&self, _ctx: gluon::Context, info: FrameInfo) {
		_ = self.frame_sender.send(info);
	}
}
