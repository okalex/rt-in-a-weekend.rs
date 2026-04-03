import package::{
    constants,
    float,
};

const ZEROS = vec3f(0, 0, 0);
const ONES = vec3f(1, 1, 1);

struct OptionVec3f {
    is_some: bool,
    vector: vec3f,
};

fn none() -> OptionVec3f {
    return OptionVec3f(false, vec3f(0.0, 0.0, 0.0));
}

fn some(vector: vec3f) -> OptionVec3f {
    return OptionVec3f(true, vector);
}

fn length_squared(v: vec3f) -> f32 {
    return dot(v, v);
}

fn refract(vec: vec3f, n: vec3f, etai_over_etat: f32) -> vec3f {
    let cos_theta = min(dot(-vec, n), 1.0);
    let r_out_perp = (vec + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -sqrt(abs(1.0 - length_squared(r_out_perp)));
    return r_out_perp + r_out_parallel;
}

fn normalize_if_valid(vector: vec3f) -> OptionVec3f {
    let length_sqrd = length_squared(vector);
    if !float::is_finite(length_sqrd) || length_sqrd <= constants::HALF_VECTOR_EPSILON {
        return none();
    }

    return some(vector / sqrt(length_sqrd));
}

fn half_vector(a: vec3f, b: vec3f) -> OptionVec3f {
    return normalize_if_valid(a + b);
}
