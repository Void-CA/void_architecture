use super::command::FeatureCommand;

pub fn handle(command: FeatureCommand) {
    println!("Generating feature: {}", command.name);
}