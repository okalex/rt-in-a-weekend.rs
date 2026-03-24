fn transform_point(point: vec3f, transform: mat4x4f) -> vec3f {
    let p4 = transform * vec4f(point, 1.0);
    return p4.xyz / p4.w;
}

fn transform_direction(direction: vec3f, transform: mat4x4f) -> vec3f {
    return (transform * vec4f(direction, 0.0)).xyz;
}
