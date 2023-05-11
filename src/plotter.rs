use std::cmp::max;
use std::io::stdin;
use std::iter::Rev;
use std::ops::Range;
use termion::event::Key;
use crate::terminal::Terminal;
use termion::input::TermRead;

// - Create struct of plot that keeps his own state, such as the y_values, x_values and data vector
// - It should also contain the terminal in the struct
// - Refactor the terminal struct to that we can increase and decrease the cursor row and col from the struct itself
// - plot_y and plot_x will be internal functions
// - main public functions can be plot_wpm and plot_accuracy and plot_all
// - correctly make max_ticks dynamic based on the range given
// - make it readable and maintainable

pub fn plot() {
    let mut terminal = Terminal::new();
    terminal.clear_before_cursor();

    let data: Vec<u32> = vec![10, 40, 20, 34, 56, 20, 25, 60, 50, 44];

    let values_y: Vec<u32> = vec![80, 60, 40, 20, 0];
    let values_x: Vec<&str> = vec!["05-11", "05-10", "05-09", "05-08", "05-07", "05-06", "05-05", "05-04", "05-03", "05-02"];

    plot_y(&mut terminal, &values_y);
    plot_x(&mut terminal, &values_x, &data);

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
    }
}

// make sure terminal struct keeps state of current cursor_col and cursor_row. So that we can
// fetch is with a method. terminal.cursor_row; terminal.cursor_col;
// terminal can take care of this with it's own internal methods instead of keeping cursor state here.
// e.g: cursor_advance etc..

pub fn plot_y(terminal: &mut Terminal, values_y: &Vec<u32>) {
    let mut index = 0;
    let mut cursor_col: u16 = 1;
    let mut cursor_row: u16 = 1;
    let max_ticks = 25;

    for current_tick in 0..max_ticks {
        if current_tick % (max_ticks / 4) == 0 {
            let y_value = values_y.get(index).expect("Could not find value");
            cursor_col = 1;
            terminal.set_cursor(cursor_col, cursor_row);

            terminal.render_text(&y_value.to_string());
            if *y_value < 10 {
                terminal.render_text(&String::from("  |"));
            } else {
                terminal.render_text(&String::from(" |"));
            }

            cursor_row += 1;
            cursor_col += 2;
            terminal.set_cursor(cursor_col, cursor_row);

            index += 1;
        } else {
            terminal.render_text(&String::from(" |"));
            cursor_row += 1;
            terminal.set_cursor(cursor_col, cursor_row);
        }
    }
}

pub fn plot_x(terminal: &mut Terminal, values_x: &Vec<&str>, data: &Vec<u32>) {
    let mut index = 0;
    let max_ticks = 100;
    let mut cursor_col = terminal.cursor_col;
    let mut cursor_row = terminal.cursor_row;

    // print tricks
    cursor_col += 2;
    terminal.set_cursor_col(cursor_col);

    for _ in 0..max_ticks {
        terminal.render_text(&String::from('-'))
    }

    // move one row below the ticks
    cursor_row += 1;
    terminal.set_cursor_row(cursor_row);

    // print the x_values under the ticks
    for tick in 0..max_ticks {
        if tick % (max_ticks / 9) == 0 {
            let x_value = *values_x.get(index).unwrap();
            terminal.render_text(&String::from(x_value));

            let previous_row = cursor_row.clone();

            let data_value: f64 = *data.get(index).unwrap() as f64;

            let rows = 25.0; //max ticks of the Y axis
            let max_y_value = 90.0;
            let value_per_row = max_y_value / rows;
            let row = rows - (data_value / value_per_row);


            terminal.set_cursor_row(row as u16);
            terminal.render_text(&String::from('◯'));
            terminal.render_text(&data_value.to_string());

            terminal.set_cursor_row(previous_row);

            index += 1;
        } else {
            cursor_col += 1;
            terminal.set_cursor_col(cursor_col);
        }
    }
}