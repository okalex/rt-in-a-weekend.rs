import package::{
    bindings::gpu_meta,
    ray::{Ray},
};

fn get_ray(x: u32, y: u32) -> Ray {
    let pixel_sample = pixel_loc(x, y);
    let origin = gpu_meta.camera.position;
    let direction = pixel_sample - origin;
    return Ray(origin, direction);
}

fn pixel_loc(x: u32, y: u32) -> vec3<f32> {
    return gpu_meta.viewport.pixel00_loc + 
        gpu_meta.viewport.delta_u * f32(x) + 
        gpu_meta.viewport.delta_v * f32(y);
}
