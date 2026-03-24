import package::{
    hit_record,
    interval,
    ray,
    types,
};

fn hit(triangle: types::GpuPrimitive_Triangle, ray: ray::Ray, ray_t: interval::Interval) -> hit_record::HitRecord {
    let h = cross(ray.dir, triangle.e02);
    let a = dot(triangle.e01, h);

    if abs(a) < 1e-8 {
        // Parallel to plane
        return hit_record::no_hit();
    }

    let f = 1.0 / a;
    let s = ray.orig - triangle.v0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return hit_record::no_hit();
    }

    let q = cross(s, triangle.e01);
    let v = f * dot(ray.dir, q);

    if v < 0.0 || (u + v) > 1.0 {
        return hit_record::no_hit();
    }

    let t = f * dot(triangle.e02, q);

    if !interval::contains(ray_t, t) {
        return hit_record::no_hit();
    }

    let w = 1.0 - u - v;
    let uv = triangle.uv0 * w + triangle.uv1 * u + triangle.uv2 * v;

    let point = ray::at(ray, t);
    let is_outward = a >= 0.0;

    var normal = u * triangle.normal0 + v * triangle.normal1 + w * triangle.normal2;
    if !is_outward {
        normal = -normal;
    }

    return hit_record::hit(
        point,
        normal,
        is_outward,
        t,
        uv[0],
        uv[1],
    );
}
