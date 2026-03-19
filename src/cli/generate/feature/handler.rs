use clap::{Args, ValueEnum};

use super::stacks;

#[derive(Args)]
pub struct FeatureCommand {
    pub name: String,
    #[arg(long, value_enum)]
    pub stack: FeatureStack,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum FeatureStack {
    React,
    Tauri,
}

pub fn handle(command: FeatureCommand) {
    match command.stack {
        FeatureStack::React => stacks::react::handle(&command.name),
        FeatureStack::Tauri => stacks::tauri::handle(&command.name),
    }
}