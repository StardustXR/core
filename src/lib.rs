mod client;
//mod flex;
mod messenger;
mod scenegraph;

#[cfg(test)]
mod tests {
	#[test]
	fn connect() {
		let socket = super::client::connect().expect("Socket not connected");
		let peer_addr = socket.peer_addr().expect("Couldn't get peer address");
		println!("Socket peer address is {}", peer_addr.as_pathname().unwrap().to_str().unwrap());
	}
}

