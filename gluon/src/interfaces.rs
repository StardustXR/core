use crate::impl_queryable_for_proxy;
use zbus::Result;

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

#[zbus::proxy(interface = "org.stardustxr.Reparentable")]
/// You need to implement at least SpatialRef but optionally FieldRef
pub trait Reparentable {
	/// Ask the reparentable to parent itself to the given SpatialRef
	fn parent(&self, new_parent: u64) -> Result<()>;
	/// Ask the reparentable to unparent itself from the given SpatialRef
	fn unparent(&self) -> Result<()>;
	/// Set the transform of the reparentable relative to the given SpatialRef to zero
	fn reset_transform(&self, spatial_ref: u64) -> Result<()>;
}
#[zbus::proxy(interface = "org.stardustxr.ReparentLock")]
/// You need to implement reparentable
pub trait ReparentLock {
	/// Ask to make it so nothing else can reparent this
	fn lock(&self) -> Result<()>;
	fn unlock(&self) -> Result<()>;
}

#[zbus::proxy(interface = "org.stardustxr.Destroy")]
/// Destroy an object.
/// Implement SpatialRef and/or FieldRef for spatial context.
/// This trait might be implemented on the root to close the whole client or individual objects to delete.
pub trait Destroy {
	fn destroy(&self) -> Result<()>;
}

impl_queryable_for_proxy!(
	SpatialRefProxy,
	FieldRefProxy,
	PlaySpaceProxy,
	ReparentableProxy,
	ReparentLockProxy,
	DestroyProxy
);
