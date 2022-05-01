pub mod client;
pub mod flex;
pub mod fusion;
pub mod messenger;
pub mod scenegraph;

#[rustfmt::skip]
mod schemas;

#[cfg(test)]
mod tests {
	#[test]
	fn connect() {
		let socket = super::client::connect().expect("Socket not connected");
		let peer_addr = socket.peer_addr().expect("Couldn't get peer address");
		println!(
			"Socket peer address is {}",
			peer_addr.as_pathname().unwrap().to_str().unwrap()
		);
	}

	#[test]
	fn create_text() {
		let socket = super::client::connect().expect("Socket not connected");
		let mut messenger = super::messenger::Messenger::new(socket);
		messenger
			.send_signal(
				"/drawable",
				"createText",
				super::flex::flexbuffer_from_arguments(|fbb| {
					let mut vec = fbb.start_vector();
					vec.push("rustytext");
					vec.push("/");
					let mut pos = vec.start_vector();
					pos.push(0_f32);
					pos.push(0_f32);
					pos.push(0_f32);
					pos.end_vector();
					let mut rot = vec.start_vector();
					rot.push(0_f32);
					rot.push(0_f32);
					rot.push(0_f32);
					rot.push(1_f32);
					rot.end_vector();
					vec.push("Rusty Text :D");
					vec.push("");
					vec.push(0.1_f32);
					vec.push(((1 << 1) | (1 << 2)) as u8);
					let mut bounds = vec.start_vector();
					bounds.push(1_f32);
					bounds.push(1_f32);
					bounds.end_vector();
					vec.push((1 << 4) as u8);
					vec.push(((1 << 1) | (1 << 2)) as u8);
					let mut color = vec.start_vector();
					color.push(1_f32);
					color.push(1_f32);
					color.push(1_f32);
					color.push(1_f32);
					color.end_vector();
					vec.end_vector();
				})
				.as_slice(),
			)
			.expect("Message failed to send");
		std::thread::sleep(std::time::Duration::from_secs(900));
	}
}
