//! Anything the user can see such as lines, models and text.

mod lines;
mod model;
mod text;

pub use lines::*;
pub use model::*;
use stardust_xr::schemas::flex::serialize;
pub use text::*;

use crate::{client::Client, node::NodeError};
use serde::{Serialize, Serializer};
use std::{
	fmt::Debug,
	path::{Path, PathBuf},
};

/// Set only the sky texture to this equirectangular `.hdr` file.
pub fn set_sky_tex(client: &Client, file: &impl AsRef<Path>) -> Result<(), NodeError> {
	set_sky(client, file, true, false)
}
/// Set only the sky lighting to this equirectangular `.hdr` file.
pub fn set_sky_light(client: &Client, file: &impl AsRef<Path>) -> Result<(), NodeError> {
	set_sky(client, file, false, true)
}
/// Set the sky texture and lighting to this equirectangular `.hdr` file.
pub fn set_sky_tex_light(client: &Client, file: &impl AsRef<Path>) -> Result<(), NodeError> {
	set_sky(client, file, true, true)
}

fn set_sky(
	client: &Client,
	file: &impl AsRef<Path>,
	tex: bool,
	light: bool,
) -> Result<(), NodeError> {
	if !file.as_ref().exists() {
		return Err(NodeError::InvalidPath);
	}
	let file_str = file.as_ref().to_str().ok_or(NodeError::InvalidPath)?;
	client
		.message_sender_handle
		.signal(
			"/drawable",
			"set_sky_file",
			&serialize(&(file_str, tex, light)).map_err(|_| NodeError::Serialization)?,
			&[],
		)
		.map_err(|e| NodeError::MessengerError { e })
}

#[tokio::test]
async fn fusion_sky() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let sky_path = std::path::PathBuf::from(manifest_dir_macros::file_relative_path!(
		"res/fusion/sky.hdr"
	));

	set_sky_tex(&client, &sky_path).unwrap();
	set_sky_light(&client, &sky_path).unwrap();
	set_sky_tex_light(&client, &sky_path).unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(5)).await;
}

#[derive(Debug, Clone)]
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
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.parse())
	}
}
