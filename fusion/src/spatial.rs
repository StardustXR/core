//! Nodes that represent spatial objects and zones to manipulate certain spatials from other clients.
//!
//! Spatials are part of most nodes such as fields and models, but can be created on their own.
//! They include a parent, transform, and zoneable boolean.
//! They're an infinitely small point in space with a translation, rotation, and scale, so they're invisible.
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

pub use crate::protocol::spatial::*;
use crate::{client::ClientHandle, node::NodeResult};
use stardust_xr::values::*;
use std::{hash::Hash, sync::Arc};

impl Transform {
	pub const fn none() -> Self {
		Transform {
			translation: None,
			rotation: None,
			scale: None,
		}
	}
	pub const fn identity() -> Self {
		Transform {
			translation: Some(Vector3 {
				x: 0.0,
				y: 0.0,
				z: 0.0,
			}),
			rotation: Some(Quaternion {
				v: Vector3 {
					x: 0.0,
					y: 0.0,
					z: 0.0,
				},
				s: 1.0,
			}),
			scale: Some(Vector3 {
				x: 1.0,
				y: 1.0,
				z: 1.0,
			}),
		}
	}

	pub fn from_translation(translation: impl Into<Vector3<f32>>) -> Self {
		Transform {
			translation: Some(translation.into()),
			rotation: None,
			scale: None,
		}
	}
	pub fn from_rotation(rotation: impl Into<Quaternion>) -> Self {
		Transform {
			translation: None,
			rotation: Some(rotation.into()),
			scale: None,
		}
	}
	pub fn from_scale(scale: impl Into<Vector3<f32>>) -> Self {
		Transform {
			translation: None,
			rotation: None,
			scale: Some(scale.into()),
		}
	}

	pub fn from_translation_rotation(
		translation: impl Into<Vector3<f32>>,
		rotation: impl Into<Quaternion>,
	) -> Self {
		Transform {
			translation: Some(translation.into()),
			rotation: Some(rotation.into()),
			scale: None,
		}
	}
	pub fn from_rotation_scale(
		rotation: impl Into<Quaternion>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			translation: None,
			rotation: Some(rotation.into()),
			scale: Some(scale.into()),
		}
	}

	pub fn from_translation_scale(
		translation: impl Into<Vector3<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			translation: Some(translation.into()),
			rotation: None,
			scale: Some(scale.into()),
		}
	}

	pub fn from_translation_rotation_scale(
		translation: impl Into<Vector3<f32>>,
		rotation: impl Into<Quaternion>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			translation: Some(translation.into()),
			rotation: Some(rotation.into()),
			scale: Some(scale.into()),
		}
	}
}
impl Copy for Transform {}
impl Hash for Transform {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		if let Some(translation) = &self.translation {
			translation.x.to_bits().hash(state);
			translation.y.to_bits().hash(state);
			translation.z.to_bits().hash(state);
		}
		if let Some(rotation) = &self.rotation {
			rotation.v.x.to_bits().hash(state);
			rotation.v.y.to_bits().hash(state);
			rotation.v.z.to_bits().hash(state);
			rotation.s.to_bits().hash(state);
		}
		if let Some(scale) = &self.scale {
			scale.x.to_bits().hash(state);
			scale.y.to_bits().hash(state);
			scale.z.to_bits().hash(state);
		}
	}
}

impl SpatialRef {
	pub async fn import(client: &Arc<ClientHandle>, uid: u64) -> NodeResult<Self> {
		import_spatial_ref(client, uid).await
	}
}

impl Spatial {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
	) -> NodeResult<Self> {
		let client = spatial_parent.client();
		create_spatial(client, client.generate_id(), spatial_parent, transform)
	}
}

// TODO: write tests to ensure transform order and such is correct

#[tokio::test]
async fn fusion_spatial() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let spatial = Spatial::create(
		client.get_root(),
		Transform::from_translation_scale([1.0, 0.5, 0.1], [0.5, 0.5, 0.5]),
	)
	.unwrap();
	let bounding_box = spatial
		.get_relative_bounding_box(client.get_root())
		.await
		.unwrap();
	assert_eq!(bounding_box.center, [1.0, 0.5, 0.1].into());
	assert_eq!(bounding_box.size, [0.0; 3].into());
}

#[tokio::test]
async fn fusion_spatial_import_export() {
	use crate::Client;
	let client = Client::connect().await.expect("Couldn't connect");
	let exported = Spatial::create(
		client.get_root(),
		Transform::from_translation_scale([1.0, 0.5, 0.1], [0.5, 0.5, 0.5]),
	)
	.unwrap();
	let uid = exported.export_spatial().await.unwrap();
	let imported = SpatialRef::import(&client.handle(), uid).await.unwrap();
	let relative_transform = imported.get_transform(&exported).await.unwrap();
	assert_eq!(relative_transform, Transform::identity());
}
