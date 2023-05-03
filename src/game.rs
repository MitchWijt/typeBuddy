use std::collections::HashMap;
use std::io::{stdin};
use termion::event::Key;
use termion::input::TermRead;
use crate::symbols::{Color, GREEN, RED, RESET, UNDERLINE};
use crate::terminal::Terminal;

struct GameState {
    amount_chars_correct: u32,
    amount_chars_incorrect: u32,
    duration_milliseconds: u32,
    cursor_index: u32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            amount_chars_incorrect: 0,
            amount_chars_correct: 0,
            duration_milliseconds: 0,
            cursor_index: 0,
        }
    }
}

fn string_to_hashmap(string: &String) -> HashMap<u32, String> {
    let mut hash_map = HashMap::new();
    let mut index = 0;

    for char in string.chars() {
        hash_map.insert(index, String::from(char));
        index += 1;
    }

    return hash_map;
}

struct GameText {
    raw_text: String,
    text_hashmap: HashMap<u32, String>,
}

impl GameText {
    pub fn new() -> Self {
        let text = String::from("This is the text to match");
        let hash_map = string_to_hashmap(&text);

        GameText {
            raw_text: text,
            text_hashmap: hash_map,
        }
    }

    fn hashmap_to_string(&self) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        let mut sorted: Vec<_> = self.text_hashmap.iter().collect();
        sorted.sort_by_key(|a| a.0);

        for (_, character) in sorted {
            for byte in character.as_bytes() {
                bytes.push(*byte);
            }
        }

        return String::from_utf8(bytes).unwrap();
    }

    pub fn underline_char(&mut self, cursor_index: &u32) -> Result<(), &'static str> {
        let char_to_alter = self.text_hashmap.get(cursor_index).unwrap();
        let underlined_char = String::from(format!("{}{}{}", UNDERLINE, char_to_alter, RESET));
        self.text_hashmap.insert(*cursor_index, underlined_char);

        Ok(())
    }

    pub fn remove_underlines(&mut self, cursor_index: &u32) -> Result<(), &'static str> {
        for index in 0..*cursor_index {
            let character = self.text_hashmap.get(&index).unwrap();
            let removed_underline_char = character.replace(UNDERLINE, "");
            self.text_hashmap.insert(index, removed_underline_char);
        }

        Ok(())
    }

    pub fn color_char(&mut self, cursor_index: &u32, color: Color) -> Result<(), &'static str> {
        let char_to_alter = self.text_hashmap.get(cursor_index).unwrap();
        let colored_char = match color {
            Color::Green => String::from(format!("{}{}{}", GREEN, char_to_alter, RESET)),
            Color::Red => String::from(format!("{}{}{}", RED, char_to_alter, RESET)),
        };
        self.text_hashmap.insert(*cursor_index, colored_char);

        Ok(())
    }
}

pub struct Game {
    text: GameText,
    state: GameState,
    terminal: Terminal
}

impl Game {
    pub fn new() -> Self {
        Game {
            text: GameText::new(),
            state: GameState::new(),
            terminal: Terminal::new(),
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        self.terminal.clear_console();
        self.terminal.hide_cursor();

        self.text.underline_char(&self.state.cursor_index).unwrap();
        self.terminal.render_text(&self.text.hashmap_to_string());

        let text = self.text.raw_text.clone();
        let mut peekable_chars = text.chars().peekable();

        for key in stdin().keys() {
            match key.unwrap() {
                Key::Char('q') => break,
                Key::Ctrl(key) => {
                    if key == 'c' {
                        break;
                    }
                }
                Key::Char(key) => {
                    let current_char_to_match = peekable_chars.peek();

                    match current_char_to_match {
                        Some(char) => {
                            if &key == char {
                                peekable_chars.next();
                                self.text.color_char(&self.state.cursor_index, Color::Green).unwrap();
                                self.state.cursor_index += 1;
                            } else {
                                self.text.color_char(&self.state.cursor_index, Color::Red).unwrap();
                            }

                            if  &self.state.cursor_index < &((self.text.raw_text.len()) as u32) {
                                self.text.remove_underlines(&self.state.cursor_index).unwrap();
                                self.text.underline_char(&self.state.cursor_index).unwrap();

                                self.terminal.clear_console();
                                self.terminal.render_text(&self.text.hashmap_to_string());
                            }
                        },
                        None => {}
                    }
                },
                _ => {}
            };
        }
        Ok(())
    }
}