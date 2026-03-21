struct Interval {
    min: f32,
    max: f32,
};

fn interval_contains(interval: Interval, x: f32) -> bool {
    return interval.min <= x && x <= interval.max;
}
