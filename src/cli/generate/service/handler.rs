use super::command::ServiceCommand;

pub fn handle(command: ServiceCommand) {
    println!("Generating service: {}", command.name);
}