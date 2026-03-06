#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min: min, max: max }
    }

    pub fn empty() -> Interval {
        Self::new(0.0, 0.0)
    }

    pub fn union(a: &Interval, b: &Interval) -> Interval {
        Self::new(
            if a.min <= b.min { a.min } else { b.min },
            if a.max >= b.max { a.max } else { b.max },
        )
    }

    pub fn update_min(&self, new_min: f64) -> Interval {
        Self::new(new_min, self.max)
    }

    pub fn update_max(&self, new_max: f64) -> Interval {
        Self::new(self.min, new_max)
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }
}
