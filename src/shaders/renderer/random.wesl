import package::{
    constants::PI,
    vector::{length_squared, ZEROS},
};

var<private> rng_state: u32;

fn pcg(n: u32) -> u32 {
    var h = n * 747796405u + 2891336453u;
    h = ((h >> ((h >> 28u) + 4u)) ^ h) * 277803737u;
    return (h >> 22u) ^ h;
}

fn init_rng(x: u32, y: u32, frame: u32) {
    rng_state = pcg(x + y * 1920u + frame * 1920u * 1080u);
}

fn rand() -> f32 {
    rng_state = pcg(rng_state);
    return f32(rng_state) / f32(0xffffffffu);
}

fn rand_range(min: f32, max: f32) -> f32 {
    return min + (max - min) * rand();
}

fn rand_vector() -> vec3<f32> {
    return vec3<f32>(rand(), rand(), rand());
}

fn rand_range_vector(min: f32, max: f32) -> vec3<f32> {
    return vec3<f32>(
        rand_range(min, max),
        rand_range(min, max),
        rand_range(min, max),
    );
}

fn rand_unit_vector() -> vec3<f32> {
    loop {
        let p = rand_range_vector(-1, 1);
        let lensq = length_squared(p);
        if lensq > 1e-10 && lensq <= 1.0 {
            return p / sqrt(lensq);
        }
    }
    return vec3<f32>(0, 1, 0); // unreachable, satisfies naga validator
}

fn rand_vector_on_hemisphere(normal: vec3<f32>) -> vec3<f32> {
    let on_unit_sphere = rand_unit_vector();
    if dot(on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

fn rand_in_unit_disk() -> vec3<f32> {
    loop {
        let p = vec3<f32>(rand_range(-1.0, 1.0), rand_range(-1.0, 1.0), 0.0);
        if length_squared(p) < 1.0 {
            return p;
        }
    }
    return ZEROS;
}

fn rand_cos_vector() -> vec3<f32> {
    let r1 = rand();
    let r2 = rand();
    let sqrt_r2 = sqrt(r2);

    let phi = 2.0 * PI * r1;
    let x = cos(phi) * sqrt_r2;
    let y = sin(phi) * sqrt_r2;
    let z = sqrt(1.0 - r2);

    return vec3<f32>(x, y, z);
}
