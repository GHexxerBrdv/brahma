use brahma_core::cli::{Cli, Commands};
use brahma_core::errors::Context;
use brahma_core::project_brahma;
use clap::Parser;

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
