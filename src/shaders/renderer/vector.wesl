const ZEROS = vec3f(0, 0, 0);
const ONES = vec3f(1, 1, 1);

fn length_squared(v: vec3f) -> f32 {
    return dot(v, v);
}

fn reflect(vec: vec3f, normal: vec3f) -> vec3f {
    return vec - normal * dot(vec, normal) * 2.0;
}

fn refract(vec: vec3f, n: vec3f, etai_over_etat: f32) -> vec3f {
    let cos_theta = min(dot(-vec, n), 1.0);
    let r_out_perp = (vec + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -sqrt(abs(1.0 - length_squared(r_out_perp)));
    return r_out_perp + r_out_parallel;
}
