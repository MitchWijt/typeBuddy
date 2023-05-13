use std::collections::HashMap;
use std::fs;
use std::fs::{read_to_string};
use rand::Rng;
use crate::symbols::{Color, GREEN, RED, RESET, UNDERLINE};

pub struct GameText {
    pub raw_text: String,
    pub length: u32,
    pub text_hashmap: HashMap<u32, String>,
}

impl GameText {
    pub fn new() -> Self {
        let paragraph = Paragraph::new();
        let _ = paragraph.random();
        let text2 = String::from("Hello");

        let mut hash_map = HashMap::new();
        let mut index = 0;

        for char in text2.chars() {
            hash_map.insert(index, String::from(char));
            index += 1;
        }

        GameText {
            length: text2.len() as u32,
            raw_text: text2,
            text_hashmap: hash_map,
        }
    }

    pub fn hashmap_to_string(&self) -> String {
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

    pub fn reset_hashmap(&mut self) {
        let mut index = 0;

        for char in self.raw_text.chars() {
            self.text_hashmap.insert(index, String::from(char));
            index += 1;
        }
    }

    pub fn underline_char(&mut self, cursor_index: &u32) -> Result<(), &'static str> {
        let char_to_alter = self.text_hashmap.get(cursor_index).unwrap();
        let underlined_char = String::from(format!("{}{}{}", UNDERLINE, char_to_alter, RESET));
        self.text_hashmap.insert(*cursor_index, underlined_char);

        Ok(())
    }

    pub fn remove_underlines(&mut self) -> Result<(), &'static str> {
        for index in 0..(self.raw_text.len()) as u32 {
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

struct Paragraph {
    excerpts: Vec<String>
}

impl Paragraph {
    pub fn new() -> Self {
        let mut excerpts = Vec::new();

        let files = fs::read_dir("./assets/excerpts").unwrap();
        for dir_entry in files {
            let file_path = dir_entry.unwrap().path();

            let excerpt = read_to_string(file_path).unwrap();
            excerpts.push(excerpt);
        };

        Paragraph {
            excerpts
        }
    }

    pub fn random(&self) -> String {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.excerpts.len());

        self.excerpts.get(random_index).unwrap().clone()
    }
}
