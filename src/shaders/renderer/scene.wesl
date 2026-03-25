import package::{
    bindings::{instance_bvh, instances, lights, primitives},
    bvh,
    hit_record,
    interval,
    primitive,
    ray,
    stack,
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
    let transformed_ray = ray::transform(ray, instance.inv_transform);
    let hit_record = primitive::hit(primitive, transformed_ray, ray_t);
    return hit_record::transform(hit_record, instance_id, instance.transform, instance.inv_transform);
}
