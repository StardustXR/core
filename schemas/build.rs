use std::fmt::Write;
use std::fs;

use anyhow::Context;

fn main() -> anyhow::Result<()> {
	// HACK(philpax): the other modules don't actually build right now, so
	// whitelist the ones we can actually use
	const WHITELISTED: &[&str] = &["message"];

	println!("cargo:rerun-if-changed=schemas");
	let out_dir = std::path::Path::new(&std::env::var("OUT_DIR")?).to_owned();

	let files: Vec<_> = fs::read_dir("src")?
		.filter_map(Result::ok)
		.map(|d| d.path())
		.filter(|p| p.extension().unwrap_or_default() == "fbs")
		.filter(|p| {
			WHITELISTED
				.iter()
				.any(|w| *w == p.file_stem().unwrap().to_string_lossy())
		})
		.collect();

	let args: Vec<_> = [
		"--rust",
		"--gen-mutable",
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
		.context("failed to execute flatc")?;

	if !output.status.success() {
		return Err(anyhow::anyhow!(
			"{}",
			String::from_utf8_lossy(&output.stdout)
		));
	}

	let mut buf = String::with_capacity(files.len() * 100);
	for file in files {
		let stem = file.file_stem().unwrap().to_str().unwrap();
		let rs_file = file.with_extension("rs");
		let name = rs_file.file_name().unwrap().to_str().unwrap();

		write!(
			buf,
			"pub mod {} {{ \n\tinclude!(concat!(env!(\"OUT_DIR\"), \"/{}\")); \n}}\n",
			stem, name
		)?;
	}

	fs::write(out_dir.join("mod.rs"), buf)?;

	Ok(())
}
