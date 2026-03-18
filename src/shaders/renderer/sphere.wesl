import package::{
    hit_record::{HitRecord, no_hit},
    interval::Interval,
    ray::{Ray, ray_at},
    types,
    vector::length_squared,
};

fn hit_sphere(ray: Ray, ray_t: Interval, sphere: types::GpuShape_Sphere) -> HitRecord {
    let oc = sphere.center - ray.orig;
    let a = length_squared(ray.dir);
    let h = dot(ray.dir, oc);
    let c = length_squared(oc) - sphere.radius * sphere.radius;

    let discriminant = h * h - a * c;
    if discriminant < 0 {
        return no_hit();
    }

    let sqrtd = sqrt(discriminant);

    var root = (h - sqrtd) / a;
    if (root <= ray_t.min || ray_t.max <= root) {
        root = (h + sqrtd) / a;
        if (root <= ray_t.min || ray_t.max <= root) {
            return no_hit();
        }
    }

    let hit_point = ray_at(ray, root);
    var normal = (hit_point - sphere.center) / sphere.radius;
    let is_outward = dot(ray.dir, normal) < 0.0;
    if !is_outward {
        normal = -normal;
    }

    return HitRecord(
        true,
        hit_point,
        normal,
        is_outward,
        root,
        sphere.mat_idx,
    );
}
