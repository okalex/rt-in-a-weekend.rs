use std::cell::RefCell;

use rand::RngExt;
use rand::rngs::SmallRng;

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(rand::make_rng());
}

fn with_rng<T>(f: impl FnOnce(&mut SmallRng) -> T) -> T {
    RNG.with(|rng| f(&mut *rng.borrow_mut()))
}

pub fn rand() -> f64 {
    with_rng(|rng| rng.random())
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand()
}

pub fn rand_int(min: i32, max: i32) -> i32 {
    with_rng(|rng| rng.random_range(min..max))
}

pub fn rand_usize(min: usize, max: usize) -> usize {
    with_rng(|rng| rng.random_range(min..max) as usize)
}
