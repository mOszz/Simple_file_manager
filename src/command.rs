use std::error::Error;
pub struct Command {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub description: String,
    pub action: fn(Vec<String>) -> Result<(), Box<dyn Error>>,
}

impl Command {
    pub fn new(name: String, command: String, args: Vec<String>, description: String, action: fn(Vec<String>) -> Result<(), Box<dyn Error>>) -> Self {
        Command {
            name,
            command,
            args,
            description,
            action,
        }
    }
}
