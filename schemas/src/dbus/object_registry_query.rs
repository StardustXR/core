use std::{
	collections::{HashMap, HashSet},
	marker::PhantomData,
	ops::{Deref, DerefMut},
	sync::Arc,
	time::Duration,
};

use futures_util::StreamExt as _;
use tokio::{
	sync::{RwLock, RwLockReadGuard, mpsc, watch},
	task::AbortHandle,
	time::timeout,
};
use variadics_please::all_tuples;
use zbus::{
	Connection, Proxy, fdo,
	names::{BusName, OwnedBusName},
	proxy::{Defaults, ProxyImpl},
	zvariant::OwnedObjectPath,
};

use crate::dbus::object_registry::{InternalBusRecord, ObjectInfo, ObjectRegistry, Objects};

pub struct ObjectRegistryQuery<Q: ObjectRegistryQueryable> {
	update_task_handle: AbortHandle,
	callback_handle: AbortHandle,
	_phantom_data: PhantomData<Q>,
}

pub trait ObjectRegistryQueryable: Sized + 'static + Send + Sync {
	fn try_new(
		connection: &Connection,
		object: &ObjectInfo,
		contains_interface: &(impl Fn(&str) -> bool + Send + Sync),
	) -> impl std::future::Future<Output = Option<Self>> + Send + Sync;
}

pub enum QueryEvent<Q: ObjectRegistryQueryable> {
	NewMatch(ObjectInfo, Q),
	MatchModified(ObjectInfo, Q),
	MatchLost(ObjectInfo),
}

impl<Q: ObjectRegistryQueryable> ObjectRegistryQuery<Q> {
	pub fn new<
		C: Fn(QueryEvent<Q>) -> F + 'static + Send + Sync,
		F: Future<Output = ()> + 'static + Send + Sync,
	>(
		connection: Connection,
		event_handler: C,
	) -> Self {
		let (tx, mut rx) = mpsc::channel(32);
		let callback_handle = tokio::spawn(async move {
			while let Some(e) = rx.recv().await {
				event_handler(e).await;
			}
		})
		.abort_handle();
		let update_task_handle = tokio::spawn(Self::update_task(connection, tx)).abort_handle();
		Self {
			update_task_handle,
			callback_handle,
			_phantom_data: PhantomData,
		}
	}
}

impl<T: ProxyImpl<'static> + Defaults + Send + Sync + From<Proxy<'static>>> ObjectRegistryQueryable
	for ObjectProxy<T>
{
	async fn try_new(
		connection: &Connection,
		object: &ObjectInfo,
		contains_interface: &(impl Fn(&str) -> bool + Send + Sync),
	) -> Option<Self> {
		let interface = T::INTERFACE.as_ref()?.to_string();
		if !contains_interface(&interface) {
			return None;
		}
		Some(ObjectProxy(
			object.to_proxy(connection, interface).await.ok()?.into(),
		))
	}
}

pub struct ObjectProxy<T: Defaults + Send + Sync + From<Proxy<'static>> + 'static>(pub T);
impl<T: Defaults + Send + Sync + From<Proxy<'static>> + 'static> Deref for ObjectProxy<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl<T: Defaults + Send + Sync + From<Proxy<'static>> + 'static> DerefMut for ObjectProxy<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

macro_rules! impl_queryable {
    ($($T:ident),*) => {
        impl<$($T: ObjectRegistryQueryable),*> ObjectRegistryQueryable for ($($T,)*) {
			#[allow(unused_variables)]
			async fn try_new(
				connection: &Connection,
				object: &ObjectInfo,
				contains_interface: &(impl Fn(&str) -> bool + Send + Sync),
			) -> Option<Self> {
				Some(($($T::try_new(connection, object, contains_interface).await?,)*))
			}
        }
    };
}

all_tuples!(impl_queryable, 0, 15, T);

impl<Q: ObjectRegistryQueryable> ObjectRegistryQuery<Q> {
	async fn new_match(
		tx: &mpsc::Sender<QueryEvent<Q>>,
		matching_objects: Arc<RwLock<HashSet<ObjectInfo>>>,
		object: ObjectInfo,
		data: Q,
	) {
		matching_objects.write().await.insert(object.clone());
		_ = tx.send(QueryEvent::NewMatch(object, data)).await;
	}
	async fn match_lost(
		tx: &mpsc::Sender<QueryEvent<Q>>,
		object: ObjectInfo,
		matching_objects: Arc<RwLock<HashSet<ObjectInfo>>>,
	) {
		matching_objects.write().await.remove(&object);
		_ = tx.send(QueryEvent::MatchLost(object)).await;
	}
	async fn handle_interface_change(
		connection: Connection,
		tx: mpsc::Sender<QueryEvent<Q>>,
		name: OwnedBusName,
		object_manager: fdo::ObjectManagerProxy<'static>,
		matching_objects: Arc<RwLock<HashSet<ObjectInfo>>>,
		object_path: OwnedObjectPath,
	) {
		let object = ObjectInfo {
			bus_name: name.clone(),
			object_path,
		};

		let Ok(Ok(objects)) = timeout(
			Duration::from_millis(5),
			object_manager.get_managed_objects(),
		)
		.await
		else {
			return;
		};
		let Some(interfaces) = objects.get(&object.object_path) else {
			return;
		};
		let already_matching = matching_objects.read().await.contains(&object);
		let new_data = Q::try_new(&connection, &object, &|interface| {
			interfaces.contains_key(interface)
		})
		.await;
		match (new_data, already_matching) {
			(Some(q), true) => {
				_ = tx.send(QueryEvent::MatchModified(object, q)).await;
			}
			(Some(q), false) => {
				Self::new_match(&tx, matching_objects.clone(), object, q).await;
			}
			(None, true) => {
				Self::match_lost(&tx, object, matching_objects.clone()).await;
			}
			(None, false) => {}
		};
	}
	async fn handle_interface_added(
		connection: Connection,
		tx: mpsc::Sender<QueryEvent<Q>>,
		name: OwnedBusName,
		object_manager: fdo::ObjectManagerProxy<'static>,
		matching_objects: Arc<RwLock<HashSet<ObjectInfo>>>,
	) {
		let Ok(mut stream) = object_manager.receive_interfaces_added().await else {
			return;
		};
		while let Some(interface_added) = stream.next().await {
			let Ok(args) = interface_added.args() else {
				continue;
			};
			Self::handle_interface_change(
				connection.clone(),
				tx.clone(),
				name.clone(),
				object_manager.clone(),
				matching_objects.clone(),
				args.object_path.into_owned().into(),
			)
			.await;
		}
	}
	async fn handle_interface_removed(
		connection: Connection,
		tx: mpsc::Sender<QueryEvent<Q>>,
		name: OwnedBusName,
		object_manager: fdo::ObjectManagerProxy<'static>,
		matching_objects: Arc<RwLock<HashSet<ObjectInfo>>>,
	) {
		let Ok(mut stream) = object_manager.receive_interfaces_removed().await else {
			return;
		};
		while let Some(interface_removed) = stream.next().await {
			let Ok(args) = interface_removed.args() else {
				continue;
			};
			Self::handle_interface_change(
				connection.clone(),
				tx.clone(),
				name.clone(),
				object_manager.clone(),
				matching_objects.clone(),
				args.object_path.into_owned().into(),
			)
			.await;
		}
	}
	async fn setup_namespace(
		connection: Connection,
		tx: mpsc::Sender<QueryEvent<Q>>,
		name: OwnedBusName,
		matching_objects: Arc<RwLock<HashSet<ObjectInfo>>>,
	) -> Option<NamespaceHandler> {
		let Ok(object_manager_proxy) =
			fdo::ObjectManagerProxy::new(&connection, name.clone(), "/").await
		else {
			return None;
		};
		let Ok(Ok(objects)) = timeout(
			Duration::from_millis(5),
			object_manager_proxy.get_managed_objects(),
		)
		.await
		else {
			return None;
		};

		for (object_path, interfaces) in objects.iter() {
			let object = ObjectInfo {
				bus_name: name.clone(),
				object_path: object_path.clone(),
			};
			let Some(query_item) = Q::try_new(&connection, &object, &|interface| {
				interfaces.contains_key(interface)
			})
			.await
			else {
				continue;
			};
			Self::new_match(&tx, matching_objects.clone(), object, query_item).await;
		}

		let interface_added = tokio::spawn(Self::handle_interface_added(
			connection.clone(),
			tx.clone(),
			name.clone(),
			object_manager_proxy.clone(),
			matching_objects.clone(),
		))
		.abort_handle();

		let interface_removed = tokio::spawn(Self::handle_interface_removed(
			connection.clone(),
			tx.clone(),
			name.clone(),
			object_manager_proxy.clone(),
			matching_objects.clone(),
		))
		.abort_handle();
		Some(NamespaceHandler {
			name,
			interface_added,
			interface_removed,
		})
	}
	async fn update_task(
		connection: Connection,
		tx: mpsc::Sender<QueryEvent<Q>>,
	) -> zbus::Result<()> {
		let matching_objects: Arc<RwLock<HashSet<ObjectInfo>>> = Arc::default();
		let dbus_proxy = fdo::DBusProxy::new(&connection).await?;
		let mut buses: HashMap<OwnedBusName, _> = {
			let names = dbus_proxy.list_names().await?;
			let mut buses = HashMap::new();

			for name in names {
				let Some(handles) = Self::setup_namespace(
					connection.clone(),
					tx.clone(),
					name.clone(),
					matching_objects.clone(),
				)
				.await
				else {
					continue;
				};
				buses.insert(name, handles);
			}

			buses
		};

		let mut name_owner_changed_stream = dbus_proxy.receive_name_owner_changed().await?;
		while let Some(signal) = name_owner_changed_stream.next().await {
			let args = signal.args().unwrap();
			let name: OwnedBusName = args.name.clone().into();
			if matches!(&args.name, BusName::WellKnown(_)) {
				continue;
			}

			let old_owner = args.old_owner.as_ref();
			let new_owner = args.new_owner.as_ref();

			if old_owner.is_none() && new_owner.is_some() {
				let Some(handles) = Self::setup_namespace(
					connection.clone(),
					tx.clone(),
					name.clone(),
					matching_objects.clone(),
				)
				.await
				else {
					continue;
				};
				buses.insert(name, handles);
			} else if old_owner.is_some()
				&& new_owner.is_none()
				&& let Some(handles) = buses.remove(&name)
			{
				handles.dismiss(&tx, matching_objects.clone()).await;
			}
		}

		Ok(())
	}
}

struct NamespaceHandler {
	name: OwnedBusName,
	interface_added: AbortHandle,
	interface_removed: AbortHandle,
}
impl NamespaceHandler {
	async fn dismiss<Q: ObjectRegistryQueryable>(
		self,
		tx: &mpsc::Sender<QueryEvent<Q>>,
		matching_objects: Arc<RwLock<HashSet<ObjectInfo>>>,
	) {
		let owned_objects = matching_objects
			.read()
			.await
			.iter()
			.filter(|v| v.bus_name == self.name)
			.cloned()
			.collect::<Vec<_>>();
		for object in owned_objects {
			ObjectRegistryQuery::match_lost(tx, object.clone(), matching_objects.clone()).await;
		}
	}
}
impl Drop for NamespaceHandler {
	fn drop(&mut self) {
		self.interface_added.abort();
		self.interface_removed.abort();
	}
}
