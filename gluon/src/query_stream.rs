//! Stream extension traits for object event streams.
//!
//! Provides cache_async and inspect_async combinators for transforming
//! streams of ObjectEvent items with concurrent async operations.

use crate::{
	ObjectInfo,
	object_registry::ObjectRegistry,
	query::{ObjectQuery, QueryContext, Queryable},
};
use futures::{Stream, stream::StreamExt};
use std::{
	collections::HashMap,
	future::Future,
	pin::{Pin, pin},
	sync::Arc,
	task::{Context, Poll},
};
use tokio::{
	sync::watch,
	task::{AbortHandle, JoinHandle},
};

/// Generic lifecycle event for objects tracked by registry.
///
/// Represents state transitions as objects enter/leave/change in the registry.
#[derive(Debug, Clone)]
pub enum ObjectEvent<T> {
	/// New object matching criteria discovered
	NewMatch(ObjectInfo, T),
	/// Existing match was modified
	MatchModified(ObjectInfo, T),
	/// Match was lost (object removed or no longer matches)
	MatchLost(ObjectInfo),
}

/// Wraps ObjectQuery<Q, Ctx> and implements Stream<Item = ObjectEvent<Q>>.
///
/// Converts gluon's QueryEvent to ObjectEvent and exposes as a Stream.
pub struct QueryStream<Q: Queryable<Ctx>, Ctx: QueryContext>(ObjectQuery<Q, Ctx>);
impl<Q: Queryable<Ctx> + Unpin, Ctx: QueryContext + Unpin> Stream for QueryStream<Q, Ctx> {
	type Item = ObjectEvent<Q>;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		let this = self.get_mut();
		match this.0.try_recv_event() {
			Ok(crate::query::QueryEvent::NewMatch(info, query_data)) => {
				Poll::Ready(Some(ObjectEvent::NewMatch(info, query_data)))
			}
			Ok(crate::query::QueryEvent::MatchModified(info, query_data)) => {
				Poll::Ready(Some(ObjectEvent::MatchModified(info, query_data)))
			}
			Ok(crate::query::QueryEvent::MatchLost(info)) => {
				Poll::Ready(Some(ObjectEvent::MatchLost(info)))
			}
			Ok(crate::query::QueryEvent::PhantomVariant(_)) => Poll::Ready(None),
			Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
				cx.waker().wake_by_ref();
				Poll::Pending
			}
			Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => Poll::Ready(None),
		}
	}
}

/// Handle to abort a spawned task on drop.
///
/// Used by .watch() and .trigger_and_run() to ensure the background task
/// is cleaned up when the handle is dropped.
pub struct AbortOnDrop {
	abort_handle: AbortHandle,
}
impl AbortOnDrop {
	pub fn is_finished(&self) -> bool {
		self.abort_handle.is_finished()
	}
}
impl<T> From<JoinHandle<T>> for AbortOnDrop {
	fn from(value: JoinHandle<T>) -> Self {
		Self {
			abort_handle: value.abort_handle(),
		}
	}
}

impl From<AbortHandle> for AbortOnDrop {
	fn from(abort_handle: AbortHandle) -> Self {
		Self { abort_handle }
	}
}
impl Drop for AbortOnDrop {
	fn drop(&mut self) {
		self.abort_handle.abort();
	}
}

/// Handle to watch live updates of a list via Watch channel.
///
/// Created by .watch(), allows subscribers to be notified of changes
/// to the underlying HashMap or filtered Vec.
pub struct WatchHandle<T: Send + Sync + 'static> {
	pub watch: watch::Receiver<HashMap<ObjectInfo, T>>,
	_handle: AbortOnDrop,
}

/// Extension trait for Stream<Item = ObjectEvent<T>>.
///
/// Provides chainable methods to compose object event streams with
/// concurrent async operations using futures::stream combinators.
pub trait ObjectEventStreamExt<T: Send + Sync + 'static>:
	Stream<Item = ObjectEvent<T>> + Sized + Send + 'static
{
	/// Transform items via cache closure returning a future.
	///
	/// Runs cache operations concurrently (up to 16 at a time) and maintains order.
	/// Failed caches (None) are skipped. Uses futures::stream::buffered internally.
	fn cache_async<C: Send, F: FnMut(T) -> Fut + 'static, Fut: Future<Output = Option<C>>>(
		self,
		f: F,
	) -> impl Stream<Item = ObjectEvent<C>>;

	/// Inspect events via closure returning a future.
	///
	/// Similar to Iterator::inspect - calls closure for side effects,
	/// then passes events through unchanged. Runs concurrently (up to 16 at a time).
	fn inspect_async<F: FnMut(&ObjectEvent<T>) -> Fut, Fut: Future<Output = ()>>(
		self,
		f: F,
	) -> impl Stream<Item = ObjectEvent<T>>;

	/// Watch live list updates via Watch channel.
	///
	/// Spawns task that continuously polls stream and sends HashMap updates.
	/// Returns immediately with WatchHandle.
	fn watch(self) -> WatchHandle<T>;

	/// Spawn task that waits for trigger, then fires action.
	///
	/// Receives &HashMap of all items (unfiltered).
	fn trigger_and_run<
		TrigF: for<'a> Fn(&'a HashMap<ObjectInfo, T>) -> TrigFut + Send + 'static,
		ActF: for<'a> Fn(&'a HashMap<ObjectInfo, T>) -> ActFut + Send + 'static,
		TrigFut: Future<Output = ()> + Send + 'static,
		ActFut: for<'a> Future<Output = ()> + Send,
	>(
		self,
		trigger: TrigF,
		action: ActF,
	) -> AbortOnDrop;
}

impl<T: Send + Sync + 'static, S: Stream<Item = ObjectEvent<T>> + Sized + Send + 'static>
	ObjectEventStreamExt<T> for S
{
	fn cache_async<C: Send, F: FnMut(T) -> Fut + 'static, Fut: Future<Output = Option<C>>>(
		self,
		mut f: F,
	) -> impl Stream<Item = ObjectEvent<C>> {
		self.map(move |event| {
			let cached_event = match event {
				ObjectEvent::NewMatch(info, item) => ObjectEvent::NewMatch(info, f(item)),
				ObjectEvent::MatchModified(info, item) => ObjectEvent::MatchModified(info, f(item)),
				ObjectEvent::MatchLost(info) => ObjectEvent::MatchLost(info),
			};
			async move {
				match cached_event {
					ObjectEvent::NewMatch(info, item) => {
						Some(ObjectEvent::NewMatch(info, item.await?))
					}
					ObjectEvent::MatchModified(info, item) => {
						Some(ObjectEvent::MatchModified(info, item.await?))
					}
					ObjectEvent::MatchLost(info) => Some(ObjectEvent::MatchLost(info)),
				}
			}
		})
		.buffered(16)
		.filter_map(futures::future::ready)
	}

	fn inspect_async<F: FnMut(&ObjectEvent<T>) -> Fut, Fut: Future<Output = ()>>(
		self,
		mut f: F,
	) -> impl Stream<Item = ObjectEvent<T>> {
		self.map(move |event| {
			let mapped = f(&event);
			async move {
				mapped.await;
				event
			}
		})
		.buffered(16)
	}

	fn watch(self) -> WatchHandle<T> {
		let (watch_tx, watch) = watch::channel(HashMap::new());
		let handle = tokio::task::spawn(async move {
			let mut stream = pin!(self);
			while let Some(ev) = stream.next().await {
				watch_tx.send_if_modified(|list| match ev {
					ObjectEvent::NewMatch(info, item) | ObjectEvent::MatchModified(info, item) => {
						list.insert(info, item);
						true
					}
					ObjectEvent::MatchLost(info) => list.remove(&info).is_some(),
				});
			}
		})
		.abort_handle();
		WatchHandle {
			watch,
			_handle: AbortOnDrop {
				abort_handle: handle,
			},
		}
	}

	fn trigger_and_run<
		TrigF: for<'a> Fn(&'a HashMap<ObjectInfo, T>) -> TrigFut + Send + 'static,
		ActF: for<'a> Fn(&'a HashMap<ObjectInfo, T>) -> ActFut + Send + 'static,
		TrigFut: Future<Output = ()> + Send + 'static,
		ActFut: for<'a> Future<Output = ()> + Send,
	>(
		self,
		trigger: TrigF,
		action: ActF,
	) -> AbortOnDrop {
		let handle = tokio::spawn(async move {
			let mut stream = pin!(self);
			let mut list = HashMap::new();
			loop {
				tokio::select! {
					maybe_ev = stream.next() => {
						match maybe_ev {
							Some(ObjectEvent::NewMatch(info, item)) | Some(ObjectEvent::MatchModified(info, item)) => {
								list.insert(info, item);
							}
							Some(ObjectEvent::MatchLost(info)) => { list.remove(&info); }
							None => break,
						}
					}
					_ = trigger(&list) => {
						action(&list).await;
					}
				}
			}
		})
		.abort_handle();
		AbortOnDrop {
			abort_handle: handle,
		}
	}
}

impl ObjectRegistry {
	/// Create a QueryStream for the given queryable type.
	///
	/// This returns a Stream that yields ObjectEvent<Q> as objects matching
	/// the query criteria appear, change, or disappear.
	pub fn query<Q: Queryable<Ctx>, Ctx: QueryContext>(
		self: &Arc<Self>,
		context: impl Into<Arc<Ctx>>,
	) -> QueryStream<Q, Ctx> {
		QueryStream(ObjectQuery::new(self.clone(), context))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use futures::stream::StreamExt;
	use std::sync::atomic::{AtomicUsize, Ordering};
	use std::time::Duration;
	use tokio::time::timeout;
	use zbus::interface;

	struct TestInterface;
	#[interface(name = "org.stardustxr.QueryStream.TestInterface", proxy())]
	impl TestInterface {
		pub fn ping(&self) {
			println!("ping");
		}
	}
	impl_queryable_for_proxy!(TestInterfaceProxy);

	async fn setup() -> (zbus::Connection, Arc<ObjectRegistry>) {
		tokio::task::spawn(async {
			tokio::time::sleep(Duration::from_secs(10)).await;
			panic!("Test took too long to run");
		});

		let service_conn = zbus::conn::Builder::session()
			.unwrap()
			.serve_at("/", zbus::fdo::ObjectManager)
			.unwrap()
			.serve_at("/org/stardustxr/TestObject1", TestInterface)
			.unwrap()
			.serve_at("/org/stardustxr/TestObject2", TestInterface)
			.unwrap()
			.build()
			.await
			.unwrap();

		let registry_conn = zbus::Connection::session().await.unwrap();
		let registry = ObjectRegistry::new(&registry_conn).await;

		(service_conn, registry)
	}

	#[tokio::test]
	async fn test_basic_query_stream_polling() {
		let (_service_conn, registry) = setup().await;
		let mut stream = registry.query::<TestInterfaceProxy, ()>(());
		let mut received_new_match = false;
		while let Some(event) =
			tokio::time::timeout(Duration::from_millis(500), stream.0.recv_event())
				.await
				.ok()
				.flatten()
		{
			if matches!(event, crate::query::QueryEvent::NewMatch(_, _)) {
				received_new_match = true;
				break;
			}
		}
		assert!(received_new_match, "Should receive NewMatch event");
	}

	#[tokio::test]
	async fn test_cache_async_transformation() {
		let (_service_conn, registry) = setup().await;
		let cache_count = Arc::new(AtomicUsize::new(0));
		let cache_count_clone = cache_count.clone();
		let mut cached_stream =
			registry
				.query::<TestInterfaceProxy, ()>(())
				.cache_async(move |_proxy| {
					let count = cache_count_clone.clone();
					async move {
						count.fetch_add(1, Ordering::SeqCst);
						Some("cached_value".to_string())
					}
				});

		let mut cached_count = 0;
		while let Ok(Some(event)) = timeout(Duration::from_millis(500), cached_stream.next()).await
		{
			if matches!(event, ObjectEvent::NewMatch(_, ref data) if data == "cached_value") {
				cached_count += 1;
			}
		}
		// Should receive 2 cached events (one for each test object)
		assert_eq!(cached_count, 2, "Should receive 2 cached events");
		assert_eq!(
			cache_count.load(Ordering::SeqCst),
			2,
			"Cache should have executed exactly 2 times"
		);
	}

	#[tokio::test]
	async fn test_inspect_async_side_effects() {
		let (_service_conn, registry) = setup().await;
		let inspect_count = Arc::new(AtomicUsize::new(0));
		let inspect_count_clone = inspect_count.clone();
		let mut inspected_stream =
			registry
				.query::<TestInterfaceProxy, ()>(())
				.inspect_async(move |_event| {
					let count = inspect_count_clone.clone();
					async move {
						count.fetch_add(1, Ordering::SeqCst);
					}
				});

		let mut inspected_received = false;
		while let Ok(Some(event)) =
			timeout(Duration::from_millis(500), inspected_stream.next()).await
		{
			if matches!(event, ObjectEvent::NewMatch(_, _)) {
				inspected_received = true;
				break;
			}
		}
		assert!(
			inspected_received,
			"Should receive event from inspect_async"
		);
		assert!(
			inspect_count.load(Ordering::SeqCst) > 0,
			"Inspect should have executed"
		);
	}

	#[tokio::test]
	async fn test_watch_live_updates() {
		let (_service_conn, registry) = setup().await;
		let watch_handle = registry.query::<TestInterfaceProxy, ()>(()).watch();
		let watch_rx = watch_handle.watch.clone();
		tokio::time::sleep(Duration::from_millis(200)).await;
		let initial_count = watch_rx.borrow().len();
		assert!(initial_count > 0, "Watch should have initial items");
	}

	#[tokio::test]
	async fn test_trigger_and_run_with_action() {
		let (_service_conn, registry) = setup().await;
		let (trigger_tx, trigger_rx) = tokio::sync::oneshot::channel();
		let action_count = Arc::new(AtomicUsize::new(0));
		let action_count_clone = action_count.clone();

		let trigger_rx = Arc::new(tokio::sync::Mutex::new(Some(trigger_rx)));
		let trigger_rx_clone = trigger_rx.clone();

		let abort_handle = registry
			.query::<TestInterfaceProxy, ()>(())
			.trigger_and_run(
				move |_list| {
					let rx = trigger_rx_clone.clone();
					async move {
						if let Some(rx) = rx.lock().await.take() {
							_ = rx.await;
						}
					}
				},
				move |items| {
					let count = action_count_clone.clone();
					let len = items.len();
					async move {
						count.store(len, Ordering::SeqCst);
					}
				},
			);

		tokio::time::sleep(Duration::from_millis(100)).await;
		let _ = trigger_tx.send(());
		tokio::time::sleep(Duration::from_millis(100)).await;

		let action_items = action_count.load(Ordering::SeqCst);
		// Should have processed both test objects
		assert_eq!(action_items, 2, "Action should have processed 2 items");
		assert!(
			!abort_handle.is_finished(),
			"Handle should still be running after action"
		);
	}

	#[tokio::test]
	async fn test_abort_on_drop_cleanup() {
		let (_service_conn, registry) = setup().await;
		let (trigger_tx2, trigger_rx2) = tokio::sync::oneshot::channel::<()>();
		let trigger_rx2 = Arc::new(tokio::sync::Mutex::new(Some(trigger_rx2)));
		let trigger_rx2_clone = trigger_rx2.clone();

		let abort_handle2 = registry
			.query::<TestInterfaceProxy, ()>(())
			.trigger_and_run(
				move |_list| {
					let rx = trigger_rx2_clone.clone();
					async move {
						if let Some(rx) = rx.lock().await.take() {
							_ = rx.await;
						}
					}
				},
				|_| async move {},
			);

		tokio::time::sleep(Duration::from_millis(50)).await;
		assert!(!abort_handle2.is_finished(), "Task should be running");
		let _ = trigger_tx2.send(());
		tokio::time::sleep(Duration::from_millis(50)).await;
		drop(abort_handle2);
		tokio::time::sleep(Duration::from_millis(50)).await;
	}

	#[tokio::test]
	async fn test_chained_cache_and_inspect_async() {
		let (_service_conn, registry) = setup().await;
		let transform_count = Arc::new(AtomicUsize::new(0));
		let transform_count_clone = transform_count.clone();
		let inspect_count2 = Arc::new(AtomicUsize::new(0));
		let inspect_count2_clone = inspect_count2.clone();

		let mut chained_stream = registry
			.query::<TestInterfaceProxy, ()>(())
			.cache_async(move |_proxy| {
				let count = transform_count_clone.clone();
				async move {
					count.fetch_add(1, Ordering::SeqCst);
					Some("transformed".to_string())
				}
			})
			.inspect_async(move |_event| {
				let count = inspect_count2_clone.clone();
				async move {
					count.fetch_add(1, Ordering::SeqCst);
				}
			});

		let mut event_count = 0;
		while event_count < 2 {
			if let Ok(Some(_)) = timeout(Duration::from_millis(500), chained_stream.next()).await {
				event_count += 1;
			} else {
				break;
			}
		}

		let transform_execs = transform_count.load(Ordering::SeqCst);
		let inspect_execs = inspect_count2.load(Ordering::SeqCst);
		assert!(transform_execs > 0, "Transform should have executed");
		assert!(inspect_execs > 0, "Inspect should have executed");
		assert_eq!(
			transform_execs, inspect_execs,
			"Transform and inspect should execute equally"
		);
	}

	#[tokio::test]
	async fn test_none_filtering_in_cache_async() {
		let (_service_conn, registry) = setup().await;
		let filter_count = Arc::new(AtomicUsize::new(0));
		let filter_count_clone = filter_count.clone();
		let mut filtered_stream =
			registry
				.query::<TestInterfaceProxy, ()>(())
				.cache_async(move |_proxy| {
					let count = filter_count_clone.clone();
					async move {
						count.fetch_add(1, Ordering::SeqCst);
						None::<String>
					}
				});

		let mut filtered_count = 0;
		while let Ok(Some(_)) = timeout(Duration::from_millis(500), filtered_stream.next()).await {
			filtered_count += 1;
		}
		let filter_attempts = filter_count.load(Ordering::SeqCst);
		// Should have attempted to process both objects
		assert_eq!(
			filter_attempts, 2,
			"Filter should have attempted to process 2 items"
		);
		// But none should pass through (all returned None)
		assert_eq!(filtered_count, 0, "None results should be filtered out");
	}
}
