import package::{
    hit_record::{HitRecord, no_hit},
    interval::{Interval, interval_contains},
    ray::{Ray, ray_at},
    types,
};

fn hit_quad(quad: types::GpuPrimitive_Quad, ray: Ray, ray_t: Interval) -> HitRecord {
    // TODO - precompute these
    let n = cross(quad.u, quad.v);
    var normal = normalize(n);
    let d = dot(normal, quad.q);
    let w = n / dot(n, n);
    let area = length(n);

    let denom = dot(normal, ray.dir);
    if abs(denom) < 1e-8 {
        return no_hit();
    }

    let t = (d - dot(normal, ray.orig)) / denom;
    if !interval_contains(ray_t, t) {
        return no_hit();
    }

    let intersection = ray_at(ray, t);
    let planar_hit_vec = intersection - quad.q;
    let alpha = dot(w, cross(planar_hit_vec, quad.v));
    let beta = dot(w, cross(quad.u, planar_hit_vec));

    let unit_interval = Interval(0.0, 1.0);
    if !interval_contains(unit_interval, alpha) || !interval_contains(unit_interval, beta) {
        return no_hit();
    }

    let is_outward = dot(ray.dir, normal) < 0.0;
    if !is_outward {
        normal = -normal;
    }

    return HitRecord(
        true,
        0,
        intersection,
        normal,
        is_outward,
        t,
        alpha,
        beta,
    );
}
