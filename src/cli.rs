use clap::{Parser, Subcommand};
use crate::game::Game;
use crate::plotter::plot;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Start,
    Plot
}

impl Cli {
    pub fn start() -> Result<(), &'static str> {
        let cli = Cli::parse();

        match &cli.command {
            Some(Commands::Start) => {
                let mut game = Game::new();
                game.start()?;
            },
            Some(Commands::Plot) => {
                plot();
            }
            None => {}
        }

        Ok(())
    }
}