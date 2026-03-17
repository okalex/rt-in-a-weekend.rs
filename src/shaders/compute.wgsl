#import types;

struct rgba {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
};

@group(0) @binding(0) var<storage, read_write> output: array<rgba>;
@group(0) @binding(1) var<storage, read> gpu_meta: types::GpuMeta;

@compute
@workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>
) {
    let x = global_id.x;
    let y = global_id.y;

    if (x >= gpu_meta.width || y >= gpu_meta.height) {
        return;
    }

    let idx = gpu_meta.width * y + x;
    let r = f32(x) / f32(gpu_meta.width);
    let g = f32(y) / f32(gpu_meta.height);
    let b = sqrt(r * r + g * g);
    
    output[idx] = rgba(r, g, b, 1.0);
}
