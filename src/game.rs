use std::io::{stdin};
use std::sync::{Arc, Mutex};
use termion::event::Key;
use termion::input::TermRead;
use crate::game_state::GameState;
use crate::game_text::GameText;
use crate::statistics::Statistics;
use crate::symbols::{Color};
use crate::terminal::Terminal;
use crate::timer::{Timer};

pub struct Game {
    text: GameText,
    state: GameState,
    terminal: Terminal,
    pub timer: Timer,
}

impl Game {
    pub fn new(max_minutes: &Option<u32>) -> Self {
        let timer = match max_minutes {
            Some(minutes) => {
                Timer::from_max_minutes(minutes)
            },
            None => {
                Timer::new()
            }
        };

        Game {
            text: GameText::new(),
            state: GameState::new(),
            terminal: Terminal::new(),
            timer,
        }
    }

    fn is_end(&self) -> bool {
        self.state.cursor_index == self.text.length && self.state.strike_is_correct ||
            self.state.cursor_index == (self.text.length - 1) && !self.state.strike_is_correct
    }

    fn reset(&mut self) {
        self.state = GameState::new();
        self.text.reset_hashmap();
    }

    fn initialize(&mut self) {
        self.terminal.clear_before_cursor();
        self.terminal.hide_cursor();

        self.text.underline_char(&self.state.cursor_index).unwrap();
        self.terminal.render_text(&self.text.hashmap_to_string());
    }

    fn handle_correct_strike(&mut self) {
        self.text.color_char(&self.state.cursor_index, Color::Green).unwrap();
        self.state.strike_is_correct = true;

        self.state.previous_indexes.insert(self.state.cursor_index);
        self.state.cursor_index += 1;
    }

    fn handle_incorrect_strike(&mut self) {
        self.state.register_incorrect_char(&self.text.text_hashmap);
        self.text.color_char(&self.state.cursor_index, Color::Red).unwrap();
        self.state.strike_is_correct = false;

        self.state.previous_indexes.insert(self.state.cursor_index);
        self.state.amount_chars_incorrect += 1.0;
    }

    pub fn stop(&mut self, duration: Arc<Mutex<f32>>) -> Result<(), &'static str> {
        self.state.duration_in_seconds = *duration.lock().unwrap();
        self.state.amount_chars_correct = &((self.text.length) as f32) - self.state.amount_chars_incorrect;

        self.terminal.clear_console();
        self.terminal.render_text(&String::from("Finesso! Congrats, Please press 'Ctrl + r' to play again. Or 'Ctr + c' to quit"));

        let stats = Statistics::from_state(&self.state);
        stats.print(&mut self.terminal);
        stats.save();

        self.state.print_heatmap(&mut self.terminal);

        Ok(())
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        self.initialize();

        let raw_text = self.text.raw_text.clone();
        let mut chars = raw_text.chars().peekable();

        self.timer.start();

        for key in stdin().keys() {
            match key.unwrap() {
                Key::Ctrl(key) => {
                    if key == 'c' {
                        self.terminal.clear_console();
                        break;
                    } else if key == 'r' {
                        self.reset();
                        self.initialize();
                        chars = raw_text.chars().peekable();
                        self.timer.reset();
                    }
                }
                Key::Char(key) => {
                    match chars.peek() {
                        Some(char) => {
                            if &key == char {
                                self.handle_correct_strike();
                                chars.next();
                            } else {
                                if self.state.is_duplicate_strike() {
                                    continue;
                                }
                                self.handle_incorrect_strike();
                            }

                            if !self.is_end() {
                                self.text.remove_underlines().unwrap();
                                self.text.underline_char(&self.state.cursor_index).unwrap();
                            }

                            self.terminal.clear_before_cursor();
                            self.terminal.render_text(&self.text.hashmap_to_string());

                            // if self.is_end || reached max_min
                            if self.is_end() {
                                self.stop(self.timer.duration.clone())?;
                            };
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