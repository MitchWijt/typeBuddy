use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(PartialEq)]
pub enum TimerState {
    STOP,
    RESET
}

pub struct Timer {}

impl Timer {
    pub fn start(tx_timer_duration: Sender<f32>, rx_timer_state: Receiver<TimerState>) {
        let mut counter: f32 = 0.0;

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                counter += 1.0;

                match rx_timer_state.try_recv() {
                    Ok(state) => {
                        if state == TimerState::STOP {
                            tx_timer_duration.send(counter).unwrap();
                        } else if state == TimerState::RESET {
                            counter = 0.0;
                        }
                    },
                    Err(_) => continue
                }
            }
        });
    }

    pub fn stop(tx_timer_state: Sender<TimerState>,) {
        tx_timer_state.send(TimerState::STOP).unwrap();
    }

    pub fn reset(tx_timer_state: Sender<TimerState>,) {
        tx_timer_state.send(TimerState::RESET).unwrap();
    }
}

