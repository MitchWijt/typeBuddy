use std::collections::HashMap;
use std::io::{stdin};
use std::iter::Peekable;
use std::str::Chars;
use std::sync::{Arc, mpsc};
use termion::event::Key;
use termion::input::TermRead;
use crate::game_state::GameState;
use crate::game_text::GameText;
use crate::symbols::{Color, GREEN, RED, RESET, UNDERLINE};
use crate::terminal::Terminal;
use crate::timer::Timer;

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
        let mut peekable_chars = raw_text.chars().peekable();

        // mpsc = multiple producer single consumer. Which is why the tx(sender) can be cloned and the rx(receiver) cannot.
        // our main thread (game) is a producer of continue_timer(tx_continue_timer) and the timer thread is the single consumer of continue_timer
        // the timer thread is a producer of timer_duration(tx_timer_duration) hence the clone. and the main thread (game) is the single consumer of timer_duration.
        let (tx_timer_duration, rx_timer_duration ) = mpsc::channel::<i32>();
        let (tx_continue_timer, rx_continue_timer) = mpsc::channel::<bool>();
        Timer::start(tx_timer_duration.clone(), rx_continue_timer);

        //TODO: Do not rerender or recapture correct/incorrect keystrokes on subsequent correct or incorrect keystrokes on the same key.
        // if the previous cursor index is the same as the current cursor_index. We should not do anything and continue;

        for key in stdin().keys() {
            match key.unwrap() {
                Key::Char('q') => break,
                Key::Ctrl(key) => {
                    if key == 'c' {
                        break;
                    } else if key == 'r' {
                        self.reset();
                        self.initialize();
                        peekable_chars = raw_text.chars().peekable();
                    }
                }
                Key::Char(key) => {
                    let current_char_to_match = peekable_chars.peek();

                    match current_char_to_match {
                        Some(char) => {
                            if &key == char {
                                self.text.color_char(&self.state.cursor_index, Color::Green).unwrap();
                                self.state.strike_is_correct = true;
                            } else {
                                self.text.color_char(&self.state.cursor_index, Color::Red).unwrap();
                                self.state.strike_is_correct = false;
                            }

                            if self.state.strike_is_correct {
                                peekable_chars.next();
                                self.state.amount_chars_correct += 1;
                                self.state.cursor_index += 1;
                            } else {
                                self.state.amount_chars_incorrect += 1;
                            }

                            if !self.is_end() {
                                self.text.remove_underlines().unwrap();
                                self.text.underline_char(&self.state.cursor_index).unwrap();
                            }

                            self.terminal.clear_console();
                            self.terminal.render_text(&self.text.hashmap_to_string());

                            if self.is_end() {
                                Timer::stop(tx_continue_timer.clone());
                                self.state.duration_in_seconds = rx_timer_duration.recv().unwrap();

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