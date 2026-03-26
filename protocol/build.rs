use gluon_codegen_rust::Derives;
use std::path::Path;

fn main() {
	gluon_codegen_rust::helpers::gen_multiple_modules(
		&[
			("server", Path::new("./gluon/org.stardustxr.Server.gluon")),
			("spatial", Path::new("./gluon/org.stardustxr.Spatial.gluon")),
		],
		&[],
		Derives::CLONE | Derives::COPY | Derives::HASH | Derives::PARTIAL_EQ | Derives::EQ,
		"protocol",
		"./src/protocol.rs",
	);
}
