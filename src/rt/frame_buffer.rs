use std::sync::{Arc, Mutex};

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub data: Arc<Mutex<Vec<u8>>>,
}

const PIXEL_SIZE: usize = 4;

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![0u8; width * height * PIXEL_SIZE];
        Self {
            width,
            height,
            data: Arc::new(Mutex::new(data)),
        }
    }

    #[allow(dead_code)]
    pub fn set_pixel(&self, x: usize, y: usize, color: [u8; 3]) {
        let mut buffer = self.data.lock().unwrap();
        let idx = self.buffer_idx(x, y);
        let rgba = Self::to_rgba(&color);
        buffer[idx..(idx + PIXEL_SIZE)].copy_from_slice(&rgba);
    }

    pub fn set_line(&self, y: usize, line: &Vec<[u8; 3]>) {
        assert_eq!(line.len(), self.width as usize);
        let mut buffer = self.data.lock().unwrap();
        let start = (y * self.width * PIXEL_SIZE) as usize;
        for (i, color) in line.iter().enumerate() {
            let rgba = Self::to_rgba(color);
            let pixel_idx = start + i * PIXEL_SIZE;
            buffer[pixel_idx..(pixel_idx + PIXEL_SIZE)].copy_from_slice(&rgba);
        }
    }

    pub fn set_frame(&self, frame: &Vec<[u8; 3]>) {
        let mut buffer = self.data.lock().unwrap();
        eprintln!("Setting frame with {} pixels", frame.len());
        for (idx, color) in frame.iter().enumerate() {
            let rgba = Self::to_rgba(color);
            let pixel_idx = idx * PIXEL_SIZE;
            buffer[pixel_idx..(pixel_idx + PIXEL_SIZE)].copy_from_slice(&rgba);
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> [u8; 3] {
        let buffer = self.data.lock().unwrap();
        let idx = self.buffer_idx(x, y);
        buffer[idx..idx + 3].try_into().expect("slice too short")
    }

    fn buffer_idx(&self, x: usize, y: usize) -> usize {
        (y * self.width + x) * PIXEL_SIZE
    }

    fn to_rgba(color: &[u8; 3]) -> [u8; PIXEL_SIZE] {
        [color[0], color[1], color[2], 255u8]
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
