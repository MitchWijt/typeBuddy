use crate::game_state::GameState;
use crate::terminal::Terminal;

#[derive(Debug)]
pub struct Statistics {
    pub wpm: u32,
    pub accuracy: u32,
    pub duration: u32,
}

impl Statistics {
    pub fn from_state(state: &GameState) -> Self {
        let total_characters = state.amount_chars_correct + state.amount_chars_incorrect;
        let minutes = state.duration_in_seconds / 60.0;
        let words = total_characters / 5.0;

        let accuracy: u32 = ((state.amount_chars_correct / total_characters) * 100.0) as u32;
        let wpm: u32 = (words / minutes).round() as u32;

        Statistics {
            wpm,
            accuracy,
            duration: state.duration_in_seconds as u32
        }
    }

    pub fn print(&self, terminal: &mut Terminal) {
        terminal.reset_cursor(3);
        terminal.render_text(&format!("{0: <10} | {1: <10} | {2: <10}",
                                           "WPM", "Accuracy", "Duration in s"));
        terminal.reset_cursor(4);
        terminal.render_text(&format!("{0: <10} | {1: <10} | {2: <10}",
                                           self.wpm, self.accuracy, self.duration));
    }
}