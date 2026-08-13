use crate::Result;
use gluon::{Handler, Ref, RefExt};
pub use stardust_xr_protocol::tracked::*;
use stardust_xr_protocol::{dir, spatial::SpatialRef, suis::Chirality, types::ResourceLoadError};

pub trait TrackedExt {
	fn hmd() -> impl std::future::Future<Output = Result<Tracked>> + Send;
	fn hmd_spatial() -> impl std::future::Future<Output = Result<SpatialRef>> + Send;
	fn stage() -> impl std::future::Future<Output = Result<Tracked>> + Send;
	fn stage_spatial() -> impl std::future::Future<Output = Result<SpatialRef>> + Send;
	fn hand(chirality: Chirality) -> impl std::future::Future<Output = Result<Tracked>> + Send;
	fn controller(
		chirality: Chirality,
	) -> impl std::future::Future<Output = Result<Tracked>> + Send;
}
impl TrackedExt for Tracked {
	fn hmd() -> impl Future<Output = Result<Tracked>> {
		get_tracked("stardust-hmd")
	}
	fn stage() -> impl Future<Output = Result<Tracked>> {
		get_tracked("stardust-stage")
	}
	fn hand(chirality: Chirality) -> impl Future<Output = Result<Tracked>> {
		get_tracked(match chirality {
			Chirality::Left => "stardust-hand/left",
			Chirality::Right => "stardust-hand/right",
		})
	}
	fn controller(chirality: Chirality) -> impl Future<Output = Result<Tracked>> {
		get_tracked(match chirality {
			Chirality::Left => "stardust-controller/left",
			Chirality::Right => "stardust-controller/right",
		})
	}

	fn hmd_spatial() -> impl std::future::Future<Output = Result<SpatialRef>> + Send {
		get_tracked_spatial("stardust-hmd")
	}

	fn stage_spatial() -> impl std::future::Future<Output = Result<SpatialRef>> + Send {
		get_tracked_spatial("stardust-stage")
	}
}

async fn get_tracked(name: &str) -> Result<Tracked> {
	// completely incorrect error, but there isn't really a better one
	let path = dir::find_ref_file(name).ok_or(ResourceLoadError::NotFound)?;
	let handle = Ref::connect(path)
		.await
		// even more incorrect error, but there isn't really a better one
		.map_err(|_| ResourceLoadError::InvalidRef)?;
	// TODO: do proper checks to make sure this is actually a tracked
	Ok(Tracked::from_ref(handle))
}

async fn get_tracked_spatial(name: &str) -> Result<SpatialRef> {
	let tracked = get_tracked(name).await?;
	let handler_ref = TrackedStateReceiver::new_service(TrackedHandlerNoop).unwrap();
	let (spatial, _, _) = tracked.get(handler_ref).await?;
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
		std::future::ready(())
	}
}
