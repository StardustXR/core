use futures_util::StreamExt;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::sync::watch;
use tokio::task::AbortHandle;
use zbus::names::{BusName, InterfaceName, OwnedBusName, OwnedInterfaceName};
use zbus::proxy::Defaults;
use zbus::zvariant::OwnedObjectPath;
use zbus::{fdo, Connection, Proxy, Result};

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

pub type Objects = HashMap<OwnedInterfaceName, HashSet<ObjectInfo>>;
pub struct ObjectRegistry {
	connection: Connection,
	objects_tx: watch::Sender<Objects>,
	objects_rx: watch::Receiver<Objects>,
	abort_handle: AbortHandle,
}
impl ObjectRegistry {
	pub async fn new(connection: &Connection) -> Result<Self> {
		let objects = Self::get_all_objects(connection).await?;
		let (objects_tx, objects_rx) = watch::channel(objects);

		let abort_handle = tokio::spawn(Self::update_task(
			connection.clone(),
			objects_tx.clone(),
			objects_rx.clone(),
		))
		.abort_handle();

		Ok(ObjectRegistry {
			connection: connection.clone(),
			objects_tx,
			objects_rx,
			abort_handle,
		})
	}

	async fn update_task(
		connection: Connection,
		objects_tx: watch::Sender<Objects>,
		objects_rx: watch::Receiver<Objects>,
	) -> Result<()> {
		let mut bus_names: HashSet<OwnedBusName> =
			HashSet::from_iter(Self::get_bus_names(&connection).await?.into_iter());

		let dbus_proxy = fdo::DBusProxy::new(&connection).await?;
		let mut name_owner_changed_stream = dbus_proxy.receive_name_owner_changed().await?;
		let mut objects = objects_rx.borrow().clone();
		while let Some(signal) = name_owner_changed_stream.next().await {
			let mut updated = false;
			let args = signal.args().unwrap();
			let name: OwnedBusName = args.name.clone().into();
			if matches!(&args.name, BusName::WellKnown(_)) {
				continue;
			}
			// println!("updating for bus {name}");
			let old_owner = args.old_owner.as_ref();
			let new_owner = args.new_owner.as_ref();

			if old_owner.is_none() && new_owner.is_some() {
				// New bus appeared
				// println!("new bus {name} appeared");
				bus_names.insert(name.clone());
				Self::add_objects_for_name(&connection, name, &mut objects).await;
				updated = true;
			} else if old_owner.is_some() && new_owner.is_none() {
				// Bus disappeared
				// println!("bus {name} disappeared");
				Self::remove_objects_for_bus(&mut objects, name.clone());
				bus_names.remove::<OwnedBusName>(&name);
				updated = true;
			}
			if updated {
				// println!("Sending new objects {objects:?}");
				let _ = objects_tx.send(objects.clone());
			}
		}

		Ok(())
	}

	async fn get_bus_names(connection: &Connection) -> Result<Vec<OwnedBusName>> {
		let proxy = fdo::DBusProxy::new(connection).await?;
		Ok(proxy.list_names().await?)
	}

	async fn get_all_objects(
		connection: &Connection,
	) -> Result<HashMap<OwnedInterfaceName, HashSet<ObjectInfo>>> {
		let names = Self::get_bus_names(connection).await?;

		let mut objects = HashMap::new();

		for name in names {
			if matches!(name.as_ref(), BusName::WellKnown(_)) {
				continue;
			}
			Self::add_objects_for_name(connection, name, &mut objects).await;
		}

		Ok(objects)
	}

	async fn add_objects_for_name(
		connection: &Connection,
		name: OwnedBusName,
		objects: &mut HashMap<OwnedInterfaceName, HashSet<ObjectInfo>>,
	) {
		let Ok(object_manager) =
			fdo::ObjectManagerProxy::new(connection, name.to_owned(), "/").await
		else {
			return;
		};

		let Ok(Ok(managed_objects)) = tokio::time::timeout(
			Duration::from_millis(5),
			object_manager.get_managed_objects(),
		)
		.await
		else {
			return;
		};

		for (path, interfaces) in managed_objects {
			for interface in interfaces.keys() {
				objects
					.entry(interface.clone())
					.or_default()
					.insert(ObjectInfo {
						bus_name: name.clone(),
						object_path: path.clone(),
					});
			}
		}
	}

	fn remove_objects_for_bus(
		objects: &mut HashMap<OwnedInterfaceName, HashSet<ObjectInfo>>,
		bus_name: OwnedBusName,
	) {
		for object_set in objects.values_mut() {
			object_set.retain(|obj| obj.bus_name.inner() != &bus_name);
		}
	}

	pub fn get_objects(&self, interface: &OwnedInterfaceName) -> HashSet<ObjectInfo> {
		self.objects_rx
			.borrow()
			.get(interface)
			.cloned()
			.unwrap_or_default()
	}

	pub fn get_watch(&self) -> watch::Receiver<Objects> {
		self.objects_rx.clone()
	}
}

impl Drop for ObjectRegistry {
	fn drop(&mut self) {
		self.abort_handle.abort();
	}
}

#[tokio::test]
async fn object_registry_test() -> Result<()> {
	// Set up test interface
	struct TestInterface;
	#[zbus::interface(name = "org.example.TestInterface")]
	impl TestInterface {
		async fn test_method(&self) -> String {
			"Test method called".to_string()
		}
	}

	fn registry_contains_test_service(registry: &ObjectRegistry) -> bool {
		registry.get_watch().borrow().values().any(|set| {
			set.iter()
				.any(|obj| obj.object_path.as_str() == "/org/example/TestObject")
		})
	}

	// Set up connections and registry
	let registry_connection = zbus::connection::Builder::session()?
		.name("org.stardustxr.ObjectRegistry")?
		.build()
		.await?;
	let registry = ObjectRegistry::new(&registry_connection).await?;
	tokio::time::sleep(Duration::from_millis(250)).await;

	// Start monitoring for changes
	let mut watch = registry.get_watch().clone();

	// Set up test service
	let test_connection = zbus::connection::Builder::session()?
		.name("org.stardustxr.Object.TestService")?
		.serve_at("/", zbus::fdo::ObjectManager)?
		.serve_at("/org/example/TestObject", TestInterface)?
		.build()
		.await?;

	// Wait for service to register and verify presence
	watch.changed().await.unwrap();
	println!("Objects updated: {:?}", watch.borrow().clone());
	assert!(registry_contains_test_service(&registry));

	// Simulate service disappearance and verify removal
	drop(test_connection);
	watch.changed().await.unwrap();
	println!("Objects updated: {:?}", watch.borrow().clone());
	assert!(!registry_contains_test_service(&registry));

	Ok(())
}
