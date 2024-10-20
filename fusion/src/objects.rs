#![allow(async_fn_in_trait)]

use crate::{
	client::ClientHandle,
	fields::{Field, FieldRef},
	node::NodeResult,
	spatial::{Spatial, SpatialAspect, SpatialRef},
};
use interfaces::FieldRefProxy;
pub use stardust_xr::schemas::dbus::*;
use stardust_xr::{
	schemas::{
		dbus::interfaces::{PlaySpaceProxy, SpatialRefProxy},
		zbus::Connection,
	},
	values::Vector2,
};
use std::sync::Arc;

pub trait SpatialRefProxyExt {
	async fn import(&self, stardust_client: &Arc<ClientHandle>) -> Option<SpatialRef>;
}
impl SpatialRefProxyExt for SpatialRefProxy<'_> {
	async fn import(&self, stardust_client: &Arc<ClientHandle>) -> Option<SpatialRef> {
		let uid = self.uid().await.ok()?;
		SpatialRef::import(stardust_client, uid).await.ok()
	}
}
pub struct SpatialObject(u64, Spatial);
impl SpatialObject {
	pub async fn new(spatial: Spatial) -> NodeResult<Self> {
		Ok(Self(spatial.export_spatial().await?, spatial))
	}
}
#[zbus::interface(name = "org.stardustxr.SpatialRef")]
impl SpatialObject {
	#[zbus(property)]
	async fn uid(&self) -> u64 {
		self.0
	}
}

pub trait FieldRefProxyExt {
	async fn import(&self, stardust_client: &Arc<ClientHandle>) -> Option<FieldRef>;
}
impl FieldRefProxyExt for FieldRefProxy<'_> {
	async fn import(&self, stardust_client: &Arc<ClientHandle>) -> Option<FieldRef> {
		let uid = self.uid().await.ok()?;
		FieldRef::import(stardust_client, uid).await.ok()
	}
}
pub struct FieldObject(u64, Field);
impl FieldObject {
	pub async fn new(field: Field) -> NodeResult<Self> {
		Ok(Self(field.export_spatial().await?, field))
	}
}
#[zbus::interface(name = "org.stardustxr.FieldRef")]
impl FieldObject {
	#[zbus(property)]
	async fn uid(&self) -> u64 {
		self.0
	}
}

pub async fn hmd(client: &Arc<ClientHandle>) -> Option<SpatialRef> {
	let connection = Connection::session().await.ok()?;
	let spatial_ref =
		SpatialRefProxy::new(&connection, "org.stardustxr.HMD", "/org/stardustxr/HMD")
			.await
			.ok()?;
	spatial_ref.import(client).await
}

#[tokio::test]
async fn fusion_objects_hmd() {
	use crate::spatial::SpatialRefAspect;

	let client = crate::Client::connect().await.unwrap();
	let client_handle = client.handle();
	client.async_event_loop();

	let hmd = hmd(&client_handle).await.unwrap();
	assert!(hmd.get_transform(client_handle.get_root()).await.is_ok())
}

pub struct PlaySpace {
	pub spatial: SpatialRef,
	pub bounds_polygon: Vec<Vector2<f32>>,
}
pub async fn play_space(client: &Arc<ClientHandle>) -> Option<PlaySpace> {
	let connection = Connection::session().await.ok()?;
	let spatial_proxy = SpatialRefProxy::new(
		&connection,
		"org.stardustxr.PlaySpace",
		"/org/stardustxr/PlaySpace",
	)
	.await
	.ok()?;
	let spatial = spatial_proxy.import(client).await?;
	let play_space_proxy = PlaySpaceProxy::new(&connection).await.ok()?;
	let bounds_polygon = play_space_proxy.bounds().await.ok()?;
	Some(PlaySpace {
		spatial,
		bounds_polygon: bounds_polygon
			.into_iter()
			.map(|(x, y)| [x as f32, y as f32].into())
			.collect(),
	})
}
