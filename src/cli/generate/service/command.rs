use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct ServiceCommand {
    pub name: String,
    #[arg(long, value_enum)]
    pub stack: ServiceStack,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ServiceStack {
    React,
    Tauri,
}