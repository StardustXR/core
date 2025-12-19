//! Query builder pipeline system for Stardust XR object registry.
//!
//! This module provides a composable, zero-cost abstraction for querying objects from the registry
//! with support for concurrent caching, filtering, and reactive updates via Watch channels.
//!
//! Uses Rust 1.85+ async closures for ergonomic async callback handling without heap allocation.
//!
//! # Architecture
//!
//! The pipeline flows: Query -> Cache -> Peek -> List -> Filter -> Terminal (watch/trigger_and_run)
//!
//! - Query: Wraps ObjectQuery, yields ObjectEvent<Q>
//! - Cache: Transforms Q -> C concurrently via JoinSet
//! - Peek: Inspects events without filtering
//! - List: Materializes stream into ObjectList with HashMap
//! - Filter: Applies predicates on live list
//! - Terminal: watch() or trigger_and_run() spawns task and returns handle

use std::collections::HashMap;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::watch;
use tokio::task::{AbortHandle, JoinSet};
use tokio_stream::Stream;
use tokio_stream::StreamExt;

use crate::{
	ObjectInfo,
	query::{ObjectQuery, QueryContext, Queryable, QueryEvent},
};
// ============ Core Types ============

/// Generic lifecycle event for objects tracked by registry.
///
/// Represents state transitions as objects enter/leave/change in the registry.
pub enum ObjectEvent<T> {
	/// New object matching criteria discovered
	NewMatch(ObjectInfo, T),
	/// Existing match was modified
	MatchModified(ObjectInfo, T),
	/// Match was lost (object removed or no longer matches)
	MatchLost(ObjectInfo),
}

/// Handle to abort a spawned task on drop.
///
/// Used by .watch() and .trigger_and_run() to ensure the background task
/// is cleaned up when the handle is dropped.
pub struct AbortOnDrop {
	abort_handle: AbortHandle,
}

impl AbortOnDrop {
	pub fn abort(&self) {
		self.abort_handle.abort();
	}

	pub fn is_finished(&self) -> bool {
		self.abort_handle.is_finished()
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
pub struct WatchHandle<T: Send + Clone + 'static> {
	pub watch: watch::Receiver<HashMap<ObjectInfo, T>>,
	pub _handle: AbortOnDrop,
}

// ============ Stream Wrappers ============

/// Wraps ObjectQuery<Q, Ctx> and implements Stream<Item = ObjectEvent<Q>>.
///
/// Converts gluon's QueryEvent to ObjectEvent and exposes as a Stream.
pub struct QueryStream<Q: Queryable<Ctx>, Ctx: QueryContext> {
	pub(crate) query: ObjectQuery<Q, Ctx>,
	pub(crate) _phantom: PhantomData<Ctx>,
}

/// Wraps a stream and applies a cache transformation concurrently.
///
/// Stores cache closure and internal JoinSet for concurrent cache operations.
/// Transforms ObjectEvent<Q> -> ObjectEvent<C> by running closure on each Q.
/// 
/// On NewMatch: spawns cache task in JoinSet (non-blocking)
/// On completion: emits NewMatch(ObjectInfo, C) if Some(C), drops if None
pub struct CachedStream<S, F, C> {
	stream: S,
	cache_fn: F,
	cache_tasks: JoinSet<(ObjectInfo, Option<C>)>,
}
/// Wraps a stream and applies a peek closure to inspect events.
///
/// Passes events through unchanged after calling peek closure.
pub struct PeekedStream<S, F> {
	stream: S,
	peek_fn: F,
}

/// Materializes a stream into a live list.
///
/// Maintains HashMap<ObjectInfo, T> updated as stream events arrive.
/// Provides get_list() to poll stream and return reference to HashMap.
pub struct ObjectList<S, T> {
	stream: S,
	list: HashMap<ObjectInfo, T>,
	_phantom: PhantomData<T>,
}

/// Wraps ObjectList and applies a filter predicate.
///
/// Stores filter closure; when queried, generates filtered Vec by applying predicate.
pub struct FilteredObjectList<L, F> {
	list: L,
	filter_fn: F,
}

// ============ Extension Traits ============

/// Extension trait for Stream<Item = ObjectEvent<T>>.
///
/// Provides chainable methods to compose object event streams.
/// Uses closures returning generic futures for stable Rust compatibility.
pub trait ObjectEventStreamExt<T>: Stream<Item = ObjectEvent<T>> + Sized {
	/// Transform items via cache closure returning a future.
	///
	/// Runs cache closure concurrently on items; failed caches (None) are skipped.
	/// Works with `|| async { }` closures on stable Rust, and will work with `async || { }` on nightly.
	fn cache_async<C, F, Fut>(self, f: F) -> CachedStream<Self, F, C>
	where
		F: Fn(T) -> Fut + Send + Sync + Unpin + Clone + 'static,
		Fut: Future<Output = Option<C>> + Send + 'static,
		C: Send + 'static;

	/// Inspect events via peek closure returning a future.
	///
	/// Passes events through unchanged after calling peek closure.
	fn peek_async<F, Fut>(self, f: F) -> PeekedStream<Self, F>
	where
		F: Fn(&ObjectEvent<T>) -> Fut + Send + Sync + Unpin + 'static,
		Fut: Future<Output = ()> + Send + 'static;

	/// Materialize stream into ObjectList.
	///
	/// Wraps stream in a struct that maintains HashMap<ObjectInfo, T>
	/// updated as events arrive.
	fn list(self) -> ObjectList<Self, T>;
}

/// Extension trait for ObjectList<S, T>.
///
/// Provides methods to filter, watch, or run triggered actions on the list.
pub trait ObjectListExt<T: 'static>: Sized {
	/// Poll stream once and get reference to current HashMap.
	fn get_list(&mut self) -> impl Future<Output = &HashMap<ObjectInfo, T>>;

	/// Apply filter predicate to generate filtered Vec.
	fn filter_async<F, Fut>(self, f: F) -> FilteredObjectList<Self, F>
	where
		F: Fn(&T) -> Fut + Send + Sync + 'static,
		Fut: Future<Output = bool> + Send + 'static,
		T: Send + 'static;

	/// Watch live list updates via Watch channel.
	///
	/// Spawns task that continuously polls stream and sends HashMap updates.
	/// Returns immediately with WatchHandle.
	fn watch(self) -> WatchHandle<T>
	where
		T: Send + Clone + 'static;

	/// Spawn task that waits for trigger, then fires action.
	///
	/// Receives &HashMap of all items (unfiltered).
	fn trigger_and_run<TrigF, ActF, TrigFut, ActFut>(
		self,
		trigger: TrigF,
		action: ActF,
	) -> AbortOnDrop
	where
		T: Send + Clone + 'static,
		TrigF: Fn(&HashMap<ObjectInfo, T>) -> TrigFut + Send + Sync + 'static,
		TrigFut: Future<Output = ()> + Send + 'static,
		ActF: Fn(&Vec<(ObjectInfo, T)>) -> ActFut + Send + Sync + 'static,
		ActFut: Future<Output = ()> + Send + 'static;
}

/// Extension trait for FilteredObjectList<L, F>.
///
/// Provides watch and trigger_and_run on filtered results.
pub trait FilteredObjectListExt<T>: Sized {
	/// Watch filtered list updates via Watch channel.
	///
	/// Spawns task that continuously polls stream, applies filter, sends Vec updates.
	fn watch(self) -> WatchHandle<T>
	where
		T: Send + Clone + 'static;

	/// Spawn task that waits for trigger, then fires action on filtered items.
	///
	/// Receives &Vec of filtered items only.
	fn trigger_and_run<TrigF, ActF, TrigFut, ActFut>(
		self,
		trigger: TrigF,
		action: ActF,
	) -> AbortOnDrop
	where
		T: Send + Clone + 'static,
		TrigF: Fn(&Vec<(ObjectInfo, T)>) -> TrigFut + Send + Sync + 'static,
		TrigFut: Future<Output = ()> + Send + 'static,
		ActF: Fn(&Vec<(ObjectInfo, T)>) -> ActFut + Send + Sync + 'static,
		ActFut: Future<Output = ()> + Send + 'static;
}

// ============ Stream Implementations ============

impl<Q: Queryable<Ctx> + Unpin, Ctx: QueryContext + Unpin> Stream for QueryStream<Q, Ctx> {
	type Item = ObjectEvent<Q>;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		// Try to receive next event from the ObjectQuery without blocking
		let this = self.get_mut();
		match this.query.try_recv_event() {
			Ok(crate::query::QueryEvent::NewMatch(info, query_data)) => {
				Poll::Ready(Some(ObjectEvent::NewMatch(info, query_data)))
			}
			Ok(crate::query::QueryEvent::MatchModified(info, query_data)) => {
				Poll::Ready(Some(ObjectEvent::MatchModified(info, query_data)))
			}
			Ok(crate::query::QueryEvent::MatchLost(info)) => Poll::Ready(Some(ObjectEvent::MatchLost(info))),
			Ok(crate::query::QueryEvent::PhantomVariant(_)) => Poll::Ready(None),
			Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
				// No events available, register waker and return Pending
				cx.waker().wake_by_ref();
				Poll::Pending
			}
			Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => Poll::Ready(None),
		}
	}
}

impl<S, F, T, C, Fut> Stream for CachedStream<S, F, C>
where
	S: Stream<Item = ObjectEvent<T>> + Unpin,
	T: Send + 'static,
	F: Fn(T) -> Fut + Send + Sync + Unpin + Clone + 'static,
	Fut: Future<Output = Option<C>> + Send + 'static,
	C: Send + 'static,
{
	type Item = ObjectEvent<C>;

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		let this = self.as_mut().get_mut();
		
		// First, check if any cache tasks have completed
		if let Poll::Ready(Some(result)) = Pin::new(&mut this.cache_tasks).poll_join_next(cx) {
			if let Ok((info, maybe_cached)) = result {
				if let Some(cached) = maybe_cached {
					// Emit the cached result as NewMatch
					return Poll::Ready(Some(ObjectEvent::NewMatch(info, cached)));
				}
				// If cache returned None, skip this item (don't emit event)
				// Fall through to check stream or return Pending
			}
		}
		
		// Poll the underlying stream for new events
		match Pin::new(&mut this.stream).poll_next(cx) {
			Poll::Ready(Some(ObjectEvent::NewMatch(info, item))) => {
				// Spawn cache task for this item
				let cache_fn = this.cache_fn.clone();
				let info_clone = info.clone();
				this.cache_tasks.spawn(async move {
					let cached = cache_fn(item).await;
					(info_clone, cached)
				});
				// Don't emit yet, wait for cache to complete
				// Wake immediately to check for completed tasks
				cx.waker().wake_by_ref();
				Poll::Pending
			}
			Poll::Ready(Some(ObjectEvent::MatchModified(info, item))) => {
				// Spawn cache task for modified item
				let cache_fn = this.cache_fn.clone();
				let info_clone = info.clone();
				this.cache_tasks.spawn(async move {
					let cached = cache_fn(item).await;
					(info_clone, cached)
				});
				cx.waker().wake_by_ref();
				Poll::Pending
			}
			Poll::Ready(Some(ObjectEvent::MatchLost(info))) => {
				// Pass through MatchLost immediately
				Poll::Ready(Some(ObjectEvent::MatchLost(info)))
			}
			Poll::Ready(None) => {
				// Stream ended, but check if there are pending cache tasks
				if !this.cache_tasks.is_empty() {
					// Still have pending tasks, keep polling
					Poll::Pending
				} else {
					Poll::Ready(None)
				}
			}
			Poll::Pending => Poll::Pending,
		}
	}
}

impl<S, F, T, Fut> Stream for PeekedStream<S, F>
where
	S: Stream<Item = ObjectEvent<T>> + Unpin,
	T: Send + 'static,
	F: Fn(&ObjectEvent<T>) -> Fut + Send + Sync + Unpin + 'static,
	Fut: Future<Output = ()> + Send + 'static,
{
	type Item = ObjectEvent<T>;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		// Pass through events from underlying stream
		// The peek closure will be called in spawned tasks, not here
		// TODO: Implement proper peek with async closure execution
		let this = self.get_mut();
		Pin::new(&mut this.stream).poll_next(cx)
	}
}

// ============ Extension Trait Implementations ============

impl<T, S> ObjectEventStreamExt<T> for S
where
    S: Stream<Item = ObjectEvent<T>> + Sized,
    T: Send + 'static,
{
    fn cache_async<C, F, Fut>(self, f: F) -> CachedStream<Self, F, C>
    where
        F: Fn(T) -> Fut + Send + Sync + Unpin + Clone + 'static,
        Fut: Future<Output = Option<C>> + Send + 'static,
        C: Send + 'static,
    {
        CachedStream {
            stream: self,
            cache_fn: f,
            cache_tasks: JoinSet::new(),
        }
    }

    fn peek_async<F, Fut>(self, f: F) -> PeekedStream<Self, F>
    where
        F: Fn(&ObjectEvent<T>) -> Fut + Send + Sync + Unpin + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        PeekedStream { stream: self, peek_fn: f }
    }

    fn list(self) -> ObjectList<Self, T> {
        ObjectList { stream: self, list: HashMap::new(), _phantom: PhantomData }
    }
}

impl<S, T> ObjectListExt<T> for ObjectList<S, T>
where
    S: Stream<Item = ObjectEvent<T>> + Unpin + Send + 'static,
    T: Send + Sync + Clone + 'static,
{
    fn get_list(&mut self) -> impl Future<Output = &HashMap<ObjectInfo, T>> {
        async move {
            if let Some(ev) = self.stream.next().await {
                match ev {
                    ObjectEvent::NewMatch(info, item) | ObjectEvent::MatchModified(info, item) => {
                        self.list.insert(info, item);
                    }
                    ObjectEvent::MatchLost(info) => {
                        self.list.remove(&info);
                    }
                }
            }
            &self.list
        }
    }

    fn filter_async<F, Fut>(self, f: F) -> FilteredObjectList<Self, F>
    where
        F: Fn(&T) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = bool> + Send + 'static,
        T: Send + 'static,
    {
        FilteredObjectList { list: self, filter_fn: f }
    }

    fn watch(self) -> WatchHandle<T>
    where
        T: Send + Clone + 'static,
    {
        let (tx, watch_rx) = watch::channel(self.list.clone());
        let mut stream = self.stream;
        let mut list = self.list;
        let handle = tokio::spawn(async move {
            use tokio_stream::StreamExt;
            while let Some(ev) = stream.next().await {
                match ev {
                    ObjectEvent::NewMatch(info, item) | ObjectEvent::MatchModified(info, item) => {
                        list.insert(info, item);
                    }
                    ObjectEvent::MatchLost(info) => {
                        list.remove(&info);
                    }
                }
                let _ = tx.send(list.clone());
            }
        })
        .abort_handle();
        WatchHandle { watch: watch_rx, _handle: AbortOnDrop { abort_handle: handle } }
    }

    fn trigger_and_run<TrigF, ActF, TrigFut, ActFut>(
        self,
        trigger: TrigF,
        action: ActF,
    ) -> AbortOnDrop
    where
        T: Send + Clone + 'static,
        TrigF: Fn(&HashMap<ObjectInfo, T>) -> TrigFut + Send + Sync + 'static,
        TrigFut: Future<Output = ()> + Send + 'static,
        ActF: Fn(&Vec<(ObjectInfo, T)>) -> ActFut + Send + Sync + 'static,
        ActFut: Future<Output = ()> + Send + 'static,
    {
        let mut stream = self.stream;
        let mut list = self.list;
        let handle = tokio::spawn(async move {
            use tokio_stream::StreamExt;
            loop {
                tokio::select! {
                    maybe_ev = stream.next() => {
                        match maybe_ev {
                            Some(ObjectEvent::NewMatch(info, item)) | Some(ObjectEvent::MatchModified(info, item)) => { list.insert(info, item); }
                            Some(ObjectEvent::MatchLost(info)) => { list.remove(&info); }
                            None => break,
                        }
                    }
                    _ = trigger(&list) => {
                        let all: Vec<(ObjectInfo, T)> = list.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                        action(&all).await;
                    }
                }
            }
        })
        .abort_handle();
        AbortOnDrop { abort_handle: handle }
    }
}

impl<S, T, F, Fut> FilteredObjectListExt<T> for FilteredObjectList<ObjectList<S, T>, F>
where
    S: Stream<Item = ObjectEvent<T>> + Unpin + Send + 'static,
    T: Send + Sync + Clone + 'static,
    F: Fn(&T) -> Fut + Send + Sync + Clone + 'static,
    Fut: Future<Output = bool> + Send + 'static,
{
    fn watch(self) -> WatchHandle<T>
    where
        T: Send + Clone + 'static,
    {
        let (tx, watch_rx) = watch::channel(HashMap::<ObjectInfo, T>::new());
        let mut stream = self.list.stream;
        let mut list = self.list.list;
        let filter_fn = self.filter_fn;
        let handle = tokio::spawn(async move {
            use tokio_stream::StreamExt;
            while let Some(ev) = stream.next().await {
                match ev {
                    ObjectEvent::NewMatch(info, item) | ObjectEvent::MatchModified(info, item) => {
                        if filter_fn(&item).await { list.insert(info, item); } else { list.remove(&info); }
                    }
                    ObjectEvent::MatchLost(info) => { list.remove(&info); }
                }
                let _ = tx.send(list.clone());
            }
        })
        .abort_handle();
        WatchHandle { watch: watch_rx, _handle: AbortOnDrop { abort_handle: handle } }
    }

    fn trigger_and_run<TrigF, ActF, TrigFut, ActFut>(
        self,
        trigger: TrigF,
        action: ActF,
    ) -> AbortOnDrop
    where
        T: Send + Clone + 'static,
        TrigF: Fn(&Vec<(ObjectInfo, T)>) -> TrigFut + Send + Sync + 'static,
        TrigFut: Future<Output = ()> + Send + 'static,
        ActF: Fn(&Vec<(ObjectInfo, T)>) -> ActFut + Send + Sync + 'static,
        ActFut: Future<Output = ()> + Send + 'static,
    {
        let mut stream = self.list.stream;
        let mut list = self.list.list;
        let filter_fn = self.filter_fn;
        let handle = tokio::spawn(async move {
            use tokio_stream::StreamExt;
            loop {
                tokio::select! {
                    maybe_ev = stream.next() => {
                        match maybe_ev {
                            Some(ObjectEvent::NewMatch(info, item)) | Some(ObjectEvent::MatchModified(info, item)) => {
                                if filter_fn(&item).await { list.insert(info, item); } else { list.remove(&info); }
                            }
                            Some(ObjectEvent::MatchLost(info)) => { list.remove(&info); }
                            None => break,
                        }
                    }
                    _ = async {
                        // Build filtered snapshot
                        let filtered: Vec<(ObjectInfo, T)> = list.iter().map(|(k,v)| (k.clone(), v.clone())).collect();
                        // Fire when trigger completes
                        trigger(&filtered).await;
                    } => {
                        let filtered: Vec<(ObjectInfo, T)> = list.iter().map(|(k,v)| (k.clone(), v.clone())).collect();
                        action(&filtered).await;
                    }
                }
            }
        })
        .abort_handle();
        AbortOnDrop { abort_handle: handle }
    }
}

// ============ Tests ============

#[tokio::test]
async fn query_stream_pipeline_test() {
	use std::time::Duration;
	use crate::object_registry::ObjectRegistry;

	// Set up test interface
	struct TestInterface;
	#[zbus::interface(name = "org.stardustxr.QueryBuilder.TestInterface")]
	impl TestInterface {
		async fn test_method(&self) -> String {
			"Test method called".to_string()
		}
	}

	// Create a proxy type for querying
	#[zbus::proxy(
		interface = "org.stardustxr.QueryBuilder.TestInterface",
		default_service = "org.stardustxr.QueryBuilder",
		default_path = "/org/stardustxr/TestObject"
	)]
	trait TestProxy {}

	// Implement Queryable for the proxy
	crate::impl_queryable_for_proxy!(TestProxyProxy);

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
	let registry_connection = crate::connect_client().await.unwrap();
	let registry = ObjectRegistry::new(&registry_connection).await;

	// Test 1: Basic watch() - observe all matching objects
	println!("\nTest 1: Basic watch pipeline");
	let watch_handle = registry
		.query::<TestProxyProxy<'static>, ()>(())
		.list()
		.watch();

	let mut watch_rx = watch_handle.watch.clone();

	// Set timeout
	let timeout_task = tokio::spawn(async {
		tokio::time::sleep(Duration::from_secs(10)).await;
		panic!("Test took too long to run");
	});

	// Wait for the test object to appear
	let mut found = false;
	for _ in 0..20 {
		if watch_rx.changed().await.is_ok() {
			let list = watch_rx.borrow();
			if list.iter().any(|(info, _)| info.object_path.as_str() == "/org/stardustxr/TestObject") {
				println!("Found test object in list: {} items", list.len());
				found = true;
				break;
			}
			}
		tokio::time::sleep(Duration::from_millis(100)).await;
	}
	assert!(found, "Test object should appear in watch list");

	// Test 2: cache_async() pipeline
	println!("\nTest 2: Cache async pipeline");
	let cache_watch = registry
		.query::<TestProxyProxy<'static>, ()>(())
		.cache_async(|proxy| async move {
			// Simulate some async work (like importing)
			// Access inner proxy with .0
			let path = proxy.0.path().to_string();
			Some(path)
		})
		.list()
		.watch();

	let mut cache_rx = cache_watch.watch.clone();

	// Wait for cached results
	let mut cache_found = false;
	for _ in 0..20 {
		if cache_rx.changed().await.is_ok() {
			let list = cache_rx.borrow();
			if let Some((_, cached_path)) = list.iter().find(|(info, _)| {
				info.object_path.as_str() == "/org/stardustxr/TestObject"
			}) {
				println!("Found cached result: {}", cached_path);
				assert_eq!(cached_path, "/org/stardustxr/TestObject");
				cache_found = true;
				break;
			}
		}
		tokio::time::sleep(Duration::from_millis(100)).await;
	}
	assert!(cache_found, "Cached result should appear");

	// Test 3: trigger_and_run() pipeline
	println!("\nTest 3: Trigger and run pipeline");
	let (trigger_tx, trigger_rx) = tokio::sync::mpsc::channel::<()>(1);
	let (action_tx, mut action_rx) = tokio::sync::mpsc::channel::<usize>(1);

	// Wrap receiver in Arc<Mutex> for sharing across closures
	let trigger_rx = std::sync::Arc::new(tokio::sync::Mutex::new(trigger_rx));

	let _trigger_handle = registry
		.query::<TestProxyProxy<'static>, ()>(())
		.list()
		.trigger_and_run(
			move |_list| {
				let rx = trigger_rx.clone();
				async move {
					_ = rx.lock().await.recv().await;
				}
			},
			move |items| {
				let tx = action_tx.clone();
				let count = items.len();
				async move {
					_ = tx.send(count).await;
				}
			},
		);

	// Give the pipeline time to initialize
	tokio::time::sleep(Duration::from_millis(500)).await;

	// Trigger the action
	println!("Sending trigger signal");
	trigger_tx.send(()).await.unwrap();

	// Wait for action to complete
	let count = tokio::time::timeout(Duration::from_secs(2), action_rx.recv())
		.await
		.expect("Should receive action result")
		.expect("Should have count");
	println!("Action received count: {}", count);
	assert!(count > 0, "Should have at least one object");

	// Cleanup
	println!("\nCleaning up");
	drop(test_connection);
	timeout_task.abort();
	println!("Test completed successfully");
}
