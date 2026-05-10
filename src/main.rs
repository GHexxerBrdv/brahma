use anyhow::Ok;
use clap::Parser;

mod cli;
mod errors;
mod project_brahma;

use cli::{Cli, Commands};
use errors::{Context, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Create { name, template } => {
            project_brahma::project::create_project(&name, template)
                .context("Failed to create project")?
        }
    }
    Ok(())
}
