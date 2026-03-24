import package::{
    hit_record,
    interval,
    random,
    ray,
    types,
    vector,
};

fn hit_medium(
    medium: types::GpuPrimitive_Medium,
    entry_rec: hit_record::HitRecord,
    exit_rec: hit_record::HitRecord,
    ray: ray::Ray,
    ray_t: interval::Interval
) -> hit_record::HitRecord {

    if !entry_rec.is_hit || !exit_rec.is_hit {
        return hit_record::no_hit();
    }

    var entry = max(entry_rec.t, ray_t.min);
    let exit = min(exit_rec.t, ray_t.max);

    if entry >= exit {
        return hit_record::no_hit();
    }

    if entry < 0.0 {
        entry = 0.0;
    }

    let ray_len = length(ray.dir);
    let dist_inside = (exit - entry) * ray_len;
    let hit_dist = medium.neg_inv_density * log(random::rand());

    if hit_dist > dist_inside {
        return hit_record::no_hit();
    }

    let t = entry + hit_dist / ray_len;
    let point = ray::ray_at(ray, t);
    let normal = vec3f(1.0, 0.0, 0.0);

    return hit_record::hit(point, normal, true, t, 0.0, 0.0);
}
