import package::{
    bindings::{gpu_meta, bvh, objects},
    bvh::{intersects_bvh_node, is_leaf},
    constants::INF,
    hit_record::{HitRecord, no_hit},
    interval::Interval,
    material::{get_material, scatter, emitted},
    ray::Ray,
    sphere::hit_sphere,
    stack::{new_stack, push, pop, is_empty},
    types,
    vector::ZEROS,
};

fn get_ray_color(ray: Ray) -> vec3<f32> {
    var color = vec3<f32>(1, 1, 1);
    var hit_ray = ray;
    let ray_t = Interval(0.001, INF);

    for (var bounces = 0u; bounces < gpu_meta.max_depth; bounces = bounces + 1) {
        let hit_record = hit_objects(hit_ray, ray_t);
        if hit_record.is_hit {

            let material = get_material(hit_record.mat_idx);
            let emitted_color = emitted(material, hit_ray, hit_record);
            let scatter_record = scatter(material, hit_ray, hit_record);

            if scatter_record.is_scattered {
                if scatter_record.skip_pdf {
                    color = color * (emitted_color + scatter_record.attenuation);
                    hit_ray = scatter_record.scattered;
                } else {
                    // Discard NaNs
                    if scatter_record.pdf_value > 1e-8 {
                        color = color * (emitted_color + scatter_record.attenuation * scatter_record.scattering_pdf / scatter_record.pdf_value);
                    }

                    hit_ray = scatter_record.scattered;
                }
            } else {
                return color * emitted_color;
            }
        } else {
            return color * gpu_meta.background;
        }
    }

    return ZEROS;
}

fn hit_objects(ray: Ray, ray_t: Interval) -> HitRecord {
    var curr_ray_t = ray_t;
    var curr_hit = no_hit();
    var stack = new_stack();
    let inv_dir = 1.0 / ray.dir;
    push(&stack, 0u);

    while !is_empty(&stack) {
        let node_idx = pop(&stack);
        let node = bvh.nodes[node_idx];

        if is_leaf(node) {
            let object = objects.objects[node.left_or_prim];
            let test_hit = hit_object(ray, curr_ray_t, object);
            if test_hit.is_hit && test_hit.t < curr_hit.t {
                curr_hit = test_hit;
                curr_ray_t = Interval(ray_t.min, curr_hit.t);
            }
        } else {
            let left = bvh.nodes[node.left_or_prim];
            let right = bvh.nodes[node.left_or_prim + 1];
            if intersects_bvh_node(ray, inv_dir, curr_ray_t, right) {
                push(&stack, node.left_or_prim + 1);
            }
            if intersects_bvh_node(ray, inv_dir, curr_ray_t, left) {
                push(&stack, node.left_or_prim);
            }
        }
    }

    return curr_hit;
}

fn hit_object(ray: Ray, ray_t: Interval, object: types::GpuShape) -> HitRecord {
    switch object._type {
        case types::GPUSHAPE_SPHERE: {
            let sphere = types::unpack_GpuShape_Sphere(object);
            return hit_sphere(sphere, ray, ray_t);
        }

        default: {
            return no_hit();
        }
    }
}
