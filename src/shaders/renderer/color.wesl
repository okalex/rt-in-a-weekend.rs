fn to_rgba(color: vec3f) -> vec4f {
    let clamped = clamp(color, vec3(0.0), vec3(1.0));
    return vec4f(clamped, 1.0);
}

fn is_black(color: vec3f) -> bool {
    let threshold = 1.0 / 256.0;
    return color.x < threshold && color.y < threshold && color.z < threshold;
}
