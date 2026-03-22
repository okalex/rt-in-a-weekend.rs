import package::{
    bindings::{gpu_meta, bvh, primitives, instances},
    bvh::{intersects_bvh_node, is_leaf},
    constants::INF,
    hit_record::{HitRecord, no_hit},
    interval::Interval,
    material::{get_material, scatter, emitted},
    quad::hit_quad,
    ray::Ray,
    sphere::hit_sphere,
    stack::{new_stack, push, pop, is_empty},
    triangle::hit_triangle,
    types,
    vector::ZEROS,
};

fn get_ray_color(ray: Ray) -> vec3<f32> {
    var color = vec3<f32>(1, 1, 1);
    var hit_ray = ray;
    let ray_t = Interval(0.001, INF);

    for (var bounces = 0u; bounces < gpu_meta.max_depth; bounces = bounces + 1) {
        let hit_record = hit_instances(hit_ray, ray_t);
        if hit_record.is_hit {
            let material = get_material(hit_record.instance_id);
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

fn hit_instances(ray: Ray, ray_t: Interval) -> HitRecord {
    var curr_ray_t = ray_t;
    var curr_hit = no_hit();
    var stack = new_stack();
    let inv_dir = 1.0 / ray.dir;
    push(&stack, 0u);

    while !is_empty(&stack) {
        let node_idx = pop(&stack);
        let node = bvh.nodes[node_idx];

        if is_leaf(node) {
            let instance_id = node.left_or_prim;
            let test_hit = hit_instance(ray, curr_ray_t, instance_id);
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

fn hit_instance(ray: Ray, ray_t: Interval, instance_id: u32) -> HitRecord {
    let instance = instances.instances[instance_id];
    let primitive = primitives.primitives[instance.primitive_id];
    let transformed_ray = transform_ray(ray, instance.inv_transform);

    switch primitive._type {
        case types::GPUPRIMITIVE_SPHERE: {
            let sphere = types::unpack_GpuPrimitive_Sphere(primitive);
            var hit_record = hit_sphere(sphere, transformed_ray, ray_t);
            return transform_hit_record(hit_record, instance_id, instance.transform, instance.inv_transform);
        }

        case types::GPUPRIMITIVE_TRIANGLE: {
            let triangle = types::unpack_GpuPrimitive_Triangle(primitive);
            var hit_record = hit_triangle(triangle, transformed_ray, ray_t);
            return transform_hit_record(hit_record, instance_id, instance.transform, instance.inv_transform);
        }

        case types::GPUPRIMITIVE_QUAD: {
            let quad = types::unpack_GpuPrimitive_Quad(primitive);
            var hit_record = hit_quad(quad, transformed_ray, ray_t);
            return transform_hit_record(hit_record, instance_id, instance.transform, instance.inv_transform);
        }

        default: {
            return no_hit();
        }
    }
}

fn transform_ray(ray: Ray, transform: mat4x4<f32>) -> Ray {
    let orig = transform_point(ray.orig, transform);
    let dir = transform_direction(ray.dir, transform);
    return Ray(orig, dir);
}

fn transform_hit_record(
    hit_record: HitRecord,
    instance_id: u32,
    transform: mat4x4<f32>,
    inv_transform: mat4x4<f32>
) -> HitRecord {
    var new_record = hit_record;

    new_record.instance_id = instance_id;
    new_record.point = transform_point(hit_record.point, transform);
    new_record.normal = transform_normal(hit_record.normal, inv_transform);

    return new_record;
}

fn transform_point(point: vec3<f32>, transform: mat4x4<f32>) -> vec3<f32> {
    let p4 = transform * vec4<f32>(point, 1.0);
    return p4.xyz / p4.w;
}

fn transform_direction(direction: vec3<f32>, transform: mat4x4<f32>) -> vec3<f32> {
    return (transform * vec4<f32>(direction, 0.0)).xyz;
}

fn transform_normal(normal: vec3<f32>, inv_transform: mat4x4<f32>) -> vec3<f32> {
    return normalize((transpose(inv_transform) * vec4<f32>(normal, 0.0)).xyz);
}
