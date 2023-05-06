use std::io::{stdin};
use std::sync::{mpsc};
use termion::event::Key;
use termion::input::TermRead;
use crate::game_state::GameState;
use crate::game_text::GameText;
use crate::statistics::Statistics;
use crate::symbols::{Color};
use crate::terminal::Terminal;
use crate::timer::{Timer, TimerState};

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

    fn is_end(&self) -> bool {
        self.state.cursor_index == self.text.length && self.state.strike_is_correct ||
            self.state.cursor_index == (self.text.length - 1) && !self.state.strike_is_correct
    }

    fn reset(&mut self) {
        self.state = GameState::new();
        self.text.reset_hashmap();
    }

    fn initialize(&mut self) {
        self.terminal.clear_console();
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
        self.text.color_char(&self.state.cursor_index, Color::Red).unwrap();
        self.state.strike_is_correct = false;

        self.state.previous_indexes.insert(self.state.cursor_index);
        self.state.amount_chars_incorrect += 1.0;
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        self.initialize();

        let raw_text = self.text.raw_text.clone();
        let mut chars = raw_text.chars().peekable();

        let (tx_timer_duration, rx_timer_duration ) = mpsc::channel::<f32>();
        let (tx_timer_state, rx_timer_state) = mpsc::channel::<TimerState>();
        Timer::start(tx_timer_duration.clone(), rx_timer_state);

        for key in stdin().keys() {
            match key.unwrap() {
                Key::Char('q') => {
                    self.terminal.clear_console();
                    break;
                },
                Key::Ctrl(key) => {
                    if key == 'c' {
                        self.terminal.clear_console();
                        break;
                    } else if key == 'r' {
                        self.reset();
                        self.initialize();
                        chars = raw_text.chars().peekable();
                        Timer::reset(tx_timer_state.clone());
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

                            self.terminal.clear_console();
                            self.terminal.render_text(&self.text.hashmap_to_string());

                            if self.is_end() {
                                Timer::stop(tx_timer_state.clone());

                                self.state.duration_in_seconds = rx_timer_duration.recv().unwrap();
                                self.state.amount_chars_correct = &((self.text.length) as f32) - self.state.amount_chars_incorrect;


                                self.terminal.clear_console();
                                self.terminal.render_text(&String::from("Finesso! Congrats, Please press 'Ctrl + r' to play again. Or 'q' to quit"));

                                let stats = Statistics::from_state(&self.state);
                                stats.print(&mut self.terminal);
                                stats.save();
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