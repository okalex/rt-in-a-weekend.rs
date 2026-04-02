import package::{
    bindings::gpu_meta,
    color,
    constants::INF,
    hit_record,
    interval,
    material,
    pdf,
    random,
    ray,
    scene,
    types,
    vector,
};

fn get_ray_color(ray: ray::Ray) -> vec3f {
    var color = vec3f(1, 1, 1);
    var hit_ray = ray;
    let ray_t = interval::Interval(0.001, INF);

    for (var bounces = 0u; bounces < gpu_meta.max_depth; bounces = bounces + 1) {

        let hit_record = scene::hit_instances(hit_ray, ray_t);

        if hit_record.is_hit {

            let material = material::get_for_instance(hit_record.instance_id);
            let emitted_color = material::emitted(material, hit_ray, hit_record);
            let scatter_record = material::scattered(material, hit_ray, hit_record);

            if scatter_record.is_scattered {
                let scatter_result = scatter_ray(material, hit_record, scatter_record, bounces);
                color = color * (emitted_color + scatter_result.color);
                hit_ray = scatter_result.ray;
            } else {
                return color * emitted_color;
            }
        } else {
            return color * gpu_meta.background;
        }

        // Return if ray isn't contributing
        // if color::is_black(color) {
        //     return vector::ZEROS;
        // }
    }

    return vector::ZEROS;
}

struct ScatterResult {
    ray: ray::Ray,
    color: vec3f,
};

fn scatter_ray(
    material: types::GpuMaterial,
    hit_record: hit_record::HitRecord,
    scatter_record: material::ScatterRecord,
    depth: u32,
) -> ScatterResult {

    if scatter_record.skip_pdf {
        return ScatterResult(
            scatter_record.scattered,
            scatter_record.attenuation
        );
    }

    let use_importance_sampling = gpu_meta.importance_sampling > 0 && scene::has_lights();
    let lights_pdf = pdf::LightsPdf(hit_record.point);

    var scattered_dir: vec3f;
    if use_importance_sampling {
        let chance = f32(random::rand() < 0.5);
        scattered_dir = chance * scatter_record.scattered.dir + (1.0 - chance) * pdf::lights_pdf_generate(lights_pdf);
    } else {
        scattered_dir = scatter_record.scattered.dir;
    }

    let mat_pdf = material::pdf_value(material, hit_record, scattered_dir);
    var pdf_value = mat_pdf;
    if use_importance_sampling {
        pdf_value = 0.5 * mat_pdf + 0.5 * pdf::lights_pdf_value(lights_pdf, scattered_dir);
    }

    var scattered_color = vector::ZEROS;
    if pdf_value > 1e-8 {
        scattered_color = scatter_record.attenuation * mat_pdf / pdf_value;
    }

    return ScatterResult(
        ray::Ray(scatter_record.scattered.orig, scattered_dir),
        scattered_color
    );
}
