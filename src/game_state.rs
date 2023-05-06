use std::collections::HashSet;

#[derive(Debug)]
pub struct GameState {
    pub amount_chars_correct: u32,
    pub amount_chars_incorrect: u32,
    pub duration_in_seconds: i32,
    pub cursor_index: u32,
    pub previous_indexes: HashSet<u32>,
    pub strike_is_correct: bool
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            amount_chars_incorrect: 0,
            amount_chars_correct: 0,
            duration_in_seconds: 0,
            cursor_index: 0,
            previous_indexes: HashSet::new(),
            strike_is_correct: true
        }
    }

    pub fn is_duplicate_strike(&self) -> bool {
        self.previous_indexes.contains(&self.cursor_index)
    }

    pub fn save() {

    }
}