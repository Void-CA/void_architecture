mod cli;

use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::cli::generate;
use crate::cli::generate::feature::FeatureStack;

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
        Commands::ReactFeature { name } => {
            generate::feature::execute(&name, FeatureStack::React);
        }
        Commands::TauriFeature { name } => {
            generate::feature::execute(&name, FeatureStack::Tauri);
        }
    }
}