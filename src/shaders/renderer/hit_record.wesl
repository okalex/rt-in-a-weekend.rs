struct HitRecord {
    is_hit: bool,
    point: vec3<f32>,
    normal: vec3<f32>,
    is_outward: bool,
    t: f32,
    u: f32,
    v: f32,
    mat_idx: u32,
};

fn no_hit() -> HitRecord {
    return HitRecord(false, vec3(0, 0, 0), vec3(0, 0, 0), true, 0, 0, 0, 0);
}
