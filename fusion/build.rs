use std::env;
use std::path::PathBuf;

fn main() {
    // Watch for changes to KDL schema files
    let schema_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .join("schemas/src/protocol");
    
    // Tell cargo to rerun if any KDL files change
    println!("cargo:rerun-if-changed={}", schema_dir.join("*.kdl").display());
    
    // Also watch the specific files since glob patterns might not work
    let schema_files = [
        "root.kdl", "node.kdl", "spatial.kdl", "field.kdl", 
        "audio.kdl", "drawable.kdl", "input.kdl", "item.kdl",
        "item_camera.kdl", "item_panel.kdl"
    ];
    
    for file in &schema_files {
        println!("cargo:rerun-if-changed={}", schema_dir.join(file).display());
    }
    
    // Also rerun if the codegen library itself changes
    println!("cargo:rerun-if-changed=codegen/src/lib.rs");
    
    // Use the codegen library directly to regenerate protocol files
    let protocols = stardust_xr_fusion_codegen::get_all_protocols();
    let output_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("src/protocol");
    
    stardust_xr_fusion_codegen::generate_protocol_files(&protocols, &output_dir, false)
        .expect("Failed to generate protocol files");
    
    println!("Protocol files regenerated successfully");
}
