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
    Start {
        #[clap(long)]
        max_minutes: Option<u32>
    },
    Plot {
        #[clap(long, default_value_t = false)]
        wpm: bool,

        #[clap(long, default_value_t = false)]
        accuracy: bool,
    }
}

impl Cli {
    pub fn start() -> Result<(), &'static str> {
        let cli = Cli::parse();

        match &cli.command {
            Some(Commands::Start {max_minutes}) => {
                let mut game = Game::new();
                game.start()?;
            },
            Some(Commands::Plot {wpm, accuracy}) => {
                match env::var("TB_STATS_DIR") {
                    Ok(..) => {},
                    Err(..) => return Err("Cannot plot stats, No ENV variable defined to stats path")
                };

                let wpm = *wpm;
                let accuracy = *accuracy;

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
                let mut plotter = Plotter::new(plot_data, (70, 15));

                plotter.plot()?;
            }
            None => {
                return Err("Unknown command TypeBuddy")
            }
        }

        Ok(())
    }
}