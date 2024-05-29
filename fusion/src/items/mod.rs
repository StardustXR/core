pub mod camera;
pub mod panel;

use crate::{
	impl_aspects,
	node::{NodeResult, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect},
};

stardust_xr_fusion_codegen::codegen_item_protocol!();
impl_aspects!(Item: OwnedAspect, SpatialRefAspect, SpatialAspect);
impl_aspects!(ItemAcceptor: OwnedAspect, SpatialRefAspect, SpatialAspect);
