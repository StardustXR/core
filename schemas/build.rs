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

	// Generate a single module containing all of the generated modules;
	// This is really messy :(
	fs::write(
		out_dir.join("mod.rs"),
		&files
			.iter()
			.map(|path| {
				let module_name = path.file_stem().unwrap().to_string_lossy();
				let generated_path = out_dir.join(path.with_extension("rs").file_name().unwrap());
				let module_contents = fs::read_to_string(generated_path).unwrap_or_default();

				format!("pub mod {} {{ {} }}", module_name, module_contents)
			})
			.collect::<Vec<_>>()
			.join("\n"),
	)?;

	Ok(())
}
