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

#[zbus::proxy(interface = "org.stardustxr.KeyAcceptor")]
// this is associated with the `SpatialRef` at the same paath, all points are relative to it`
pub trait KeyAcceptor {
	fn keymap(&self, keymap: &str) -> Result<()>;
	fn key_pressed(&self, key: u32) -> Result<()>;
	fn key_released(&self, key: u32) -> Result<()>;
}
