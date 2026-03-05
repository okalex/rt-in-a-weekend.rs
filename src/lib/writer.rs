use crate::lib::frame_buffer::FrameBuffer;

pub trait Writer: Send + Sync {
  fn init(&self);
  fn write_pixel(&self, x: usize, y: usize, color: [u8; 3]);
  fn write_line(&self, y: usize, line: &Vec<[u8; 3]>);
  fn close(&self);
}

pub struct PpmWriter {
  img_width: u32,
  img_height: u32,
  max_color_val: u32,
  pub buffer: FrameBuffer,
}

impl PpmWriter {
  pub fn new(img_width: u32, img_height: u32, max_color_val: u32) -> PpmWriter {
    PpmWriter {
      img_width: img_width,
      img_height: img_height,
      max_color_val: max_color_val,
      buffer: FrameBuffer::new(img_width as usize, img_height as usize),
    }
  }
}

impl Writer for PpmWriter {

  fn init(&self) {
  }

  fn write_pixel(&self, x: usize, y: usize, color: [u8; 3]) {
    self.buffer.set_pixel(x, y, color);
  }

  fn write_line(&self, y: usize, line: &Vec<[u8; 3]>) {
    self.buffer.set_line(y, line);
  }

  fn close(&self) {
    println!("P3");
    println!("{} {}", self.img_width, self.img_height);
    println!("{}", self.max_color_val);

    for j in 0..self.img_height {
      for i in 0..self.img_width {
        let color = self.buffer.get_pixel(i as usize, j as usize);
        print!("{} {} {} ", color[0], color[1], color[2]);
      }
      println!("");
    }
  }

}
