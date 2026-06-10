use gluon_codegen::{Derives, TypeProxy};
use std::path::Path;

fn main() {
	gluon_codegen::helpers::gen_multiple_modules(
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
			("keymap", Path::new("./gluon/org.stardustxr.Keymap.gluon")),
		],
		&[],
		Derives::CLONE
			| Derives::COPY
			| Derives::HASH
			| Derives::PARTIAL_EQ
			| Derives::EQ
			| Derives::SERDE,
		&[
			TypeProxy {
				protocol_type_name: "types::Size2".into(),
				rust_type: "crate::types::Size2".into(),
				derives: Derives::CLONE
					| Derives::COPY | Derives::PARTIAL_EQ
					| Derives::EQ | Derives::HASH
					| Derives::SERDE,
			},
			TypeProxy {
				protocol_type_name: "types::Size3".into(),
				rust_type: "crate::types::Size3".into(),
				derives: Derives::CLONE
					| Derives::COPY | Derives::PARTIAL_EQ
					| Derives::EQ | Derives::HASH
					| Derives::SERDE,
			},
			TypeProxy {
				protocol_type_name: "types::Vec2f".into(),
				rust_type: "crate::types::Vec2F".into(),
				derives: Derives::CLONE | Derives::COPY | Derives::PARTIAL_EQ | Derives::SERDE,
			},
			TypeProxy {
				protocol_type_name: "types::Vec3f".into(),
				rust_type: "crate::types::Vec3F".into(),
				derives: Derives::CLONE | Derives::COPY | Derives::PARTIAL_EQ | Derives::SERDE,
			},
			TypeProxy {
				protocol_type_name: "types::Vec2i".into(),
				rust_type: "crate::types::Vec2I".into(),
				derives: Derives::CLONE | Derives::COPY | Derives::PARTIAL_EQ | Derives::SERDE,
			},
			TypeProxy {
				protocol_type_name: "types::Vec3i".into(),
				rust_type: "crate::types::Vec3I".into(),
				derives: Derives::CLONE | Derives::COPY | Derives::PARTIAL_EQ | Derives::SERDE,
			},
			TypeProxy {
				protocol_type_name: "types::Vec4f".into(),
				rust_type: "crate::types::Vec4F".into(),
				derives: Derives::CLONE | Derives::COPY | Derives::PARTIAL_EQ | Derives::SERDE,
			},
			TypeProxy {
				protocol_type_name: "types::Quatf".into(),
				rust_type: "crate::types::QuatF".into(),
				derives: Derives::CLONE | Derives::COPY | Derives::PARTIAL_EQ | Derives::SERDE,
			},
			TypeProxy {
				protocol_type_name: "types::Mat4f".into(),
				rust_type: "crate::types::Mat4F".into(),
				derives: Derives::CLONE | Derives::COPY | Derives::PARTIAL_EQ | Derives::SERDE,
			},
			TypeProxy {
				protocol_type_name: "types::Color".into(),
				rust_type: "crate::types::Color".into(),
				derives: Derives::CLONE | Derives::COPY | Derives::PARTIAL_EQ | Derives::SERDE,
			},
		],
		true,
		"./src/protocol",
	);
}
