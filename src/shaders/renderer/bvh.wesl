import package::{
    interval::Interval,
    ray::Ray,
    types,
};

fn intersects_bvh_node(ray: Ray, inv_dir: vec3<f32>, ray_t: Interval, node: types::GpuBvhNode) -> bool {
    let t0s = (node.aabb_min - ray.orig) * inv_dir;
    let t1s = (node.aabb_max - ray.orig) * inv_dir;

    let tsmall = min(t0s, t1s);
    let tbig = max(t0s, t1s);

    let t_min = max(ray_t.min, max(tsmall.x, max(tsmall.y, tsmall.z)));
    let t_max = min(ray_t.max, min(tbig.x, min(tbig.y, tbig.z)));

    return t_max > t_min;
}

fn is_leaf(node: types::GpuBvhNode) -> bool {
    return node.count > 0;
}
