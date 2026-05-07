use stardust_xr_fusion::{client::Client, project_local_resources};
use tokio::sync::broadcast::error::RecvError;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	tracing_subscriber::fmt::init();
	let (client, _) = Client::auto_connect(&[&project_local_resources!("res")])
		.await
		.unwrap();

	let mut frame_recv = client.frame_receiver();
	let info = match frame_recv.recv().await {
		Ok(v) => v,
		Err(RecvError::Lagged(n)) => {
			panic!("lost {n} frame events");
		}
		Err(RecvError::Closed) => {
			panic!("Client dropped?? somehow?");
		}
	};
	println!("Frame info: {info:?}");
}
