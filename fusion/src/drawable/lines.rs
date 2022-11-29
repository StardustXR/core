use crate::{
	node::{Node, NodeError, NodeType},
	spatial::Spatial,
};
use anyhow::Result;
use color::{rgba, Rgba};
use mint::Vector3;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use stardust_xr::values::Transform;
use std::ops::Deref;

pub trait ToLines {
	fn to_lines(&self) -> Vec<LinePoint>;
}

#[derive(Debug, Clone, Copy)]
pub struct LinePoint {
	pub point: Vector3<f32>,
	pub thickness: f32,
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

#[derive(Debug)]
pub struct Lines {
	spatial: Spatial,
}
#[buildstructor::buildstructor]
impl Lines {
	#[builder(entry = "builder")]
	pub fn create<'a>(
		spatial_parent: &'a Spatial,
		position: Option<mint::Vector3<f32>>,
		rotation: Option<mint::Quaternion<f32>>,
		scale: Option<mint::Vector3<f32>>,
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
						Transform {
							position,
							rotation,
							scale,
						},
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
	pub fn set_cyclic(&self, cyclic: bool) -> Result<(), NodeError> {
		self.node().send_remote_signal("set_cyclic", &cyclic)
	}
}
impl NodeType for Lines {
	fn node(&self) -> &Node {
		&self.spatial.node()
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
	let _lines = Lines::builder()
		.spatial_parent(client.get_root())
		.points(&points)
		.cyclic(true)
		.build()
		.unwrap();

	tokio::time::sleep(core::time::Duration::from_secs(60)).await;
}
