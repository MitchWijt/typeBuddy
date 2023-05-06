use std::{env, thread};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;
use std::time::Duration;
use crate::game_state::GameState;
use crate::terminal::Terminal;
use serde_json::{json, to_writer, to_vec, from_reader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
    pub fn save(self) {
        let stats_dir_path = env::var("TB_STATS_DIR");

        match stats_dir_path {
            Ok((dir_path)) => {
                let path = format!("{}/type_buddy_stats.json", dir_path);

                let handle = thread::spawn(move || {
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(&path)
                        .unwrap();

                    let reader = BufReader::new(&file);
                    let mut stats = match from_reader(reader) {
                        Ok(v) => v,
                        Err(_) => Vec::new()
                    };
                    stats.push(self);

                    file.set_len(0).unwrap();
                    file.seek(SeekFrom::Start(0)).unwrap();

                    let mut writer = BufWriter::new(&file);
                    to_writer(&mut writer, &stats).unwrap();
                    writer.flush().unwrap();
                });

                handle.join().unwrap();
            }
            Err(_) => return
        }
    }
}