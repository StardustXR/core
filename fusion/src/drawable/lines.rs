use crate::{
	node::{NodeAspect, NodeResult},
	spatial::{SpatialAspect, Transform},
};
use color::rgba_linear;
use nanoid::nanoid;

stardust_xr_fusion_codegen::codegen_drawable_lines_client_protocol!();
impl Lines {
	pub fn create(
		spatial_parent: &impl SpatialAspect,
		transform: Transform,
		lines: &[Line],
	) -> NodeResult<Self> {
		create_lines(
			&spatial_parent.client()?,
			&nanoid!(),
			spatial_parent,
			transform,
			lines,
		)
	}
}
impl Default for LinePoint {
	fn default() -> Self {
		Self {
			point: [0.0; 3].into(),
			thickness: Default::default(),
			color: rgba_linear!(1.0, 1.0, 1.0, 1.0),
		}
	}
}

#[tokio::test]
async fn fusion_lines() {
	color_eyre::install().unwrap();
	let (client, _event_loop) = crate::client::Client::connect_with_async_loop()
		.await
		.unwrap();

	let points = vec![
		LinePoint {
			point: mint::Vector3 {
				x: 1.0,
				y: 0.0,
				z: 0.0,
			},
			thickness: 0.0025,
			..Default::default()
		},
		LinePoint {
			thickness: 0.0025,
			..Default::default()
		},
		LinePoint {
			point: mint::Vector3 {
				x: 0.0,
				y: 1.0,
				z: 0.0,
			},
			thickness: 0.0025,
			..Default::default()
		},
	];
	let line = Line {
		points,
		cyclic: true,
	};
	let _lines = Lines::create(client.get_root(), Transform::none(), &[line]).unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}
