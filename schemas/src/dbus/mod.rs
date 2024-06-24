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
