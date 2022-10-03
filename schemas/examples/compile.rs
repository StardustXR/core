use manifest_dir_macros::{directory_relative_path, file_relative_path};
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
	println!("cargo:rerun-if-changed=schemas");
	let out_dir = PathBuf::from_str(directory_relative_path!("src/generated")).unwrap();
	fs::remove_dir_all(&out_dir).unwrap();
	fs::create_dir_all(&out_dir).unwrap();

	let files: Vec<_> = fs::read_dir(directory_relative_path!("fbs"))
		.unwrap()
		.filter_map(Result::ok)
		.map(|d| d.path())
		.filter(|p| p.extension().unwrap_or_default() == "fbs")
		.collect();

	let args: Vec<_> = [
		"--rust",
		"--gen-mutable",
		"--gen-object-api",
		"--gen-name-strings",
		"-o",
		out_dir.to_str().unwrap(),
		"--filename-suffix",
		"",
	]
	.iter()
	.map(|s| s.to_string())
	.chain(files.iter().map(|p| p.to_string_lossy().to_string()))
	.collect();

	let output = std::process::Command::new("flatc")
		.args(&args)
		.output()
		.expect("failed to execute flatc");

	if !output.status.success() {
		panic!("{}", String::from_utf8_lossy(&output.stdout));
	}

	let mut buf = String::with_capacity(files.len() * 150);
	for file in files {
		let stem = file.file_stem().unwrap().to_str().unwrap();
		let rs_file = file.with_extension("rs");
		let name = rs_file.file_name().unwrap().to_str().unwrap();

		write!(
			buf,
			"pub mod {} {{ \n\tpub use self::stardust_xr::*;\n\tinclude!(\"{}\"); \n}}\n",
			stem, name
		)
		.unwrap();
	}

	fs::write(file_relative_path!("src/generated/mod.rs"), buf).unwrap();
}
