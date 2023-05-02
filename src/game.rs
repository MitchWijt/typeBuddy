use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

pub struct GameState {
    amount_chars_correct: u32,
    amount_chars_incorrect: u32,
    total_words: usize,
    duration_milliseconds: u32,
    cursor_index: u32,
}

impl GameState {
    pub fn new(text: GameText) -> Self {
        GameState {
            amount_chars_incorrect: 0,
            amount_chars_correct: 0,
            total_words: text.raw_text.len(),
            duration_milliseconds: 0,
            cursor_index: 0,
        }
    }
}

struct GameText<'a> {
    raw_text: String,
    text_hashmap: HashMap<u32, String>,
    text_chars: Peekable<Chars<'a>>
}

impl GameText {
    pub fn new() -> Self {
        let raw_text = String::from("This is the text to match");
        let hash_map = Self.to_hashmap(&raw_text);
        let chars = raw_text.chars().peekable();

        GameText {
            raw_text,
            text_hashmap: hash_map,
            text_chars: chars,
        }
    }

    pub fn from_string(string: String) -> Self {
        let hash_map = Self.to_hashmap(&string);
        let chars = string.chars().peekable();

        GameText {
            raw_text: string,
            text_hashmap: hash_map,
            text_chars: chars,
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

    pub fn underline_char(&self) {
        // use hashmap
    }

    pub fn remove_underlines(&self) {
        // use hashmap
    }

    pub fn color_char(&self) {
        // use hashmap
    }
}

struct Game<'a> {
    text_to_match_hashmap: HashMap<u32, String>,
    text_to_match_chars: Peekable<Chars<'a>>,

}