use cluFlock::ExclusiveFlock;
use dirs::runtime_dir;
use std::{
	fs::{self, File},
	path::PathBuf,
};

/// Get the lowest numbered socket path not taken by another server, if available.
pub fn get_free_socket_path() -> Option<PathBuf> {
	// Get the base XDG directories
	let runtime_dir = runtime_dir()?;
	let mut socket_number = 0_u8;

	loop {
		if socket_number > 32 {
			return None;
		}
		let socket_path = runtime_dir.join(format!("stardust-{socket_number}"));
		let socket_lock_path = runtime_dir.join(format!("stardust-{socket_number}.lock"));
		match File::create(socket_lock_path) {
			Err(_) => continue,
			Ok(file) => match file.try_lock() {
				Err(_) => socket_number += 1,
				Ok(_) => match fs::remove_file(socket_path.clone()) {
					Ok(_) => return Some(socket_path),
					Err(err) => match err.kind() {
						std::io::ErrorKind::NotFound => return Some(socket_path),
						_ => continue,
					},
				},
			},
		}
	}
}

#[test]
fn server_get_free_socket_path() {
	let socket_path = get_free_socket_path().expect("Unable to set up socket!");
	println!("Socket is free up at {}", socket_path.display());
}
