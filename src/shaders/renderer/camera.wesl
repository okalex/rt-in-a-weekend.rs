import package::{
    bindings::gpu_meta,
    random,
    ray,
};

fn get_ray(x: u32, y: u32) -> ray::Ray {
    let pixel_sample = pixel_loc(x, y);
    var origin = gpu_meta.camera.position;
    if gpu_meta.camera.defocus_angle > 0.0 {
        origin = defocus_disk_sample();
    }
    let direction = pixel_sample - origin;
    return ray::Ray(origin, direction);
}

fn pixel_loc(x: u32, y: u32) -> vec3f {
    let jitter_x = random::rand() - 0.5;
    let jitter_y = random::rand() - 0.5;
    return gpu_meta.viewport.pixel00_loc + 
        gpu_meta.viewport.delta_u * (f32(x) + jitter_x) + 
        gpu_meta.viewport.delta_v * (f32(y) + jitter_y);
}

fn defocus_disk_sample() -> vec3f {
    let camera_pos = gpu_meta.camera.position;
    let defocus_u = gpu_meta.camera.defocus_disk_u;
    let defocus_v = gpu_meta.camera.defocus_disk_v;
    let p = random::rand_in_unit_disk();
    return camera_pos + defocus_u * p.x + defocus_v * p.y;
}
