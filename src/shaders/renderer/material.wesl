import package::{
    bindings::materials,
    color::Rgba,
    texture::get_texture_color,
    types,
};

fn get_material(mat_idx: u32) -> types::GpuMaterial {
    return materials.materials[mat_idx];
}

// TODO: uv mapping
fn get_material_color(material: types::GpuMaterial) -> vec3<f32> {
    switch material._type {
        case types::GPUMATERIAL_LAMBERTIAN: {
            let lambertian = types::unpack_GpuMaterial_Lambertian(material);
            return get_texture_color(lambertian.texture);
        }

        default: {
            return vec3<f32>(0, 0, 0);
        }
    }
}
