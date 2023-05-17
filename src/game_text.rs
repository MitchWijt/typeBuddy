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
        let text = paragraph.random();

        let mut hash_map = HashMap::new();
        let mut index = 0;

        for char in text.chars() {
            hash_map.insert(index, String::from(char));
            index += 1;
        }

        GameText {
            length: text.len() as u32,
            raw_text: text,
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
            let remove_reset_char = removed_underline_char.replace(RESET, "");

            self.text_hashmap.insert(index, remove_reset_char);
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

#[cfg(test)]
mod tests {
    use crate::game::Game;
    use super::*;

    #[test]
    fn test_game_text_hashmap_to_string() {
        //given
        let game_text = GameText {
            length: 5,
            raw_text: String::from("Hello"),
            text_hashmap: HashMap::from([
                (0, String::from('H')),
                (1, String::from('e')),
                (2, String::from('l')),
                (3, String::from('l')),
                (4, String::from('o')),
            ])
        };

        //when
        let game_text_string = game_text.hashmap_to_string();

        //then
        assert_eq!(game_text_string, String::from("Hello"));
    }

    #[test]
    fn test_game_text_underline_char() -> Result<(), &'static str> {
        //given
        let mut game_text = GameText {
            length: 5,
            raw_text: String::from("Hello"),
            text_hashmap: HashMap::from([
                (0, String::from('H')),
                (1, String::from('e')),
                (2, String::from('l')),
                (3, String::from('l')),
                (4, String::from('o')),
            ])
        };

        //when
        game_text.underline_char(&2)?;

        //then
        let altered_char = game_text.text_hashmap.get(&2).unwrap();
        assert_eq!(altered_char, &String::from("\x1b[4ml\x1b[0m"));

        Ok(())
    }

    #[test]
    fn test_game_text_remove_underlines() -> Result<(), &'static str> {
        //given
        let mut game_text = GameText {
            length: 5,
            raw_text: String::from("Hello"),
            text_hashmap: HashMap::from([
                (0, String::from('H')),
                (1, String::from('e')),
                (2, String::from("\x1b[4ml\x1b[0m")),
                (3, String::from('l')),
                (4, String::from('o')),
            ])
        };

        //when
        game_text.remove_underlines()?;

        //then
        let char = game_text.text_hashmap.get(&2).unwrap();
        assert_eq!(char, &String::from("l"));

        Ok(())
    }

    #[test]
    fn test_game_text_color_char() -> Result<(), &'static str> {
        //given
        let mut game_text = GameText {
            length: 5,
            raw_text: String::from("Hello"),
            text_hashmap: HashMap::from([
                (0, String::from('H')),
                (1, String::from('e')),
                (2, String::from("l")),
                (3, String::from('l')),
                (4, String::from('o')),
            ])
        };

        //when
        game_text.color_char(&2, Color::Green)?;

        //then
        let char = game_text.text_hashmap.get(&2).unwrap();
        assert_eq!(char, &String::from("\x1b[38;5;46ml\x1b[0m"));

        Ok(())
    }

    #[test]
    fn test_game_text_reset_hashmap() -> Result<(), &'static str> {
        //given
        let mut game_text = GameText {
            length: 5,
            raw_text: String::from("Hello"),
            text_hashmap: HashMap::from([
                (0, String::from("\x1b[38;5;46mH\x1b[0m")),
                (1, String::from("\x1b[38;5;46me\x1b[0m")),
                (2, String::from("\x1b[38;5;46ml\x1b[0m")),
                (3, String::from("\x1b[38;5;46ml\x1b[0m")),
                (4, String::from("\x1b[38;5;46mo\x1b[0m")),
            ])
        };

        //when
        game_text.reset_hashmap();

        //then
        assert_eq!(game_text.text_hashmap.len(), 5);
        for (_, char) in game_text.text_hashmap {
            let includes_csi = char.contains("\x1b");
            assert_eq!(includes_csi, false);
        }

        Ok(())
    }
}
