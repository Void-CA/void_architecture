use super::command::ServiceStack;
use super::stacks;

pub fn execute(name: &str, stack: ServiceStack) {
    match stack {
        ServiceStack::React => stacks::react::execute(name),
        ServiceStack::Tauri => stacks::tauri::execute(name),
    }
}