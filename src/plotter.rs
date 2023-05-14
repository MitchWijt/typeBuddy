use std::cmp::max;
use std::io::stdin;
use termion::event::Key;
use crate::terminal::Terminal;
use termion::input::TermRead;

#[derive(Debug)]
pub struct PlotData {
    pub data: Vec<(u32, String)>,
    pub y_values: Vec<u32>,
}

pub struct Plotter {
    terminal: Terminal,
    data: Vec<(u32, String)>,
    y_values: Vec<u32>,
    dimension_x: i32,
    dimension_y: i32,
}

impl Plotter {
    pub fn new(plot_data: PlotData, dimension: (i32, i32)) -> Self {
        let terminal = Terminal::new();
        let dimension_x = dimension.0;
        let dimension_y = dimension.1;

        Plotter {
            terminal,
            data: plot_data.data,
            y_values: plot_data.y_values,
            dimension_x,
            dimension_y,
        }
    }

    pub fn plot(&mut self) -> Result<(), &'static str> {
        self.plot_y()?;
        self.plot_x()?;

        for key in stdin().keys() {
            match key.unwrap() {
                Key::Char('q') => {
                    break;
                }
                Key::Ctrl(key) => {
                    if key == 'c' {
                        break;
                    }
                },
                _ => {}
            }
        };

        Ok(())
    }

    fn plot_y(&mut self) -> Result<(), &'static str> {
        let mut cursor_col: u16 = 1;
        let mut cursor_row: u16 = 1;

        let mut ticks = 0;
        let max_ticks= self.dimension_y;
        let ticks_per_value = max_ticks / ((self.y_values.len() - 1) as i32);

        for y_value in &self.y_values {
            cursor_col = 1;
            self.terminal.set_cursor(cursor_col, cursor_row);
            self.terminal.render_text(&y_value.to_string());

            for _ in 0..ticks_per_value {
                if *y_value > 99 {
                    self.terminal.render_text(&String::from("|"));
                    cursor_col = 4;
                } else if *y_value >= 10 {
                    self.terminal.render_text(&String::from(" |"));
                    cursor_col = 3;
                } else {
                    self.terminal.render_text(&String::from("  |"));
                    cursor_col = 2;
                }

                cursor_row += 1;
                self.terminal.set_cursor(cursor_col, cursor_row);

                if ticks >= max_ticks {
                    break;
                }

                ticks += 1;
            }
        }

        Ok(())
    }

    fn plot_x(&mut self) -> Result<(), &'static str> {
        let mut cursor_col = self.terminal.cursor_col;
        let mut cursor_row = self.terminal.cursor_row;

        let bottom_cursor_row = cursor_row + 1;
        let mut ticks = 0;
        let max_ticks = self.dimension_x;
        let ticks_per_value = max_ticks / ((self.data.len() - 1) as i32);

        // we set the cursor col + 4 so it lines up with the Y axis.
        cursor_col += 4;
        self.terminal.set_cursor(cursor_col, cursor_row);

        for (data_value, x_value) in &self.data {
            self.terminal.set_cursor_row(bottom_cursor_row);
            self.terminal.render_text(x_value);

            let rows: f64 = self.dimension_y as f64;
            let max_y_value: f64 = *self.y_values.get(0).unwrap() as f64;
            let value_per_row: f64 = max_y_value / rows;
            let row: u16 = (rows - (*data_value as f64 / value_per_row)) as u16;

            self.terminal.set_cursor_row(row + 1);
            self.terminal.render_text(&String::from('◯'));
            self.terminal.render_text(&data_value.to_string());

            self.terminal.set_cursor_row(bottom_cursor_row - 1);
            for _ in 0..ticks_per_value {
                self.terminal.render_text(&String::from('-'));

                cursor_col += 1;
                self.terminal.set_cursor_col(cursor_col);

                if ticks >= max_ticks {
                    break;
                }

                ticks += 1;
            }
        }

        Ok(())
    }
}