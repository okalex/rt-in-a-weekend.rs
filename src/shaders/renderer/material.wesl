import package::{
    bindings::materials,
    color::Rgba,
    constants::{PI, ZEROS},
    hit_record::HitRecord,
    pdf::{HemispherePdf, new_hemisphere_pdf, hemisphere_generate, hemisphere_pdf_value},
    random::rand_unit_vector,
    ray::Ray,
    texture::get_texture_color,
    vector::reflect,
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

struct ScatterRecord {
    is_scattered: bool,
    attenuation: vec3<f32>,
    scattered: Ray,
    scattering_pdf: f32,
    pdf_value: f32,
    skip_pdf: bool,
}

fn no_scatter() -> ScatterRecord {
    return ScatterRecord(
        false,
        vec3<f32>(0, 0, 0),
        Ray(ZEROS, ZEROS),
        0.0,
        0.0,
        false,
    );
}

fn scatter_with_pdf(attenuation: vec3<f32>, scattered: Ray, scattering_pdf: f32, pdf_value: f32) -> ScatterRecord {
    return ScatterRecord(
        true,
        attenuation,
        scattered,
        scattering_pdf,
        pdf_value,
        false,
    );
}

fn skip_pdf(attenuation: vec3<f32>, skip_pdf_ray: Ray) -> ScatterRecord {
    return ScatterRecord(
        true,
        attenuation,
        skip_pdf_ray,
        0.0,
        0.0,
        true,
    );
}

fn scatter(mat: types::GpuMaterial, ray_in: Ray, hit_record: HitRecord) -> ScatterRecord {
    switch mat._type {
        case types::GPUMATERIAL_LAMBERTIAN {
            let lambertian = types::unpack_GpuMaterial_Lambertian(mat);
            let color = get_texture_color(lambertian.texture);

            let pdf = new_hemisphere_pdf(hit_record.normal);
            let scattered_dir = hemisphere_generate(pdf);
            let scattered = Ray(hit_record.point, scattered_dir);

            return scatter_with_pdf(
                color,
                scattered,
                lambertian_scattering_pdf(hit_record, scattered),
                hemisphere_pdf_value(pdf),
            );
        }

        case types::GPUMATERIAL_METAL {
            let metal = types::unpack_GpuMaterial_Metal(mat);
            let reflected = normalize(reflect(ray_in.dir, hit_record.normal)) + rand_unit_vector() * metal.fuzz;
            return skip_pdf(
                metal.albedo,
                Ray(hit_record.point, reflected)
            );
        }

        default: {
            return no_scatter();
        }
    }
}

fn lambertian_scattering_pdf(hit_record: HitRecord, scattered: Ray) -> f32 {
    let cos_theta = dot(hit_record.normal, normalize(scattered.dir));
    return max(0.0, cos_theta / PI);
}
