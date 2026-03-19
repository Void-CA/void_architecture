mod cli;

use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::cli::generate;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(cmd) => {
            generate::handler::handle(cmd.resource);
        }
    }
}