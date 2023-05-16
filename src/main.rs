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

use std::borrow::{Borrow, BorrowMut};
use std::cell::{Cell, Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::process::exit;
use std::rc::Rc;
use rand::distributions::uniform::SampleBorrow;
use crate::cli::Cli;

fn main() {
    match Cli::start() {
        Ok(_) => ..,
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    };
}


