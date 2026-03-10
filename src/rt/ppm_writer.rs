use std::sync::Arc;

use crate::rt::frame_buffer::FrameBuffer;

pub struct PpmWriter {
    max_color_val: u32,
    pub frame_buffer: Arc<FrameBuffer>,
}

impl PpmWriter {
    pub fn new(frame_buffer: Arc<FrameBuffer>, max_color_val: u32) -> PpmWriter {
        PpmWriter {
            max_color_val,
            frame_buffer,
        }
    }

    pub fn write(&self) {
        println!("P3");
        println!("{} {}", self.frame_buffer.width, self.frame_buffer.height);
        println!("{}", self.max_color_val);

        for j in 0..self.frame_buffer.height {
            for i in 0..self.frame_buffer.width {
                let color = self.frame_buffer.get_pixel(i as usize, j as usize);
                print!("{} {} {} ", color[0], color[1], color[2]);
            }
            println!("");
        }
    }
}
