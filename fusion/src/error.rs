use stardust_xr_protocol::{
	dmatex::DmatexImportError,
	query::QueryableError,
	types::{CreateError, ResourceLoadError},
};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Unable to open servers pion file: {0}")]
	PionFile(std::io::Error),
	#[error("Could not find the stardust server instance")]
	NoServerFile,
	#[error("Could not connect to the stardust server")]
	ConnectionFailure,
	#[error("Gluon error: {0}")]
	Gluon(#[from] gluon::SendError),

	#[error("Create error: {0}")]
	Create(#[from] CreateError),
	#[error("Dmatex import error: {0}")]
	DmatexImport(#[from] DmatexImportError),
	#[error("Resource loading error: {0}")]
	ResourceLoad(#[from] ResourceLoadError),
	#[error("Queryable error: {0}")]
	Queryable(#[from] QueryableError),
}
