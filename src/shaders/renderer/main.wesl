import package::{
    bindings::{output, gpu_meta},
    camera::get_ray,
    color::{to_rgba},
    random::{init_rng},
    renderer::get_ray_color,
};

@compute
@workgroup_size(64, 1, 1)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
) {
    let x = global_id.x;
    let y = global_id.y;

    if x >= gpu_meta.width || y >= gpu_meta.height {
        return;
    }

    init_rng(x, y, gpu_meta.frame_num);

    var color = vec3f(0, 0, 0);
    for (var i = 0u; i < gpu_meta.num_samples; i = i + 1) {
        let camera_ray = get_ray(x, y);
        // let sample_color = clamp(get_ray_color(camera_ray), vec3(0.0), vec3(f32(gpu_meta.num_samples)));
        let sample_color = clamp(get_ray_color(camera_ray), vec3(0.0), vec3(4.0));
        // let sample_color = get_ray_color(camera_ray);
        color = color + sample_color / f32(gpu_meta.num_samples);
    }

    // Accumulate result
    let idx = gpu_meta.width * y + x;
    let frame_num = f32(gpu_meta.frame_num);
    let out = (output[idx].xyz * (frame_num - 1.0) + color) / frame_num;
    output[idx] = to_rgba(out);
}

