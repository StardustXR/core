use dirs::runtime_dir;
use std::io::{Error, ErrorKind};
use tokio::net::UnixStream;

/// Connect to the first available stardust server, opening a Tokio UnixStream to its socket.
pub async fn connect() -> Result<UnixStream, std::io::Error> {
	// Is here so if you launch a stardust client from another stardust client, and somehow errored your way
	// into an invalid value, it resolves it somehow
	let stardust_instance: u8 = std::env::var("STARDUST_INSTANCE")
		.ok()
		.and_then(|s| s.parse::<u8>().ok())
		.unwrap_or(0);
	std::env::set_var("STARDUST_INSTANCE", stardust_instance.to_string());

	// Tries to connect the client to the server.
	let socket_path = runtime_dir()
		.ok_or_else(|| Error::from(ErrorKind::AddrNotAvailable))?
		.join(format!("stardust-{stardust_instance}"));
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
