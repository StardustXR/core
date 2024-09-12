use color_eyre::Report;
use stardust_xr_schemas::flex::flexbuffers::DeserializationError;
use std::os::unix::io::OwnedFd;
use thiserror::Error;
use tokio::sync::oneshot;

/// Error for all scenegraph-related things.
#[derive(Error, Debug)]
pub enum ScenegraphError {
	#[error("Node not found")]
	NodeNotFound,
	#[error("Alias has broken")]
	BrokenAlias,
	#[error("Aspect not found")]
	AspectNotFound,
	#[error("Signal/method not found")]
	MemberNotFound,
	#[error("Signal/method error: {error}")]
	MemberError { error: String },
}
impl From<DeserializationError> for ScenegraphError {
	fn from(value: DeserializationError) -> Self {
		Self::MemberError {
			error: format!("Deserialization error: {value:?}"),
		}
	}
}
impl From<Report> for ScenegraphError {
	fn from(value: Report) -> Self {
		Self::MemberError {
			error: value.to_string(),
		}
	}
}

/// Handles node signals and method calls for the messenger.
pub type MethodResponse = oneshot::Sender<Result<(Vec<u8>, Vec<OwnedFd>), ScenegraphError>>;
pub trait Scenegraph {
	fn send_signal(
		&self,
		node_id: u64,
		aspect: u64,
		method: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), ScenegraphError>;
	fn execute_method(
		&self,
		node_id: u64,
		aspect: u64,
		method: u64,
		data: &[u8],
		fds: Vec<OwnedFd>,
		response: MethodResponse,
	);
}
