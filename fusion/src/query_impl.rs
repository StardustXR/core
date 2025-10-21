use crate::{
	ClientHandle,
	fields::FieldRef,
	objects::{FieldRefProxyExt, SpatialRefProxyExt as _},
	spatial::SpatialRef,
};
use parking_lot::Mutex;
use stardust_xr::schemas::dbus::{
	interfaces::{FieldRefProxy, SpatialRefProxy},
	query::{QueryContext, Queryable},
};
use std::{
	collections::HashMap,
	sync::{Arc, LazyLock},
};
use zbus::names::InterfaceName;

impl QueryContext for ClientHandle {}

pub trait ClientQueryContext: QueryContext {
	fn get_client_handle(self: &Arc<Self>) -> &Arc<ClientHandle>;
}
impl ClientQueryContext for ClientHandle {
	fn get_client_handle(self: &Arc<Self>) -> &Arc<ClientHandle> {
		self
	}
}

impl<Ctx: ClientQueryContext> Queryable<Ctx> for SpatialRef {
	async fn try_new(
		connection: &zbus::Connection,
		ctx: &Arc<Ctx>,
		object: &stardust_xr::schemas::dbus::ObjectInfo,
		contains_interface: &(impl Fn(&InterfaceName) -> bool + Send + Sync),
	) -> Option<Self> {
		static CACHE: LazyLock<Mutex<HashMap<u64, SpatialRef>>> = LazyLock::new(Mutex::default);
		let proxy = SpatialRefProxy::try_new(connection, ctx, object, contains_interface).await?;
		let id = proxy.uid().await.ok()?;
		let v = CACHE.lock().get(&id).cloned();
		Some(match v {
			Some(v) => v,
			None => {
				let spatial_ref = proxy.import(ctx.get_client_handle()).await?;
				CACHE.lock().insert(id, spatial_ref.clone());
				spatial_ref
			}
		})
	}
}

impl<Ctx: ClientQueryContext> Queryable<Ctx> for FieldRef {
	async fn try_new(
		connection: &zbus::Connection,
		ctx: &Arc<Ctx>,
		object: &stardust_xr::schemas::dbus::ObjectInfo,
		contains_interface: &(impl Fn(&InterfaceName) -> bool + Send + Sync),
	) -> Option<Self> {
		static CACHE: LazyLock<Mutex<HashMap<u64, FieldRef>>> = LazyLock::new(Mutex::default);
		let proxy = FieldRefProxy::try_new(connection, ctx, object, contains_interface).await?;
		let id = proxy.uid().await.ok()?;
		let v = CACHE.lock().get(&id).cloned();
		Some(match v {
			Some(v) => v,
			None => {
				let spatial_ref = proxy.import(ctx.get_client_handle()).await?;
				CACHE.lock().insert(id, spatial_ref.clone());
				spatial_ref
			}
		})
	}
}
