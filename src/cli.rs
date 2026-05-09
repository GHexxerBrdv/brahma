use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "brahma")]
#[command(about = "lightweight project scaffolder", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Create {
        #[arg(help = "name of the project")]
        name: String,

        #[arg(long, short)]
        #[arg(
            help = "create a well structured and professional project using a predefined templates"
        )]
        template: bool,
    },
}
