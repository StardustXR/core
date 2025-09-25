use std::{collections::HashMap, ops::Deref, sync::Arc};

use tokio::sync::RwLock;

use crate::dbus::{
	ObjectInfo,
	query::{ObjectQuery, QueryEvent, Queryable},
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
	pub fn from_query<Q: Queryable>(
		query: ObjectQuery<Q>,
	) -> (ObjectListQuery<T>, ListQueryMapper<Q, T>) {
		let list: Arc<RwLock<HashMap<ObjectInfo, T>>> = Arc::default();
		(
			ObjectListQuery { list: list.clone() },
			ListQueryMapper { list, query },
		)
	}
}
impl<Q: Queryable, T: Send + Sync + 'static> ListQueryMapper<Q, T> {
	/// this should be used with tokio::spawn
	pub async fn init(mut self, mapper: impl AsyncFn(ListEvent<Q>) -> Option<T>) {
		while let Some(e) = self.query.recv_event().await {
			let (obj, e) = match e {
				QueryEvent::NewMatch(obj, v) => (obj, ListEvent::NewMatch(v)),
				QueryEvent::MatchModified(obj, v) => (obj, ListEvent::Modified(v)),
				QueryEvent::MatchLost(obj) => (obj, ListEvent::MatchLost),
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

pub enum ListEvent<Q: Queryable> {
	NewMatch(Q),
	Modified(Q),
	MatchLost,
}
pub struct ListQueryMapper<Q: Queryable, T: Send + Sync + 'static> {
	list: Arc<RwLock<HashMap<ObjectInfo, T>>>,
	query: ObjectQuery<Q>,
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

	use crate::dbus::{list_query::ListEvent, query::ObjectQuery};

	struct TestInterface;
	#[interface(name = "org.stardustxr.TestInterface.list", proxy())]
	impl TestInterface {
		fn hello(&self) {
			println!("hello world");
		}
	}
	impl_queryable_for_proxy!(TestInterfaceProxy);

	#[tokio::test]
	async fn query() {
		let query_conn = Connection::session().await.unwrap();
		let other_conn = Connection::session().await.unwrap();
		_ = other_conn.object_server().at("/", ObjectManager).await;
		let (query, mapper) = ObjectQuery::<TestInterfaceProxy>::new(query_conn).to_list_query();
		tokio::spawn(mapper.init(async |e| match e {
			ListEvent::NewMatch(_) => Some(()),
			ListEvent::Modified(_) => Some(()),
			ListEvent::MatchLost => dbg!(None),
		}));
		assert_eq!(query.iter().await.len(), 0);
		_ = other_conn
			.object_server()
			.at("/org/stardustxr/core/schemas/test", TestInterface)
			.await;
		sleep(Duration::from_millis(250)).await;
		assert_eq!(query.iter().await.len(), 1);
		drop(other_conn);
		sleep(Duration::from_millis(250)).await;
		assert_eq!(query.iter().await.len(), 0);
	}
}
