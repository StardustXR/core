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
use crate::{client::ClientHandle, fields::FieldAspect, node::NodeResult};
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
		zoneable: bool,
	) -> NodeResult<Self> {
		let client = spatial_parent.client();
		create_spatial(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			zoneable,
		)
	}
}

impl Zone {
	pub fn create(
		spatial_parent: &impl SpatialRefAspect,
		transform: Transform,
		field: &impl FieldAspect,
	) -> NodeResult<Self> {
		let client = spatial_parent.client();
		create_zone(
			&client,
			client.generate_id(),
			spatial_parent,
			transform,
			field,
		)
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
		false,
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
		false,
	)
	.unwrap();
	let uid = exported.export_spatial().await.unwrap();
	let imported = SpatialRef::import(&client.handle(), uid).await.unwrap();
	let relative_transform = imported.get_transform(&exported).await.unwrap();
	assert_eq!(relative_transform, Transform::identity());
}

#[tokio::test]
async fn fusion_zone() {
	use crate::node::NodeType;
	use crate::root::*;
	let mut client = crate::Client::connect().await.expect("Couldn't connect");

	let root = crate::spatial::Spatial::create(client.get_root(), Transform::none(), true).unwrap();

	let gyro_gem = stardust_xr::values::ResourceID::new_namespaced("fusion", "gyro_gem");
	let _model = crate::drawable::Model::create(&root, Transform::none(), &gyro_gem).unwrap();

	let field = crate::fields::Field::create(
		client.get_root(),
		Transform::identity(),
		crate::fields::Shape::Sphere(0.1),
	)
	.unwrap();

	let mut zone_spatials: rustc_hash::FxHashMap<u64, SpatialRef> = Default::default();

	let zone = Zone::create(client.get_root(), Transform::none(), &field).unwrap();

	let event_loop = client.sync_event_loop(|client, stop| {
		while let Some(event) = client.get_root().recv_root_event() {
			match event {
				RootEvent::Ping { response } => {
					response.send_ok(());
				}
				RootEvent::Frame { info: _ } => zone.update().unwrap(),
				RootEvent::SaveState { response } => response.send_ok(ClientState::default()),
			}
		}
		while let Some(zone_event) = zone.recv_zone_event() {
			match zone_event {
				ZoneEvent::Enter { spatial } => {
					println!("Spatial {spatial:?} entered zone");
					zone.capture(&spatial).unwrap();
					zone_spatials.insert(spatial.id(), spatial);
				}
				ZoneEvent::Capture { spatial } => {
					println!("Spatial {spatial:?} was captured");
					zone.release(&spatial).unwrap();
				}
				ZoneEvent::Release { id } => {
					println!("Spatial {id} was released");
					root.set_local_transform(Transform::from_translation([0.0, 1.0, 0.0]))
						.unwrap();
					zone.update().unwrap();
				}
				ZoneEvent::Leave { id } => {
					println!("Spatial {id} left zone");
					stop.stop();
				}
			}
		}
	});
	tokio::time::timeout(std::time::Duration::from_secs(1), event_loop)
		.await
		.unwrap()
		.unwrap();
}
