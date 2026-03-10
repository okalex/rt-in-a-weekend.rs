use std::cell::RefCell;

use nalgebra::Vector3;
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

pub fn rand_vector() -> Vector3<f64> {
    Vector3::new(rand(), rand(), rand())
}

pub fn rand_range_vector(min: f64, max: f64) -> Vector3<f64> {
    Vector3::new(
        rand_range(min, max),
        rand_range(min, max),
        rand_range(min, max),
    )
}

pub fn rand_unit_vector() -> Vector3<f64> {
    loop {
        let p = rand_range_vector(-1.0, 1.0);
        let lensq = p.magnitude_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn rand_on_hemisphere(normal: &Vector3<f64>) -> Vector3<f64> {
    let on_unit_sphere = rand_unit_vector();
    
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn rand_in_unit_disk() -> Vector3<f64> {
    loop {
        let p = Vector3::new(
            rand_range(-1.0, 1.0),
            rand_range(-1.0, 1.0),
            0.0,
        );
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}
