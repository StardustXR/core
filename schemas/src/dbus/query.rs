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
		broadcast::error::RecvError,
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
	pub fn new(object_registry: Arc<ObjectRegistry>, context: impl Into<Arc<Ctx>>) -> Self {
		let (tx, rx) = mpsc::channel(32);
		let update_task_handle =
			tokio::spawn(Self::update_task(context.into(), object_registry, tx)).abort_handle();
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
		matching_objects: &mut HashSet<ObjectInfo>,
		object: ObjectInfo,
		data: Q,
	) {
		matching_objects.insert(object.clone());
		_ = tx.send(QueryEvent::NewMatch(object, data)).await;
	}
	async fn match_lost(
		tx: &mpsc::Sender<QueryEvent<Q, Ctx>>,
		object: ObjectInfo,
		matching_objects: &mut HashSet<ObjectInfo>,
	) {
		matching_objects.remove(&object);
		_ = tx.send(QueryEvent::MatchLost(object)).await;
	}
	async fn update_task(
		ctx: Arc<Ctx>,
		object_registry: Arc<ObjectRegistry>,
		tx: mpsc::Sender<QueryEvent<Q, Ctx>>,
	) -> zbus::Result<()> {
		let mut matching_objects: HashSet<ObjectInfo> = HashSet::new();
		let connection = object_registry.get_connection();
		let v = object_registry
			.get_watch()
			.borrow()
			.object_to_interfaces
			.clone();

		for (object, interfaces) in v {
			let data = Q::try_new(connection, &ctx, &object, &|i| interfaces.contains(i)).await;
			let already_matching = matching_objects.contains(&object);
			match (data, already_matching) {
				(None, true) => Self::match_lost(&tx, object, &mut matching_objects).await,
				(None, false) => {}
				(Some(data), true) => {
					_ = tx.send(QueryEvent::MatchModified(object, data)).await;
				}
				(Some(data), false) => {
					Self::new_match(&tx, &mut matching_objects, object, data).await
				}
			}
		}

		let mut recv = object_registry.get_object_changed_receiver();
		loop {
			let objs = match recv.recv().await {
				Ok(objs) => objs,
				Err(RecvError::Closed) => break,
				Err(RecvError::Lagged(_)) => continue,
			};
			for object in objs {
				let interfaces = object_registry
					.get_watch()
					.borrow()
					.object_to_interfaces
					.get(&object)
					.cloned();
				let data = if let Some(interfaces) = interfaces {
					Q::try_new(connection, &ctx, &object, &|i| interfaces.contains(i)).await
				} else {
					None
				};
				let already_matching = matching_objects.contains(&object);
				match (data, already_matching) {
					(None, true) => Self::match_lost(&tx, object, &mut matching_objects).await,
					(None, false) => {}
					(Some(data), true) => {
						_ = tx.send(QueryEvent::MatchModified(object, data)).await;
					}
					(Some(data), false) => {
						Self::new_match(&tx, &mut matching_objects, object, data).await
					}
				}
			}
		}

		Ok(())
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

	use crate::dbus::{
		object_registry::ObjectRegistry,
		query::{ObjectQuery, QueryContext},
	};

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
		let other_conn = Connection::session().await.unwrap();
		println!("name: {:?}", other_conn.unique_name());
		_ = other_conn.object_server().at("/", ObjectManager).await;
		_ = other_conn
			.object_server()
			.at("/org/stardustxr/core/schemas/test", TestInterface)
			.await;
		let query_conn = Connection::session().await.unwrap();
		let object_registry = ObjectRegistry::new(&query_conn).await.unwrap();
		let match_lost = Arc::new(AtomicBool::new(false));
		let match_gained = Arc::new(AtomicBool::new(false));
		let mut query = ObjectQuery::<TestInterfaceProxy, ()>::new(object_registry, ());
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
		sleep(Duration::from_millis(50)).await;
		assert!(match_gained.load(Ordering::Relaxed));
		assert!(!match_lost.load(Ordering::Relaxed));
		drop(other_conn);
		sleep(Duration::from_millis(50)).await;
		assert!(match_lost.load(Ordering::Relaxed));
	}
}
