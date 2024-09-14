use crate::client::ClientHandle;
use crate::impl_aspects;
use crate::spatial::{SpatialRef, SpatialRefAspect};
use color_eyre::eyre::Result;
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use stardust_xr::schemas::flex::flexbuffers;
use std::sync::Arc;

stardust_xr_fusion_codegen::codegen_root_protocol!();
impl_aspects!(Root: SpatialRefAspect);

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
	) -> Result<Self> {
		Ok(ClientState {
			data: data.map(flexbuffers::to_vec).transpose()?,
			root: root.node().id(),
			spatial_anchors: spatial_anchors
				.into_iter()
				.map(|(k, v)| (k, v.node().id()))
				.collect(),
		})
	}
	pub fn from_data_root<T: Serialize>(
		data: Option<T>,
		root: &impl SpatialRefAspect,
	) -> Result<Self> {
		Self::new(data, root, FxHashMap::<String, &SpatialRef>::default())
	}
	pub fn from_root_anchors(
		root: &impl SpatialRefAspect,
		spatial_anchors: FxHashMap<String, &impl SpatialRefAspect>,
	) -> Result<Self> {
		Self::new(None::<()>, root, spatial_anchors)
	}
	pub fn from_root(root: &impl SpatialRefAspect) -> Result<Self> {
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
