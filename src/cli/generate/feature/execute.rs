use super::command::FeatureStack;
use super::stacks;

pub fn execute(name: &str, stack: FeatureStack) {
    match stack {
        FeatureStack::React => stacks::react::execute(name),
        FeatureStack::Tauri => stacks::tauri::execute(name),
    }
}