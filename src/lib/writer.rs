use crate::lib::color;

pub trait Writer {
  fn init(&self);
  fn write_string(&self, str: String);
  fn write_color(&self, color: color::Color);
  fn close(&self);
}

pub struct PpmWriter {
  img_width: u32,
  img_height: u32,
  max_color_val: u32,
}

impl Writer for PpmWriter {

  fn init(&self) {
    println!("P3");
    println!("{} {}", self.img_width, self.img_height);
    println!("{}", self.max_color_val);
  }

  fn write_string(&self, str: String) {
    println!("{}", str);
  }

  fn write_color(&self, color: color::Color) {
    println!("{} ", color.to_string());
  }

  fn close(&self) {}

}

pub fn new_ppm_writer(img_width: u32, img_height: u32, max_color_val: u32) -> PpmWriter {
  return PpmWriter {
    img_width: img_width,
    img_height: img_height,
    max_color_val: max_color_val,
  };
}
