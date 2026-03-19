use clap::Parser;

use crate::cli::Commands;

#[derive(Parser)]
#[command(name = "varch")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}