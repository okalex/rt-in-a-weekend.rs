import package::{
    bindings::materials,
    color::Rgba,
    constants::{PI},
    hit_record::HitRecord,
    pdf,
    random::{rand, rand_unit_vector},
    ray::Ray,
    texture::get_texture_color,
    vector::{reflect, refract, ZEROS, ONES},
    types,
};

fn get_material(mat_idx: u32) -> types::GpuMaterial {
    return materials.materials[mat_idx];
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
            let color = get_texture_color(lambertian.texture, hit_record.u, hit_record.v, hit_record.point);

            let pdf = pdf::new_cosine_pdf(hit_record.normal);
            let scattered_dir = pdf::cosine_generate(pdf);
            let scattered = Ray(hit_record.point, scattered_dir);

            return scatter_with_pdf(
                color,
                scattered,
                lambertian_scattering_pdf(hit_record, scattered),
                pdf::cosine_pdf_value(pdf, scattered_dir),
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

        case types::GPUMATERIAL_DIELECTRIC {
            let dielectric = types::unpack_GpuMaterial_Dielectric(mat);

            var ri = dielectric.refraction_idx;
            if hit_record.is_outward {
                ri = 1.0 / ri;
            }

            let unit_dir = normalize(ray_in.dir);
            let cos_theta = min(dot(-unit_dir, hit_record.normal), 1.0);
            let sin_theta = sqrt(1.0 - cos_theta * cos_theta);

            let cannot_refract = (ri * sin_theta) > 1.0;
            let reflectance = reflectance(cos_theta, ri);

            var dir: vec3<f32>;
            if cannot_refract || reflectance > rand() {
                dir = reflect(unit_dir, hit_record.normal);
            } else {
                dir = refract(unit_dir, hit_record.normal, ri);
            }

            return skip_pdf(
                ONES,
                Ray(hit_record.point, dir),
            );
        }

        default: {
            return no_scatter();
        }
    }
}

fn reflectance(cosine: f32, refraction_idx: f32) -> f32 {
    let r0_tmp = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    let r0 = r0_tmp * r0_tmp;
    return r0 + (1.0 - r0) * pow(1.0 - cosine, 5.0);
}

fn lambertian_scattering_pdf(hit_record: HitRecord, scattered: Ray) -> f32 {
    let cos_theta = dot(hit_record.normal, normalize(scattered.dir));
    return max(0.0, cos_theta / PI);
}
