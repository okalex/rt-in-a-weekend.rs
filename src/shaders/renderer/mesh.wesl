import package::{
    bindings::{meshes, mesh_bvhs},
    bvh::{is_leaf, intersects_bvh_node},
    hit_record::{HitRecord, no_hit},
    interval::{Interval, interval_contains},
    ray::{Ray, ray_at},
    stack::{new_stack, push, pop, is_empty},
    triangle::{hit_triangle},
    types,
};

fn hit_mesh(mesh: types::GpuPrimitive_Mesh, ray: Ray, ray_t: Interval) -> HitRecord {
    let bvh_offset = mesh.mesh_bvh_id;
    var curr_ray_t = ray_t;
    var curr_hit = no_hit();
    var stack = new_stack();
    let inv_dir = 1.0 / ray.dir;
    push(&stack, bvh_offset);

    while !is_empty(&stack) {
        let node_idx = pop(&stack);
        let node = mesh_bvhs.nodes[node_idx];

        if is_leaf(node) {
            let triangle_id = node.left_or_prim;
            let triangle = types::unpack_GpuPrimitive_Triangle(meshes.triangles[triangle_id]);
            let test_hit = hit_triangle(triangle, ray, curr_ray_t);
            if test_hit.is_hit && test_hit.t < curr_hit.t {
                curr_hit = test_hit;
                curr_ray_t = Interval(ray_t.min, curr_hit.t);
            }
        } else {
            let left = mesh_bvhs.nodes[node.left_or_prim];
            let right = mesh_bvhs.nodes[node.left_or_prim + 1];
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
