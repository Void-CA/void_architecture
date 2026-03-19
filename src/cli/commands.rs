use clap::Subcommand;

use crate::cli::generate::GenerateCommand;

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "generate", about = "Generate code for features and services")]
    Generate(GenerateCommand),

    #[command(name = "rf", about = "Generate a React feature")]
    ReactFeature { name: String },

    #[command(name = "tf", about = "Generate a Tauri feature")]
    TauriFeature { name: String },
}
