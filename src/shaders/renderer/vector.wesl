fn length_squared(v: vec3<f32>) -> f32 {
    return dot(v, v);
}

fn length(v: vec3<f32>) -> f32 {
    return sqrt(length_squared(v));
}
