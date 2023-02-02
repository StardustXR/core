//! Nodes that represent spatial objects and zones to manipulate certain spatials from other clients.
//!
//! Spatials are part of most nodes such as fields and models, but can be created on their own.
//! They include a parent, transform, and zoneable boolean.
//! They're an infinitely small point in space with a position, rotation, and scale, so they're invisible.
//!
//! In Stardust, everything is relative to something else spatially.
//! In the case of creating your first spatials in your client, it'll be relative to the HMD or the client's root.
//! Clients can be spawned in with a root at a spatial's transform using the `StartupSettings` node.
//!
//! Zones are nodes that can transform any spatial inside their field with the zoneable property set to true.
//! They're very useful for grabbing large collections of objects at once and arranging them into a grid or for workspaces.
//! Zones can set the transform of any spatials they see.
//! Zones can capture spatials, temporarily parenting them to the zone until they are released.
//! Zones can see zoneable spatials if they're closer to the surface of the field than any zone that captured them, so no zones can steal and hoard them.

mod zone;
pub use zone::*;

use super::{
	client::Client,
	node::{Node, NodeError, NodeType},
};
use crate::fields::UnknownField;
use mint::{Quaternion, Vector3};
use nanoid::nanoid;
use stardust_xr::values::Transform;
use std::{future::Future, sync::Arc};

/// A node with spatial attributes (position, rotation, scale) that can be manipulated by zones if zoneable.
///
/// Equivalent to a Transform in Unity, Spatial in Godot, etc.
#[derive(Debug)]
pub struct Spatial {
	pub(crate) node: Node,
}
impl Spatial {
	/// Create a new spatial. If the position, rotation, or scale values are `None` they'll be the identity values.
	pub fn create(
		spatial_parent: &Spatial,
		transform: Transform,
		zoneable: bool,
	) -> Result<Self, NodeError> {
		let id = nanoid!();
		Ok(Spatial {
			node: Node::new(
				&spatial_parent.node().client()?,
				"/spatial",
				"create_spatial",
				"/spatial/spatial",
				true,
				&id.clone(),
				(id, spatial_parent.node().get_path()?, transform, zoneable),
			)?,
		})
	}

	pub(crate) fn from_path(
		client: &Arc<Client>,
		parent: impl ToString,
		name: impl ToString,
		destroyable: bool,
	) -> Self {
		Spatial {
			node: Node::from_path(client, parent, name, destroyable),
		}
	}

	/// Get the position, rotation, and scale relative to some other spatial node.
	pub fn get_position_rotation_scale(
		&self,
		relative_space: &Spatial,
	) -> Result<
		impl Future<Output = Result<(Vector3<f32>, Quaternion<f32>, Vector3<f32>), NodeError>>,
		NodeError,
	> {
		self.node
			.execute_remote_method("get_transform", &relative_space.node().get_path()?)
	}

	/// Set the position of this spatial relative to another node, or `None` for relative to its parent node.
	pub fn set_position(
		&self,
		relative_space: Option<&Spatial>,
		position: impl Into<mint::Vector3<f32>>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, Transform::from_position(position))
	}
	/// Set the rotation of this spatial relative to another node, or `None` for relative to its parent node.
	pub fn set_rotation(
		&self,
		relative_space: Option<&Spatial>,
		rotation: impl Into<mint::Quaternion<f32>>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, Transform::from_rotation(rotation))
	}
	/// Set the scale of this spatial relative to another node, or `None` for relative to its parent node.
	pub fn set_scale(
		&self,
		relative_space: Option<&Spatial>,
		scale: impl Into<mint::Vector3<f32>>,
	) -> Result<(), NodeError> {
		self.set_transform(relative_space, Transform::from_scale(scale))
	}
	/// Set the transform of this spatial relative to another node, or `None` for relative to its parent node.
	pub fn set_transform(
		&self,
		relative_space: Option<&Spatial>,
		transform: Transform,
	) -> Result<(), NodeError> {
		let relative_space = match relative_space {
			Some(space) => Some(space.node().get_path()?),
			None => None,
		};
		self.node
			.send_remote_signal("set_transform", &(relative_space, transform))
	}

	/// Set the spatial parent with its local transform remaining the same.
	/// It will silently error and not set the spatial parent if it is to a child of itself.
	pub fn set_spatial_parent(&self, parent: &Spatial) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("set_spatial_parent", &parent.node().get_path()?)
	}
	/// Set the spatial parent with its "global" transform remaining the same.
	/// It will silently error and not set the spatial parent if it is to a child of itself.
	pub fn set_spatial_parent_in_place(&self, parent: &Spatial) -> Result<(), NodeError> {
		self.node
			.send_remote_signal("set_spatial_parent_in_place", &parent.node().get_path()?)
	}

	/// Set if this spatial is zoneable or not. You may want to set this to false when being grabbed or interacted with, then back to true when it's floating inert in space.
	pub fn set_zoneable(&self, zoneable: bool) -> Result<(), NodeError> {
		self.node.send_remote_signal("set_zoneable", &zoneable)
	}

	/// Get the distance to a bunch of fields
	pub fn field_distance(
		&self,
		point: impl Into<Vector3<f32>>,
		fields: impl IntoIterator<Item = UnknownField>,
	) -> Result<impl Future<Output = Result<Vec<Option<f32>>, NodeError>>, NodeError> {
		let field_paths = fields
			.into_iter()
			.filter_map(|f| f.node().get_path().ok())
			.collect::<Vec<String>>();
		Ok(Box::pin(self.node.execute_remote_method(
			"field_distance",
			&(point.into(), field_paths),
		)?))
	}

	pub fn field_normal(
		&self,
		point: impl Into<Vector3<f32>>,
		fields: impl IntoIterator<Item = UnknownField>,
	) -> Result<impl Future<Output = Result<Vec<Option<Vector3<f32>>>, NodeError>>, NodeError> {
		let field_paths = fields
			.into_iter()
			.filter_map(|f| f.node().get_path().ok())
			.collect::<Vec<String>>();
		self.node
			.execute_remote_method("field_normal", &(point.into(), field_paths))
	}

	pub fn field_closest_point(
		&self,
		point: impl Into<Vector3<f32>>,
		fields: impl IntoIterator<Item = UnknownField>,
	) -> Result<impl Future<Output = Result<Vec<Option<Vector3<f32>>>, NodeError>>, NodeError> {
		let field_paths = fields
			.into_iter()
			.filter_map(|f| f.node().get_path().ok())
			.collect::<Vec<String>>();
		self.node
			.execute_remote_method("field_closest_point", &(point.into(), field_paths))
	}
}
impl NodeType for Spatial {
	fn node(&self) -> &Node {
		&self.node
	}
	fn alias(&self) -> Spatial {
		Spatial {
			node: self.node.alias(),
		}
	}
}

#[tokio::test]
async fn fusion_spatial() {
	use super::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let spatial = Spatial::create(client.get_root(), Transform::default(), false).unwrap();
	drop(spatial);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
