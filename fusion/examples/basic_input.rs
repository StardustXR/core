use stardust_xr_fusion::{
	client::Client,
	fields::{Field, FieldExt, FieldRef, Shape},
	project_local_resources,
	spatial::{Spatial, SpatialExt, SpatialRef, Transform},
	suis::{InputHandlerHandler, InputMethod, SemanticData, SpatialData},
};
use stardust_xr_protocol::{suis::InputHandler as InputHandlerProxy, types::Timestamp};
use std::collections::{HashMap, HashSet};
use tokio::sync::{RwLock, broadcast::error::RecvError};

#[tokio::main(flavor = "current_thread")]
async fn main() {
	tracing_subscriber::fmt::init();
	let (client, root) = Client::auto_connect(&[&project_local_resources!("res")])
		.await
		.unwrap();

	let handler_spatial = Spatial::new(&client, &root, Transform::IDENTITY)
		.await
		.unwrap();
	let field_spatial = Spatial::new(
		&client,
		&handler_spatial.spatial_ref().await.unwrap(),
		Transform::IDENTITY,
	)
	.await
	.unwrap();
	let field = Field::new(
		&client,
		&field_spatial,
		Shape::Torus {
			major_radius: 0.02,
			minor_radius: 0.01,
		},
	)
	.await
	.unwrap();
	let field_spatial_ref = field_spatial.spatial_ref().await.unwrap();
	let field_ref = field.field_ref().await.unwrap();

	let input_handler = client.pion_device().register_object(InputHandler {
		field,
		spatial: handler_spatial,
		methods: RwLock::default(),
	});
	let queryable = client
		.query_interface()
		.register_queryable(field_spatial_ref, field_ref)
		.await
		.unwrap();
	let _guard = queryable
		.add_interface(&input_handler, "org.stardustxr.SUIS.Handler".to_string())
		.await
		.unwrap();
	let handler_proxy = InputHandlerProxy::from_handler(&input_handler);
	let mut frame_recv = client.frame_receiver();
	loop {
		let info = match frame_recv.recv().await {
			Ok(v) => v,
			Err(RecvError::Lagged(n)) => {
				eprintln!("lost {n} frame events");
				continue;
			}
			Err(RecvError::Closed) => {
				break;
			}
		};
		for method in input_handler.methods.read().await.iter() {
			let spatial_data = method
				.get_spatial_data(handler_proxy.clone(), info.predicted_display_time)
				.await
				.unwrap();
			println!("spatial data, {method:?}, {spatial_data:?}");
		}
	}
}

#[derive(Debug, gluon::Handler)]
struct InputHandler {
	field: Field,
	spatial: Spatial,
	methods: RwLock<HashSet<InputMethod>>,
}
impl InputHandlerHandler for InputHandler {
	async fn get_spatial(&self, _ctx: gluon::Context) -> SpatialRef {
		self.spatial.spatial_ref().await.unwrap()
	}

	async fn get_field(&self, _ctx: gluon::Context) -> FieldRef {
		self.field.field_ref().await.unwrap()
	}

	async fn suggested_bindings(&self, _ctx: gluon::Context) -> HashMap<String, Vec<String>> {
		let mut bindings = HashMap::new();
		bindings.insert("a".to_string(), vec!["pinch_strength".to_string()]);
		bindings.insert("b".to_string(), vec!["grab_strength".to_string()]);
		bindings.insert(
			"c".to_string(),
			vec!["pinch_strength".to_string(), "grab_strength".to_string()],
		);
		bindings
	}

	async fn handler_groups(&self, _ctx: gluon::Context) -> Vec<String> {
		vec!["org.stardustxr.fusion.InputExample".to_string()]
	}

	async fn input_gained(
		&self,
		_ctx: gluon::Context,
		method: InputMethod,
		time: Timestamp,
		spatial: SpatialData,
		semantic: SemanticData,
	) {
		println!("input gained, {method:?}, {time:?}, {spatial:?}, {semantic:?}");
		self.methods.write().await.insert(method);
	}

	async fn input_updated(
		&self,
		_ctx: gluon::Context,
		method: InputMethod,
		time: Timestamp,
		spatial: SpatialData,
		semantic: SemanticData,
	) {
		println!("input updated, {method:?}, {time:?}, {spatial:?}, {semantic:?}");
	}

	async fn input_left(&self, _ctx: gluon::Context, method: InputMethod, _time: Timestamp) {
		self.methods.write().await.remove(&method);
		println!("input left, {method:?}");
	}
}
