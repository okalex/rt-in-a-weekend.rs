struct dimensions {
    width: u32,
    height: u32,
};

struct color {
    r: f32,
    g: f32,
    b: f32,
}

@group(0) @binding(0) var<storage, read> dims: dimensions;
@group(0) @binding(2) var<storage, read_write> output: array<color>;

@compute
@workgroup_size(16, 16, 1)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>
) {
    let x = global_id.x;
    let y = global_id.y;

    if (x >= dims.width || y >= dims.height) {
        return;
    }

    let idx = dims.width * y + x;
    let color = color(f32(x) / f32(dims.width), f32(y) / f32(dims.height), 0.0);
    output[idx] = color;
}
