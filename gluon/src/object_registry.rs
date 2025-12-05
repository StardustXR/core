use crate::ObjectInfo;
use std::{
	collections::{HashMap, HashSet},
	sync::Arc,
};
use tokio::{
	sync::{broadcast, mpsc, watch},
	task::{AbortHandle, JoinSet},
};
use tokio_stream::{StreamExt, StreamMap, wrappers::UnboundedReceiverStream};
use zbus::{
	Connection, Result, fdo,
	names::{BusName, OwnedBusName, OwnedInterfaceName},
};

#[derive(Clone, Debug)]
pub struct ObjectEvent {
	pub object: ObjectInfo,
	pub interfaces: Vec<OwnedInterfaceName>,
	pub added: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Objects {
	pub interface_to_objects: HashMap<OwnedInterfaceName, HashSet<ObjectInfo>>,
	pub object_to_interfaces: HashMap<ObjectInfo, HashSet<OwnedInterfaceName>>,
}
pub struct ObjectRegistry {
	connection: Connection,
	objects_rx: watch::Receiver<Objects>,
	object_change_events_tx: broadcast::Sender<ObjectEvent>,
	bus_task_abort: AbortHandle,
	interface_task_abort: AbortHandle,
}
impl ObjectRegistry {
	pub async fn new(connection: &Connection) -> Arc<Self> {
		// Initialize with empty objects - the update_task will populate them
		let (objects_tx, objects_rx) = watch::channel(Objects::default());
		let (object_change_events_tx, _) = broadcast::channel(256);

		// Create MPSC channels
		let (service_appear_tx, service_appear_rx) = mpsc::unbounded_channel();
		let (service_disappear_tx, service_disappear_rx) = mpsc::unbounded_channel();

		// Send initial services
		if let Ok(names) = Self::get_bus_names(connection).await {
			for name in names {
				service_appear_tx.send(name).unwrap();
			}
		}

		// Spawn service event task
		let bus_task_abort = tokio::spawn(Self::bus_task(
			connection.clone(),
			service_appear_tx.clone(),
			service_disappear_tx.clone(),
		))
		.abort_handle();

		// Spawn interface event task
		let interface_task_abort = tokio::spawn(Self::interface_event_task(
			connection.clone(),
			service_appear_rx.into(),
			service_disappear_rx.into(),
			object_change_events_tx.clone(),
			objects_tx,
		))
		.abort_handle();

		Arc::new(ObjectRegistry {
			connection: connection.clone(),
			objects_rx,
			object_change_events_tx,
			bus_task_abort,
			interface_task_abort,
		})
	}

	/// Spawns a task to handle service lifecycle events via MPSC
	async fn bus_task(
		connection: Connection,
		service_appear_tx: mpsc::UnboundedSender<OwnedBusName>,
		service_disappear_tx: mpsc::UnboundedSender<OwnedBusName>,
	) {
		// Monitor name owner changes
		let Ok(dbus_proxy) = fdo::DBusProxy::new(&connection).await else {
			return;
		};
		let Ok(mut name_stream) = dbus_proxy.receive_name_owner_changed().await else {
			return;
		};

		while let Some(signal) = name_stream.next().await {
			let Ok(args) = signal.args() else {
				continue;
			};
			if matches!(&args.name, BusName::WellKnown(_)) {
				continue;
			}

			let name: OwnedBusName = args.name.clone().into();
			let old_owner = args.old_owner.as_ref();
			let new_owner = args.new_owner.as_ref();

			if old_owner.is_none() && new_owner.is_some() {
				let _ = service_appear_tx.send(name);
			} else if old_owner.is_some() && new_owner.is_none() {
				let _ = service_disappear_tx.send(name);
			} else {
				continue;
			};
		}
	}

	/// Task that manages ObjectManager interface streams and sends events
	async fn interface_event_task(
		connection: Connection,
		mut service_appear_rx: UnboundedReceiverStream<OwnedBusName>,
		mut service_disappear_rx: UnboundedReceiverStream<OwnedBusName>,
		object_event_tx: broadcast::Sender<ObjectEvent>,
		objects_tx: watch::Sender<Objects>,
	) {
		let mut service_added_joinset = JoinSet::new();
		let mut interfaces_added_streams = StreamMap::new();
		let mut interfaces_removed_streams = StreamMap::new();

		loop {
			tokio::select! {
				// Handle service lifecycle events
				Some(name) = service_appear_rx.next() => {
					let connection = connection.clone();
					service_added_joinset.spawn(tokio::time::timeout(std::time::Duration::from_millis(500), async move {
						// Try to create ObjectManager proxy
						let object_manager =
							fdo::ObjectManagerProxy::new(&connection, name.to_owned(), "/")
								.await
								.ok()?;

						let initial_objects = object_manager.get_managed_objects().await.ok()?;

						let interfaces_added_stream = object_manager
							.receive_interfaces_added()
							.await
							.ok()?;
						let interfaces_removed_stream = object_manager
							.receive_interfaces_removed()
							.await
							.ok()?;

						let initial_object_stream = tokio_stream::iter(initial_objects.into_iter().filter_map({
							let name = name.clone();
							move |(path, interfaces)| {
								if interfaces.is_empty() {
									return None;
								}
								Some(ObjectEvent {
									object: ObjectInfo {
										bus_name: name.clone(),
										object_path: path.clone(),
									},
									interfaces: interfaces.into_keys().collect(),
									added: true,
								})
							}
						}));

						let interfaces_added = interfaces_added_stream
							.filter_map({
								let name = name.clone();
								move |iface| {
									let args = iface.args().ok()?;
									if args.interfaces_and_properties().is_empty() {
										return None;
									}

									let object = ObjectInfo {
										bus_name: name.clone(),
										object_path: args.object_path.clone().into(),
									};
									let interfaces = args
										.interfaces_and_properties()
										.keys()
										.map(|n| n.to_owned().into())
										.collect();
									Some(ObjectEvent {
										object,
										interfaces,
										added: true,
									})
								}
							});
						let interfaces_removed = interfaces_removed_stream
							.filter_map({
								let name = name.clone();
								move |iface| {
									let args = iface.args().ok()?;
									if args.interfaces().is_empty() {
										return None;
									}

									let object = ObjectInfo {
										bus_name: name.clone(),
										object_path: args.object_path.clone().into(),
									};

									let interfaces = args
										.interfaces
										.iter()
										.map(|n| n.to_owned().into())
										.collect();
									Some(ObjectEvent {
										object,
										interfaces,
										added: false,
									})
								}
							});

						Some((
							name,
							initial_object_stream.merge(interfaces_added),
							interfaces_removed,
						))
					}));
				}
				// Some(Ok(Ok(Some((name, added_stream, removed_stream))))) = service_added_joinset.join_next() => {
				Some(service) = service_added_joinset.join_next() => {
					let Ok(service) = service else {
						continue;
					};
					let Ok(service) = service else {
						continue;
					};
					let Some((name, added_stream, removed_stream)) = service else {
						continue;
					};
					interfaces_added_streams.insert(name.clone(), added_stream);
					interfaces_removed_streams.insert(name.clone(), removed_stream);
				}
				Some(name) = service_disappear_rx.next() => {
					// Remove streams for this service
					interfaces_added_streams.remove(&name);
					interfaces_removed_streams.remove(&name);

					objects_tx.send_if_modified(|objects| {
						let mut changed = false;
					objects.object_to_interfaces.retain(|k, v| {
						let should_keep = k.bus_name != name;
						if !should_keep {
							let _ = object_event_tx.send(ObjectEvent { object: k.clone(), interfaces: v.drain().collect(), added: false });
							changed |= true;
						}
						should_keep
					});
						objects.interface_to_objects.retain(|_, v| {
							v.retain(|obj| {
								let result = obj.bus_name != name;
								changed |= !result;
								result
							});

							!v.is_empty()
						});
						changed
					});
				}
				// Handle InterfacesAdded events
				Some((_, object_event)) = interfaces_added_streams.next() => {
					objects_tx.send_if_modified(|objects| {
						let mut changed = false;
						for interface in &object_event.interfaces {
							if objects
								.interface_to_objects
								.entry(interface.clone())
								.or_default()
								.insert(object_event.object.clone())
							{
								changed = true;
							}
						}
						objects
							.object_to_interfaces
							.entry(object_event.object.clone())
							.or_default()
							.extend(object_event.interfaces.iter().cloned());
						changed
					});
					let _ = object_event_tx.send(object_event);
				}

				// Handle InterfacesRemoved events
				Some((_, object_event)) = interfaces_removed_streams.next() => {
					objects_tx.send_if_modified(|objects| {
						let mut changed = false;
						for interface in &object_event.interfaces {
							if let Some(object_set) =
								objects.interface_to_objects.get_mut(interface)
								&& object_set.remove(&object_event.object)
							{
								changed = true;
							}
						}
						if let Some(interface_set) = objects.object_to_interfaces.get_mut(&object_event.object) {
							for interface in &object_event.interfaces {
								interface_set.remove(interface);
							}
						}
						changed
					});
					let _ = object_event_tx.send(object_event);
				}
			}
		}
	}

	async fn get_bus_names(connection: &Connection) -> Result<impl Iterator<Item = OwnedBusName>> {
		let proxy = fdo::DBusProxy::new(connection).await?;
		Ok(proxy
			.list_names()
			.await?
			.into_iter()
			.filter(|n| !matches!(n.as_ref(), BusName::WellKnown(_))))
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
	pub fn get_object_events_receiver(&self) -> broadcast::Receiver<ObjectEvent> {
		self.object_change_events_tx.subscribe()
	}
	pub fn get_connection(&self) -> &zbus::Connection {
		&self.connection
	}
}
impl Drop for ObjectRegistry {
	fn drop(&mut self) {
		self.bus_task_abort.abort();
		self.interface_task_abort.abort();
	}
}

#[tokio::test]
async fn object_registry_test() {
	use std::time::Duration;

	// Set up test interface
	struct TestInterface;
	#[zbus::interface(name = "org.stardustxr.ObjectRegistry.TestInterface")]
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
					.any(|obj| obj.object_path.as_str() == "/org/stardustxr/TestObject")
			})
	}

	println!("Set up test service");
	let test_connection = zbus::connection::Builder::session()
		.unwrap()
		.serve_at("/", zbus::fdo::ObjectManager)
		.unwrap()
		.serve_at("/org/stardustxr/TestObject", TestInterface)
		.unwrap()
		.build()
		.await
		.unwrap();

	println!("Set up connections and registry");
	let registry_connection = super::connect_client().await.unwrap();
	let registry = ObjectRegistry::new(&registry_connection).await;

	println!("Start monitoring for changes");
	let mut watch = registry.get_watch().clone();

	println!(
		"Test connection unique name: {:?}",
		test_connection.unique_name()
	);

	tokio::task::spawn(async {
		tokio::time::sleep(Duration::from_secs(10)).await;
		panic!("Took too long to run");
	});

	while !registry_contains_test_service(&registry) {
		println!("Wait for service to register and verify presence");
		watch.changed().await.unwrap();
	}
	println!("Objects updated: {:#?}", watch.borrow().clone());

	println!("Simulate service disappearance and verify removal");
	drop(test_connection);
	while registry_contains_test_service(&registry) {
		println!("Wait for service to stop being here");
		watch.changed().await.unwrap();
	}
	println!("Objects updated: {:#?}", watch.borrow().clone());
	assert!(!registry_contains_test_service(&registry));
}
