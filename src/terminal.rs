use std::io::{Stdout, stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

pub struct Terminal {
    pub cursor_col: u16,
    pub cursor_row: u16,
    stdout: AlternateScreen<RawTerminal<Stdout>>
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            cursor_col: 1,
            cursor_row: 1,
            stdout: stdout()
                .into_raw_mode()
                .unwrap()
                .into_alternate_screen()
                .unwrap()
        }
    }

    pub fn clear_before_cursor(&mut self) {
        self.cursor_row = 1;
        self.cursor_col = 1;

        write!(self.stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::BeforeCursor).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn clear_console(&mut self) {
        self.cursor_row = 1;
        self.cursor_col = 1;

        write!(self.stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor_row(&mut self, cursor_row: u16) {
        self.cursor_row = cursor_row;

        write!(self.stdout, "{}",termion::cursor::Goto(self.cursor_col, cursor_row)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor_col(&mut self, cursor_col: u16) {
        self.cursor_col = cursor_col;

        write!(self.stdout, "{}",termion::cursor::Goto(cursor_col, self.cursor_row)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor(&mut self, cursor_col: u16, cursor_row: u16) {
        self.cursor_row = cursor_row;
        self.cursor_col = cursor_col;

        write!(self.stdout, "{}",termion::cursor::Goto(cursor_col, cursor_row)).unwrap();
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

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}