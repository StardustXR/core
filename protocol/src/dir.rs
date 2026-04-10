use std::fs::{self, File};
use std::path::{Path, PathBuf};

fn lock_path(dir: &Path) -> PathBuf {
	let mut lock = dir.as_os_str().to_owned();
	lock.push(".lock");
	lock.into()
}

/// Check if a lockfile exists and is actively held by another process.
fn is_locked(dir: &Path) -> bool {
	let lock = lock_path(dir);
	let Ok(file) = File::open(&lock) else {
		return false;
	};
	// If we can't get the lock, someone else holds it — the service is alive.
	file.try_lock().is_err()
}

/// Try to create/open the lockfile and acquire an exclusive lock.
/// Returns the held `File` on success (caller must keep it alive).
fn acquire_lock(dir: &Path) -> Option<File> {
	let lock = lock_path(dir);
	let file = File::create(&lock).ok()?;
	file.try_lock().ok()?;
	Some(file)
}

/// Find the pion path for Stardust XR related services.
/// First check if a file at $XDG_RUNTIME_DIR/{service}/$STARDUST_INSTANCE exists and isn't lockable
/// else try $XDG_RUNTIME_DIR/{service}/stardust-0
pub fn find_pion_file(service: &str) -> Option<PathBuf> {
	let service_dir = xdg::BaseDirectories::new().runtime_dir?.join(service);

	if let Ok(instance) = std::env::var("STARDUST_INSTANCE") {
		let file = service_dir.join(&instance);
		if file.is_file() && is_locked(&file) {
			return Some(file);
		}
	}

	let file = service_dir.join("stardust-0");
	if file.is_file() && is_locked(&file) {
		return Some(file);
	}

	None
}

/// Make a runtime directory for Stardust XR and hold its lockfile.
/// First check if a file at $XDG_RUNTIME_DIR/{service}/$STARDUST_INSTANCE exists that has an inert lockfile next to it
/// then try to make it if it doesn't exist, if it does increment up to $XDG_RUNTIME_DIR/stardust-0 and try again over and over until you find a free one and make the directory.
///
/// Returns the path and the held lockfile. Drop the `File` to release the lock.
pub fn create_pion_file(service: &str, instance: &str) -> Option<(PathBuf, File)> {
	let service_dir = xdg::BaseDirectories::new().runtime_dir?.join(service);
	if !service_dir.is_dir() {
		fs::create_dir_all(&service_dir).ok()?;
	}

	let file = service_dir.join(instance);
	if let Some(lock) = acquire_lock(&file) {
		return Some((file, lock));
	}

	None
}

/// Find a free STARDUST_INSTANCE
/// This is done by searching for the first file at $XDG_RUNTIME_DIR/stardust-server/stardust-{n}
/// that either doesn't exist or doesn't have a valid lock
pub fn find_free_instace() -> Option<String> {
	let service_dir = xdg::BaseDirectories::new()
		.runtime_dir?
		.join("stardust-server");
	// try stardust-0, stardust-1, ...
	for i in 0u32.. {
		let instance = format!("stardust-{i}");
		let file = service_dir.join(&instance);
		if !is_locked(&file) {
			return Some(instance);
		}
	}
	None
}
