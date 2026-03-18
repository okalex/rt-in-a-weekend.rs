import package::{
    bindings::{output, gpu_meta, objects},
    camera::get_ray,
    color::{Rgba, to_rgba},
    constants::{PI},
    hit_record::{HitRecord, no_hit},
    interval::Interval,
    material::{get_material, scatter},
    random::{init_rng, rand_f32, rand_vector_on_hemisphere},
    ray::Ray,
    sphere::hit_sphere,
    types,
    vector::{length_squared, ZEROS},
};

@compute
@workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>
) {
    let x = global_id.x;
    let y = global_id.y;

    if x >= gpu_meta.width || y >= gpu_meta.height {
        return;
    }

    init_rng(x, y, 1);

    let idx = gpu_meta.width * y + x;

    var color = vec3<f32>(0, 0, 0);
    for (var i = 0u; i < gpu_meta.num_samples; i = i + 1) {
        let camera_ray = get_ray(x, y);
        color = color + get_ray_color(camera_ray);
    }
    color = color / f32(gpu_meta.num_samples);

    output[idx] = to_rgba(color);
}

fn get_ray_color(ray: Ray) -> vec3<f32> {
    var color = vec3<f32>(1, 1, 1);
    var hit_ray = ray;
    let ray_t = Interval(0.001, 1000000000000);

    for (var bounces = 0u; bounces < gpu_meta.max_depth; bounces = bounces + 1) {
        let hit_record = hit_objects(hit_ray, ray_t);
        if hit_record.is_hit {

            let material = get_material(hit_record.mat_idx);
            let scatter_record = scatter(material, hit_ray, hit_record);
            if scatter_record.is_scattered {
                if scatter_record.skip_pdf {
                    color = color * scatter_record.attenuation;
                    hit_ray = scatter_record.scattered;
                } else {
                    color = color * scatter_record.attenuation * scatter_record.scattering_pdf / scatter_record.pdf_value;
                    hit_ray = scatter_record.scattered;
                }
            } else {
                // TODO
                return ZEROS;
            }
        } else {
            return color * gpu_meta.background;
        }
    }
    return ZEROS;
}

fn hit_objects(ray: Ray, ray_t: Interval) -> HitRecord {
    var hit_record = no_hit();
    var closest = ray_t.max;

    for (var i: u32 = 0; i < arrayLength(&objects.objects); i = i + 1) {
        let object = objects.objects[i];
        let temp_record = hit_object(ray, Interval(ray_t.min, closest), object);
        if temp_record.is_hit {
            hit_record = temp_record;
            closest = hit_record.t;
        }
    }
    return hit_record;
}

fn hit_object(ray: Ray, ray_t: Interval, object: types::GpuShape) -> HitRecord {
    switch object._type {
        case types::GPUSHAPE_SPHERE: {
            let sphere = types::unpack_GpuShape_Sphere(object);
            return hit_sphere(ray, ray_t, sphere);
        }

        default: {
            return no_hit();
        }
    }
}


