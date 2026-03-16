use std::cell::RefCell;

use rand::RngExt;
use rand::rngs::SmallRng;

use crate::rt::types::{Float, Int, PI, Vector};

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(rand::make_rng());
}

fn with_rng<T>(f: impl FnOnce(&mut SmallRng) -> T) -> T {
    RNG.with(|rng| f(&mut *rng.borrow_mut()))
}

pub fn rand() -> Float {
    with_rng(|rng| rng.random())
}

pub fn rand_range(min: Float, max: Float) -> Float {
    min + (max - min) * rand()
}

pub fn rand_int(min: Int, max: Int) -> Int {
    with_rng(|rng| rng.random_range(min..=max))
}

#[allow(dead_code)]
pub fn rand_usize(min: usize, max: usize) -> usize {
    with_rng(|rng| rng.random_range(min..max) as usize)
}

#[allow(dead_code)]
pub fn rand_vector() -> Vector {
    Vector::new(rand(), rand(), rand())
}

pub fn rand_range_vector(min: Float, max: Float) -> Vector {
    Vector::new(
        rand_range(min, max),
        rand_range(min, max),
        rand_range(min, max),
    )
}

pub fn rand_unit_vector() -> Vector {
    loop {
        let p = rand_range_vector(-1.0, 1.0);
        let lensq = p.magnitude_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

#[allow(dead_code)]
pub fn rand_on_hemisphere(normal: &Vector) -> Vector {
    let on_unit_sphere = rand_unit_vector();

    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn rand_in_unit_disk() -> Vector {
    loop {
        let p = Vector::new(rand_range(-1.0, 1.0), rand_range(-1.0, 1.0), 0.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}

pub fn rand_cos_dir() -> Vector {
    let r1 = rand();
    let r2 = rand();
    let sqrt_r2 = r2.sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * sqrt_r2;
    let y = phi.sin() * sqrt_r2;
    let z = (1.0 - r2).sqrt();

    Vector::new(x, y, z)
}
