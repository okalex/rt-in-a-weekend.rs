use std::ops::Add;

use crate::util::types::{Float, INFINITY};

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: Float,
    pub max: Float,
}

impl Interval {
    pub fn new(min: Float, max: Float) -> Interval {
        if min <= max {
            Interval { min, max }
        } else {
            Interval { min: max, max: min }
        }
    }

    #[allow(dead_code)]
    pub fn empty() -> Interval {
        Self::new(0.0, 0.0)
    }

    #[allow(dead_code)]
    pub fn universe() -> Interval {
        Self::new(-INFINITY, INFINITY)
    }

    #[allow(dead_code)]
    pub fn union(a: &Interval, b: &Interval) -> Interval {
        Self::new(
            if a.min <= b.min { a.min } else { b.min },
            if a.max >= b.max { a.max } else { b.max },
        )
    }

    #[allow(dead_code)]
    pub fn update_min(&self, new_min: Float) -> Interval {
        Self::new(new_min, self.max)
    }

    pub fn update_max(&self, new_max: Float) -> Interval {
        Self::new(self.min, new_max)
    }

    #[allow(dead_code)]
    pub fn size(&self) -> Float {
        self.max - self.min
    }

    pub fn contains(&self, x: Float) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: Float) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: Float) -> Float {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    #[allow(dead_code)]
    pub fn expand(&self, delta: Float) -> Interval {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }
}

impl Add<Float> for Interval {
    type Output = Self;

    fn add(self, b: Float) -> Self {
        Self::new(self.min + b, self.max + b)
    }
}
