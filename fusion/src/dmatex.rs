pub use stardust_xr_protocol::dmatex::*;

use crate::{client::Client, error::ServerError};
pub trait DmatexExt {
	fn import(
		client: &Client,
		size: DmatexSize,
		format: DmatexFormat,
		array_layers: u32,
		planes: Vec<DmatexPlane>,
		timeline_syncobj_fd: std::os::fd::OwnedFd,
	) -> impl std::future::Future<Output = Result<DmatexRef, ServerError>> + Send;
}
impl DmatexExt for DmatexRef {
	async fn import(
		client: &Client,
		size: DmatexSize,
		format: DmatexFormat,
		array_layers: u32,
		planes: Vec<DmatexPlane>,
		timeline_syncobj_fd: std::os::fd::OwnedFd,
	) -> Result<DmatexRef, ServerError> {
		// should this just return a Gluon error or something like that? there aren't an
		// SpatialRef handles or anything that could be wrong, unless you count the timeline
		// syncobj fd
		Ok(client
			.dmatex_interface()
			.import_dmatex(size, format, array_layers, planes, timeline_syncobj_fd)
			.await?)
	}
}
