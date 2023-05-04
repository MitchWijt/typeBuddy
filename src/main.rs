mod symbols;
mod cli;
mod game;
mod terminal;
mod game_state;
mod game_text;
mod timer;

use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::cli::Cli;

fn main() {
    match Cli::start() {
        Ok(_) => ..,
        Err(_) => {
            eprintln!("Unknown command typeBuddy");
            exit(1);
        }
    };
}


