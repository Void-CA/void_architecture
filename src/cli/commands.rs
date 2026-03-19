use clap::Subcommand;

use crate::cli::generate::GenerateCommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "generate", about = "Generate code for features and services")]
    Generate(GenerateCommand),
}
