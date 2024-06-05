use crate::impl_aspects;
use crate::node::{NodeResult, NodeType};
use crate::spatial::{SpatialRef, SpatialRefAspect};
use color_eyre::eyre::Result;
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use stardust_xr::schemas::flex::flexbuffers;

stardust_xr_fusion_codegen::codegen_root_protocol!();
impl_aspects!(Root: SpatialRefAspect);

/// The persistent state of a Stardust client.
#[derive(Debug)]
pub struct ClientStateParsed<T: Serialize + DeserializeOwned> {
	/// Data specific to your client, put anything you like here and it'll be saved/restored intact.
	pub data: Option<T>,
	/// The root node of this client.
	pub root: SpatialRef,
	/// Spatials that will be in the same place you left them.
	pub spatial_anchors: FxHashMap<String, SpatialRef>,
}
impl ClientStateParsed<()> {
	pub fn from_root_anchors(
		root: &impl SpatialRefAspect,
		spatial_anchors: FxHashMap<String, &impl SpatialRefAspect>,
	) -> Self {
		Self::new(None::<()>, root, spatial_anchors)
	}
}
impl<T: Serialize + DeserializeOwned> ClientStateParsed<T> {
	pub fn new(
		data: Option<T>,
		root: &impl SpatialRefAspect,
		spatial_anchors: FxHashMap<String, &impl SpatialRefAspect>,
	) -> Self {
		ClientStateParsed {
			data,
			root: SpatialRef(root.node().alias()),
			spatial_anchors: spatial_anchors
				.into_iter()
				.map(|(k, v)| (k, SpatialRef(v.node().alias())))
				.collect(),
		}
	}
	pub fn from_data_root(data: Option<T>, root: &impl SpatialRefAspect) -> Self {
		Self::new(data, root, FxHashMap::<String, &SpatialRef>::default())
	}
	pub fn serialize(&self) -> NodeResult<ClientState> {
		Ok(ClientState {
			data: self.data.as_ref().and_then(|d| flexbuffers::to_vec(d).ok()),
			root: self.root.node().get_id()?,
			spatial_anchors: self
				.spatial_anchors
				.iter()
				.filter_map(|(k, v)| Some((k.clone(), v.node().get_id().ok()?)))
				.collect(),
		})
	}
}
