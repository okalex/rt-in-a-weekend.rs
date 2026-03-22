use std::{
    env,
    path::PathBuf,
};

use anyhow::*;
use fs_extra::{
    copy_items,
    dir::CopyOptions,
};

fn main() -> Result<()> {
    let _ = copy_assets();
    let _ = build_shaders();

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

fn build_shaders() -> Result<()> {
    println!("cargo:rerun-if-changed=src/shaders");

    let input = vec![PathBuf::from("src/")];
    let output = PathBuf::from("src/shaders/renderer/types.wesl");

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let wgsl = wgsl_autogen::generate_wgsl_from_files(&input, false).expect("failed to generate WGSL");
    std::fs::write(&output, wgsl).expect("failed to write WGSL file");

    wesl::Wesl::new("src/shaders/renderer").build_artifact(&"package::main".parse().unwrap(), "render_shader");

    Ok(())
}
