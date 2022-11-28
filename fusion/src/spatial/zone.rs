use crate::{
	fields::Field,
	node::{ClientOwned, Node, NodeError, NodeType},
	HandlerWrapper, WeakNodeRef, WeakWrapped,
};

use super::Spatial;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use stardust_xr::{schemas::flex::deserialize, values::Transform};
use std::sync::Arc;

pub trait ZoneHandler: Send + Sync {
	fn enter(&mut self, zone: &Zone, uid: &str, spatial: &Spatial);
	fn capture(&mut self, zone: &Zone, uid: &str, spatial: &Spatial);
	fn release(&mut self, zone: &Zone, uid: &str);
	fn leave(&mut self, zone: &Zone, uid: &str);
}

#[derive(Debug)]
pub struct Zone {
	pub spatial: Spatial,
	pub captured: Mutex<FxHashMap<String, Spatial>>,
}

impl<'a> Zone {
	pub fn create<F, Fi: Field + ClientOwned, T>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		field: &'a Fi,
		wrapped_init: F,
	) -> Result<HandlerWrapper<Zone, T>, NodeError>
	where
		F: FnOnce(WeakNodeRef<Zone>, &Zone) -> T,
		T: ZoneHandler + 'static,
	{
		let id = nanoid::nanoid!();
		let zone = Zone {
			spatial: Spatial {
				node: Node::new(
					spatial_parent.node.client.clone(),
					"/spatial",
					"create_zone",
					"/spatial/zone",
					true,
					&id.clone(),
					(
						id,
						spatial_parent,
						Transform {
							position,
							rotation,
							scale: None,
						},
						&field.node(),
					),
				)?,
			},
			captured: Mutex::new(FxHashMap::default()),
		};

		let handler_wrapper = HandlerWrapper::new(zone, |weak_handler, weak_node_ref, zone| {
			zone.node().local_signals.lock().insert(
				"enter".to_string(),
				Arc::new({
					let weak_handler: WeakWrapped<dyn ZoneHandler> = weak_handler.clone();
					let weak_node_ref = weak_node_ref.clone();
					move |data| {
						let uid: &str = deserialize(data)?;
						if let Some(handler) = weak_handler.upgrade() {
							weak_node_ref
								.with_node(|zone| -> anyhow::Result<()> {
									let spatial = Spatial::from_path(
										zone.node().client.clone(),
										&(zone.node().get_path().to_string() + "/" + uid),
										false,
									)?;
									handler.lock().enter(zone, uid, &spatial);
									Ok(())
								})
								.transpose()?;
							// handler.lock().enter(, spatial)
						}
						Ok(())
					}
				}),
			);
			zone.node().local_signals.lock().insert(
				"capture".to_string(),
				Arc::new({
					let weak_handler: WeakWrapped<dyn ZoneHandler> = weak_handler.clone();
					let weak_node_ref = weak_node_ref.clone();
					move |data| {
						let uid: &str = deserialize(data)?;
						if let Some(handler) = weak_handler.upgrade() {
							weak_node_ref
								.with_node(|zone| -> anyhow::Result<()> {
									let spatial = Spatial::from_path(
										zone.node().client.clone(),
										&(zone.node().get_path().to_string() + "/" + uid),
										false,
									)?;
									zone.captured.lock().insert(uid.to_string(), spatial);
									let captured = zone.captured.lock();
									let spatial = captured.get(uid).unwrap();
									handler.lock().capture(zone, uid, spatial);
									Ok(())
								})
								.transpose()?;
							// handler.lock().enter(, spatial)
						}
						Ok(())
					}
				}),
			);
			zone.node().local_signals.lock().insert(
				"release".to_string(),
				Arc::new({
					let weak_handler: WeakWrapped<dyn ZoneHandler> = weak_handler.clone();
					let weak_node_ref = weak_node_ref.clone();
					move |data| {
						let uid: &str = deserialize(data)?;
						if let Some(handler) = weak_handler.upgrade() {
							weak_node_ref
								.with_node(|zone| -> anyhow::Result<()> {
									zone.captured.lock().remove(uid);
									handler.lock().release(zone, uid);
									Ok(())
								})
								.transpose()?;
							// handler.lock().enter(, spatial)
						}
						Ok(())
					}
				}),
			);
			zone.node().local_signals.lock().insert(
				"leave".to_string(),
				Arc::new({
					let weak_handler: WeakWrapped<dyn ZoneHandler> = weak_handler;
					let weak_node_ref = weak_node_ref.clone();
					move |data| {
						let uid: &str = deserialize(data)?;
						if let Some(handler) = weak_handler.upgrade() {
							weak_node_ref
								.with_node(|zone| -> anyhow::Result<()> {
									handler.lock().leave(zone, uid);
									Ok(())
								})
								.transpose()?;
							// handler.lock().enter(, spatial)
						}
						Ok(())
					}
				}),
			);
			wrapped_init(weak_node_ref, zone)
		});

		// handler_wrapper.
		Ok(handler_wrapper)
	}

	pub fn update(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("update", &())
	}
	pub fn capture(&self, uid: &str) -> Result<(), NodeError> {
		self.node.send_remote_signal("capture", &uid)
	}
	pub fn release(&self, uid: &str) -> Result<(), NodeError> {
		self.node.send_remote_signal("release", &uid)
	}
}
impl NodeType for Zone {
	fn node(&self) -> &Node {
		&self.spatial.node
	}
}
impl std::ops::Deref for Zone {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
	}
}

#[tokio::test]
async fn fusion_zone() {
	use crate::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	client.set_base_prefixes(&[manifest_dir_macros::directory_relative_path!("res")]);

	let model_parent = Spatial::builder()
		.spatial_parent(client.get_root())
		.zoneable(true)
		.build()
		.unwrap();
	let _model = crate::drawable::Model::builder()
		.spatial_parent(&model_parent)
		.resource(&crate::resource::NamespacedResource::new(
			"fusion",
			"gyro_gem.glb",
		))
		.build()
		.unwrap();

	let field = crate::fields::SphereField::builder()
		.spatial_parent(client.get_root())
		.radius(0.1)
		.build()
		.unwrap();

	struct LifeCycle(HandlerWrapper<Zone, ZoneTest>);
	impl crate::client::LifeCycleHandler for LifeCycle {
		fn logic_step(&mut self, info: crate::client::LogicStepInfo) {
			self.0.node().update().unwrap();
			for (_, spatial) in self.0.node().captured.lock().iter() {
				spatial
					.set_position(None, glam::vec3(0.0, info.elapsed.sin() as f32 * 0.1, 0.0))
					.unwrap();
			}
		}
	}

	struct ZoneTest;
	impl ZoneHandler for ZoneTest {
		fn enter(&mut self, zone: &Zone, uid: &str, _spatial: &Spatial) {
			println!("Spatial {} entered zone", uid);
			zone.capture(uid).unwrap();
		}
		fn capture(&mut self, _zone: &Zone, uid: &str, _spatial: &Spatial) {
			println!("Spatial {} was captured", uid);
		}
		fn release(&mut self, _zone: &Zone, uid: &str) {
			println!("Spatial {} was released", uid);
		}
		fn leave(&mut self, _zone: &Zone, uid: &str) {
			println!("Spatial {} left zone", uid);
		}
	}
	let demo =
		LifeCycle(Zone::create(client.get_root(), None, None, &field, |_, _| ZoneTest).unwrap());

	let _handler = client.wrap_root(demo);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
