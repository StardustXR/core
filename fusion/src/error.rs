use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
	#[error("Some verified handle wasn't owned by the server")]
	InvalidHandle,
	#[error("Gluon error: {0}")]
	GluonError(#[from] gluon::SendError),
}
