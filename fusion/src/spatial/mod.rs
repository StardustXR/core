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

use crate::{
	fields::FieldAspect,
	node::{NodeAspect, NodeResult, NodeType},
};
use mint::{Quaternion, Vector3};
use nanoid::nanoid;

stardust_xr_fusion_codegen::codegen_spatial_client_protocol!();
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
	pub fn from_rotation(rotation: impl Into<Quaternion<f32>>) -> Self {
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
		rotation: impl Into<Quaternion<f32>>,
	) -> Self {
		Transform {
			translation: Some(translation.into()),
			rotation: Some(rotation.into()),
			scale: None,
		}
	}
	pub fn from_rotation_scale(
		rotation: impl Into<Quaternion<f32>>,
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
		rotation: impl Into<Quaternion<f32>>,
		scale: impl Into<Vector3<f32>>,
	) -> Self {
		Transform {
			translation: Some(translation.into()),
			rotation: Some(rotation.into()),
			scale: Some(scale.into()),
		}
	}
}

impl Spatial {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		zoneable: bool,
	) -> NodeResult<Self> {
		create_spatial(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			zoneable,
		)
	}
}
impl Zone {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		field: &impl FieldAspect,
	) -> NodeResult<Self> {
		create_zone(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			field,
		)
	}
}

#[tokio::test]
async fn fusion_spatial() {
	color_eyre::install().unwrap();
	use super::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
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
	drop(spatial);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}

#[tokio::test]
async fn fusion_zone() {
	color_eyre::install().unwrap();
	let (client, event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let model_parent =
		crate::spatial::Spatial::create(client.get_root(), Transform::none(), true).unwrap();

	let gyro_gem = stardust_xr::values::ResourceID::new_namespaced("fusion", "gyro_gem");
	let _model =
		crate::drawable::Model::create(&model_parent, Transform::none(), &gyro_gem).unwrap();

	let field =
		crate::fields::SphereField::create(client.get_root(), mint::Vector3::from([0.0; 3]), 0.1)
			.unwrap();

	struct ZoneTest {
		client: std::sync::Arc<crate::client::Client>,
		root: crate::spatial::Spatial,
		zone: Zone,
	}
	impl ZoneHandler for ZoneTest {
		fn enter(&mut self, uid: String, spatial: crate::spatial::Spatial) {
			println!("Spatial {} entered zone", uid);
			self.zone.capture(&spatial).unwrap();
		}
		fn capture(&mut self, uid: String) {
			println!("Spatial {} was captured", uid);
			self.zone.release(&uid).unwrap();
		}
		fn release(&mut self, uid: String) {
			println!("Spatial {} was released", uid);
			self.root
				.set_local_transform(Transform::from_translation([0.0, 1.0, 0.0]))
				.unwrap();
			self.zone.update().unwrap();
		}
		fn leave(&mut self, uid: String) {
			println!("Spatial {} left zone", uid);
			self.client.stop_loop();
		}
	}
	let zone = Zone::create(client.get_root(), Transform::none(), &field).unwrap();
	let zone_handler = ZoneTest {
		client,
		root: model_parent,
		zone: zone.alias(),
	};
	let zone = zone.wrap(zone_handler).unwrap();
	zone.node().update().unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
