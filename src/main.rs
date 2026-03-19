mod cli;

use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::cli::generate;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate(cmd) => {
            match cmd.resource {
                generate::GenerateResourceCommand::Feature(command) => {
                    generate::feature::handler::handle(command);
                }
                generate::GenerateResourceCommand::Service(command) => {
                    generate::service::handler::handle(command);
                }
            }
        }
    }
}