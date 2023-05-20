use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct TimerState {
    pub duration: f32,
    max_minutes: Option<f32>,
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
            max_minutes: Some(*max_minutes as f32),
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
                state.lock().unwrap().duration += 1.0;

                if let Some(minutes) = max_minutes {
                    let max_seconds = minutes * 60.0;
                    let current_seconds = state.lock().unwrap().duration;

                    if current_seconds >= max_seconds {
                        state.lock().unwrap().reached_max_min = true;
                    }
                }

                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    pub fn reset(&self) {
        self.state.lock().unwrap().duration = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_starting_timer() {
        //given
        let timer = Timer::new();

        //when
        timer.start();

        //then
        thread::sleep(Duration::from_secs(3));
        let duration = timer.state.lock().unwrap().duration;

        assert_eq!(duration, 3.0);
    }

    #[test]
    fn test_starting_timer_max_minutes() {
        //given
        let timer_state = TimerState {
            max_minutes: Some(0.1),
            duration: 0.0,
            reached_max_min: false
        };

        let timer = Timer {
            state: Arc::new(Mutex::new(timer_state))
        };

        //when
        timer.start();

        //then
        thread::sleep(Duration::from_secs(6));
        let max_minutes_reached = timer.state.lock().unwrap().reached_max_min;

        assert_eq!(max_minutes_reached, true);
    }
}

