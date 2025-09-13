mod checks;
mod constants;
mod rbw;
mod utils;

use clap::Parser;
use clap::Subcommand;

use crate::checks::health;
use crate::rbw::vault;

// use crate::checks::health;

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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Show => {
            vault::unlock();
        }
        Commands::Health => {
            health::requirements();
        }
    }
}
