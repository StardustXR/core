use stardust_xr_protocol::{
	dmatex::DmatexImportError,
	model::MaterialParamError,
	query::QueryableError,
	types::{CreateError, ResourceLoadError},
};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("Could not find the stardust server instance")]
	NoServerFile,
	#[error("Could not connect to the stardust server")]
	ConnectionFailure,

	#[error("Gluon send error: {0}")]
	GluonSend(#[from] gluon::SendError),
	#[error("Gluon read error: {0}")]
	GluonRead(#[from] gluon::ReadError),
	#[error("Gluon node error: {0}")]
	GluonNode(#[from] gluon::NodeError),

	#[error("Create error: {0}")]
	Create(#[from] CreateError),
	#[error("Dmatex import error: {0}")]
	DmatexImport(#[from] DmatexImportError),
	#[error("Material param error: {0}")]
	MaterialParam(#[from] MaterialParamError),
	#[error("Resource loading error: {0}")]
	ResourceLoad(#[from] ResourceLoadError),
	#[error("Queryable error: {0}")]
	Queryable(#[from] QueryableError),
}
