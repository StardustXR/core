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
use tokio_stream::StreamMap;
use zbus::{
	Connection, Proxy, Result, fdo,
	names::{BusName, InterfaceName, OwnedBusName, OwnedInterfaceName},
	proxy::Defaults,
	zvariant::OwnedObjectPath,
};

use crate::dbus::ObjectInfo;

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
		// Initialize with empty objects - the update_task will populate them
		let (objects_tx, objects_rx) = watch::channel(Objects::default());
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
		_objects_rx: watch::Receiver<Objects>,
		changed_tx: broadcast::Sender<Vec<ObjectInfo>>,
	) -> Result<()> {
		// Set up D-Bus name change monitoring
		let dbus_proxy = fdo::DBusProxy::new(&connection).await?;
		let mut name_owner_changed_stream = dbus_proxy.receive_name_owner_changed().await?;

		// Initialize objects and two StreamMaps
		let mut objects = Objects::default();
		let mut interfaces_added_streams = StreamMap::new();
		let mut interfaces_removed_streams = StreamMap::new();

		// Scan existing services and add streams for those with ObjectManager
		let names = Self::get_bus_names(&connection).await?;
		for name in names {
			if let Some(object_manager) = Self::add_objects_for_name(
				&connection,
				name.clone(),
				Some(changed_tx.clone()),
				&mut objects,
			)
			.await
			{
				if let Ok(added_stream) = object_manager.receive_interfaces_added().await {
					interfaces_added_streams.insert(name.clone(), added_stream);
				}
				if let Ok(removed_stream) = object_manager.receive_interfaces_removed().await {
					interfaces_removed_streams.insert(name, removed_stream);
				}
			}
		}

		// Send initial objects state
		let _ = objects_tx.send(objects.clone());

		// Main event loop
		loop {
			tokio::select! {
				// Handle service lifecycle (NameOwnerChanged)
				Some(signal) = name_owner_changed_stream.next() => {
					let Ok(args) = signal.args() else { continue; };
					let name: OwnedBusName = args.name.clone().into();
					if matches!(&args.name, BusName::WellKnown(_)) {
						continue;
					}

					let old_owner = args.old_owner.as_ref();
					let new_owner = args.new_owner.as_ref();

					if old_owner.is_none() && new_owner.is_some() {
						// New service appeared
						if let Some(object_manager) = Self::add_objects_for_name(
							&connection,
							name.clone(),
							Some(changed_tx.clone()),
							&mut objects,
						)
						.await
						{
							if let Ok(added_stream) = object_manager.receive_interfaces_added().await {
								interfaces_added_streams.insert(name.clone(), added_stream);
							}
							if let Ok(removed_stream) = object_manager.receive_interfaces_removed().await {
								interfaces_removed_streams.insert(name.clone(), removed_stream);
							}
							let _ = objects_tx.send(objects.clone());
						}
					} else if old_owner.is_some() && new_owner.is_none() {
						// Service disappeared
						Self::remove_objects_for_bus(&mut objects, name.clone(), changed_tx.clone());
						interfaces_added_streams.remove(&name);
						interfaces_removed_streams.remove(&name);
						let _ = objects_tx.send(objects.clone());
					}
				}

				// Handle InterfacesAdded events
				Some((service_name, signal)) = interfaces_added_streams.next() => {
					if let Ok(args) = signal.args() {
						let obj = ObjectInfo {
							bus_name: service_name,
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
				}

				// Handle InterfacesRemoved events
				Some((service_name, signal)) = interfaces_removed_streams.next() => {
					if let Ok(args) = signal.args() {
						let obj = ObjectInfo {
							bus_name: service_name,
							object_path: args.object_path.clone().into(),
						};
						objects_tx.send_if_modified(|objects| {
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
						_ = changed_tx.send(vec![obj]);
					}
				}

				else => break,
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
