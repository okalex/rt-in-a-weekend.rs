const ZEROS = vec3<f32>(0, 0, 0);
const ONES = vec3<f32>(1, 1, 1);

fn length_squared(v: vec3<f32>) -> f32 {
    return dot(v, v);
}

fn reflect(vec: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    return vec - normal * dot(vec, normal) * 2.0;
}

fn refract(vec: vec3<f32>, n: vec3<f32>, etai_over_etat: f32) -> vec3<f32> {
    let cos_theta = min(dot(-vec, n), 1.0);
    let r_out_perp = (vec + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -sqrt(abs(1.0 - length_squared(r_out_perp)));
    return r_out_perp + r_out_parallel;
}
