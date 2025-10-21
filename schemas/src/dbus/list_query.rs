use std::{collections::HashMap, marker::PhantomData, ops::Deref, sync::Arc};

use tokio::sync::RwLock;

use crate::dbus::{
	ObjectInfo,
	query::{ObjectQuery, QueryContext, QueryEvent, Queryable},
};

pub struct ObjectListQuery<T: Send + Sync + 'static> {
	list: Arc<RwLock<HashMap<ObjectInfo, T>>>,
}
impl<T: Send + Sync + 'static> ObjectListQuery<T> {
	pub async fn iter(&self) -> impl Deref<Target = HashMap<ObjectInfo, T>> {
		self.list.read().await
	}
	pub fn iter_blocking(&self) -> impl Deref<Target = HashMap<ObjectInfo, T>> {
		self.list.blocking_read()
	}
	pub fn from_query<Q: Queryable<Ctx>, Ctx: QueryContext>(
		query: ObjectQuery<Q, Ctx>,
	) -> (ObjectListQuery<T>, ListQueryMapper<Q, T, Ctx>) {
		let list: Arc<RwLock<HashMap<ObjectInfo, T>>> = Arc::default();
		(
			ObjectListQuery { list: list.clone() },
			ListQueryMapper { list, query },
		)
	}
}
impl<Q: Queryable<Ctx>, T: Send + Sync + 'static, Ctx: QueryContext> ListQueryMapper<Q, T, Ctx> {
	/// this should be used with tokio::spawn
	pub async fn init(mut self, mapper: impl AsyncFn(ListEvent<Q, Ctx>) -> Option<T>) {
		while let Some(e) = self.query.recv_event().await {
			let (obj, e) = match e {
				QueryEvent::NewMatch(obj, v) => (obj, ListEvent::NewMatch(v)),
				QueryEvent::MatchModified(obj, v) => (obj, ListEvent::Modified(v)),
				QueryEvent::MatchLost(obj) => (obj, ListEvent::MatchLost),
				// this is never triggered
				QueryEvent::PhantomVariant(_) => continue,
			};
			let op = mapper(e).await;
			let mut list = self.list.write().await;
			match op {
				Some(v) => list.insert(obj, v),
				None => list.remove(&obj),
			};
		}
	}
}

pub enum ListEvent<Q: Queryable<Ctx>, Ctx: QueryContext> {
	NewMatch(Q),
	Modified(Q),
	MatchLost,
	PhantomVariant(PhantomData<Ctx>),
}
pub struct ListQueryMapper<Q: Queryable<Ctx>, T: Send + Sync + 'static, Ctx: QueryContext> {
	list: Arc<RwLock<HashMap<ObjectInfo, T>>>,
	query: ObjectQuery<Q, Ctx>,
}

mod test {
	use std::{
		sync::{
			Arc,
			atomic::{AtomicBool, Ordering},
		},
		time::Duration,
	};

	use tokio::time::sleep;
	use zbus::{Connection, fdo::ObjectManager, interface};

	use crate::dbus::{
		list_query::ListEvent,
		object_registry::{self, ObjectRegistry},
		query::ObjectQuery,
	};

	struct TestInterface;
	#[interface(name = "org.stardustxr.TestInterface.list", proxy())]
	impl TestInterface {
		fn hello(&self) {
			println!("hello world");
		}
	}
	impl_queryable_for_proxy!(TestInterfaceProxy);

	#[tokio::test]
	async fn list_query_test() {
		let query_conn = Connection::session().await.unwrap();
		let other_conn = Connection::session().await.unwrap();
		_ = other_conn.object_server().at("/", ObjectManager).await;
		let object_registry = ObjectRegistry::new(&query_conn).await;
		let (query, mapper) =
			ObjectQuery::<TestInterfaceProxy, _>::new(object_registry.clone(), ()).to_list_query();
		tokio::spawn(mapper.init(async |e| match e {
			ListEvent::NewMatch(_) => Some(()),
			ListEvent::Modified(_) => Some(()),
			ListEvent::MatchLost => None,
			_ => None,
		}));
		assert_eq!(query.iter().await.len(), 0);
		sleep(Duration::from_millis(50)).await;
		_ = other_conn
			.object_server()
			.at("/org/stardustxr/core/schemas/test", TestInterface)
			.await
			.unwrap();
		sleep(Duration::from_millis(50)).await;
		assert_eq!(query.iter().await.len(), 1);
		drop(other_conn);
		sleep(Duration::from_millis(50)).await;
		assert_eq!(query.iter().await.len(), 0);
	}
}
