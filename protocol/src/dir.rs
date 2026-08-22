use std::fs::{self, File};
use std::os::unix::fs::FileTypeExt;
use std::path::{Path, PathBuf};

/// Is there a listening socket at this path?
///
/// Since gluon, a service's pion path *is* the socket clients connect to, rather than a
/// regular file with a ref written into it. Checking the type rather than mere existence
/// is what stops a leftover regular file from an older server being handed back as
/// something connectable.
fn is_socket(path: &Path) -> bool {
	fs::metadata(path).is_ok_and(|meta| meta.file_type().is_socket())
}

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

/// Find the socket path for Stardust XR related services.
/// First check if a socket at $XDG_RUNTIME_DIR/{service}/$STARDUST_INSTANCE exists and isn't lockable
/// else try $XDG_RUNTIME_DIR/{service}/stardust-0
///
/// Both conditions earn their keep: the lock says a server is *alive*, and the socket type
/// says there is something there to connect to. A crashed server leaves the socket behind
/// with the lock released, and an old enough one leaves a regular file.
pub fn find_ref_file(service: &str) -> Option<PathBuf> {
	let service_dir = xdg::BaseDirectories::new().runtime_dir?.join(service);

	if let Ok(instance) = std::env::var("STARDUST_INSTANCE") {
		let socket = service_dir.join(&instance);
		if is_socket(&socket) && is_locked(&socket) {
			return Some(socket);
		}
	}

	let socket = service_dir.join("stardust-0");
	if is_socket(&socket) && is_locked(&socket) {
		return Some(socket);
	}

	None
}

/// Claim $XDG_RUNTIME_DIR/{service}/{instance} for this process to listen on.
///
/// Makes the service directory if it isn't there, then takes the lockfile beside the path.
/// Failing to take the lock means another process holds this instance, and the caller
/// should pick a different one — see [`find_free_instace`].
///
/// Clears anything already sitting at the path, which the lock makes safe: holding it means
/// nothing else is alive here, so whatever is there is a socket a crashed predecessor left
/// behind. Binding refuses to clobber an existing path — correctly, since it cannot know
/// that — so this is where the corpse gets cleared.
///
/// Returns the path to bind and the held lockfile. Drop the `File` to release the lock.
pub fn create_server_file(service: &str, instance: &str) -> Option<(PathBuf, File)> {
	let service_dir = xdg::BaseDirectories::new().runtime_dir?.join(service);
	if !service_dir.is_dir() {
		fs::create_dir_all(&service_dir).ok()?;
	}

	let path = service_dir.join(instance);
	let lock = acquire_lock(&path)?;
	let _ = fs::remove_file(&path);
	Some((path, lock))
}

/// The path $XDG_RUNTIME_DIR/{service}/{instance} for a service to listen on, creating the
/// service directory if it isn't there.
///
/// Unlike [`create_server_file`] this takes no lock and clears nothing: it is for callers
/// whose binding layer owns the lockfile itself (`strong_ipc::RefFsBinding` does), where
/// taking it here as well would deadlock the process against its own flock — flock is per
/// open file description, so a second handle on the same lock in the same process is still
/// a conflict.
pub fn server_file_path(service: &str, instance: &str) -> Option<PathBuf> {
	let service_dir = xdg::BaseDirectories::new().runtime_dir?.join(service);
	if !service_dir.is_dir() {
		fs::create_dir_all(&service_dir).ok()?;
	}
	Some(service_dir.join(instance))
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
