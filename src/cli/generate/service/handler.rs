use super::command::ServiceCommand;
use super::execute::execute;

pub fn handle(command: ServiceCommand) {
    execute(&command.name, command.stack);
}