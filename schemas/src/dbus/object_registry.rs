use futures_util::StreamExt;
use std::{
	collections::{HashMap, HashSet},
	hash::Hash,
	time::Duration,
};
use tokio::{sync::watch, task::AbortHandle};
use zbus::{
	Connection, Proxy, Result, fdo,
	names::{BusName, InterfaceName, OwnedBusName, OwnedInterfaceName},
	proxy::Defaults,
	zvariant::OwnedObjectPath,
};

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

#[derive(Debug)]
pub(in crate::dbus) struct InternalBusRecord([AbortHandle; 2]);
impl InternalBusRecord {
	pub(in crate::dbus) fn new(
		name: OwnedBusName,
		object_manager: fdo::ObjectManagerProxy<'static>,
		objects_tx: watch::Sender<Objects>,
	) -> Self {
		let name_2 = name.clone();
		let object_manager_2 = object_manager.clone();
		let objects_tx_2 = objects_tx.clone();
		InternalBusRecord([
			tokio::spawn(async move {
				let Ok(mut interfaces_added_stream) =
					object_manager.clone().receive_interfaces_added().await
				else {
					return;
				};
				while let Some(interface_added) = interfaces_added_stream.next().await {
					let Ok(args) = interface_added.args() else {
						continue;
					};
					objects_tx.send_if_modified(|objects| {
						let mut changed = false;
						for interface in args.interfaces_and_properties().keys() {
							if objects.entry(interface.to_string()).or_default().insert(
								ObjectInfo {
									bus_name: name.clone(),
									object_path: args.object_path.clone().into(),
								},
							) {
								changed = true
							};
						}
						changed
					});
				}
			})
			.abort_handle(),
			tokio::spawn(async move {
				let Ok(mut interfaces_removed_stream) =
					object_manager_2.receive_interfaces_removed().await
				else {
					return;
				};
				while let Some(interface_removed) = interfaces_removed_stream.next().await {
					let Ok(args) = interface_removed.args() else {
						continue;
					};
					objects_tx_2.send_if_modified(|objects| {
						let mut changed = false;
						for interface in args.interfaces().as_ref() {
							let Some(object_interface) = objects.get_mut(&interface.to_string())
							else {
								continue;
							};

							if object_interface.remove(&ObjectInfo {
								bus_name: name_2.clone(),
								object_path: args.object_path.clone().into(),
							}) {
								changed = true;
							};
						}
						changed
					});
				}
			})
			.abort_handle(),
		])
	}
}
impl Drop for InternalBusRecord {
	fn drop(&mut self) {
		self.0[0].abort();
		self.0[1].abort();
	}
}

pub type Objects = HashMap<String, HashSet<ObjectInfo>>;
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
		let mut buses: HashMap<OwnedBusName, InternalBusRecord> = {
			let names = Self::get_bus_names(&connection).await?;
			let mut buses = HashMap::new();

			for name in names {
				let Ok(object_manager_proxy) =
					fdo::ObjectManagerProxy::new(&connection, name.clone(), "/").await
				else {
					continue;
				};

				let bus_record =
					InternalBusRecord::new(name.clone(), object_manager_proxy, objects_tx.clone());
				buses.insert(name, bus_record);
			}

			buses
		};

		let dbus_proxy = fdo::DBusProxy::new(&connection).await?;
		let mut name_owner_changed_stream = dbus_proxy.receive_name_owner_changed().await?;
		while let Some(signal) = name_owner_changed_stream.next().await {
			let mut objects = objects_rx.borrow().clone();
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
				if let Some(object_manager) =
					Self::add_objects_for_name(&connection, name.clone(), &mut objects).await
				{
					buses.insert(
						name.clone(),
						InternalBusRecord::new(name, object_manager, objects_tx.clone()),
					);
					updated = true;
				}
			} else if old_owner.is_some() && new_owner.is_none() {
				// Bus disappeared
				// println!("bus {name} disappeared");
				Self::remove_objects_for_bus(&mut objects, name.clone());
				buses.remove::<OwnedBusName>(&name);
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

	async fn get_all_objects(connection: &Connection) -> Result<Objects> {
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
		objects: &mut Objects,
	) -> Option<fdo::ObjectManagerProxy<'static>> {
		let object_manager = fdo::ObjectManagerProxy::new(connection, name.to_owned(), "/")
			.await
			.ok()?;

		let managed_objects = tokio::time::timeout(
			Duration::from_millis(5),
			object_manager.get_managed_objects(),
		)
		.await
		.ok()?
		.ok()?;

		for (path, interfaces) in managed_objects {
			for interface in interfaces.keys() {
				objects
					.entry(interface.to_string())
					.or_default()
					.insert(ObjectInfo {
						bus_name: name.clone(),
						object_path: path.clone(),
					});
			}
		}
		Some(object_manager)
	}

	fn remove_objects_for_bus(objects: &mut Objects, bus_name: OwnedBusName) {
		for object_set in objects.values_mut() {
			object_set.retain(|obj| obj.bus_name.inner() != &bus_name);
		}
	}

	pub fn get_objects(&self, interface: &str) -> HashSet<ObjectInfo> {
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
