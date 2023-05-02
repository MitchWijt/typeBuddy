mod symbols;

use std::collections::HashMap;
use clap::{Parser, Subcommand};
use std::io::{stdin, stdout, Stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use symbols::{UNDERLINE, RESET};
use crate::symbols::{Color, GREEN, RED};

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
    text_length: usize,
    cursor_index: u32,
    is_current_strike_match: bool,
}

impl State {
    pub fn new() -> Self {
        let text = String::from("This is the text you must rewrite");
        let length = text.len();
        State {
            text_to_match: text ,
            cursor_index: 0,
            text_length: length,
            is_current_strike_match: true,
        }
    }
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

    //initial char
    underline_char(&mut text_to_match_char_hashmap, &state.cursor_index).unwrap();
    rerender_text_to_match_from_hashmap(&text_to_match_char_hashmap, &mut stdout).unwrap();

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

                match current_char_to_match {
                    Some(char) => {
                        if &key == char {
                            text_to_match_peekable_chars.next();
                            color_char(&mut text_to_match_char_hashmap, &state.cursor_index, Color::Green).unwrap();

                            state.cursor_index += 1;
                            state.is_current_strike_match = true;
                        } else {
                            color_char(&mut text_to_match_char_hashmap, &state.cursor_index, Color::Red).unwrap();
                            state.is_current_strike_match = false;
                        }

                        if state.cursor_index < (state.text_length) as u32 {
                            remove_underlines(&mut text_to_match_char_hashmap, &state.cursor_index).unwrap();
                            underline_char(&mut text_to_match_char_hashmap, &state.cursor_index).unwrap();
                            rerender_text_to_match_from_hashmap(&text_to_match_char_hashmap, &mut stdout).unwrap();
                        }
                    },
                    None => {}
                }
            },
            _ => {}
        };
    }
}

fn chars_into_hashmap(chars: Vec<char>) -> HashMap<u32, String> {
    let mut hash_map = HashMap::new();
    let mut index = 0;

    for char in chars {
        hash_map.insert(index, String::from(char));
        index += 1;
    }

    return hash_map;
}

fn underline_char(hash_map: &mut HashMap<u32, String>, cursor_index: &u32) -> Result<(), &'static str> {
    let char_to_alter = hash_map.get(cursor_index).unwrap();
    let underlined_char = String::from(format!("{}{}{}", UNDERLINE, char_to_alter, RESET));
    hash_map.insert(*cursor_index, underlined_char);

    Ok(())
}

fn remove_underlines(hash_map: &mut HashMap<u32, String>, cursor_index: &u32) -> Result<(), &'static str> {
    for index in 0..*cursor_index {
        let character = hash_map.get(&index).unwrap();
        let removed_underline_char = character.replace(UNDERLINE, "");
        hash_map.insert(index, removed_underline_char);
    }

    Ok(())
}

fn color_char(hash_map: &mut HashMap<u32, String>, cursor_index: &u32, color: Color) -> Result<(), &'static str> {
    let char_to_alter = hash_map.get(cursor_index).unwrap();
    let colored_char = match color {
        Color::Green => String::from(format!("{}{}{}", GREEN, char_to_alter, RESET)),
        Color::Red => String::from(format!("{}{}{}", RED, char_to_alter, RESET)),
    };
    hash_map.insert(*cursor_index, colored_char);

    Ok(())
}



fn hashmap_to_string(hash_map: &HashMap<u32, String>) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    let mut sorted: Vec<_> = hash_map.iter().collect();
    sorted.sort_by_key(|a| a.0);

    for (_, character) in sorted {
        for byte in character.as_bytes() {
            bytes.push(*byte);
        }
    }

    return String::from_utf8(bytes).unwrap();
}

fn rerender_text_to_match_from_hashmap(hash_map: &HashMap<u32, String>, stdout: &mut RawTerminal<Stdout>) -> Result<(), &'static str> {
    let text_to_match_string = hashmap_to_string(hash_map);
    write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
    write!(stdout, "{}", text_to_match_string).unwrap();
    stdout.flush().unwrap();

    Ok(())
}
