use clap::Parser;

use super::resource::ResourceCommand;

#[derive(Parser)]
pub struct GenerateCommand {
    #[command(subcommand)]
    pub resource: ResourceCommand,
}