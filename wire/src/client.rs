use dirs::runtime_dir;
use std::{
	io::{Error, ErrorKind},
	path::PathBuf,
	str::FromStr,
};
use tokio::net::UnixStream;

/// Connect to the first available stardust server, opening a Tokio UnixStream to its socket.
pub async fn connect() -> Result<UnixStream, std::io::Error> {
	let mut socket_path = std::env::var_os("STARDUST_INSTANCE")
		.map(PathBuf::from)
		.unwrap_or_else(|| PathBuf::from_str("stardust-0").unwrap());
	if !socket_path.has_root() {
		socket_path = runtime_dir()
		.ok_or_else(|| Error::from(ErrorKind::AddrNotAvailable))?
		.join(socket_path);
	}

	// Tries to connect the client to the server.
	UnixStream::connect(socket_path).await
}

#[tokio::test]
async fn client_connect() {
	let socket = super::client::connect()
		.await
		.expect("Socket not connected");
	let peer_addr = socket.peer_addr().expect("Couldn't get peer address");
	println!(
		"Socket peer address is {}",
		peer_addr.as_pathname().unwrap().to_str().unwrap()
	);
}
