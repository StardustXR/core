use futures_util::StreamExt as _;
use std::{
	collections::{HashMap, HashSet},
	marker::PhantomData,
	ops::{Deref, DerefMut},
	sync::Arc,
	time::{Duration, Instant},
};
use tokio::{
	sync::{
		RwLock, RwLockReadGuard,
		mpsc::{self, error::TryRecvError},
		watch,
	},
	task::{AbortHandle, JoinSet},
	time::timeout,
};
use variadics_please::all_tuples;
use zbus::{
	Connection, Proxy, fdo,
	names::{BusName, OwnedBusName},
	proxy::{Defaults, ProxyImpl},
	zvariant::{ObjectPath, OwnedObjectPath},
};

use crate::dbus::{
	ObjectInfo,
	interfaces::SpatialRefProxy,
	list_query::{ListQueryMapper, ObjectListQuery},
	object_registry::{ObjectRegistry, Objects},
};

pub struct ObjectQuery<Q: Queryable<Ctx>, Ctx: QueryContext> {
	update_task_handle: AbortHandle,
	event_reader: mpsc::Receiver<QueryEvent<Q, Ctx>>,
}

pub trait Queryable<Ctx: QueryContext>: Sized + 'static + Send + Sync {
	fn try_new(
		connection: &Connection,
		ctx: &Arc<Ctx>,
		object: &ObjectInfo,
		contains_interface: &(impl Fn(&str) -> bool + Send + Sync),
	) -> impl std::future::Future<Output = Option<Self>> + Send;
}

pub trait QueryContext: Sized + 'static + Send + Sync {}

pub enum QueryEvent<Q: Queryable<Ctx> + Send + Sync, Ctx: QueryContext> {
	NewMatch(ObjectInfo, Q),
	MatchModified(ObjectInfo, Q),
	MatchLost(ObjectInfo),
	PhantomVariant(PhantomData<Ctx>),
}

#[macro_export]
macro_rules! impl_queryable_for_proxy {
	($($T:ident),*) => {
		$(impl<Ctx: $crate::dbus::query::QueryContext> $crate::dbus::query::Queryable<Ctx> for $T<'static> {
			async fn try_new(
				connection: &::zbus::Connection,
				_ctx: &std::sync::Arc<Ctx>,
				object: &$crate::dbus::ObjectInfo,
				contains_interface: &(impl Fn(&str) -> bool + Send + Sync),
			) -> Option<Self> {
				use ::zbus::proxy::Defaults;
				let interface = $T::INTERFACE.as_ref()?.to_string();
				if !contains_interface(&interface) {
					return None;
				}
				Some(
					object.to_proxy(connection, interface).await.ok()?.into(),
				)
			}
		})*
	};
}

impl<Q, Ctx> ObjectQuery<Q, Ctx>
where
	Ctx: QueryContext,
	Q: Queryable<Ctx>,
{
	pub fn new(connection: Connection, context: impl Into<Arc<Ctx>>) -> Self {
		let (tx, rx) = mpsc::channel(32);
		let update_task_handle =
			tokio::spawn(Self::update_task(context.into(), connection, tx)).abort_handle();
		Self {
			update_task_handle,
			event_reader: rx,
		}
	}
	pub async fn recv_event(&mut self) -> Option<QueryEvent<Q, Ctx>> {
		self.event_reader.recv().await
	}
	pub fn try_recv_event(&mut self) -> Result<QueryEvent<Q, Ctx>, TryRecvError> {
		self.event_reader.try_recv()
	}
	pub fn to_list_query<T: Send + Sync + 'static>(
		self,
	) -> (ObjectListQuery<T>, ListQueryMapper<Q, T, Ctx>) {
		ObjectListQuery::from_query(self)
	}
}

impl<T: Queryable<Ctx>, Ctx: QueryContext> Queryable<Ctx> for Option<T> {
	async fn try_new(
		connection: &Connection,
		ctx: &Arc<Ctx>,
		object: &ObjectInfo,
		contains_interface: &(impl Fn(&str) -> bool + Send + Sync),
	) -> Option<Self> {
		Some(T::try_new(connection, ctx, object, contains_interface).await)
	}
}

impl<T: ProxyImpl<'static> + Defaults + Send + Sync + From<Proxy<'static>>, Ctx: QueryContext>
	Queryable<Ctx> for ObjectProxy<T>
{
	async fn try_new(
		connection: &Connection,
		_ctx: &Arc<Ctx>,
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
        impl<Ctx: QueryContext, $($T: Queryable<Ctx>),*> Queryable<Ctx> for ($($T,)*) {
			#[allow(unused_variables)]
			async fn try_new(
				connection: &Connection,
				ctx: &Arc<Ctx>,
				object: &ObjectInfo,
				contains_interface: &(impl Fn(&str) -> bool + Send + Sync),
			) -> Option<Self> {
				Some(($($T::try_new(connection, ctx, object, contains_interface).await?,)*))
			}
        }
    };
}

all_tuples!(impl_queryable, 0, 15, T);

impl<Q: Queryable<Ctx>, Ctx: QueryContext> ObjectQuery<Q, Ctx> {
	async fn new_match(
		tx: &mpsc::Sender<QueryEvent<Q, Ctx>>,
		matching_objects: Arc<RwLock<HashSet<ObjectInfo>>>,
		object: ObjectInfo,
		data: Q,
	) {
		matching_objects.write().await.insert(object.clone());
		_ = tx.send(QueryEvent::NewMatch(object, data)).await;
	}
	async fn match_lost(
		tx: &mpsc::Sender<QueryEvent<Q, Ctx>>,
		object: ObjectInfo,
		matching_objects: Arc<RwLock<HashSet<ObjectInfo>>>,
	) {
		matching_objects.write().await.remove(&object);
		_ = tx.send(QueryEvent::MatchLost(object)).await;
	}
	async fn handle_interface_change(
		ctx: Arc<Ctx>,
		connection: Connection,
		tx: mpsc::Sender<QueryEvent<Q, Ctx>>,
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
		let new_data = Q::try_new(&connection, &ctx, &object, &|interface| {
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
		ctx: Arc<Ctx>,
		connection: Connection,
		tx: mpsc::Sender<QueryEvent<Q, Ctx>>,
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
				ctx.clone(),
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
		ctx: Arc<Ctx>,
		connection: Connection,
		tx: mpsc::Sender<QueryEvent<Q, Ctx>>,
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
				ctx.clone(),
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
		ctx: Arc<Ctx>,
		connection: Connection,
		tx: mpsc::Sender<QueryEvent<Q, Ctx>>,
		name: OwnedBusName,
		matching_objects: Arc<RwLock<HashSet<ObjectInfo>>>,
	) -> Option<NamespaceHandler> {
		let Ok(object_manager_proxy) =
			fdo::ObjectManagerProxy::new(&connection, name.clone(), "/").await
		else {
			return None;
		};
		let Ok(Ok(objects)) = timeout(
			Duration::from_millis(100),
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
			let Some(query_item) = Q::try_new(&connection, &ctx, &object, &|interface| {
				interfaces.contains_key(interface)
			})
			.await
			else {
				continue;
			};
			Self::new_match(&tx, matching_objects.clone(), object, query_item).await;
		}

		let interface_added = tokio::spawn(Self::handle_interface_added(
			ctx.clone(),
			connection.clone(),
			tx.clone(),
			name.clone(),
			object_manager_proxy.clone(),
			matching_objects.clone(),
		))
		.abort_handle();

		let interface_removed = tokio::spawn(Self::handle_interface_removed(
			ctx.clone(),
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
		ctx: Arc<Ctx>,
		connection: Connection,
		tx: mpsc::Sender<QueryEvent<Q, Ctx>>,
	) -> zbus::Result<()> {
		let matching_objects: Arc<RwLock<HashSet<ObjectInfo>>> = Arc::default();
		let dbus_proxy = fdo::DBusProxy::new(&connection).await?;
		let mut buses: HashMap<OwnedBusName, _> = {
			let names = dbus_proxy.list_names().await?;
			let mut buses = HashMap::new();
			let mut joinset = JoinSet::new();
			for name in names {
				joinset.spawn(Self::setup_namespace(
					ctx.clone(),
					connection.clone(),
					tx.clone(),
					name,
					matching_objects.clone(),
				));
			}
			while let Some(v) = joinset.join_next().await {
				if let Ok(Some(handler)) = v {
					buses.insert(handler.name.clone(), handler);
				}
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
					ctx.clone(),
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

#[derive(Debug)]
struct NamespaceHandler {
	name: OwnedBusName,
	interface_added: AbortHandle,
	interface_removed: AbortHandle,
}
impl NamespaceHandler {
	async fn dismiss<Q: Queryable<Ctx>, Ctx: QueryContext>(
		self,
		tx: &mpsc::Sender<QueryEvent<Q, Ctx>>,
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
			ObjectQuery::match_lost(tx, object.clone(), matching_objects.clone()).await;
		}
	}
}
impl Drop for NamespaceHandler {
	fn drop(&mut self) {
		self.interface_added.abort();
		self.interface_removed.abort();
	}
}

mod test {
	use std::{
		sync::{
			Arc,
			atomic::{AtomicBool, Ordering},
		},
		thread,
		time::Duration,
	};

	use tokio::{sync::Notify, time::sleep};
	use zbus::{Connection, fdo::ObjectManager, interface};

	use crate::dbus::query::{ObjectQuery, QueryContext};

	struct TestInterface;
	#[interface(name = "org.stardustxr.TestInterface", proxy())]
	impl TestInterface {
		fn hello(&self) {
			println!("hello world");
		}
	}
	impl_queryable_for_proxy!(TestInterfaceProxy);
	impl QueryContext for () {}

	#[tokio::test]
	async fn query() {
		console_subscriber::init();
		let notify = Arc::new(Notify::new());
		let notify_2 = notify.clone();
		let thread = thread::spawn(move || {
			let rt = tokio::runtime::Builder::new_current_thread()
				.enable_all()
				.build()
				.unwrap();
			rt.block_on(async move {
				let other_conn = Connection::session().await.unwrap();
				println!("name: {:?}", other_conn.unique_name());
				_ = other_conn.object_server().at("/", ObjectManager).await;
				_ = other_conn
					.object_server()
					.at("/org/stardustxr/core/schemas/test", TestInterface)
					.await;
				notify_2.notified().await;
			})
		});
		let query_conn = Connection::session().await.unwrap();
		let match_lost = Arc::new(AtomicBool::new(false));
		let match_gained = Arc::new(AtomicBool::new(false));
		let mut query = ObjectQuery::<TestInterfaceProxy, ()>::new(query_conn, ());
		tokio::spawn({
			let match_lost = match_lost.clone();
			let match_gained = match_gained.clone();
			async move {
				while let Some(e) = query.recv_event().await {
					match e {
						super::QueryEvent::NewMatch(_object_info, p) => {
							_ = p.hello().await;
							match_gained.store(true, Ordering::Relaxed);
						}
						super::QueryEvent::MatchModified(_object_info, _) => {}
						super::QueryEvent::MatchLost(_object_info) => {
							match_lost.store(true, Ordering::Relaxed);
						}
						super::QueryEvent::PhantomVariant(_) => {}
					}
				}
			}
		});
		sleep(Duration::from_millis(500)).await;
		assert!(match_gained.load(Ordering::Relaxed));
		assert!(!match_lost.load(Ordering::Relaxed));
		notify.notify_one();
		sleep(Duration::from_millis(5)).await;
		assert!(match_lost.load(Ordering::Relaxed));
		thread.join().unwrap();
	}
}
