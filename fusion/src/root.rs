use crate::client::ClientHandle;
use crate::spatial::{SpatialRef, SpatialRefAspect};
use rustc_hash::FxHashMap;
use serde::Serialize;
use serde::de::DeserializeOwned;
use stardust_xr_wire::flex::flexbuffers::{self, SerializationError};
use std::sync::Arc;

pub use crate::protocol::root::*;

/// The persistent state of a Stardust client.
impl Default for ClientState {
	fn default() -> Self {
		ClientState {
			data: None,
			root: 0,
			spatial_anchors: Default::default(),
		}
	}
}
impl ClientState {
	pub fn new<T: Serialize>(
		data: Option<T>,
		root: &impl SpatialRefAspect,
		spatial_anchors: FxHashMap<String, &impl SpatialRefAspect>,
	) -> Result<Self, SerializationError> {
		Ok(ClientState {
			data: data.map(flexbuffers::to_vec).transpose()?,
			root: root.id(),
			spatial_anchors: spatial_anchors
				.into_iter()
				.map(|(k, v)| (k, v.id()))
				.collect(),
		})
	}
	pub fn from_data_root<T: Serialize>(
		data: Option<T>,
		root: &impl SpatialRefAspect,
	) -> Result<Self, SerializationError> {
		Self::new(data, root, FxHashMap::<String, &SpatialRef>::default())
	}
	pub fn from_root_anchors(
		root: &impl SpatialRefAspect,
		spatial_anchors: FxHashMap<String, &impl SpatialRefAspect>,
	) -> Result<Self, SerializationError> {
		Self::new(None::<()>, root, spatial_anchors)
	}
	pub fn from_root(root: &impl SpatialRefAspect) -> Result<Self, SerializationError> {
		Self::from_data_root(None::<()>, root)
	}

	pub fn data<T: DeserializeOwned>(&self) -> Option<T> {
		flexbuffers::from_buffer(&self.data.as_ref()?.as_slice()).ok()
	}
	pub fn root(&self, client: &Arc<ClientHandle>) -> SpatialRef {
		SpatialRef::from_id(client, self.root, false)
	}
	pub fn spatial_anchors(&self, client: &Arc<ClientHandle>) -> FxHashMap<String, SpatialRef> {
		self.spatial_anchors
			.iter()
			.map(|(k, v)| (k.to_string(), SpatialRef::from_id(client, *v, false)))
			.collect()
	}
}

#[tokio::test]
async fn fusion_root_save_state() {
	use crate::Client;
	let mut client = Client::connect().await.expect("Couldn't connect");
	client
		.sync_event_loop(|client, flow| {
			let root = client.get_root();
			while let Some(event) = root.recv_root_event() {
				match event {
					RootEvent::Ping { response } => response.send_ok(()),
					RootEvent::Frame { info: _ } => (),
					RootEvent::SaveState { response } => {
						response.send(ClientState::from_data_root(Some(()), root));
						flow.stop();
					}
				}
			}
		})
		.await
		.unwrap();
}
