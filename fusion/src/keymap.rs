use std::{ffi::CString, fs::File, io::Write, str::FromStr};

use gluon::Ref;
use rustix::fs::MemfdFlags;
pub use stardust_xr_protocol::keymap::*;
use stardust_xr_protocol::{dir, types::ResourceLoadError};

pub trait KeymapStoreExt {
	fn connect() -> impl std::future::Future<Output = crate::Result<KeymapStore>> + Send;
	fn exchange_string(
		&self,
		keymap: &str,
	) -> impl std::future::Future<Output = Option<Result<Keymap, KeymapExchangeError>>> + Send;
}
impl KeymapStoreExt for KeymapStore {
	async fn connect() -> crate::Result<KeymapStore> {
		// completely incorrect error, but there isn't really a better one
		let path =
			dir::find_ref_file("stardust-keymap-store").ok_or(ResourceLoadError::NotFound)?;
		let handle = Ref::connect(path)
			.await
			// even more incorrect error, but there isn't really a better one
			.map_err(|_| ResourceLoadError::InvalidRef)?;
		// TODO: do proper checks to make sure this is actually a tracked
		Ok(KeymapStore::from_ref(handle))
	}

	async fn exchange_string(&self, keymap: &str) -> Option<Result<Keymap, KeymapExchangeError>> {
		let mem = rustix::fs::memfd_create("stardust-fusion-keymap", MemfdFlags::CLOEXEC)
			.inspect_err(|err| tracing::error!("failed to create keymap memfd: {err}"))
			.ok()?;
		let mut mem = File::from(mem);
		let keymap = CString::from_str(keymap).ok()?;
		mem.set_len(keymap.as_bytes_with_nul().len() as u64).ok()?;
		mem.write_all(keymap.as_bytes_with_nul()).ok()?;
		self.exchange(XkbcommonKeymapFd {
			fd: mem.into(),
			size: keymap.as_bytes_with_nul().len() as u32,
		})
		.await
		.ok()
	}
}
