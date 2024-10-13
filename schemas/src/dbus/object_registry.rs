use futures_util::StreamExt;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use tokio::sync::watch;
use zbus::names::{BusName, InterfaceName, OwnedBusName, OwnedInterfaceName};
use zbus::proxy::ProxyDefault;
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
	pub async fn to_typed_proxy<P: From<Proxy<'static>> + ProxyDefault + 'static>(
		&self,
		conn: &Connection,
	) -> Result<P> {
		Ok(self
			.to_proxy(conn, P::INTERFACE.unwrap().to_string())
			.await?
			.into())
	}
}

pub type Objects = HashMap<OwnedInterfaceName, HashSet<ObjectInfo>>;
pub struct ObjectRegistry {
	connection: Connection,
	objects_tx: watch::Sender<Objects>,
	objects_rx: watch::Receiver<Objects>,
}
impl ObjectRegistry {
	pub async fn new(connection: &Connection) -> Result<Self> {
		let (objects_tx, objects_rx) = watch::channel(HashMap::new());

		let registry = Self {
			connection: connection.clone(),
			objects_tx,
			objects_rx,
		};

		registry.refresh_all().await?;
		registry.start_background_task();

		Ok(registry)
	}

	pub async fn refresh_all(&self) -> Result<()> {
		let proxy = fdo::DBusProxy::new(&self.connection).await?;
		let names = proxy.list_names().await?;

		let mut objects = HashMap::new();

		for name in names {
			Self::add_objects_for_name(&self.connection, name.inner().clone(), &mut objects).await;
		}

		let _ = self.objects_tx.send(objects);
		Ok(())
	}

	fn start_background_task(&self) {
		let connection_clone = self.connection.clone();
		let objects_tx = self.objects_tx.clone();
		let objects_rx = self.objects_rx.clone();

		tokio::spawn(async move {
			let dbus_proxy = fdo::DBusProxy::new(&connection_clone).await.unwrap();
			let mut name_owner_changed_stream =
				dbus_proxy.receive_name_owner_changed().await.unwrap();

			while let Some(signal) = name_owner_changed_stream.next().await {
				let args = signal.args().unwrap();
				let name = &args.name;
				if matches!(&args.name, BusName::WellKnown(_)) {
					continue;
				}
				let old_owner = args.old_owner.as_ref();
				let new_owner = args.new_owner.as_ref();

				let mut objects = objects_rx.borrow().clone();
				if old_owner.is_none() && new_owner.is_some() {
					// New bus appeared
					Self::add_objects_for_name(&connection_clone, name.clone(), &mut objects).await;
				} else if old_owner.is_some() && new_owner.is_none() {
					// Bus disappeared
					Self::remove_objects_for_bus(&mut objects, name.clone());
				}
				let _ = objects_tx.send(objects);
			}
		});
	}

	async fn add_objects_for_name(
		connection: &Connection,
		name: BusName<'_>,
		objects: &mut HashMap<OwnedInterfaceName, HashSet<ObjectInfo>>,
	) {
		let Ok(object_manager) =
			fdo::ObjectManagerProxy::new(connection, name.to_owned(), "/").await
		else {
			return;
		};

		let Ok(Ok(managed_objects)) = tokio::time::timeout(
			Duration::from_millis(1),
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
						bus_name: name.clone().into(),
						object_path: path.clone(),
					});
			}
		}
	}

	fn remove_objects_for_bus(
		objects: &mut HashMap<OwnedInterfaceName, HashSet<ObjectInfo>>,
		bus_name: BusName<'_>,
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
	let registry_connection = Connection::session().await?;
	let registry = ObjectRegistry::new(&registry_connection).await?;

	// Start monitoring for changes
	let mut watch = registry.get_watch().clone();
	let monitor_task = tokio::spawn(async move {
		while watch.changed().await.is_ok() {
			println!("Objects updated: {:?}", watch.borrow().clone());
		}
	});

	// Set up test service
	let test_connection = zbus::ConnectionBuilder::session()?
		.name("org.stardustxr.Object.TestService")?
		.serve_at("/", zbus::fdo::ObjectManager)?
		.serve_at("/org/example/TestObject", TestInterface)?
		.build()
		.await?;

	// Wait for service to register and verify presence
	tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
	assert!(registry_contains_test_service(&registry));

	// Simulate service disappearance and verify removal
	drop(test_connection);
	tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
	assert!(!registry_contains_test_service(&registry));

	// Clean up
	monitor_task.abort();

	Ok(())
}
