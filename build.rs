use anyhow::*;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::env;
use std::path::PathBuf;

fn main() -> Result<()> {
    let _ = copy_assets();
    let _ = generate_shader_types();

    Ok(())
}

fn copy_assets() -> Result<()> {
    println!("cargo:rerun-if-changed=assets/**/*");

    let out_dir = env::var("OUT_DIR")?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut paths_to_copy = Vec::new();
    paths_to_copy.push("assets/");
    let _ = copy_items(&paths_to_copy, out_dir, &copy_options);

    Ok(())
}

fn generate_shader_types() -> Result<()> {
    println!("cargo:rerun-if-changed=src/");

    let input = vec![PathBuf::from("src/")];
    let output = PathBuf::from("src/shaders/types.wgsl");

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let wgsl =
        wgsl_autogen::generate_wgsl_from_files(&input, true).expect("failed to generate WGSL");
    std::fs::write(&output, wgsl).expect("failed to write WGSL file");

    Ok(())
}
