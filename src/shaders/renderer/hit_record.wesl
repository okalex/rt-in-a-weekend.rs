import package::{
    constants::INF,
};

struct HitRecord {
    is_hit: bool,
    instance_id: u32,
    point: vec3<f32>,
    normal: vec3<f32>,
    is_outward: bool,
    t: f32,
    u: f32,
    v: f32,
};

fn no_hit() -> HitRecord {
    return HitRecord(false, 0, vec3(0, 0, 0), vec3(0, 0, 0), true, INF, 0, 0);
}
