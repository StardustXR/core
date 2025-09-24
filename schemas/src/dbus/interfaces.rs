use zbus::Result;

use crate::impl_queryable_for_proxy;

#[zbus::proxy(interface = "org.stardustxr.SpatialRef")]
pub trait SpatialRef {
	#[zbus(property)]
	/// Put this UID into `import_spatial``
	fn uid(&self) -> Result<u64>;
}

#[zbus::proxy(interface = "org.stardustxr.FieldRef")]
pub trait FieldRef {
	#[zbus(property)]
	/// Put this UID into `import_field``
	fn uid(&self) -> Result<u64>;
}

#[zbus::proxy(
	interface = "org.stardustxr.PlaySpace",
	default_service = "org.stardustxr.PlaySpace",
	default_path = "/org/stardustxr/PlaySpace"
)]
// this is associated with the `SpatialRef` at the same paath, all points are relative to it`
pub trait PlaySpace {
	#[zbus(property)]
	fn bounds(&self) -> Result<Vec<(f64, f64)>>;
	// #[zbus(property)]
	// fn set_bounds(&self, bounds: Vec<(f64, f64)>) -> Result<()>;
}

#[zbus::proxy(interface = "org.stardustxr.Zoneable")]
/// You need to implement at least SpatialRef but optionally FieldRef
pub trait Zoneable {
	/// Ask the zoneable to parent itself to the given SpatialRef
	fn parent(&self, new_parent: u64) -> Result<()>;
	/// Set the transform of the zoneable relative to the given SpatialRef to zero
	fn reset_transform(&self, spatial_ref: u64) -> Result<()>;
}
#[zbus::proxy(interface = "org.stardustxr.CaptureZoneable")]
/// You need to implement Zoneable
pub trait CaptureZoneable {
	fn capture(&self) -> Result<()>;
	fn release(&self) -> Result<()>;
}

impl_queryable_for_proxy!(
	SpatialRefProxy,
	FieldRefProxy,
	PlaySpaceProxy,
	ZoneableProxy,
	CaptureZoneableProxy
);
