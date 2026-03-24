fn to_rgba(color: vec3f) -> vec4f {
    let clamped = clamp(color, vec3(0.0), vec3(1.0));
    return vec4f(clamped, 1.0);
}
