use gluon_codegen_rust::Derives;
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
			("camera",Path::new("./gluon/org.stardustxr.Camera.gluon"))
		],
		&[],
		Derives::CLONE | Derives::COPY | Derives::HASH | Derives::PARTIAL_EQ | Derives::EQ,
		"./src/protocol",
	);
}
