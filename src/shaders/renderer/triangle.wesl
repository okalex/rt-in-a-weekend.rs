import package::{
    hit_record::{HitRecord, no_hit},
    interval::{Interval, interval_contains},
    ray::{Ray, ray_at},
    types,
};

fn hit_triangle(triangle: types::GpuPrimitive_Triangle, ray: Ray, ray_t: Interval) -> HitRecord {
    let h = cross(ray.dir, triangle.e02);
    let a = dot(triangle.e01, h);

    if abs(a) < 1e-8 {
        // Parallel to plane
        return no_hit();
    }

    let f = 1.0 / a;
    let s = ray.orig - triangle.v0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return no_hit();
    }

    let q = cross(s, triangle.e01);
    let v = f * dot(ray.dir, q);

    if v < 0.0 || (u + v) > 1.0 {
        return no_hit();
    }

    let t = f * dot(triangle.e02, q);

    if !interval_contains(ray_t, t) {
        return no_hit();
    }

    let point = ray_at(ray, t);
    let is_outward = a >= 0.0;
    var normal = triangle.normal;
    if !is_outward {
        normal = -normal;
    }

    let w = 1.0 - u - v;
    let uv = triangle.uv0 * w + triangle.uv1 * u + triangle.uv2 * v;

    return HitRecord(
        true,
        0,
        point,
        normal,
        is_outward,
        t,
        uv[0],
        uv[1],
    );
}
