use std::collections::{HashMap, HashSet};
use crate::terminal::Terminal;
use termion::style;

#[derive(Debug)]
pub struct GameState {
    pub amount_chars_correct: f32,
    pub amount_chars_incorrect: f32,
    pub duration_in_seconds: f32,
    pub cursor_index: u32,
    pub previous_indexes: HashSet<u32>,
    pub heatmap_incorrect_chars: HashMap<String, u32>,
    pub strike_is_correct: bool,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            amount_chars_incorrect: 0.0,
            amount_chars_correct: 0.0,
            duration_in_seconds: 0.0,
            cursor_index: 0,
            previous_indexes: HashSet::new(),
            heatmap_incorrect_chars: HashMap::new(),
            strike_is_correct: true
        }
    }

    pub fn register_incorrect_char(&mut self, text_hashmap: &HashMap<u32, String>) {
        let char = text_hashmap.get(&self.cursor_index).unwrap();

        let no_underline = char.replace(&style::Underline.to_string(), "");
        let no_reset = no_underline.replace(&style::Reset.to_string(), "");

        self.heatmap_incorrect_chars
            .entry(no_reset)
            .and_modify(|e| {*e += 1})
            .or_insert(1);
    }

    pub fn is_duplicate_strike(&self) -> bool {
        self.previous_indexes.contains(&self.cursor_index)
    }

    pub fn print_heatmap(&self, terminal: &mut Terminal) {
        terminal.set_cursor_row(6);
        terminal.render_text(&String::from("Error breakdown:"));
        terminal.set_cursor_row(7);

        let mut heatmap_vec: Vec<(&String, &u32)> = self.heatmap_incorrect_chars.iter().collect();
        heatmap_vec.sort_by(|a, b| b.1.cmp(a.1));

        let mut index = 0;
        let mut cursor_row = 7;

        for (char, amount_incorrect) in heatmap_vec {
            if index % 6 == 0 {
                cursor_row += 2;
                terminal.set_cursor_row(cursor_row);
            }

            if char == " " {
                terminal.render_text(&String::from(format!("' ':{}\t", amount_incorrect)));
            } else {
                terminal.render_text(&String::from(format!("{}:{}\t", char, amount_incorrect)));
            }

            index += 1;
        }
    }
}
