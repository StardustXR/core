use cluFlock::ExclusiveFlock;
use std::{fs, fs::File, os::unix::net::UnixListener};

pub fn setup_socket() -> Option<(UnixListener, String)> {
	// Get the base XDG directories
	let xdg_dirs = xdg::BaseDirectories::new().ok()?;
	let runtime_dir = xdg_dirs.get_runtime_directory().ok()?.to_str()?;
	let mut socket_number = 0_u8;

	loop {
		if socket_number > 32 {
			return None;
		}
		let socket_path = format!("{}/stardust-{}", runtime_dir, socket_number);
		let socket_lock_path = format!("{}.lock", socket_path);
		match File::create(socket_lock_path) {
			Err(_) => continue,
			Ok(file) => match file.try_lock() {
				Err(_) => socket_number += 1,
				Ok(_) => {
					fs::remove_file(socket_path.clone()).ok()?;
					return Some((UnixListener::bind(socket_path.clone()).ok()?, socket_path));
				}
			},
		}
	}
}

#[test]
fn test_setup_socket() {
	let (_socket_listener, socket_path) = setup_socket().expect("Unable to set up socket!");
	println!("Socket is set up at {}", socket_path);
}
