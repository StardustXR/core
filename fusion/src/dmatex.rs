pub use stardust_xr_protocol::dmatex::*;

use crate::{Result, client::Client};
use stardust_xr_protocol::client::ClientHandler;

pub trait DmatexExt {
	fn import<H: ClientHandler>(
		client: &Client<H>,
		size: DmatexSize,
		format: DmatexFormat,
		array_layers: u32,
		planes: Vec<DmatexPlane>,
		timeline_syncobj_fd: std::os::fd::OwnedFd,
	) -> impl std::future::Future<Output = Result<DmatexRef>> + Send;
}
impl DmatexExt for DmatexRef {
	async fn import<H: ClientHandler>(
		client: &Client<H>,
		size: DmatexSize,
		format: DmatexFormat,
		array_layers: u32,
		planes: Vec<DmatexPlane>,
		timeline_syncobj_fd: std::os::fd::OwnedFd,
	) -> Result<DmatexRef> {
		Ok(client
			.dmatex_interface()
			.import_dmatex(size, format, array_layers, planes, timeline_syncobj_fd)
			.await??)
	}
}
