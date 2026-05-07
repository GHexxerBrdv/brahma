use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "brahma")]
#[command(about = "Project scaffolder", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        name: String,

        #[arg(long, short)]
        template: bool,
    },
}
