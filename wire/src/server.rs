use dirs::runtime_dir;
use std::{
	fs::{self, File},
	path::PathBuf,
};

pub struct LockedSocket {
	pub socket_path: PathBuf,
	_file: File,
}
impl LockedSocket {
	pub fn get_free() -> Option<Self> {
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
						Ok(_) => {
							return Some(Self {
								socket_path,
								_file: file,
							});
						}
						Err(err) => match err.kind() {
							std::io::ErrorKind::NotFound => {
								return Some(Self {
									socket_path,
									_file: file,
								});
							}
							_ => continue,
						},
					},
				},
			}
		}
	}
}

#[test]
fn server_get_free_socket_path() {
	let socket_path = LockedSocket::get_free().expect("Unable to set up socket!");
	println!("Socket is free up at {}", socket_path.socket_path.display());
}
