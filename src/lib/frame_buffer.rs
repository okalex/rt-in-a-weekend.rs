use std::sync::{Arc, Mutex};

type FrameVec = Vec<Vec<[u8; 3]>>;

pub struct FrameBuffer {
  pub width: usize,
  pub height: usize,
  pub data: Arc<Mutex<FrameVec>>,
}

impl FrameBuffer {
  pub fn new(width: usize, height: usize) -> Self {
    let data = vec![vec![[0u8; 3]; width]; height];
    Self {
      width,
      height,
      data: Arc::new(Mutex::new(data)),
    }
  }

  pub fn set_pixel(&self, x: usize, y: usize, color: [u8; 3]) {
    let mut buffer = self.data.lock().unwrap();
    buffer[y][x] = color;
  }

  pub fn set_line(&self, y: usize, line: &Vec<[u8; 3]>) {
    let mut buffer = self.data.lock().unwrap();
    buffer[y] = line.to_vec();
  }

  pub fn get_pixel(&self, x: usize, y: usize) -> [u8; 3] {
    let buffer = self.data.lock().unwrap();
    buffer[y][x]
  }
}

impl Clone for FrameBuffer {
  fn clone(&self) -> Self {
    Self {
      width: self.width,
      height: self.height,
      data: Arc::clone(&self.data),
    }
  }
}
