use crate::{
	fields::Field,
	node::{HandledNodeType, Node, NodeError, NodeType},
	HandlerWrapper,
};

use super::Spatial;
use anyhow::{anyhow, Result};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use rustc_hash::FxHashMap;
use stardust_xr::{schemas::flex::deserialize, values::Transform};
use std::{ops::Deref, sync::Arc};

/// Hamdle spatials entering the zone's field, leaving it, being captured, and released.
pub trait ZoneHandler: Send + Sync {
	fn enter(&mut self, uid: &str, spatial: Spatial);
	fn capture(&mut self, uid: &str, spatial: Spatial);
	fn release(&mut self, uid: &str);
	fn leave(&mut self, uid: &str);
}

/// Node to manipulate spatial nodes across clients.
#[derive(Debug)]
pub struct Zone {
	spatial: Spatial,
	spatials: Arc<RwLock<FxHashMap<String, Spatial>>>,
	captured: Arc<RwLock<FxHashMap<String, Spatial>>>,
}
impl<'a> Zone {
	/// Create a zone given a field, this zone will become inactive if the field is dropped.
	///
	/// Keep in mind the zone and its field are different spatials, they can move independently.
	pub fn create<Fi: Field>(
		spatial_parent: &'a Spatial,
		transform: Transform,
		field: &'a Fi,
	) -> Result<Zone, NodeError> {
		let id = nanoid::nanoid!();
		Ok(Zone {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/spatial",
					"create_zone",
					"/spatial/zone",
					true,
					&id.clone(),
					(
						id,
						spatial_parent.node().get_path()?,
						transform,
						field.node().get_path()?,
					),
				)?,
			},
			spatials: Arc::new(RwLock::new(FxHashMap::default())),
			captured: Arc::new(RwLock::new(FxHashMap::default())),
		})
	}

	/// Wrap this node and a `ZoneHandler` in a `HandlerWrapper` to run code ASAP. Instead, you can also get the `spatials()` and `captured()` hashmaps.
	#[must_use = "Dropping this handler wrapper would immediately drop the handler"]
	pub fn wrap<H: ZoneHandler>(self, handler: H) -> Result<HandlerWrapper<Self, H>, NodeError> {
		let handler_wrapper = HandlerWrapper::new(self, handler);
		handler_wrapper.add_handled_signal("enter", Self::handle_enter)?;
		handler_wrapper.add_handled_signal("capture", Self::handle_capture)?;
		handler_wrapper.add_handled_signal("release", Self::handle_release)?;
		handler_wrapper.add_handled_signal("leave", Self::handle_leave)?;
		Ok(handler_wrapper)
	}

	fn handle_enter<H: ZoneHandler>(
		zone: Arc<Zone>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let uid: &str = deserialize(data)?;
		let spatial_stored =
			Spatial::from_path(&zone.node().client()?, zone.node().get_path()?, uid, false);
		let spatial = spatial_stored.alias();
		zone.spatials
			.write()
			.insert(uid.to_string(), spatial_stored);
		handler.lock().capture(uid, spatial);
		Ok(())
	}
	fn handle_capture<H: ZoneHandler>(
		zone: Arc<Zone>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let uid: &str = deserialize(data)?;
		let captured = zone.captured.read();
		let spatial = captured
			.get(uid)
			.ok_or_else(|| anyhow!("Spatial was captured before in range"))?;
		let spatial = spatial.alias();
		drop(captured);
		zone.captured
			.write()
			.insert(uid.to_string(), spatial.alias());
		handler.lock().capture(uid, spatial.alias());
		Ok(())
	}
	fn handle_release<H: ZoneHandler>(
		zone: Arc<Zone>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let uid: &str = deserialize(data)?;
		zone.captured.write().remove(uid);
		handler.lock().release(uid);
		Ok(())
	}
	fn handle_leave<H: ZoneHandler>(
		zone: Arc<Zone>,
		handler: Arc<Mutex<H>>,
		data: &[u8],
	) -> Result<()> {
		let uid: &str = deserialize(data)?;
		zone.spatials.write().remove(uid);
		handler.lock().leave(uid);
		Ok(())
	}

	/// Check for new spatials. Any outdated spatials may not be transformable.
	pub fn update(&self) -> Result<(), NodeError> {
		self.node.send_remote_signal("update", &())
	}
	/// Try to capture a spatial.
	/// If you sucessfully capture it you'll get a `capture()` call to the handler if wrapped and it added to the `captured` hashmap.
	pub fn capture(&self, uid: &str) -> Result<(), NodeError> {
		self.node.send_remote_signal("capture", &uid)
	}
	/// Try to release a spatial.
	/// If the spatial was already released, this does nothing.
	pub fn release(&self, uid: &str) -> Result<(), NodeError> {
		self.node.send_remote_signal("release", &uid)
	}

	/// Get the list of spatials that are visible to this zone.
	/// You still need to call `update()` to update this list.
	pub fn spatials(&self) -> RwLockReadGuard<FxHashMap<String, Spatial>> {
		self.spatials.read()
	}
	/// Get the list of spatials that are captured (temporarily parented) to this zone (not its field).
	/// You do not need to call `update()` for this list.
	pub fn captured(&self) -> RwLockReadGuard<FxHashMap<String, Spatial>> {
		self.captured.read()
	}
}
impl NodeType for Zone {
	fn node(&self) -> &Node {
		&self.spatial.node
	}

	fn alias(&self) -> Self {
		Zone {
			spatial: self.spatial.alias(),
			spatials: self.spatials.clone(),
			captured: self.captured.clone(),
		}
	}
}
impl HandledNodeType for Zone {}
impl Deref for Zone {
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

	let model_parent = Spatial::create(client.get_root(), Transform::default(), true).unwrap();

	let gyro_gem = crate::drawable::ResourceID::new_namespaced("fusion", "gyro_gem");
	let _model =
		crate::drawable::Model::create(&model_parent, Transform::default(), &gyro_gem).unwrap();

	let field =
		crate::fields::SphereField::create(client.get_root(), mint::Vector3::from([0.0; 3]), 0.1)
			.unwrap();

	struct LifeCycle(HandlerWrapper<Zone, ZoneTest>);
	impl crate::client::LifeCycleHandler for LifeCycle {
		fn logic_step(&mut self, info: crate::client::LogicStepInfo) {
			self.0.node().update().unwrap();
			for (_, spatial) in self.0.node().captured.read().iter() {
				spatial
					.set_position(None, glam::vec3(0.0, info.elapsed.sin() as f32 * 0.1, 0.0))
					.unwrap();
			}
		}
	}

	struct ZoneTest(Arc<Client>, Zone);
	impl ZoneHandler for ZoneTest {
		fn enter(&mut self, uid: &str, _spatial: Spatial) {
			println!("Spatial {} entered zone", uid);
			self.1.capture(uid).unwrap();
		}
		fn capture(&mut self, uid: &str, _spatial: Spatial) {
			println!("Spatial {} was captured", uid);
			self.1.release(uid).unwrap();
		}
		fn release(&mut self, uid: &str) {
			println!("Spatial {} was released", uid);
			self.0.stop_loop();
		}
		fn leave(&mut self, uid: &str) {
			println!("Spatial {} left zone", uid);
		}
	}
	let zone = Zone::create(client.get_root(), Transform::default(), &field).unwrap();
	let zone_handler = ZoneTest(client.clone(), zone.alias());
	let demo = LifeCycle(zone.wrap(zone_handler).unwrap());

	let _handler = client.wrap_root(demo);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		e = event_loop => e.unwrap().unwrap(),
	};
}
