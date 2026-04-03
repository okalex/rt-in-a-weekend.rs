fn is_finite(float: f32) -> bool {
    return (bitcast<u32>(float) & 0x7f800000u) != 0x7f800000u;
}
