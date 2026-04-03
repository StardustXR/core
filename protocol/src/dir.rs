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
	// If we can't get the lock, someone else holds it — the server is alive.
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

/// Find the runtime directory for Stardust XR.
/// First check if a directory at $XDG_RUNTIME_DIR/$STARDUST_INSTANCE exists
/// if not, find the first `stardust-*` directory in $XDG_RUNTIME_DIR with a `stardust-*.lock` file next to it that isn't lockable.
pub fn find_runtime_dir() -> Option<PathBuf> {
	let runtime_dir = xdg::BaseDirectories::new().runtime_dir?;

	if let Ok(instance) = std::env::var("STARDUST_INSTANCE") {
		let dir = runtime_dir.join(&instance);
		if dir.is_dir() && is_locked(&dir) {
			return Some(dir);
		}
	}

	let entries = fs::read_dir(&runtime_dir).ok()?;
	for entry in entries.flatten() {
		let path = entry.path();
		if path.is_dir()
			&& entry
				.file_name()
				.to_str()
				.is_some_and(|n| n.starts_with("stardust-"))
			&& is_locked(&path)
		{
			return Some(path);
		}
	}

	None
}

/// Make a runtime directory for Stardust XR and hold its lockfile.
/// First check if a directory at $XDG_RUNTIME_DIR/$STARDUST_INSTANCE exists that has an inert lockfile next to it
/// then try to make it if it doesn't exist, if it does increment up to $XDG_RUNTIME_DIR/stardust-0 and try again over and over until you find a free one and make the directory.
///
/// Returns the path and the held lockfile. Drop the `File` to release the lock.
pub fn make_runtime_dir(instance: &str) -> Option<(PathBuf, File)> {
	let runtime_dir = xdg::BaseDirectories::new().runtime_dir?;

	let dir = runtime_dir.join(instance);
	if !dir.is_dir() {
		fs::create_dir_all(&dir).ok()?;
	}
	if let Some(lock) = acquire_lock(&dir) {
		return Some((dir, lock));
	}

	// The requested instance name is taken, try stardust-0, stardust-1, ...
	for i in 0u32.. {
		let dir = runtime_dir.join(format!("stardust-{i}"));
		if !dir.is_dir() {
			fs::create_dir_all(&dir).ok()?;
		}
		if let Some(lock) = acquire_lock(&dir) {
			return Some((dir, lock));
		}
	}

	None
}
