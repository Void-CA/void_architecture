use super::command::FeatureCommand;
use super::execute::execute;

pub fn handle(command: FeatureCommand) {
    execute(&command.name, command.stack);
}