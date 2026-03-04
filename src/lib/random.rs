use rand::RngExt;

pub fn rand() -> f64 {
  let mut rng = rand::rng();
  return rng.random();
}

pub fn rand_range(min: f64, max: f64) -> f64 {
  return min + (max - min) * rand();
}
