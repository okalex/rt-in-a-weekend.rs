struct Ray {
    orig: vec3<f32>,
    dir: vec3<f32>,
};

fn ray_at(ray: Ray, t: f32) -> vec3<f32> {
    return ray.orig + ray.dir * t;
}
