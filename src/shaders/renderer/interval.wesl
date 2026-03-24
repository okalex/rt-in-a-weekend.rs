struct Interval {
    min: f32,
    max: f32,
};

fn contains(interval: Interval, x: f32) -> bool {
    return interval.min <= x && x <= interval.max;
}
