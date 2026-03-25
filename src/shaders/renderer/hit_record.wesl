import package::{
    constants::INF,
    transform::transform_point,
};

struct HitRecord {
    is_hit: bool,
    instance_id: u32,
    point: vec3f,
    normal: vec3f,
    is_outward: bool,
    t: f32,
    u: f32,
    v: f32,
};

fn no_hit() -> HitRecord {
    return HitRecord(false, 0, vec3(0, 0, 0), vec3(0, 0, 0), true, INF, 0, 0);
}

fn hit(
    point: vec3f,
    normal: vec3f,
    is_outward: bool,
    t: f32,
    u: f32,
    v: f32,
) -> HitRecord {
    return HitRecord(true, 0, point, normal, is_outward, t, u, v);
}

fn transform(
    hit_record: HitRecord,
    instance_id: u32,
    transform: mat4x4f,
    inv_transform: mat4x4f
) -> HitRecord {

    var new_record = hit_record;
    new_record.instance_id = instance_id;
    new_record.point = transform_point(hit_record.point, transform);
    new_record.normal = transform_normal(hit_record.normal, inv_transform);

    return new_record;
}

fn transform_normal(
    normal: vec3f,
    inv_transform: mat4x4f
) -> vec3f {
    return normalize((transpose(inv_transform) * vec4f(normal, 0.0)).xyz);
}
