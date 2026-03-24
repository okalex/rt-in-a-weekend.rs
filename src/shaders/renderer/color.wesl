fn to_rgba(color: vec3<f32>) -> vec4<f32> {
    let clamped = clamp(color, vec3(0.0), vec3(1.0));
    return vec4<f32>(clamped, 1.0);
}
