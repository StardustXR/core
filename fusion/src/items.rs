use crate::{
	drawable::ModelPartAspect,
	fields::{Field, FieldAspect},
	impl_aspects,
	node::{NodeResult, NodeType, OwnedAspect},
	spatial::{SpatialAspect, SpatialRefAspect, Transform},
};
use stardust_xr::values::*;

stardust_xr_fusion_codegen::codegen_item_protocol!();
impl_aspects!(Item: OwnedAspect, SpatialRefAspect, SpatialAspect);
impl_aspects!(ItemAcceptor: OwnedAspect, SpatialRefAspect, SpatialAspect);
impl_aspects!(PanelItem: OwnedAspect, SpatialRefAspect, SpatialAspect, ItemAspect);
impl_aspects!(PanelItemAcceptor: OwnedAspect, SpatialRefAspect, SpatialAspect, ItemAcceptorAspect);
