use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::game::Game;

pub struct Timer {
    pub duration: Arc<Mutex<f32>>,
    max_minutes: Option<u32>
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            duration: Arc::new(Mutex::new(0.0)),
            max_minutes: None
        }
    }

    pub fn from_max_minutes(max_minutes: &u32) -> Self {
        Timer {
            duration: Arc::new(Mutex::new(0.0)),
            max_minutes: Some(max_minutes.clone())
        }
    }

    pub fn start(&self) {
        let duration = self.duration.clone();
        let max_minutes = self.max_minutes;

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                *duration.lock().unwrap() += 1.0;

                if let Some(minutes) = max_minutes {
                    let max_seconds = (minutes * 60) as f32;
                    let current_seconds = *duration.lock().unwrap();

                    if current_seconds >= max_seconds {
                        println!("END!");
                        return;
                    }
                }
            }
        });
    }

    pub fn reset(&self) {
        *self.duration.lock().unwrap() = 0.0;
    }
}

