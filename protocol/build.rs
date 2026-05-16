use gluon_codegen_rust::{Derives, TypeProxy};
use std::path::Path;

fn main() {
	gluon_codegen_rust::helpers::gen_multiple_modules(
		&[
			("types", Path::new("./gluon/org.stardustxr.Types.gluon")),
			("server", Path::new("./gluon/org.stardustxr.Server.gluon")),
			("spatial", Path::new("./gluon/org.stardustxr.Spatial.gluon")),
			("field", Path::new("./gluon/org.stardustxr.Field.gluon")),
			("dmatex", Path::new("./gluon/org.stardustxr.Dmatex.gluon")),
			("lines", Path::new("./gluon/org.stardustxr.Lines.gluon")),
			("model", Path::new("./gluon/org.stardustxr.Model.gluon")),
			("text", Path::new("./gluon/org.stardustxr.Text.gluon")),
			("sky", Path::new("./gluon/org.stardustxr.Sky.gluon")),
			("camera", Path::new("./gluon/org.stardustxr.Camera.gluon")),
			("client", Path::new("./gluon/org.stardustxr.Client.gluon")),
			("audio", Path::new("./gluon/org.stardustxr.Audio.gluon")),
			("suis", Path::new("./gluon/org.stardustxr.SUIS.gluon")),
			("query", Path::new("./gluon/org.stardustxr.Query.gluon")),
			(
				"spatial_query",
				Path::new("./gluon/org.stardustxr.SpatialQuery.gluon"),
			),
			("tracked", Path::new("./gluon/org.stardustxr.Tracked.gluon")),
		],
		&[],
		Derives::CLONE | Derives::COPY | Derives::HASH | Derives::PARTIAL_EQ | Derives::EQ,
		&[
			TypeProxy {
				protocol_type_name: "types::Vec2f".into(),
				rust_type: "mint::Vector2<f32>".into(),
				derives: Derives::CLONE | Derives::COPY,
			},
			TypeProxy {
				protocol_type_name: "types::Vec3f".into(),
				rust_type: "mint::Vector3<f32>".into(),
				derives: Derives::CLONE | Derives::COPY,
			},
			TypeProxy {
				protocol_type_name: "types::Quatf".into(),
				rust_type: "mint::Quaternion<f32>".into(),
				derives: Derives::CLONE | Derives::COPY,
			},
			TypeProxy {
				protocol_type_name: "types::Mat4f".into(),
				rust_type: "mint::ColumnMatrix4<f32>".into(),
				derives: Derives::CLONE | Derives::COPY,
			},
			TypeProxy {
				protocol_type_name: "types::Color".into(),
				rust_type: "crate::Color".into(),
				derives: Derives::CLONE | Derives::COPY,
			},
		],
		"./src/protocol",
	);
}
