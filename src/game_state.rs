#[derive(Debug)]
pub struct GameState {
    pub amount_chars_correct: u32,
    pub amount_chars_incorrect: u32,
    pub duration_in_seconds: i32,
    pub cursor_index: u32,
    pub previous_cursor_index: u32,
    pub strike_is_correct: bool
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            amount_chars_incorrect: 0,
            amount_chars_correct: 0,
            duration_in_seconds: 0,
            cursor_index: 0,
            previous_cursor_index: 0,
            strike_is_correct: true
        }
    }
}