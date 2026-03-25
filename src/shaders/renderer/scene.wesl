import package::{
    bindings::{instance_bvh, instances, lights, primitives},
    bvh,
    hit_record,
    interval,
    medium,
    mesh,
    quad,
    ray,
    sphere,
    stack,
    transform,
    triangle,
    types,
};

fn has_lights() -> bool {
    return arrayLength(&lights.lights) > 0;
}

fn hit_instances(
    ray: ray::Ray,
    ray_t: interval::Interval
) -> hit_record::HitRecord {

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
            let test_hit = hit_instance(instance_id, ray, curr_ray_t);
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

fn hit_instance(
    instance_id: u32,
    ray: ray::Ray,
    ray_t: interval::Interval
) -> hit_record::HitRecord {

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

fn transform_ray(
    ray: ray::Ray,
    transform: mat4x4f
) -> ray::Ray {
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

fn transform_normal(
    normal: vec3f,
    inv_transform: mat4x4f
) -> vec3f {
    return normalize((transpose(inv_transform) * vec4f(normal, 0.0)).xyz);
}
