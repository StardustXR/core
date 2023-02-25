use crate::{
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
};
use color::{rgba, Rgba};

use mint::Vector3;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use stardust_xr::values::Transform;
use std::ops::Deref;

/// A single point on a line.
#[derive(Debug, Clone, Copy)]
pub struct LinePoint {
	/// Relative to the `Lines`' transform.
	pub point: Vector3<f32>,
	/// Objective thickness in meters, ignores the scale of the Lines node.
	pub thickness: f32,
	/// This will be blended with other line points using vertex colors (sRGB RGBA gradient).
	pub color: Rgba<f32>,
}
impl Default for LinePoint {
	fn default() -> Self {
		Self {
			point: Vector3 {
				x: 0.0,
				y: 0.0,
				z: 0.0,
			},
			thickness: 0.01,
			color: rgba!(1.0, 1.0, 1.0, 1.0),
		}
	}
}
impl Serialize for LinePoint {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let mut struct_ser = serializer.serialize_struct("LinePoint", 3)?;
		struct_ser.serialize_field("point", &self.point)?;
		struct_ser.serialize_field("thickness", &self.thickness)?;
		struct_ser.serialize_field(
			"thickness",
			&[self.color.c.r, self.color.c.g, self.color.c.b, self.color.a],
		)?;
		struct_ser.end()
	}
}

/// A single continuous polyline.
///
/// # Example
/// ```
/// let points = vec![
/// 	LinePoint {
/// 		point: Vector3 {
/// 			x: 1.0,
/// 			y: 0.0,
/// 			z: 0.0,
/// 		},
/// 		thickness: 0.0025,
/// 		..Default::default()
/// 	},
/// 	LinePoint {
/// 		thickness: 0.0025,
/// 		..Default::default()
/// 	},
/// 	LinePoint {
/// 		point: Vector3 {
/// 			x: 0.0,
/// 			y: 1.0,
/// 			z: 0.0,
/// 		},
/// 		thickness: 0.0025,
/// 		..Default::default()
/// 	},
/// ];
/// let _lines = Lines::create(client.get_root(), Transform::none(), &points, true).unwrap();
/// ```
#[derive(Debug)]
pub struct Lines {
	spatial: Spatial,
}
impl Lines {
	/// Create a new Lines node.
	///
	/// Cyclic means the start and end points are connected together.
	pub fn create<'a>(
		spatial_parent: &'a Spatial,
		transform: Transform,
		points: &'a [LinePoint],
		cyclic: bool,
	) -> Result<Self, NodeError> {
		let id = nanoid::nanoid!();
		Ok(Lines {
			spatial: Spatial {
				node: Node::new(
					&spatial_parent.node.client()?,
					"/drawable",
					"create_lines",
					"/drawable/lines",
					true,
					&id.clone(),
					(
						id,
						spatial_parent.node().get_path()?,
						transform,
						points,
						cyclic,
					),
				)?,
			},
		})
	}

	pub fn update_points(&self, points: &[LinePoint]) -> Result<(), NodeError> {
		self.node().send_remote_signal("set_points", &points)
	}
	/// Cyclic means the start and end points are connected together.
	pub fn set_cyclic(&self, cyclic: bool) -> Result<(), NodeError> {
		self.node().send_remote_signal("set_cyclic", &cyclic)
	}
}
impl NodeType for Lines {
	fn node(&self) -> &Node {
		&self.spatial.node()
	}

	fn alias(&self) -> Self {
		Lines {
			spatial: self.spatial.alias(),
		}
	}
}
impl Deref for Lines {
	type Target = Spatial;

	fn deref(&self) -> &Self::Target {
		&self.spatial
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
			point: Vector3 {
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
			point: Vector3 {
				x: 0.0,
				y: 1.0,
				z: 0.0,
			},
			thickness: 0.0025,
			..Default::default()
		},
	];
	let _lines = Lines::create(client.get_root(), Transform::default(), &points, true).unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}
