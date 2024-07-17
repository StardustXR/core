pub mod interfaces;
pub mod object_registry;

use zbus::{conn, fdo::ObjectManager, proxy::Builder, Connection, Result};

pub async fn connect_client() -> zbus::Result<zbus::Connection> {
	conn::Builder::session()?
		.serve_at("/", ObjectManager)?
		.name("org.stardustxr.Object.o".to_string() + &nanoid::nanoid!())?
		.build()
		.await
}
