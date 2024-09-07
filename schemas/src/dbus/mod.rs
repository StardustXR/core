pub mod interfaces;
pub mod object_registry;

use zbus::{conn, fdo::ObjectManager, proxy::Builder, Connection, Result};

pub async fn connect_client() -> zbus::Result<zbus::Connection> {
	conn::Builder::session()?
		.serve_at("/", ObjectManager)?
		.build()
		.await
}

#[tokio::test]
async fn connect_client_test() {
	for _ in 0..32 {
		let _ = connect_client().await.unwrap();
	}
}
