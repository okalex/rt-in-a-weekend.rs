fn transform_point(point: vec3<f32>, transform: mat4x4<f32>) -> vec3<f32> {
    let p4 = transform * vec4<f32>(point, 1.0);
    return p4.xyz / p4.w;
}

fn transform_direction(direction: vec3<f32>, transform: mat4x4<f32>) -> vec3<f32> {
    return (transform * vec4<f32>(direction, 0.0)).xyz;
}
