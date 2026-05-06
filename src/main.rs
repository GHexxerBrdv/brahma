use clap::Parser;

mod cli;
mod generator;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::New { name } => generator::project::create_project(&name).unwrap(),
    }
}
