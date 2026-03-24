import package::{
    bindings::{instances, materials},
    constants::{PI},
    hit_record,
    pdf,
    random,
    ray,
    texture,
    vector,
    types,
};

fn get_for_instance(instance_id: u32) -> types::GpuMaterial {
    let instance = instances.instances[instance_id];
    return materials.materials[instance.material_id];
}

struct ScatterRecord {
    is_scattered: bool,
    attenuation: vec3f,
    scattered: ray::Ray,
    scattering_pdf: f32,
    pdf_value: f32,
    skip_pdf: bool,
}

fn no_scatter() -> ScatterRecord {
    return ScatterRecord(
        false,
        vec3f(0, 0, 0),
        ray::Ray(vector::ZEROS, vector::ZEROS),
        0.0,
        0.0,
        false,
    );
}

fn scatter_with_pdf(
    attenuation: vec3f,
    scattered: ray::Ray,
    scattering_pdf: f32,
    pdf_value: f32
) -> ScatterRecord {
    return ScatterRecord(
        true,
        attenuation,
        scattered,
        scattering_pdf,
        pdf_value,
        false,
    );
}

fn skip_pdf(attenuation: vec3f, skip_pdf_ray: ray::Ray) -> ScatterRecord {
    return ScatterRecord(
        true,
        attenuation,
        skip_pdf_ray,
        0.0,
        0.0,
        true,
    );
}

fn scattered(mat: types::GpuMaterial, ray_in: ray::Ray, hit_record: hit_record::HitRecord) -> ScatterRecord {
    switch mat._type {
        case types::GPUMATERIAL_LAMBERTIAN {
            let lambertian = types::unpack_GpuMaterial_Lambertian(mat);
            let color = texture::get_color(lambertian.texture, hit_record.u, hit_record.v, hit_record.point);

            let pdf = pdf::new_cosine_pdf(hit_record.normal);
            let scattered_dir = pdf::cosine_generate(pdf);
            let scattered = ray::Ray(hit_record.point, scattered_dir);

            return scatter_with_pdf(
                color,
                scattered,
                lambertian_scattering_pdf(hit_record, scattered),
                pdf::cosine_pdf_value(pdf, scattered_dir),
            );
        }

        case types::GPUMATERIAL_METAL {
            let metal = types::unpack_GpuMaterial_Metal(mat);
            let reflected = normalize(reflect(ray_in.dir, hit_record.normal)) + random::rand_unit_vector() * metal.fuzz;
            return skip_pdf(
                metal.albedo,
                ray::Ray(hit_record.point, reflected)
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

            var attenuation: vec3f;
            var dir: vec3f;
            if cannot_refract || reflectance > random::rand() {
                attenuation = vector::ONES;
                dir = reflect(unit_dir, hit_record.normal);
            } else {
                attenuation = dielectric.albedo;
                dir = refract(unit_dir, hit_record.normal, ri);
            }

            return skip_pdf(
                attenuation,
                ray::Ray(hit_record.point, dir),
            );
        }

        case types::GPUMATERIAL_ISOTROPIC {
            let isotropic = types::unpack_GpuMaterial_Isotropic(mat);
            let color = texture::get_color(isotropic.texture, hit_record.u, hit_record.v, hit_record.point);

            let scattered_dir = pdf::sphere_generate();
            let scattered = ray::Ray(hit_record.point, scattered_dir);

            return scatter_with_pdf(
                color,
                scattered,
                1.0 / (4.0 * PI),
                pdf::sphere_pdf_value(),
            );
        }

        default: {
            return no_scatter();
        }
    }
}

fn emitted(mat: types::GpuMaterial, ray_in: ray::Ray, hit_record: hit_record::HitRecord) -> vec3f {
    switch mat._type {
        case types::GPUMATERIAL_EMISSIVE {
            if hit_record.is_outward {
                let emissive = types::unpack_GpuMaterial_Emissive(mat);
                return emissive.color;
            } else {
                return vector::ZEROS;
            }
        }

        default: {
            return vector::ZEROS;
        }
    }
}

fn reflectance(cosine: f32, refraction_idx: f32) -> f32 {
    let r0_tmp = (1.0 - refraction_idx) / (1.0 + refraction_idx);
    let r0 = r0_tmp * r0_tmp;
    return r0 + (1.0 - r0) * pow(1.0 - cosine, 5.0);
}

fn lambertian_scattering_pdf(hit_record: hit_record::HitRecord, scattered: ray::Ray) -> f32 {
    let cos_theta = dot(hit_record.normal, normalize(scattered.dir));
    return max(0.0, cos_theta / PI);
}

fn scattering_pdf(mat: types::GpuMaterial, hit_record: hit_record::HitRecord, scattered: ray::Ray) -> f32 {
    switch mat._type {
        case types::GPUMATERIAL_LAMBERTIAN {
            return lambertian_scattering_pdf(hit_record, scattered);
        }
        case types::GPUMATERIAL_ISOTROPIC {
            return 1.0 / (4.0 * PI);
        }
        default: {
            return 0.0;
        }
    }
}

fn pdf_value(mat: types::GpuMaterial, hit_record: hit_record::HitRecord, direction: vec3f) -> f32 {
    switch mat._type {
        case types::GPUMATERIAL_LAMBERTIAN {
            let pdf = pdf::new_cosine_pdf(hit_record.normal);
            return pdf::cosine_pdf_value(pdf, direction);
        }
        case types::GPUMATERIAL_ISOTROPIC {
            return pdf::sphere_pdf_value();
        }
        default: {
            return 0.0;
        }
    }
}
