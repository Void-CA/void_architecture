use super::resource::ResourceCommand;

pub fn handle(resource: ResourceCommand) {
    match resource {
        ResourceCommand::Feature { name } => {
            println!("Generating feature: {}", name);
        }
        ResourceCommand::Service { name } => {
            println!("Generating service: {}", name);
        }
    }
}