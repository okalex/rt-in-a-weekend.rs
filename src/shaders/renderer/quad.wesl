import package::{
    constants::INF,
    hit_record::{HitRecord, no_hit},
    interval::{Interval, interval_contains},
    random::rand,
    ray::{Ray, ray_at},
    types,
    vector::length_squared,
};

fn hit_quad(quad: types::GpuPrimitive_Quad, ray: Ray, ray_t: Interval) -> HitRecord {
    let denom = dot(quad.normal, ray.dir);
    if abs(denom) < 1e-8 {
        return no_hit();
    }

    let t = (quad.d - dot(quad.normal, ray.orig)) / denom;
    if !interval_contains(ray_t, t) {
        return no_hit();
    }

    let intersection = ray_at(ray, t);
    let planar_hit_vec = intersection - quad.q;
    let alpha = dot(quad.w, cross(planar_hit_vec, quad.v));
    let beta = dot(quad.w, cross(quad.u, planar_hit_vec));

    let unit_interval = Interval(0.0, 1.0);
    if !interval_contains(unit_interval, alpha) || !interval_contains(unit_interval, beta) {
        return no_hit();
    }

    var normal = quad.normal;
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

fn quad_pdf_value(quad: types::GpuPrimitive_Quad, origin: vec3<f32>, direction: vec3<f32>) -> f32 {
    // Slightly optimized hit test
    let denom = dot(quad.normal, direction);
    if abs(denom) < 1e-8 { return 0.0; }

    let t = (quad.d - dot(quad.normal, origin)) / denom;
    if t < 0.001 { return 0.0; }

    let intersection = origin + direction * t;
    let hit_vec = intersection - quad.q;
    let alpha = dot(quad.w, cross(hit_vec, quad.v));
    let beta = dot(quad.w, cross(quad.u, hit_vec));
    if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 { return 0.0; }

    let dist_sqrd = t * t * length_squared(direction);
    let cos = abs(denom) / length(direction); // reuses denom!
    return dist_sqrd / (cos * quad.area);
}

fn quad_random(quad: types::GpuPrimitive_Quad, origin: vec3<f32>) -> vec3<f32> {
    let p = quad.q + rand() * quad.u + rand() * quad.v;
    return p - origin;
}
