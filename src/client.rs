use std::os::unix::net::UnixStream;

pub fn connect() -> Option<UnixStream> {
	let xdg_dirs = xdg::BaseDirectories::new().unwrap();

	let stardust_instance: u8 = std::env::var("STARDUST_INSTANCE").map(|s| s.parse::<u8>().unwrap_or(0)).unwrap_or(0);
	std::env::set_var("STARDUST_INSTANCE", stardust_instance.to_string());

	let socket_path = format!("{}/stardust-{}", xdg_dirs.get_runtime_directory().unwrap().to_str()?, stardust_instance);
	match UnixStream::connect(socket_path) {
		Ok(sock) => Some(sock),
		Err(e) => {
			println!("Couldn't connect: {:?}", e);
			None
		},
	}
}

