use std::{
    fs,
    io::{BufReader, Cursor},
    path::Path,
    sync::Arc,
};

use crate::rt::{
    materials::{lambertian::Lambertian, material::Material},
    objects::mesh::Mesh,
};

pub fn load_string_path(path: &Path) -> anyhow::Result<String> {
    let path = Path::new(env!("OUT_DIR")).join("assets").join(path);
    println!("Loading text from {}", path.file_name().unwrap().display());
    let txt = fs::read_to_string(path)?;
    Ok(txt)
}

pub fn load_string(file_name: &str) -> anyhow::Result<String> {
    let path = Path::new(env!("OUT_DIR")).join("assets").join(file_name);
    load_string_path(path.as_path())
}

pub fn load_model_with_mat(file_name: &str, mat: Arc<dyn Material>) -> anyhow::Result<Vec<Mesh>> {
    println!("Loading model from {}", file_name);

    let obj_text = load_string(file_name)?;
    let obj_cursor = Cursor::new(obj_text);
    let mut obj_reader = BufReader::new(obj_cursor);

    #[allow(unused)]
    let (models, obj_materials) = tobj::load_obj_buf(
        &mut obj_reader,
        &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        },
        |p| {
            println!("Loading material from {}", p.file_name().unwrap().display());
            let mat_text = load_string_path(p).unwrap();
            tobj::load_mtl_buf(&mut BufReader::new(Cursor::new(mat_text)))
        },
    )?;

    let meshes = models
        .into_iter()
        .map(|m| Mesh::from_tobj(&m.mesh, Arc::clone(&mat)))
        .collect::<Vec<_>>();

    Ok(meshes)
}

#[allow(unused)]
pub fn load_model(file_name: &str) -> anyhow::Result<Vec<Mesh>> {
    let mat: Arc<dyn Material> = Arc::new(Lambertian::from_color_values([0.4, 0.4, 0.6]));
    load_model_with_mat(file_name, mat)
}
