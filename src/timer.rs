use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Timer {
    pub duration: Arc<Mutex<f32>>
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            duration: Arc::new(Mutex::new(0.0))
        }
    }

    pub fn start(&self) {
        let duration = self.duration.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                *duration.lock().unwrap() += 1.0;
            }
        });
    }

    pub fn reset(&self) {
        *self.duration.lock().unwrap() = 0.0;
    }
}

