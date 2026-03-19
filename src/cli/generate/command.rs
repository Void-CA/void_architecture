use clap::{Parser, Subcommand};

use super::feature::FeatureCommand;

#[derive(Parser)]
pub struct GenerateCommand {
    #[command(subcommand)]
    pub resource: GenerateResourceCommand,
}

#[derive(Subcommand)]
pub enum GenerateResourceCommand {
    Feature(FeatureCommand),
}