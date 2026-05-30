use stardust_xr_protocol::client::ClientHandler;
pub use stardust_xr_protocol::dmatex::*;

use crate::{client::Client, error::ServerError};
pub trait DmatexExt {
	fn import<H: ClientHandler>(
		client: &Client<H>,
		size: DmatexSize,
		format: DmatexFormat,
		array_layers: u32,
		planes: Vec<DmatexPlane>,
		timeline_syncobj_fd: std::os::fd::OwnedFd,
	) -> impl std::future::Future<Output = Result<Result<DmatexRef, DmatexImportError>, ServerError>>
	+ Send;
}
impl DmatexExt for DmatexRef {
	async fn import<H: ClientHandler>(
		client: &Client<H>,
		size: DmatexSize,
		format: DmatexFormat,
		array_layers: u32,
		planes: Vec<DmatexPlane>,
		timeline_syncobj_fd: std::os::fd::OwnedFd,
	) -> Result<Result<DmatexRef, DmatexImportError>, ServerError> {
		Ok(client
			.dmatex_interface()
			.import_dmatex(size, format, array_layers, planes, timeline_syncobj_fd)
			.await?)
	}
}
