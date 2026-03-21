import package::{
    constants::PI,
    hit_record::{HitRecord, no_hit},
    interval::Interval,
    ray::{Ray, ray_at},
    types,
    vector::length_squared,
};

fn hit_sphere(sphere: types::GpuPrimitive_Sphere, ray: Ray, ray_t: Interval) -> HitRecord {
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
    if root <= ray_t.min || ray_t.max <= root {
        root = (h + sqrtd) / a;
        if root <= ray_t.min || ray_t.max <= root {
            return no_hit();
        }
    }

    let hit_point = ray_at(ray, root);
    var normal = (hit_point - sphere.center) / sphere.radius;
    let is_outward = dot(ray.dir, normal) < 0.0;
    if !is_outward {
        normal = -normal;
    }
    let uv = get_uv(sphere, normal);

    return HitRecord(
        true,
        0,
        hit_point,
        normal,
        is_outward,
        root,
        uv[0],
        uv[1],
    );
}

fn get_uv(sphere: types::GpuPrimitive_Sphere, normal: vec3<f32>) -> array<f32, 2> {
    let u = 0.5 + atan2(-normal.z, normal.x) / (2.0 * PI);
    let v = 0.5 + asin(normal.y) / PI;
    return array(u, v);
}
