use clap::Subcommand;

#[derive(Subcommand)]
pub enum ResourceCommand {
    Feature {
        name: String,
    },
    Service {
        name: String,
    },
}