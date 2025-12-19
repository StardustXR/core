pub mod interfaces;
pub mod object_registry;
#[macro_use]
pub mod query;
pub mod list_query;
pub mod query_builder;

pub use query_builder::{ObjectEvent, QueryStream, ObjectEventStreamExt, ObjectListExt, FilteredObjectListExt, WatchHandle, AbortOnDrop};

pub use zbus;

use zbus::{
	Connection, Proxy, Result, conn,
	fdo::ObjectManager,
	names::{InterfaceName, OwnedBusName},
	proxy::Defaults,
	zvariant::OwnedObjectPath,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectInfo {
	pub bus_name: OwnedBusName,
	pub object_path: OwnedObjectPath,
}
impl ObjectInfo {
	pub async fn to_proxy(
		&self,
		conn: &Connection,
		interface: impl TryInto<InterfaceName<'static>, Error = zbus::names::Error>,
	) -> Result<Proxy<'static>> {
		Proxy::new(
			conn,
			self.bus_name.clone(),
			self.object_path.clone(),
			interface,
		)
		.await
	}
	pub async fn to_typed_proxy<P: From<Proxy<'static>> + Defaults + 'static>(
		&self,
		conn: &Connection,
	) -> Result<P> {
		Ok(self
			.to_proxy(conn, P::INTERFACE.as_ref().unwrap().to_string())
			.await?
			.into())
	}
}
