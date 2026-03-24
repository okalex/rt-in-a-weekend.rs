import package::{
    bindings::{meshes, mesh_bvhs},
    bvh,
    hit_record,
    interval,
    ray,
    stack,
    triangle,
    types,
};

fn hit(mesh: types::GpuPrimitive_Mesh, ray: ray::Ray, ray_t: interval::Interval) -> hit_record::HitRecord {
    var curr_ray_t = ray_t;
    var curr_hit = hit_record::no_hit();
    var stack = stack::new_stack();
    let inv_dir = 1.0 / ray.dir;
    stack::push(&stack, mesh.mesh_bvh_id);

    while !stack::is_empty(&stack) {
        let node_idx = stack::pop(&stack);
        let node = mesh_bvhs.nodes[node_idx];

        if bvh::is_leaf(node) {
            let triangle_id = node.left_or_prim;
            let triangle = types::unpack_GpuPrimitive_Triangle(meshes.triangles[triangle_id]);
            let test_hit = triangle::hit(triangle, ray, curr_ray_t);
            if test_hit.is_hit && test_hit.t < curr_hit.t {
                curr_hit = test_hit;
                curr_ray_t = interval::Interval(ray_t.min, curr_hit.t);
            }
        } else {
            let left = mesh_bvhs.nodes[node.left_or_prim];
            let right = mesh_bvhs.nodes[node.left_or_prim + 1];
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
