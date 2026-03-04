pub struct Interval {
  min: f64,
  max: f64,
}

impl Interval {
  pub fn min(&self) -> f64 {
    return self.min;
  }

  pub fn max(&self) -> f64 {
    return self.max;
  }

  pub fn update_min(&self, new_min: f64) -> Interval {
    return new(new_min, self.max);
  }

  pub fn update_max(&self, new_max: f64) -> Interval {
    return new(self.min, new_max);
  }

  pub fn size(&self) -> f64 {
    return self.max - self.min;
  }

  pub fn contains(&self, x: f64) -> bool {
    return self.min <= x && x <= self.max;
  }

  pub fn surrounds(&self, x: f64) -> bool {
    return self.min < x && x < self.max;
  }

  pub fn clamp(&self, x: f64) -> f64 {
    if x < self.min { return self.min; }
    if x > self.max { return self.max; }
    return x;
  }
}

pub fn new(min: f64, max: f64) -> Interval {
  return Interval {
    min: min,
    max: max,
  };
}
