use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::game::Game;

pub struct TimerState {
    pub duration: f32,
    max_minutes: Option<u32>,
    pub reached_max_min: bool
}

pub struct Timer {
    pub state: Arc<Mutex<TimerState>>,
}

impl Timer {
    pub fn new() -> Self {
        let state = TimerState {
            duration: 0.0,
            max_minutes: None,
            reached_max_min: false
        };

        Timer {
            state: Arc::new(Mutex::new(state))
        }
    }

    pub fn from_max_minutes(max_minutes: &u32) -> Self {
        let state = TimerState {
            duration: 0.0,
            max_minutes: Some(*max_minutes),
            reached_max_min: false
        };

        Timer {
            state: Arc::new(Mutex::new(state))
        }
    }

    pub fn start(&self) {
        let state = self.state.clone();

        thread::spawn(move || {
            let max_minutes = state.lock().unwrap().max_minutes;

            loop {
                thread::sleep(Duration::from_secs(1));
                state.lock().unwrap().duration += 1.0;

                if let Some(minutes) = max_minutes {
                    let max_seconds = (minutes * 60) as f32;
                    let current_seconds = state.lock().unwrap().duration;

                    if current_seconds >= max_seconds {
                        state.lock().unwrap().reached_max_min = true;
                    }
                }
            }
        });
    }

    pub fn reset(&self) {
        self.state.lock().unwrap().duration = 0.0;
    }
}

