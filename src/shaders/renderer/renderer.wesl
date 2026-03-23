import package::{
    bindings::{gpu_meta, instance_bvh, primitives, instances, lights},
    bvh::{intersects_bvh_node, is_leaf},
    constants::INF,
    hit_record::{HitRecord, no_hit},
    interval::Interval,
    material::{ScatterRecord, get_material, scatter, emitted, material_scattering_pdf},
    mesh::hit_mesh,
    pdf::{LightsPdf, lights_pdf_value, lights_pdf_generate, new_cosine_pdf, cosine_pdf_value},
    quad::hit_quad,
    random::{rand, rand_range_int},
    ray::Ray,
    sphere::hit_sphere,
    stack::{new_stack, push, pop, is_empty},
    transform::{transform_point, transform_direction},
    triangle::hit_triangle,
    types,
    vector::{ZEROS, ONES},
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
                let scatter_result = scatter_ray(material, hit_record, scatter_record);
                color = color * (emitted_color + scatter_result.scattered_color);
                hit_ray = scatter_result.scattered_ray;
            } else {
                return color * emitted_color;
            }
        } else {
            return color * gpu_meta.background;
        }
    }

    return ZEROS;
}

struct ScatterResult {
    scattered_ray: Ray,
    scattered_color: vec3<f32>,
};

fn scatter_ray(material: types::GpuMaterial, hit_record: HitRecord, scatter_record: ScatterRecord) -> ScatterResult {
    var result: ScatterResult;

    if !scatter_record.skip_pdf {
        var pdf_value: f32;
        var scattered_dir: vec3<f32>;

        if gpu_meta.importance_sampling > 0 && arrayLength(&lights.lights) > 0 {
            let lights_pdf = LightsPdf(hit_record.point);

            let chance = f32(rand() < 0.5);
            scattered_dir = chance * scatter_record.scattered.dir + (1.0 - chance) * lights_pdf_generate(lights_pdf);

            let cosine_pdf = new_cosine_pdf(hit_record.normal);
            pdf_value = 0.5 * lights_pdf_value(lights_pdf, scattered_dir) + 0.5 * cosine_pdf_value(cosine_pdf, scattered_dir);
        } else {
            scattered_dir = scatter_record.scattered.dir;
            pdf_value = scatter_record.pdf_value;
        }

        result.scattered_ray = Ray(scatter_record.scattered.orig, scattered_dir);

        // Discard NaNs
        if pdf_value > 1e-8 {
            let scattering_pdf = material_scattering_pdf(material, hit_record, result.scattered_ray);
            result.scattered_color = scatter_record.attenuation * scattering_pdf / pdf_value;
        } else {
            result.scattered_color = ONES;
        }
    } else {
        result.scattered_color = scatter_record.attenuation;
        result.scattered_ray = scatter_record.scattered;
    }

    return result;
}

fn hit_instances(ray: Ray, ray_t: Interval) -> HitRecord {
    var curr_ray_t = ray_t;
    var curr_hit = no_hit();
    var stack = new_stack();
    let inv_dir = 1.0 / ray.dir;
    push(&stack, 0u);

    while !is_empty(&stack) {
        let node_idx = pop(&stack);
        let node = instance_bvh.nodes[node_idx];

        if is_leaf(node) {
            let instance_id = node.left_or_prim;
            let test_hit = hit_instance(ray, curr_ray_t, instance_id);
            if test_hit.is_hit && test_hit.t < curr_hit.t {
                curr_hit = test_hit;
                curr_ray_t = Interval(ray_t.min, curr_hit.t);
            }
        } else {
            let left = instance_bvh.nodes[node.left_or_prim];
            let right = instance_bvh.nodes[node.left_or_prim + 1];
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

        case types::GPUPRIMITIVE_MESH: {
            let mesh = types::unpack_GpuPrimitive_Mesh(primitive);
            var hit_record = hit_mesh(mesh, transformed_ray, ray_t);
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

fn transform_normal(normal: vec3<f32>, inv_transform: mat4x4<f32>) -> vec3<f32> {
    return normalize((transpose(inv_transform) * vec4<f32>(normal, 0.0)).xyz);
}
