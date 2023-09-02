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
pub trait Scenegraph {
	fn send_signal(
		&self,
		path: &str,
		method: &str,
		data: &[u8],
		fds: Vec<OwnedFd>,
	) -> Result<(), ScenegraphError>;
	fn execute_method(
		&self,
		path: &str,
		method: &str,
		data: &[u8],
		fds: Vec<OwnedFd>,
		response: oneshot::Sender<Result<(Vec<u8>, Vec<OwnedFd>), ScenegraphError>>,
	);
}
