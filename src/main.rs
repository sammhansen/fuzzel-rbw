mod checks;
mod command;
mod config;
mod fuzzel;
mod rbw;
mod utils;

use std::io::Error;

use clap::Parser;
use clap::Subcommand;

use crate::checks::health;
use crate::rbw::unlock;

#[derive(Subcommand, Debug)]
enum Commands {
    /// shows the fuzzel popup
    Show,
    /// checks if your system has all required dependencies installed
    Health,
}

#[derive(Parser, Debug)]
#[command(
    name = "frbw",
    version,
    about = "fuzzel-rbw",
    long_about = "A cli tool that allows you to use fuzzel with rbw (Bitwarden)"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Show => {
            unlock::run()?;
        }
        Commands::Health => {
            health::dependencies()?;
        }
    }

    Ok(())
}
