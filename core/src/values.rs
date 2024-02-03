use color::{color_space::LinearRgb, Rgba};
use serde::{Deserialize, Serialize, Serializer};
use std::path::{Path, PathBuf};

pub use color::rgba_linear;
pub use stardust_xr_schemas::flex::Datamap;

pub type Color = Rgba<f32, LinearRgb>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// An identifier to a resource, such as a sound or
pub enum ResourceID {
	/// An absolute path to a resource, not themed at all.
	/// You should only use this for content not included with your client.
	Direct(PathBuf),
	/// A resource that is relative to a prefix, meant for resources that are included with the client.
	/// Allows switching of prefix by the server as well to theme clients.
	///
	/// # Example
	/// ```
	/// use stardust_xr_fusion::{drawable::Model, resource::ResourceID};
	///
	/// // For a client named "star"
	/// let model_resource = ResourceID::new_namespaced("star", "icon");
	/// // Build a model at "[prefix]/star/icon.glb" if it exists
	/// let model = Model::create(client.get_root(), Transform::none(), model_resource).unwrap();
	/// ```
	Namespaced {
		/// Group that this resource is in, generally the client or library's name.
		namespace: String,
		/// Path inside the namespace for the exact file. Leave out the extension and ensure no leading slash.
		path: String,
	},
}
impl ResourceID {
	pub fn new_direct(path: impl AsRef<Path>) -> std::io::Result<ResourceID> {
		let path = path.as_ref();
		path.try_exists()?;
		if !path.is_absolute() {
			return Err(std::io::Error::new(
				std::io::ErrorKind::NotFound,
				"Path is not absolute",
			));
		}
		Ok(ResourceID::Direct(path.to_path_buf()))
	}
	pub fn new_namespaced(namespace: &str, path: &str) -> Self {
		ResourceID::Namespaced {
			namespace: namespace.to_string(),
			path: path.to_string(),
		}
	}
	pub(crate) fn parse(&self) -> String {
		match self {
			ResourceID::Direct(p) => p.to_str().unwrap().to_string(),
			ResourceID::Namespaced { namespace, path } => {
				format!("{}:{}", namespace, path)
			}
		}
	}
}
impl Serialize for ResourceID {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(&self.parse())
	}
}
impl<'de> Deserialize<'de> for ResourceID {
	fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		let v = String::deserialize(deserializer)?;
		Ok(if v.starts_with('/') {
			let path = PathBuf::from(v);
			path.metadata().map_err(serde::de::Error::custom)?;
			ResourceID::Direct(path)
		} else if let Some((namespace, path)) = v.split_once(':') {
			ResourceID::Namespaced {
				namespace: namespace.to_string(),
				path: path.to_string(),
			}
		} else {
			return Err(serde::de::Error::custom("Invalid format for string"));
		})
	}
}
