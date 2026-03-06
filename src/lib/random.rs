use rand::RngExt;

pub fn rand() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand()
}

pub fn rand_int(min: i32, max: i32) -> i32 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

pub fn rand_usize(min: usize, max: usize) -> usize {
    let mut rng = rand::rng();
    rng.random_range(min..max) as usize
}
