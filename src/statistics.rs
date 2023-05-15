use std::{env, thread};
use std::fs::{OpenOptions};
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
use chrono::{Datelike, Local, Timelike, TimeZone};
use crate::game_state::GameState;
use crate::terminal::Terminal;
use serde_json::{to_writer, from_reader};
use serde::{Deserialize, Serialize};
use crate::plotter::{PlotData};

pub enum StatisticDataType {
    WPM,
    ACCURACY
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Statistics {
    pub wpm: u32,
    pub accuracy: u32,
    pub duration: u32,
    pub timestamp: i64,
}

impl Statistics {
    pub fn from_state(state: &GameState) -> Self {
        let total_characters = state.amount_chars_correct + state.amount_chars_incorrect;
        let minutes = state.duration_in_seconds / 60.0;
        let words = total_characters / 5.0;

        let accuracy: u32 = ((state.amount_chars_correct / total_characters) * 100.0) as u32;
        let wpm: u32 = (words / minutes).round() as u32;
        let timestamp = Local::now().timestamp();

        Statistics {
            wpm,
            accuracy,
            duration: state.duration_in_seconds as u32,
            timestamp,
        }
    }

    pub fn print(&self, terminal: &mut Terminal) {
        terminal.set_cursor_row(3);
        terminal.render_text(&format!("{0: <10} | {1: <10} | {2: <10}",
                                           "WPM", "Accuracy", "Duration in s"));
        terminal.set_cursor_row(4);
        terminal.render_text(&format!("{0: <10} | {1: <10} | {2: <10}",
                                           self.wpm, self.accuracy, self.duration));
    }

    //make sure to save the most recent ones first. FIFO
    pub fn save(self) {
        let stats_dir_path = env::var("TB_STATS_DIR");

        match stats_dir_path {
            Ok(dir_path) => {
                let path = format!("{}/type_buddy_stats.json", dir_path);

                let handle = thread::spawn(move || {
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .read(true)
                        .open(&path)
                        .unwrap();

                    let reader = BufReader::new(&file);
                    let mut stats: Vec<Statistics> = match from_reader(reader) {
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

    pub fn read_all() -> Vec<Statistics> {
        let stats_dir_path = env::var("TB_STATS_DIR").unwrap();
        let path = format!("{}/type_buddy_stats.json", stats_dir_path);

        let file = OpenOptions::new()
            .read(true)
            .open(&path)
            .unwrap();

        let reader = BufReader::new(&file);
        match from_reader(reader) {
            Ok(v) => v,
            Err(_) => Vec::new()
        }
    }

    pub fn recent() -> Vec<Statistics> {
        let statistics = Self::read_all();
        let mut recent_statistics: Vec<Statistics> = Vec::new();
        let mut index = 0;

        for statistic in statistics.into_iter() {
            if index > 10 {
                break;
            }
            recent_statistics.push(statistic);
            index += 1;
        };

        recent_statistics
    }

    pub fn plottable_data(data_type: StatisticDataType) -> PlotData {
        let recent_statistics = Self::recent();
        let mut data: Vec<(u32, String)> = Vec::new();

        for statistic in recent_statistics {
            let data_value = match data_type {
                StatisticDataType::WPM => statistic.wpm,
                StatisticDataType::ACCURACY => statistic.accuracy
            };

            let date = Local.timestamp_opt(statistic.timestamp, 0).unwrap();
            let date_string = format!("{}-{} {}:{}", date.month(), date.day(), date.time().hour(), date.time().minute());
            data.push((data_value, date_string));
        };

        let y_values = vec![100, 80, 60, 40, 20, 0];

        PlotData {
            data,
            y_values
        }
    }
}