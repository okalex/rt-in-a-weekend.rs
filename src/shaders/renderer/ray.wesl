import package::{
    transform::{transform_direction, transform_point},
};

struct Ray {
    orig: vec3f,
    dir: vec3f,
};

fn at(ray: Ray, t: f32) -> vec3f {
    return ray.orig + ray.dir * t;
}

fn transform(
    ray: Ray,
    transform: mat4x4f
) -> Ray {
    return Ray(
        transform_point(ray.orig, transform),
        transform_direction(ray.dir, transform),
    );
}
