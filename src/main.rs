use clap::Parser;

mod cli;
mod errors;
mod project_brahma;

use errors::Context;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Create { name, template } => {
            project_brahma::project::create_project(&name, template)
                .context("Failed to create project")
                .unwrap();
        }
    }
}
