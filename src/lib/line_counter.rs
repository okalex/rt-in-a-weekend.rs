pub struct LineCounter {
  count: u32,
}

impl LineCounter {
  pub fn new(initial_count: u32) -> Self {
    Self {
      count: initial_count,
    }
  }

  pub fn dec(&mut self) {
    self.count -= 1;
  }

  pub fn announce(&self) {
    eprint!("\rLines remaining: {}       ", self.count);
  }
}
