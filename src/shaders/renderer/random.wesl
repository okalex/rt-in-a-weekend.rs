import package::{
    bindings::gpu_meta,
    constants::PI,
    vector,
};

var<private> rng_state: u32;

fn pcg(n: u32) -> u32 {
    var h = n * 747796405u + 2891336453u;
    h = ((h >> ((h >> 28u) + 4u)) ^ h) * 277803737u;
    return (h >> 22u) ^ h;
}

fn init_rng(x: u32, y: u32, frame: u32) {
    rng_state = pcg(pcg(x) + pcg(y) + pcg(frame));
}

fn rand() -> f32 {
    rng_state = pcg(rng_state);
    return f32(rng_state) / f32(0xffffffffu);
}

fn rand_range(min: f32, max: f32) -> f32 {
    return min + (max - min) * rand();
}

fn rand_range_int(min: i32, max: i32) -> i32 {
    let range = u32(max - min + 1);
    return min + i32(u32(rand() * f32(range)));
}

fn rand_vector() -> vec3f {
    return vec3f(rand(), rand(), rand());
}

fn rand_range_vector(min: f32, max: f32) -> vec3f {
    return vec3f(
        rand_range(min, max),
        rand_range(min, max),
        rand_range(min, max),
    );
}

fn rand_unit_vector() -> vec3f {
    loop {
        let p = rand_range_vector(-1, 1);
        let lensq = vector::length_squared(p);
        if lensq > 1e-10 && lensq <= 1.0 {
            return p / sqrt(lensq);
        }
    }
    return vec3f(0, 1, 0); // unreachable, satisfies naga validator
}

fn rand_vector_on_hemisphere(normal: vec3f) -> vec3f {
    let on_unit_sphere = rand_unit_vector();
    if dot(on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

fn rand_in_unit_disk() -> vec3f {
    loop {
        let p = vec3f(rand_range(-1.0, 1.0), rand_range(-1.0, 1.0), 0.0);
        if vector::length_squared(p) < 1.0 {
            return p;
        }
    }
    return vector::ZEROS;
}

fn rand_on_unit_disk() -> vec3f {
    let r = sqrt(rand());
    let phi = 2.0 * PI * rand();
    return vec3f(r * cos(phi), r * sin(phi), 0.0);
}

fn rand_cos_vector() -> vec3f {
    let r1 = rand();
    let r2 = rand();
    let sqrt_r2 = sqrt(r2);

    let phi = 2.0 * PI * r1;
    let x = cos(phi) * sqrt_r2;
    let y = sin(phi) * sqrt_r2;
    let z = sqrt(1.0 - r2);

    return vec3f(x, y, z);
}
