use clap::Subcommand;

use crate::cli::generate::GenerateCommand;

#[derive(Subcommand)]
pub enum Commands {
    Generate(GenerateCommand),
}