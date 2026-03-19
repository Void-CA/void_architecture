use clap::{Args, ValueEnum};

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