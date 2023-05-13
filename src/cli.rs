use std::env;
use clap::{Parser, Subcommand};
use crate::game::Game;
use crate::plotter::Plotter;
use crate::statistics::{StatisticDataType, Statistics};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Start,
    Plot {
        #[clap(long)]
        wpm: Option<bool>,

        #[clap(long)]
        accuracy: Option<bool>,
    }
}

impl Cli {
    pub fn start() -> Result<(), &'static str> {
        let cli = Cli::parse();

        match &cli.command {
            Some(Commands::Start) => {
                let mut game = Game::new();
                game.start()?;
            },
            Some(Commands::Plot {wpm, accuracy}) => {
                match env::var("TB_STATS_DIR") {
                    Ok(..) => {},
                    Err(..) => return Err("Cannot plot stats, No ENV variable defined to stats path")
                };

                let wpm = match wpm {
                    Some(v) => *v,
                    None => false,
                };

                let accuracy = match accuracy {
                    Some(v) => *v,
                    None => false,
                };

                if !wpm & !accuracy {
                    return Err("Please choose to either plot WPM or Accuracy");
                } else if wpm && accuracy {
                    return Err("Please choose to either plot WPM or Accuracy. Cannot plot Both");
                }

                let data_type = if wpm {
                    StatisticDataType::WPM
                } else {
                    StatisticDataType::ACCURACY
                };

                let plot_data = Statistics::plottable_data(data_type);
                if plot_data.data.len() < 1 {
                    return Err("No data to plot")
                }
                let mut plotter = Plotter::new(plot_data, (100, 25));

                plotter.plot()?;
            }
            None => {
                return Err("Unknown command TypeBuddy")
            }
        }

        Ok(())
    }
}