use std::io::{Stdout, stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

pub struct Terminal {
    stdout: AlternateScreen<RawTerminal<Stdout>>
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdout: stdout()
                .into_raw_mode()
                .unwrap()
                .into_alternate_screen()
                .unwrap()
        }
    }

    pub fn clear_before_cursor(&mut self) {
        write!(self.stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::BeforeCursor).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn clear_console(&mut self) {
        write!(self.stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor_row(&mut self, cursor_row: u16) {
        write!(self.stdout, "{}",termion::cursor::Goto(1, cursor_row)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn render_text(&mut self, text: &String) {
        write!(self.stdout, "{}", text).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn hide_cursor(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Hide).unwrap();
        self.stdout.flush().unwrap();
    }
}