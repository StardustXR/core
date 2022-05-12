use cluFlock::ExclusiveFlock;
use std::fs::{self, File};

pub fn get_free_socket_path() -> Option<String> {
	// Get the base XDG directories
	let xdg_dirs = xdg::BaseDirectories::new().ok()?;
	let runtime_dir = xdg_dirs.get_runtime_directory().ok()?.to_str()?;
	let mut socket_number = 0_u8;

	loop {
		if socket_number > 32 {
			return None;
		}
		let socket_path = format!("{}/stardust-{}", runtime_dir, socket_number);
		let socket_lock_path = format!("{}.lock", socket_path.as_str());
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
fn test_get_free_socket_path() {
	let socket_path = get_free_socket_path().expect("Unable to set up socket!");
	println!("Socket is free up at {}", socket_path);
}
