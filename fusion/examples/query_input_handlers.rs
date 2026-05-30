use std::{collections::HashMap, sync::Arc, vec};

use gluon::Handler;
use parking_lot::Mutex;
use stardust_xr_fusion::{
	client::Client,
	drawable::LinesExt,
	fields::FieldRef,
	project_local_resources,
	spatial::{Spatial, SpatialExt, SpatialRef, Transform},
	spatial_query::{Point, PointsQuery, PointsQueryHandler, PointsQueryHandlerHandler},
	suis::InputHandler,
	types::rgba_linear,
};
use stardust_xr_protocol::{
	lines::{Line, LinePoint, Lines},
	query::{InterfaceDependency, QueriedInterface, QueryableObjectRef},
};
use tokio::sync::broadcast::error::RecvError;
use tracing::warn;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	tracing_subscriber::fmt::init();
	let (client, root) = Client::auto_connect(&[&project_local_resources!("res")])
		.await
		.unwrap();
	let (spatial, _) =
		Spatial::create(&client, &root, Transform::from_translation([0.0, 0.1, 0.0]))
			.await
			.unwrap();

	let handlers: Arc<Mutex<HashMap<QueryableObjectRef, SpatialRef>>> =
		Arc::new(Mutex::new(HashMap::new()));
	let points_query_handler = client.pion_device().register_object(Querier {
		handlers: handlers.clone(),
	});

	let _points_query_handle = client
		.spatial_query_interface()
		.points_query(PointsQuery {
			handler: PointsQueryHandler::from_handler(&points_query_handler),
			interfaces: vec![InterfaceDependency {
				id: InputHandler::QUERY_INTERFACE.into(),
				optional: false,
			}],
			reference_spatial: root.clone(),
			points: vec![Point {
				point: [0.0; 3].into(),
				margin: 10000.0,
			}],
		})
		.await
		.unwrap();

	let lines = Lines::create(&client, &spatial, vec![]).await.unwrap();
	let spatial_ref = spatial.spatial_ref().await.unwrap();

	let mut frame_recv = client.frame_receiver();
	loop {
		match frame_recv.recv().await {
			Ok(_) => {}
			Err(RecvError::Lagged(n)) => {
				warn!("lost {n} frame events");
				continue;
			}
			Err(RecvError::Closed) => break,
		}

		let handler_spatials: Vec<SpatialRef> = handlers.lock().values().cloned().collect();
		let mut drawn_lines = Vec::new();
		for handler_spatial in handler_spatials {
			let Ok(Ok(transform)) = client
				.spatial_interface()
				.get_relative_transform(spatial_ref.clone(), handler_spatial)
				.await
			else {
				continue;
			};
			drawn_lines.push(Line {
				points: vec![
					LinePoint {
						point: [0.0; 3].into(),
						thickness: 0.005,
						color: rgba_linear!(0.0, 0.1, 1.0, 0.5),
						// color: rgba_linear!(1.0, 1.0, 1.0, 1.0),
					},
					LinePoint {
						point: transform.translation,
						thickness: 0.0,
						color: rgba_linear!(0.0, 0.25, 1.0, 1.0),
						// color: rgba_linear!(1.0, 1.0, 1.0, 1.0),
					},
				],
				cyclic: false,
			});
		}
		lines.set_lines(drawn_lines).unwrap();
	}
}

#[derive(Debug, Handler)]
struct Querier {
	handlers: Arc<Mutex<HashMap<QueryableObjectRef, SpatialRef>>>,
}
impl PointsQueryHandlerHandler for Querier {
	async fn entered(
		&self,
		_ctx: gluon::Context,
		obj: QueryableObjectRef,
		_field: FieldRef,
		spatial: SpatialRef,
		_interfaces: Vec<QueriedInterface>,
		_distance: f32,
	) {
		self.handlers.lock().insert(obj, spatial);
	}

	async fn interfaces_changed(
		&self,
		_ctx: gluon::Context,
		_obj: QueryableObjectRef,
		_interfaces: Vec<QueriedInterface>,
	) {
	}
	async fn moved(&self, _ctx: gluon::Context, _obj: QueryableObjectRef, _distance: f32) {}

	async fn left(&self, _ctx: gluon::Context, obj: QueryableObjectRef) {
		self.handlers.lock().remove(&obj);
	}
}
