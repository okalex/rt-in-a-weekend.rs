struct Ray {
    orig: vec3f,
    dir: vec3f,
};

fn at(ray: Ray, t: f32) -> vec3f {
    return ray.orig + ray.dir * t;
}
