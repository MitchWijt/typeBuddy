pub const UNDERLINE: &str = "\x1b[4m";
pub const GREEN: &str = "\x1b[38;5;46m";
pub const RED: &str = "\x1b[38;5;196m";
pub const RESET: &str = "\x1b[0m";

pub enum Color {
    Green,
    Red
}