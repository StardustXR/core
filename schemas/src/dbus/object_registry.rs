use futures_util::StreamExt;
use std::{
	collections::{HashMap, HashSet},
	hash::Hash,
	sync::Arc,
	time::Duration,
};
use tokio::{
	sync::{Notify, broadcast, watch},
	task::AbortHandle,
};
use zbus::{
	Connection, Proxy, Result, fdo,
	names::{BusName, InterfaceName, OwnedBusName, OwnedInterfaceName},
	proxy::Defaults,
	zvariant::OwnedObjectPath,
};

use crate::dbus::ObjectInfo;

#[derive(Debug)]
struct InternalBusRecord([AbortHandle; 2]);
impl InternalBusRecord {
	fn new(
		name: OwnedBusName,
		object_manager: fdo::ObjectManagerProxy<'static>,
		objects_tx: watch::Sender<Objects>,
		changed_tx: broadcast::Sender<Vec<ObjectInfo>>,
	) -> Self {
		let name_2 = name.clone();
		let object_manager_2 = object_manager.clone();
		let objects_tx_2 = objects_tx.clone();
		let changed_tx_2 = changed_tx.clone();
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
					let obj = ObjectInfo {
						bus_name: name.clone(),
						object_path: args.object_path.clone().into(),
					};
					objects_tx.send_if_modified(|objects| {
						let mut changed = false;
						for interface in args.interfaces_and_properties().keys() {
							if objects
								.interface_to_objects
								.entry(interface.to_string())
								.or_default()
								.insert(obj.clone())
							{
								changed = true
							};
						}
						objects
							.object_to_interfaces
							.entry(obj.clone())
							.or_default()
							.extend(
								args.interfaces_and_properties()
									.keys()
									.map(|i| i.to_string()),
							);
						changed
					});
					_ = changed_tx.send(vec![obj]);
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
					let obj = ObjectInfo {
						bus_name: name_2.clone(),
						object_path: args.object_path.clone().into(),
					};
					objects_tx_2.send_if_modified(|objects| {
						let mut changed = false;
						for interface in args.interfaces().as_ref() {
							let Some(object_interface) =
								objects.interface_to_objects.get_mut(&interface.to_string())
							else {
								continue;
							};

							if object_interface.remove(&obj) {
								changed = true;
							};
						}
						changed
					});
					_ = changed_tx_2.send(vec![obj]);
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

#[derive(Clone, Debug, Default)]
pub struct Objects {
	pub interface_to_objects: HashMap<String, HashSet<ObjectInfo>>,
	pub object_to_interfaces: HashMap<ObjectInfo, HashSet<String>>,
}
pub struct ObjectRegistry {
	connection: Connection,
	objects_tx: watch::Sender<Objects>,
	objects_rx: watch::Receiver<Objects>,
	changed_objects: broadcast::Sender<Vec<ObjectInfo>>,
	abort_handle: AbortHandle,
}
impl ObjectRegistry {
	pub async fn new(connection: &Connection) -> Result<Arc<Self>> {
		let objects = Self::get_all_objects(connection).await?;
		let (objects_tx, objects_rx) = watch::channel(objects);
		let (changed_tx, _) = broadcast::channel(16);

		let abort_handle = tokio::spawn(Self::update_task(
			connection.clone(),
			objects_tx.clone(),
			objects_rx.clone(),
			changed_tx.clone(),
		))
		.abort_handle();

		Ok(Arc::new(ObjectRegistry {
			connection: connection.clone(),
			objects_tx,
			objects_rx,
			changed_objects: changed_tx,
			abort_handle,
		}))
	}

	async fn update_task(
		connection: Connection,
		objects_tx: watch::Sender<Objects>,
		objects_rx: watch::Receiver<Objects>,
		changed_tx: broadcast::Sender<Vec<ObjectInfo>>,
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

				let bus_record = InternalBusRecord::new(
					name.clone(),
					object_manager_proxy,
					objects_tx.clone(),
					changed_tx.clone(),
				);
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
				if let Some(object_manager) = Self::add_objects_for_name(
					&connection,
					name.clone(),
					Some(changed_tx.clone()),
					&mut objects,
				)
				.await
				{
					buses.insert(
						name.clone(),
						InternalBusRecord::new(
							name,
							object_manager,
							objects_tx.clone(),
							changed_tx.clone(),
						),
					);
					updated = true;
				}
			} else if old_owner.is_some() && new_owner.is_none() {
				// Bus disappeared
				// println!("bus {name} disappeared");
				Self::remove_objects_for_bus(&mut objects, name.clone(), changed_tx.clone());
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

	async fn get_bus_names(connection: &Connection) -> Result<impl Iterator<Item = OwnedBusName>> {
		let proxy = fdo::DBusProxy::new(connection).await?;
		Ok(proxy
			.list_names()
			.await?
			.into_iter()
			.filter(|n| !matches!(n.as_ref(), BusName::WellKnown(_))))
	}

	async fn get_all_objects(connection: &Connection) -> Result<Objects> {
		let names = Self::get_bus_names(connection).await?;

		let mut objects = Objects::default();

		for name in names {
			if matches!(name.as_ref(), BusName::WellKnown(_)) {
				continue;
			}
			Self::add_objects_for_name(connection, name, None, &mut objects).await;
		}

		Ok(objects)
	}

	async fn add_objects_for_name(
		connection: &Connection,
		name: OwnedBusName,
		changed_tx: Option<broadcast::Sender<Vec<ObjectInfo>>>,
		objects: &mut Objects,
	) -> Option<fdo::ObjectManagerProxy<'static>> {
		let object_manager = fdo::ObjectManagerProxy::new(connection, name.to_owned(), "/")
			.await
			.ok()?;

		let managed_objects = tokio::time::timeout(
			Duration::from_millis(50),
			object_manager.get_managed_objects(),
		)
		.await
		.ok()?
		.ok()?;
		let mut objs = Vec::with_capacity(managed_objects.len());
		for (path, interfaces) in managed_objects {
			let obj = ObjectInfo {
				bus_name: name.clone(),
				object_path: path.clone(),
			};
			for interface in interfaces.keys() {
				objects
					.interface_to_objects
					.entry(interface.to_string())
					.or_default()
					.insert(obj.clone());
			}
			objects
				.object_to_interfaces
				.entry(obj.clone())
				.or_default()
				.extend(interfaces.keys().map(|i| i.to_string()));
			objs.push(obj);
		}
		if let Some(changed_tx) = changed_tx {
			_ = changed_tx.send(objs);
		}
		Some(object_manager)
	}

	fn remove_objects_for_bus(
		objects: &mut Objects,
		bus_name: OwnedBusName,
		changed_tx: broadcast::Sender<Vec<ObjectInfo>>,
	) {
		for object_set in objects.interface_to_objects.values_mut() {
			object_set.retain(|obj| obj.bus_name.inner() != &bus_name);
		}
		let objs = objects
			.object_to_interfaces
			.keys()
			.filter(|obj| obj.bus_name.inner() == &bus_name)
			.cloned()
			.collect();
		objects
			.object_to_interfaces
			.retain(|obj, _| obj.bus_name.inner() != &bus_name);
		_ = changed_tx.send(objs);
	}

	pub fn get_objects(&self, interface: &str) -> HashSet<ObjectInfo> {
		self.objects_rx
			.borrow()
			.interface_to_objects
			.get(interface)
			.cloned()
			.unwrap_or_default()
	}

	pub fn get_watch(&self) -> watch::Receiver<Objects> {
		self.objects_rx.clone()
	}
	pub fn get_object_changed_receiver(&self) -> broadcast::Receiver<Vec<ObjectInfo>> {
		self.changed_objects.subscribe()
	}
	pub fn get_connection(&self) -> &zbus::Connection {
		&self.connection
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
		registry
			.get_watch()
			.borrow()
			.interface_to_objects
			.values()
			.any(|set| {
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
	// why is this here? why does it need to exist? shouldn't the object registry create await be
	// enough?
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
