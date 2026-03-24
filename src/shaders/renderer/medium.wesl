import package::{
    hit_record::{HitRecord, no_hit, hit},
    interval::Interval,
    random::rand,
    ray::{Ray, ray_at},
    types,
    vector,
};

fn hit_medium(
    medium: types::GpuPrimitive_Medium,
    entry_rec: HitRecord,
    exit_rec: HitRecord,
    ray: Ray,
    ray_t: Interval
) -> HitRecord {

    if !entry_rec.is_hit || !exit_rec.is_hit {
        return no_hit();
    }

    var entry = max(entry_rec.t, ray_t.min);
    let exit = min(exit_rec.t, ray_t.max);

    if entry >= exit {
        return no_hit();
    }

    if entry < 0.0 {
        entry = 0.0;
    }

    let ray_len = length(ray.dir);
    let dist_inside = (exit - entry) * ray_len;
    let hit_dist = medium.neg_inv_density * log(rand());

    if hit_dist > dist_inside {
        return no_hit();
    }

    let t = entry + hit_dist / ray_len;
    let point = ray_at(ray, t);
    let normal = vec3<f32>(1.0, 0.0, 0.0);

    return hit(point, normal, true, t, 0.0, 0.0);
}
