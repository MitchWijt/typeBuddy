use std::ffi::c_int;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

pub struct Timer {}

impl Timer {
    pub fn start(tx_timer_duration: Sender<i32>, rx_continue_timer: Receiver<bool>) {
        let mut counter = 0;

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                counter += 1;

                match rx_continue_timer.try_recv() {
                    Ok((continue_timer)) => {
                        if !continue_timer {
                            tx_timer_duration.send(counter).unwrap();
                        }
                    },
                    Err(_) => continue,
                    _ => continue,
                }
            }
        });
    }

    pub fn stop(tx_continue_timer: Sender<bool>,) {
        tx_continue_timer.send(false).unwrap();
    }
}

