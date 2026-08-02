use std::{fs, future};

use crate::{Error, Result};
use gluon::Handler;
pub use stardust_xr_protocol::tracked::*;
use stardust_xr_protocol::{
	client::ClientHandler, dir, spatial::SpatialRef, suis::Chirality, types::ResourceLoadError,
};

use crate::client::Client;

pub trait TrackedExt {
	fn hmd(
		client: &Client<impl ClientHandler>,
	) -> impl std::future::Future<Output = Result<Tracked>> + Send;
	fn hmd_spatial(
		client: &Client<impl ClientHandler>,
	) -> impl std::future::Future<Output = Result<SpatialRef>> + Send;
	fn stage(
		client: &Client<impl ClientHandler>,
	) -> impl std::future::Future<Output = Result<Tracked>> + Send;
	fn stage_spatial(
		client: &Client<impl ClientHandler>,
	) -> impl std::future::Future<Output = Result<SpatialRef>> + Send;
	fn hand(
		client: &Client<impl ClientHandler>,
		chirality: Chirality,
	) -> impl std::future::Future<Output = Result<Tracked>> + Send;
	fn controller(
		client: &Client<impl ClientHandler>,
		chirality: Chirality,
	) -> impl std::future::Future<Output = Result<Tracked>> + Send;
}
impl TrackedExt for Tracked {
	fn hmd(client: &Client<impl ClientHandler>) -> impl Future<Output = Result<Tracked>> {
		get_tracked(client, "stardust-hmd")
	}
	fn stage(client: &Client<impl ClientHandler>) -> impl Future<Output = Result<Tracked>> {
		get_tracked(client, "stardust-stage")
	}
	fn hand(
		client: &Client<impl ClientHandler>,
		chirality: Chirality,
	) -> impl Future<Output = Result<Tracked>> {
		get_tracked(
			client,
			match chirality {
				Chirality::Left => "stardust-hand/left",
				Chirality::Right => "stardust-hand/right",
			},
		)
	}
	fn controller(
		client: &Client<impl ClientHandler>,
		chirality: Chirality,
	) -> impl Future<Output = Result<Tracked>> {
		get_tracked(
			client,
			match chirality {
				Chirality::Left => "stardust-controller/left",
				Chirality::Right => "stardust-controller/right",
			},
		)
	}

	fn hmd_spatial(
		client: &Client<impl ClientHandler>,
	) -> impl std::future::Future<Output = Result<SpatialRef>> + Send {
		get_tracked_spatial(client, "stardust-hmd")
	}

	fn stage_spatial(
		client: &Client<impl ClientHandler>,
	) -> impl std::future::Future<Output = Result<SpatialRef>> + Send {
		get_tracked_spatial(client, "stardust-stage")
	}
}

async fn get_tracked(client: &Client<impl ClientHandler>, name: &str) -> Result<Tracked> {
	// completely incorrect error, but there isn't really a better one
	let path = dir::find_pion_file(name).ok_or(ResourceLoadError::NotFound)?;
	let file = fs::OpenOptions::new()
		.read(true)
		.write(true)
		.create(false)
		.open(&path)
		.map_err(Error::PionFile)?;
	let handle = client
		.pion_device()
		.get_binder_ref_from_file(file)
		.await
		// even more incorrect error, but there isn't really a better one
		.map_err(|_| ResourceLoadError::InvalidRef)?;
	// TODO: do proper checks to make sure this is actually a tracked
	Ok(Tracked::from_object_or_ref(handle))
}

async fn get_tracked_spatial(
	client: &Client<impl ClientHandler>,
	name: &str,
) -> Result<SpatialRef> {
	let tracked = get_tracked(client, name).await?;
	let handler = client
		.pion_device()
		.register_object(TrackedHandlerNoop)
		.to_service();
	let (spatial, _, _) = tracked
		.get(TrackedStateReceiver::from_handler(&handler))
		.await?;
	Ok(spatial)
}

#[derive(Handler, Debug)]
struct TrackedHandlerNoop;
impl TrackedStateReceiverHandler for TrackedHandlerNoop {
	fn tracked(
		&self,
		_ctx: gluon::Context,
		_tracked: bool,
	) -> impl Future<Output = ()> + Send + Sync {
		future::ready(())
	}
}
