use clap::Parser;

mod cli;
mod errors;
mod project_brahma;

use cli::{Cli, Commands};
use cliclack::{intro, outro};
use errors::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    intro("Brahma is lightweight project scaffolder tool")?;

    match cli.commands {
        Commands::Create { name, template } => {
            project_brahma::project::create_project(&name, template)?;
        }
    }

    outro("Done!")?;

    Ok(())
}
