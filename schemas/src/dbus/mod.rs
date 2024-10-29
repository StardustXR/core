pub mod interfaces;
pub mod object_registry;

use zbus::{
	conn, fdo::ObjectManager, proxy::Builder, zvariant::OwnedObjectPath, Connection, Result,
};

pub async fn connect_client() -> zbus::Result<zbus::Connection> {
	conn::Builder::session()?
		.serve_at("/", ObjectManager)?
		.build()
		.await
}

pub fn random_object_name() -> OwnedObjectPath {
	OwnedObjectPath::try_from(format!(
		"/{}",
		nanoid::nanoid!(
			32,
			&[
				'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
				'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
			]
		)
	))
	.unwrap()
}

#[tokio::test]
async fn connect_client_test() {
	for _ in 0..32 {
		let _ = connect_client().await.unwrap();
	}
}
