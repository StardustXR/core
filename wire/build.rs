use manifest_dir_macros::{directory_relative_path, path};
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
	if option_env!("STARDUST_REGEN_FBS").is_none() {
		return;
	}

	println!("cargo:rerun-if-changed=fbs");
	let out_dir = PathBuf::from_str(path!("src")).unwrap();
	fs::create_dir_all(&out_dir).unwrap();

	let files: Vec<_> = fs::read_dir(directory_relative_path!("fbs"))
		.unwrap()
		.filter_map(Result::ok)
		.map(|d| d.path())
		.filter(|p| p.extension().unwrap_or_default() == "fbs")
		.collect();

	for file in &files {
		let file_name = file.with_extension("rs");
		let Some(file_name) = file_name.file_name().and_then(OsStr::to_str) else {
			continue;
		};
		let _ = fs::remove_file(out_dir.join(file_name));
	}
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
}
