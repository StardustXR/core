use crate::{
	fields::FieldAspect,
	node::{NodeAspect, NodeResult, NodeType},
	spatial::{Spatial, Transform},
};
use nanoid::nanoid;

use super::SpatialAspect;

stardust_xr_fusion_codegen::codegen_zone_client_protocol!();
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
