use std::sync::{Arc, Mutex};

use crate::util::types::Uint;

pub struct LineServer {
    lines: Arc<Mutex<Vec<Uint>>>,
}

impl LineServer {
    pub fn new(num_lines: Uint) -> Self {
        let lines: Arc<Mutex<Vec<Uint>>> = Arc::new(Mutex::new((0..num_lines).rev().collect()));
        Self { lines }
    }

    pub fn next_line(&self) -> Option<Uint> {
        self.lines.lock().unwrap().pop()
    }

    pub fn len(&self) -> Uint {
        self.lines.lock().unwrap().len() as Uint
    }
}
