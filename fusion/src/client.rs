//! Your connection to the Stardust server and other essentials.

use binderbinder::{TransactionHandler, binder_object::BinderObject, payload::PayloadBuilder};
use gluon_wire::{
	GluonCtx, GluonDataBuilder, GluonDataReader, GluonSendError, drop_tracking::DropNotifier,
};
use pion_binder::PionBinderDevice;
use stardust_xr_protocol::{
	audio::AudioInterface, client::{Client as ProtocolClient, ClientHandler, ClientState, FrameInfo}, dir::find_pion_file, dmatex::DmatexInterface, field::FieldInterface, lines::LinesInterface, model::ModelInterface, server::{Server, ServerInterface}, sky::SkyInterface, spatial::{SpatialInterface, SpatialRef}, spatial_query::SpatialQueryInterface, text::TextInterface
};
use std::{fs, path::Path, sync::Arc};
use thiserror::Error;
use tokio::sync::broadcast;
use tracing::error;

#[macro_export]
macro_rules! project_local_resources {
	($relative_path:expr) => {
		std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join($relative_path)
	};
}

#[derive(Error, Debug)]
pub enum ClientError {
	#[error("Unable to open servers pion file: {0}")]
	PionFileError(std::io::Error),
	#[error("Could not find the stardust server instance")]
	NoServerFile,
	#[error("Could not connect to the stardust server")]
	ConnectionFailure,
	#[error("Gluon error: {0}")]
	GluonError(#[from] GluonSendError),
}

/// Your connection to the Stardust server.
pub struct Client {
	pion_dev: PionBinderDevice,
	server: Server,
	root: SpatialRef,
	client_handler: Arc<BinderObject<ClientImpl>>,
	spatial_interface: SpatialInterface,
	field_interface: FieldInterface,
	dmatex_interface: DmatexInterface,
	text_interface: TextInterface,
	model_interface: ModelInterface,
	lines_interface: LinesInterface,
	sky_interface: SkyInterface,
	audio_interface: AudioInterface,
	spatial_query_interface: SpatialQueryInterface,
}

impl Client {
	pub async fn connect(
		resource_prefixes: &[&Path],
	) -> Result<(Self, ClientState), ClientError> {
		let dev = PionBinderDevice::default();
		Self::connect_with_device(&dev, resource_prefixes).await
	}
	pub async fn connect_with_device(
		pion_device: &PionBinderDevice,
		resource_prefixes: &[&Path],
	) -> Result<(Self, ClientState), ClientError> {
		let server_path = find_pion_file("stardust-server").ok_or(ClientError::NoServerFile)?;

		let paths = resource_prefixes.iter().map(|p| p.to_string_lossy().to_string());
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
			.map_err(ClientError::PionFileError)?;
		let interface = pion_device
			.get_binder_ref_from_file(file)
			.await
			.map_err(|_| ClientError::ConnectionFailure)?;
		// TODO: do proper checks to make sure this is actually a server interface
		let server_interface = ServerInterface::from_object_or_ref(interface);
		let client_handler = pion_device.register_object(ClientImpl {
			frame_sender: broadcast::channel(8).0,
			drop_notifs: Default::default(),
		});
		let client = ProtocolClient::from_handler(&client_handler);
		let (server, initial_state) = server_interface
			.connect(client, prefixes)
			.await
			.map_err(ClientError::GluonError)?;
		let root = initial_state.root.clone();
		Ok((
			Client {
				pion_dev: pion_device.clone(),
				root,
				client_handler,
				spatial_interface: server.spatial_interface().await?,
				field_interface: server.field_interface().await?,
				dmatex_interface: server.dmatex_interface().await?,
				text_interface: server.text_interface().await?,
				model_interface: server.model_interface().await?,
				lines_interface: server.lines_interface().await?,
				sky_interface: server.sky_interface().await?,
				audio_interface: server.audio_interface().await?,
				spatial_query_interface: server.spatial_query_interface().await?,
				server,
			},
			initial_state,
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

	/// Get a receiverr for frame events, only events generated after this call will be returned
	pub fn frame_receiver(&self) -> broadcast::Receiver<FrameInfo> {
		self.client_handler.frame_sender.subscribe()
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

	pub fn spatial_query_interface(&self) -> &SpatialQueryInterface {
		&self.spatial_query_interface
	}
}

#[derive(Debug)]
struct ClientImpl {
	frame_sender: broadcast::Sender<FrameInfo>,
	drop_notifs: tokio::sync::RwLock<Vec<DropNotifier>>,
}

impl ClientHandler for ClientImpl {
	// do we maybe want to wait for something in the main code paths?
	async fn ping(&self, _ctx: GluonCtx) {}

	fn frame(&self, _ctx: GluonCtx, info: FrameInfo) {
		_ = self.frame_sender.send(info);
	}

	// TODO: figure out how to enforce a response somehow, if thats possible
	async fn get_state(&self, _ctx: GluonCtx) -> ClientState {
		todo!()
	}

	async fn drop_notification_requested(&self, notifier: DropNotifier) {
		self.drop_notifs.write().await.push(notifier);
	}
}

impl TransactionHandler for ClientImpl {
	async fn handle(&self, transaction: binderbinder::device::Transaction) -> PayloadBuilder<'_> {
		let mut gluon_data = GluonDataReader::from_payload(transaction.payload);
		self.dispatch_two_way(
			transaction.code,
			&mut gluon_data,
			GluonCtx {
				sender_pid: transaction.sender_pid,
				sender_euid: transaction.sender_euid,
			},
		)
		.await
		.inspect_err(|err| error!("failed to dispatch client transaction: {err}"))
		.unwrap_or(GluonDataBuilder::new())
		.to_payload()
	}

	async fn handle_one_way(&self, transaction: binderbinder::device::Transaction) {
		let mut gluon_data = GluonDataReader::from_payload(transaction.payload);
		_ = self
			.dispatch_one_way(
				transaction.code,
				&mut gluon_data,
				GluonCtx {
					sender_pid: transaction.sender_pid,
					sender_euid: transaction.sender_euid,
				},
			)
			.await
			.inspect_err(|err| error!("failed to dispatch client transaction: {err}"));
	}
}
