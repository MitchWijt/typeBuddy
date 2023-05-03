use clap::{Parser, Subcommand};
use crate::game::Game;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Start
}

impl Cli {
    pub fn start() -> Result<(), &'static str> {
        let cli = Cli::parse();

        match &cli.command {
            Some(Commands::Start) => {
                let mut game = Game::new();
                game.start()?;
            },
            None => {}
        }

        Ok(())
    }
}