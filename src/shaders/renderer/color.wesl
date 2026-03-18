struct Rgba {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
};

fn to_rgba(color: vec3<f32>) -> Rgba {
    return Rgba(color[0], color[1], color[2], 1.0);
}
