extern crate core;

mod symbols;
mod cli;
mod game;
mod terminal;
mod game_state;
mod game_text;
mod timer;
mod statistics;
mod plotter;

use std::process::exit;
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


