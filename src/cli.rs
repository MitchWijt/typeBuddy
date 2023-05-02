use clap::{Parser, Subcommand};

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
                // initiate GameState and GameText here
                // call Game::start()
                loop_type_exercise();
            },
            None => {}
        }

        Ok(())
    }
}