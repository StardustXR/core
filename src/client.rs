use mio::net::UnixStream;
use std::io::{Error, ErrorKind};

pub fn connect() -> Result<UnixStream, std::io::Error> {
	// Get the base XDG directories
	let xdg_dirs = xdg::BaseDirectories::new().unwrap();

	// Is here so if you launch a stardust client from another stardust client, and somehow errored your way
	// into an invalid value, it resolves it somehow
	let stardust_instance: u8 = std::env::var("STARDUST_INSTANCE")
		.map(|s| s.parse::<u8>().unwrap_or(0))
		.unwrap_or(0);
	std::env::set_var("STARDUST_INSTANCE", stardust_instance.to_string());

	// Tries to connect the client to the server.
	let socket_path = format!(
		"{}/stardust-{}",
		xdg_dirs
			.get_runtime_directory()
			.unwrap()
			.to_str()
			.ok_or(Error::from(ErrorKind::AddrNotAvailable))?,
		stardust_instance
	);
	UnixStream::connect(socket_path)
}

#[test]
fn test_connect() {
	let socket = super::client::connect().expect("Socket not connected");
	let peer_addr = socket.peer_addr().expect("Couldn't get peer address");
	println!(
		"Socket peer address is {}",
		peer_addr.as_pathname().unwrap().to_str().unwrap()
	);
}
