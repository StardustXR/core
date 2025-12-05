use crate::{
	ObjectInfo,
	list_query::{ListQueryMapper, ObjectListQuery},
	object_registry::ObjectRegistry,
};
use std::{collections::HashSet, marker::PhantomData, sync::Arc};
use tokio::{
	sync::{
		broadcast::error::RecvError,
		mpsc::{self, error::TryRecvError},
	},
	task::AbortHandle,
};
use variadics_please::all_tuples;
use zbus::{Connection, names::InterfaceName};

pub struct ObjectQuery<Q: Queryable<Ctx>, Ctx: QueryContext> {
	update_task_handle: AbortHandle,
	event_reader: mpsc::Receiver<QueryEvent<Q, Ctx>>,
}
impl<Q: Queryable<Ctx>, Ctx: QueryContext> Drop for ObjectQuery<Q, Ctx> {
	fn drop(&mut self) {
		self.update_task_handle.abort();
	}
}

pub trait Queryable<Ctx: QueryContext>: Sized + 'static + Send + Sync {
	fn try_new(
		connection: &Connection,
		ctx: &Arc<Ctx>,
		object: &ObjectInfo,
		contains_interface: &(impl Fn(&InterfaceName) -> bool + Send + Sync),
	) -> impl std::future::Future<Output = Option<Self>> + Send;
}
pub trait QueryContext: Sized + 'static + Send + Sync {}
impl QueryContext for () {}

pub enum QueryEvent<Q: Queryable<Ctx> + Send + Sync, Ctx: QueryContext> {
	NewMatch(ObjectInfo, Q),
	MatchModified(ObjectInfo, Q),
	MatchLost(ObjectInfo),
	PhantomVariant(PhantomData<Ctx>),
}
impl<Q: Queryable<Ctx> + Send + Sync + std::fmt::Debug, Ctx: QueryContext + std::fmt::Debug>
	std::fmt::Debug for QueryEvent<Q, Ctx>
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NewMatch(arg0, arg1) => {
				f.debug_tuple("NewMatch").field(arg0).field(arg1).finish()
			}
			Self::MatchModified(arg0, arg1) => f
				.debug_tuple("MatchModified")
				.field(arg0)
				.field(arg1)
				.finish(),
			Self::MatchLost(arg0) => f.debug_tuple("MatchLost").field(arg0).finish(),
			Self::PhantomVariant(arg0) => f.debug_tuple("PhantomVariant").field(arg0).finish(),
		}
	}
}

#[macro_export]
macro_rules! impl_queryable_for_proxy {
	($($T:ident),*) => {
		$(impl<Ctx: $crate::query::QueryContext> $crate::query::Queryable<Ctx> for $T<'static> {
			async fn try_new(
				connection: &::zbus::Connection,
				_ctx: &std::sync::Arc<Ctx>,
				object: &$crate::ObjectInfo,
				contains_interface: &(impl Fn(&zbus::names::InterfaceName) -> bool + Send + Sync),
			) -> Option<Self> {
				use ::zbus::proxy::Defaults;
				let interface = $T::INTERFACE.as_ref()?;
				if !contains_interface(&interface) {
					return None;
				}
				object.to_typed_proxy::<Self>(connection).await.ok()
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

impl<Q: Queryable<Ctx>, Ctx: QueryContext> Queryable<Ctx> for Option<Q> {
	async fn try_new(
		connection: &Connection,
		ctx: &Arc<Ctx>,
		object: &ObjectInfo,
		contains_interface: &(impl Fn(&InterfaceName) -> bool + Send + Sync),
	) -> Option<Self> {
		Some(Q::try_new(connection, ctx, object, contains_interface).await)
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
				contains_interface: &(impl Fn(&zbus::names::InterfaceName) -> bool + Send + Sync),
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
		let mut recv = object_registry.get_object_events_receiver();
		let mut matching_objects: HashSet<ObjectInfo> = HashSet::new();
		let connection = object_registry.get_connection();
		let watch = object_registry.get_watch();
		let v = watch.borrow().object_to_interfaces.clone();

		for (object, interfaces) in v {
			let data = Q::try_new(connection, &ctx, &object, &|i| {
				interfaces.iter().any(|f| i == f)
			})
			.await;
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

		loop {
			let object_event = match recv.recv().await {
				Ok(objs) => objs,
				Err(RecvError::Closed) => break,
				Err(RecvError::Lagged(_)) => continue,
			};
			let Some(v) = watch
				.borrow()
				.object_to_interfaces
				.get(&object_event.object)
				.cloned()
			else {
				Self::match_lost(&tx, object_event.object, &mut matching_objects).await;
				continue;
			};
			let data = Q::try_new(connection, &ctx, &object_event.object, &|i| {
				v.iter().any(|j| i == j)
			})
			.await;

			let already_matching = matching_objects.contains(&object_event.object);
			match (data, already_matching) {
				(None, true) => {
					Self::match_lost(&tx, object_event.object, &mut matching_objects).await
				}
				(None, false) => {}
				(Some(data), true) => {
					_ = tx
						.send(QueryEvent::MatchModified(object_event.object, data))
						.await;
				}
				(Some(data), false) => {
					Self::new_match(&tx, &mut matching_objects, object_event.object, data).await
				}
			}
		}

		Ok(())
	}
}

#[tokio::test]
async fn query_test() {
	use crate::{object_registry::ObjectRegistry, query::ObjectQuery};
	use std::time::Duration;
	use zbus::{Connection, interface};

	struct TestInterface;
	#[interface(name = "org.stardustxr.Query.TestInterface", proxy())]
	impl TestInterface {
		fn hello(&self) {
			println!("hello world");
		}
	}
	impl_queryable_for_proxy!(TestInterfaceProxy);

	tokio::task::spawn(async {
		tokio::time::sleep(Duration::from_secs(10)).await;
		panic!("Took too long to run");
	});

	let service_conn = zbus::conn::Builder::session()
		.unwrap()
		.serve_at("/", zbus::fdo::ObjectManager)
		.unwrap()
		.serve_at("/org/stardustxr/TestObject", TestInterface)
		.unwrap()
		.build()
		.await
		.unwrap();
	println!("name: {:?}", service_conn.unique_name());

	let scan_conn = Connection::session().await.unwrap();
	let object_registry = ObjectRegistry::new(&scan_conn).await;

	println!(
		"Objects updated: {:#?}",
		object_registry.get_watch().borrow().clone()
	);
	// let mut watch = object_registry.get_watch();
	// tokio::task::spawn(async move {
	// 	while !matches!(watch.changed().await, Ok(())) {
	// 		println!("Object registry changed: {:#?}", watch.borrow());
	// 	}
	// });

	let mut query = ObjectQuery::<TestInterfaceProxy, ()>::new(object_registry, ());

	while let Some(e) = query.recv_event().await {
		println!("New event: {e:#?}");
		if let QueryEvent::NewMatch(object_info, p) = e {
			println!("New match to query, {object_info:#?}");
			if object_info.object_path.as_str() == "/org/stardustxr/TestObject" {
				p.hello().await.unwrap();
				break;
			}
		}
	}

	drop(service_conn);
	println!("Dropping the other connection");
	while let Some(e) = query.recv_event().await {
		println!("New event: {e:#?}");
		if let QueryEvent::MatchLost(object_info) = e {
			println!("Dropped match to query, {object_info:#?}");
			if object_info.object_path.as_str() == "/org/stardustxr/TestObject" {
				break;
			}
		}
	}
}
