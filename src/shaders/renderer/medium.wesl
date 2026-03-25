import package::{
    constants::INF,
    hit_record,
    interval,
    mesh,
    quad,
    random,
    ray,
    sphere,
    triangle,
    types,
    vector,
};

fn hit(
    medium: types::GpuPrimitive_Medium,
    boundary: types::GpuPrimitive,
    ray: ray::Ray,
    ray_t: interval::Interval
) -> hit_record::HitRecord {

    let hit_boundary_result = hit_boundary(boundary, ray);

    if !hit_boundary_result.entry.is_hit || !hit_boundary_result.exit.is_hit {
        return hit_record::no_hit();
    }

    var entry = max(hit_boundary_result.entry.t, ray_t.min);
    let exit = min(hit_boundary_result.exit.t, ray_t.max);

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
    let point = ray::at(ray, t);
    let normal = vec3f(1.0, 0.0, 0.0);

    return hit_record::hit(point, normal, true, t, 0.0, 0.0);
}

struct HitBoundaryResult {
    entry: hit_record::HitRecord,
    exit: hit_record::HitRecord,
};

fn hit_boundary(boundary: types::GpuPrimitive, ray: ray::Ray) -> HitBoundaryResult {
    var result = HitBoundaryResult(
        hit_record::no_hit(),
        hit_record::no_hit(),
    );

    // Can't recurse :/
    switch boundary._type {
        case types::GPUPRIMITIVE_SPHERE: {
            let sphere = types::unpack_GpuPrimitive_Sphere(boundary);
            result.entry = sphere::hit(sphere, ray, interval::Interval(-INF, INF));
            if result.entry.is_hit {
                result.exit = sphere::hit(sphere, ray, interval::Interval(result.entry.t + 0.001, INF));
            }
        }

        case types::GPUPRIMITIVE_TRIANGLE: {
            let triangle = types::unpack_GpuPrimitive_Triangle(boundary);
            result.entry = triangle::hit(triangle, ray, interval::Interval(-INF, INF));
            if result.entry.is_hit {
                result.exit = triangle::hit(triangle, ray, interval::Interval(result.entry.t + 0.001, INF));
            }
        }

        case types::GPUPRIMITIVE_QUAD: {
            let quad = types::unpack_GpuPrimitive_Quad(boundary);
            result.entry = quad::hit(quad, ray, interval::Interval(-INF, INF));
            if result.entry.is_hit {
                result.exit = quad::hit(quad, ray, interval::Interval(result.entry.t + 0.001, INF));
            }
        }

        case types::GPUPRIMITIVE_MESH: {
            let mesh = types::unpack_GpuPrimitive_Mesh(boundary);
            result.entry = mesh::hit(mesh, ray, interval::Interval(-INF, INF));
            if result.entry.is_hit {
                result.exit = mesh::hit(mesh, ray, interval::Interval(result.entry.t + 0.001, INF));
            }
        }

        default: {
            return result;
        }
    }

    return result;
}
