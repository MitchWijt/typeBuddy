mod symbols;

use std::collections::HashMap;
use clap::{Parser, Subcommand};
use std::io::{stdin, stdout, Write};
use std::str::Chars;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use symbols::{UNDERLINE, RESET};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Start
}

struct State {
    text_to_match: String,
    is_current_strike_match: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            text_to_match: String::from("This is the text you must rewrite"),
            is_current_strike_match: true,
        }
    }
}

/*
Main thing to fix at the moment is the HashMap that only can have unique values.
We need duplicate values, and also sort them by Index and able to alter the characters to
underline and color them accordingly.

Altering characters in the middle of a set is mandatory.

We also need to have keep more state / statistics
 */

//change this name, it's kind of ugly
#[derive(Debug)]
struct CharInText {
    character: String,
    index: u32,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start) => {
            loop_type_exercise();
        },
        None => {}
    }
}

fn loop_type_exercise() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut state = State::new();

    let mut text_to_match_peekable_chars = state.text_to_match.chars().peekable();
    let mut text_to_match_char_hashmap = chars_into_hashmap(state.text_to_match.chars().collect());

    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
    write!(stdout, "{}", state.text_to_match).unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    for key in stdin().keys() {
        match key.unwrap() {
            Key::Char('q') => break,
            Key::Ctrl(key) => {
                if key == 'c' {
                    break;
                }
            }
            Key::Char(key) => {
                let current_char_to_match = text_to_match_peekable_chars.peek().unwrap();

                // underline the current char to be written on the cursor index and write the string
                // to stdout.

                // get char from hashmap
                // clone the value into a new string that is altered
                // insert the character with the new string and the same index
                let char_to_alter = text_to_match_char_hashmap.get_mut(current_char_to_match).unwrap();
                char_to_alter.character = underline_char(current_char_to_match);

                hashmap_to_string(&text_to_match_char_hashmap);

                // text_to_match_char_hashmap.insert(*current_char_to_match, char_to_alter);


                if &key == current_char_to_match {
                    text_to_match_peekable_chars.next();
                    state.is_current_strike_match = true;
                } else {
                    state.is_current_strike_match = false;
                }
            },
            _ => {}
        };
    }
}

fn chars_into_hashmap(chars: Vec<char>) -> HashMap<u32, CharInText> {
    let mut hash_map = HashMap::new();
    let mut index = 0;

    for char in chars {
        let character = CharInText {
            character: String::from(char),
            index,
        };
        hash_map.insert(index, character);

        // can we do this in another way?
        index += 1;
    }

    return hash_map;
}

fn underline_char(char: &char) -> String {
    return String::from(format!("{}{}{}", UNDERLINE, char, RESET));
}

fn hashmap_to_string(hash_map: &HashMap<char, CharInText>) -> String {
    let mut sorted: Vec<_> = hash_map.iter().collect();
    sorted.sort_by(|a, b| a.1.index.cmp(&b.1.index));

    println!("{:?}", sorted);

    return String::from("sdfdsf");
}
