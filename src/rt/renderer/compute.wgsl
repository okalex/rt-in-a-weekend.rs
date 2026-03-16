struct dimensions {
    width: u32,
    height: u32,
};

struct position {
    x: u32,
    y: u32,
};

struct color {
    r: f32,
    g: f32,
    b: f32,
}

@group(0) @binding(0) var<storage, read> dims: dimensions;
@group(0) @binding(1) var<storage, read> input: array<position>;
@group(0) @binding(2) var<storage, read_write> output: array<color>;

@compute
@workgroup_size(16, 16, 1)
fn main(
    // global_invocation_id specifies our position in the invocation grid
    @builtin(global_invocation_id) global_id: vec3<u32>
) {
    let x = global_id.x;
    let y = global_id.y;

    if (x >= dims.width || y >= dims.height) {
        return;
    }

    let idx = dims.width * y + x;
    let pixel = input[idx];
    let color = color(f32(pixel.x) / f32(dims.width), f32(pixel.y) / f32(dims.height), 0.0);
    output[idx] = color;
}
