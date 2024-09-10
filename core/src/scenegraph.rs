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
	#[error("Signal not found")]
	SignalNotFound,
	#[error("Method not found")]
	MethodNotFound,
	#[error("Signal error: {error}")]
	SignalError { error: String },
	#[error("Method error: {error}")]
	MethodError { error: String },
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
