import package::{
    bindings::{gpu_meta, instance_bvh, primitives, instances, lights},
    bvh,
    constants::INF,
    hit_record,
    interval,
    material,
    medium,
    mesh,
    pdf,
    quad,
    random,
    ray,
    sphere,
    stack,
    transform,
    triangle,
    types,
    vector,
};

fn get_ray_color(ray: ray::Ray) -> vec3f {
    var color = vec3f(1, 1, 1);
    var hit_ray = ray;
    let ray_t = interval::Interval(0.001, INF);

    for (var bounces = 0u; bounces < gpu_meta.max_depth; bounces = bounces + 1) {

        let hit_record = hit_instances(hit_ray, ray_t);

        if hit_record.is_hit {

            let material = material::get_for_instance(hit_record.instance_id);
            let emitted_color = material::emitted(material, hit_ray, hit_record);
            let scatter_record = material::scattered(material, hit_ray, hit_record);

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

    return vector::ZEROS;
}

struct ScatterResult {
    scattered_ray: ray::Ray,
    scattered_color: vec3f,
};

fn scatter_ray(material: types::GpuMaterial, hit_record: hit_record::HitRecord, scatter_record: material::ScatterRecord) -> ScatterResult {

    if scatter_record.skip_pdf {
        return ScatterResult(
            scatter_record.scattered,
            scatter_record.attenuation
        );
    }

    var pdf_value: f32;
    var scattered_dir: vec3f;
    var scattered_color: vec3f;

    if gpu_meta.importance_sampling > 0 && arrayLength(&lights.lights) > 0 {
        let lights_pdf = pdf::LightsPdf(hit_record.point);

        let chance = f32(random::rand() < 0.5);
        scattered_dir = chance * scatter_record.scattered.dir + (1.0 - chance) * pdf::lights_pdf_generate(lights_pdf);

        // let cosine_pdf = new_cosine_pdf(hit_record.normal);
        // pdf_value = 0.5 * lights_pdf_value(lights_pdf, scattered_dir) + 0.5 * cosine_pdf_value(cosine_pdf, scattered_dir);

        pdf_value = 0.5 * pdf::lights_pdf_value(lights_pdf, scattered_dir) + 0.5 * material::pdf_value(material, hit_record, scattered_dir);
    } else {
        scattered_dir = scatter_record.scattered.dir;
        pdf_value = scatter_record.pdf_value;
    }

    let scattered_ray = ray::Ray(scatter_record.scattered.orig, scattered_dir);

    // Discard NaNs
    if pdf_value > 1e-8 {
        let scattering_pdf = material::scattering_pdf(material, hit_record, scattered_ray);
        scattered_color = scatter_record.attenuation * scattering_pdf / pdf_value;
    } else {
        scattered_color = vector::ZEROS;
    }

    return ScatterResult(
        scattered_ray,
        scattered_color
    );
}

fn hit_instances(ray: ray::Ray, ray_t: interval::Interval) -> hit_record::HitRecord {
    var curr_ray_t = ray_t;
    var curr_hit = hit_record::no_hit();
    var stack = stack::new_stack();
    let inv_dir = 1.0 / ray.dir;
    stack::push(&stack, 0u);

    while !stack::is_empty(&stack) {
        let node_idx = stack::pop(&stack);
        let node = instance_bvh.nodes[node_idx];

        if bvh::is_leaf(node) {
            let instance_id = node.left_or_prim;
            let test_hit = hit_instance(ray, curr_ray_t, instance_id);
            if test_hit.is_hit && test_hit.t < curr_hit.t {
                curr_hit = test_hit;
                curr_ray_t = interval::Interval(ray_t.min, curr_hit.t);
            }
        } else {
            let left = instance_bvh.nodes[node.left_or_prim];
            let right = instance_bvh.nodes[node.left_or_prim + 1];
            if bvh::intersects_node(ray, inv_dir, curr_ray_t, right) {
                stack::push(&stack, node.left_or_prim + 1);
            }
            if bvh::intersects_node(ray, inv_dir, curr_ray_t, left) {
                stack::push(&stack, node.left_or_prim);
            }
        }
    }

    return curr_hit;
}

fn hit_instance(ray: ray::Ray, ray_t: interval::Interval, instance_id: u32) -> hit_record::HitRecord {
    let instance = instances.instances[instance_id];
    let primitive = primitives.primitives[instance.primitive_id];
    let transformed_ray = transform_ray(ray, instance.inv_transform);

    switch primitive._type {
        case types::GPUPRIMITIVE_SPHERE: {
            let sphere = types::unpack_GpuPrimitive_Sphere(primitive);
            var hit_record = sphere::hit(sphere, transformed_ray, ray_t);
            return transform_hit_record(hit_record, instance_id, instance.transform, instance.inv_transform);
        }

        case types::GPUPRIMITIVE_TRIANGLE: {
            let triangle = types::unpack_GpuPrimitive_Triangle(primitive);
            var hit_record = triangle::hit(triangle, transformed_ray, ray_t);
            return transform_hit_record(hit_record, instance_id, instance.transform, instance.inv_transform);
        }

        case types::GPUPRIMITIVE_QUAD: {
            let quad = types::unpack_GpuPrimitive_Quad(primitive);
            var hit_record = quad::hit(quad, transformed_ray, ray_t);
            return transform_hit_record(hit_record, instance_id, instance.transform, instance.inv_transform);
        }

        case types::GPUPRIMITIVE_MESH: {
            let mesh = types::unpack_GpuPrimitive_Mesh(primitive);
            var hit_record = mesh::hit(mesh, transformed_ray, ray_t);
            return transform_hit_record(hit_record, instance_id, instance.transform, instance.inv_transform);
        }

        case types::GPUPRIMITIVE_MEDIUM: {
            let medium = types::unpack_GpuPrimitive_Medium(primitive);
            let boundary_id = medium.boundary_primitive_id;
            let boundary = primitives.primitives[boundary_id];
            var hit_record = medium::hit(medium, boundary, transformed_ray, ray_t);
            return transform_hit_record(hit_record, instance_id, instance.transform, instance.inv_transform);
        }

        default: {
            return hit_record::no_hit();
        }
    }
}

fn transform_ray(ray: ray::Ray, transform: mat4x4f) -> ray::Ray {
    let orig = transform::transform_point(ray.orig, transform);
    let dir = transform::transform_direction(ray.dir, transform);
    return ray::Ray(orig, dir);
}

fn transform_hit_record(
    hit_record: hit_record::HitRecord,
    instance_id: u32,
    transform: mat4x4f,
    inv_transform: mat4x4f
) -> hit_record::HitRecord {
    var new_record = hit_record;

    new_record.instance_id = instance_id;
    new_record.point = transform::transform_point(hit_record.point, transform);
    new_record.normal = transform_normal(hit_record.normal, inv_transform);

    return new_record;
}

fn transform_normal(normal: vec3f, inv_transform: mat4x4f) -> vec3f {
    return normalize((transpose(inv_transform) * vec4f(normal, 0.0)).xyz);
}
