import package::{
    constants::{PI, INF},
    hit_record,
    interval,
    onb,
    random,
    ray,
    types,
    vector,
};

fn hit_sphere(sphere: types::GpuPrimitive_Sphere, ray: ray::Ray, ray_t: interval::Interval) -> hit_record::HitRecord {
    let oc = sphere.center - ray.orig;
    let a = vector::length_squared(ray.dir);
    let h = dot(ray.dir, oc);
    let c = vector::length_squared(oc) - sphere.radius_sqrd;

    let discriminant = h * h - a * c;
    if discriminant < 0 {
        return hit_record::no_hit();
    }

    let sqrtd = sqrt(discriminant);

    var root = (h - sqrtd) / a;
    if root <= ray_t.min || ray_t.max <= root {
        root = (h + sqrtd) / a;
        if root <= ray_t.min || ray_t.max <= root {
            return hit_record::no_hit();
        }
    }

    let hit_point = ray::ray_at(ray, root);
    var normal = (hit_point - sphere.center) / sphere.radius;
    let is_outward = dot(ray.dir, normal) < 0.0;
    if !is_outward {
        normal = -normal;
    }
    let uv = get_uv(sphere, normal);

    return hit_record::hit(
        hit_point,
        normal,
        is_outward,
        root,
        uv[0],
        uv[1],
    );
}

fn get_uv(sphere: types::GpuPrimitive_Sphere, normal: vec3f) -> array<f32, 2> {
    let u = 0.5 + atan2(-normal.z, normal.x) / (2.0 * PI);
    let v = 0.5 + asin(normal.y) / PI;
    return array(u, v);
}

fn sphere_pdf_value(sphere: types::GpuPrimitive_Sphere, origin: vec3f, direction: vec3f) -> f32 {
    // Slightly optimized hit test
    let to_center = sphere.center - origin;
    let dist_sqrd = vector::length_squared(to_center);
    if dist_sqrd <= sphere.radius_sqrd { return 0.0; } // inside sphere

    let cos_theta_max = sqrt(1.0 - sphere.radius_sqrd / dist_sqrd);
    let cos_angle = dot(normalize(direction), normalize(to_center));

    if cos_angle < cos_theta_max { return 0.0; } // misses the sphere

    let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
    return 1.0 / solid_angle;
}

fn sphere_random(sphere: types::GpuPrimitive_Sphere, origin: vec3f) -> vec3f {
    let direction = sphere.center - origin;
    let dist_sqrd = vector::length_squared(direction);
    let uvw = onb::new_onb(direction);
    return onb::transform_onb(uvw, rand_to_sphere(sphere.radius, dist_sqrd));
}

fn rand_to_sphere(radius: f32, dist_sqrd: f32) -> vec3f {
    let r1 = random::rand();
    let r2 = random::rand();
    let z = 1.0 + r2 * (sqrt(1.0 - radius * radius / dist_sqrd) - 1.0);

    let phi = 2.0 * PI * r1;
    let tmp = sqrt(1.0 - z * z);
    let x = cos(phi) * tmp;
    let y = sin(phi) * tmp;

    return vec3f(x, y, z);
}
