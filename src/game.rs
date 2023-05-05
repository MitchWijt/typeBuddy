use std::io::{stdin};
use std::sync::{mpsc};
use termion::event::Key;
use termion::input::TermRead;
use crate::game_state::GameState;
use crate::game_text::GameText;
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
        &self.state.cursor_index == &((self.text.raw_text.len()) as u32) && self.state.strike_is_correct ||
            &self.state.cursor_index == &((self.text.raw_text.len() - 1) as u32) && !self.state.strike_is_correct
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

    pub fn start(&mut self) -> Result<(), &'static str> {
        self.initialize();

        let raw_text = self.text.raw_text.clone();
        let mut chars = raw_text.chars().peekable();

        let (tx_timer_duration, rx_timer_duration ) = mpsc::channel::<i32>();
        let (tx_timer_state, rx_timer_state) = mpsc::channel::<TimerState>();
        Timer::start(tx_timer_duration.clone(), rx_timer_state);

        for key in stdin().keys() {
            match key.unwrap() {
                Key::Char('q') => break,
                Key::Ctrl(key) => {
                    if key == 'c' {
                        break;
                    } else if key == 'r' {
                        self.reset();
                        self.initialize();
                        chars = raw_text.chars().peekable();
                        Timer::reset(tx_timer_state.clone());
                    }
                }
                // TODO: split this up into separate functions or modules to make it more clear what this is doing.
                Key::Char(key) => {
                    match chars.peek() {
                        Some(char) => {
                            self.state.previous_indexes.insert(self.state.cursor_index);

                            if &key == char {
                                self.text.color_char(&self.state.cursor_index, Color::Green).unwrap();
                                self.state.strike_is_correct = true;
                            } else {
                                self.text.color_char(&self.state.cursor_index, Color::Red).unwrap();
                                self.state.strike_is_correct = false;
                            }

                            if self.state.strike_is_correct {
                                chars.next();
                                self.state.cursor_index += 1;
                            } else {
                                // When duplicate strike: Ideally we do not rerender and do not increment the amount of chars.
                                if !self.state.is_duplicate_strike() {
                                    self.state.amount_chars_incorrect += 1;
                                }
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
                                self.state.amount_chars_correct = &((self.text.raw_text.len()) as u32) - self.state.amount_chars_incorrect;

                                println!("{:?}", &self.state);
                                // save state to file in a new thread

                                self.terminal.clear_console();
                                self.terminal.render_text(&String::from("Finesso! Congrats, Please press 'Ctrl + r' to play again. Or 'q' to quit"));
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