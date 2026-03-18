fn length_squared(v: vec3<f32>) -> f32 {
    return dot(v, v);
}

fn length(v: vec3<f32>) -> f32 {
    return sqrt(length_squared(v));
}

fn reflect(vec: vec3<f32>, normal: vec3<f32>) -> vec3<f32> {
    return vec - normal * dot(vec, normal) * 2.0;
}
