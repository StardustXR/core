//! References models, fonts, etc.

use std::{fmt::Debug, path::Path};

/// Represents a type that works as a resource.
/// Implemented for `AsRef<Path>` to be a literal absolute path in case namespacing is not wanted/needed.
/// You shouldn't implement this trait unless you know what you're doing, it deals with very specific formatting.
pub trait Resource: Debug {
	fn parse(&self) -> String;
}
impl<R: AsRef<Path> + Debug> Resource for R {
	fn parse(&self) -> String {
		self.as_ref().to_str().unwrap().to_string()
	}
}

/// A resource that is relative to a prefix, meant for resources that are included with the client.
/// Allows switching of prefix by the server as well to theme clients.
///
/// # Example
/// ```
/// use stardust_xr_fusion::{drawable::Model, resource::NamespacedResource};
///
/// // For a client named "star"
/// let model_resource = NamespacedResource::new("star", "icon");
/// // Build a model at "[prefix]/star/icon.glb" if it exists
/// let model = Model::builder().spatial_parent(client.get_root()).resource(model_resource).build().unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct NamespacedResource {
	/// Group that this resource is in, generally the client or library's name.
	pub namespace: String,
	/// Path inside the namespace for the exact file. Leave out the extension and ensure no leading slash.
	pub path: String,
}
impl NamespacedResource {
	/// Convenience function for when you don't want to make the struct.
	/// See `NamespacedResource` documentation for info about arguments.
	pub fn new(namespace: impl Into<String>, path: impl Into<String>) -> Self {
		NamespacedResource {
			namespace: namespace.into(),
			path: path.into(),
		}
	}
}
impl Resource for NamespacedResource {
	fn parse(&self) -> String {
		format!("{}:{}", self.namespace, self.path)
	}
}
