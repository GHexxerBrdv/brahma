use clap::Parser;

mod cli;
mod project_brahma;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Create { name, template } => {
            project_brahma::project::create_project(&name, template).unwrap();
        }
    }
}
